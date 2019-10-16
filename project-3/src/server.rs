use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::engines::KvsEngine;
use crate::Result;
use serde_json::Deserializer;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Server for the Key/Value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        TcpListener::bind(addr)?
            .incoming()
            .try_for_each(|s| self.handle_stream(s?))
    }

    pub fn handle_stream(&mut self, stream: TcpStream) -> Result<()> {
        let r = BufReader::new(&stream);
        let mut w = BufWriter::new(&stream);

        let request = Deserializer::from_reader(r).into_iter::<Request>();

        // Macro for sending reponses back over the tcp stream.
        macro_rules! send_response {
            ($response:expr) => {{
                let resp = $response;
                serde_json::to_writer(&mut w, &resp)?;
                w.flush()?;
            };};
        }

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
                Request::Remove { key } => send_response!(match self.engine.remove(key) {
                    Ok(_) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(e.to_string()),
                }),
            }
        }

        Ok(())
    }
}
