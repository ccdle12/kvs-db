use crate::common::{GetResponse, Request, SetResponse};
use crate::engines::KvsEngine;
use crate::error::KvStoreError;
use crate::Result;
use serde_json::Deserializer;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

// TODO(ccdle12): currently using listening address as a string.
// Maybe should be a custom class or an existing address class.
/// Server for the Key/Value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(s) => Ok(self.handle_stream(s)?),
                Err(e) => Err(KvStoreError::IOError(e)),
            };
        }

        Ok(())
    }

    pub fn handle_stream(&mut self, stream: TcpStream) -> Result<()> {
        // Reader and Writer buffer for reading tcp stream and writing back over
        // the stream.
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        // Deserialize the contents of buf reader to a request enum.
        let request = Deserializer::from_reader(reader).into_iter::<Request>();

        // Macro for sending reponses back over the tcp stream.
        macro_rules! send_response {
            ($response:expr) => {{
                let resp = $response;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
            };};
        }

        // Iterates over the serialized requests and handles each request type.
        for req in request {
            let req = req?;
            match req {
                Request::Set { key, value } => send_response!(match self.engine.set(key, value) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(e.to_string()),
                }),
                Request::Get { key } => send_response!(match self.engine.get(key) {
                    Ok(r) => GetResponse::Ok(r),
                    Err(e) => GetResponse::Err(e.to_string()),
                }),
                _ => send_response!(SetResponse::Ok(())),
            }
        }

        Ok(())
    }
}
