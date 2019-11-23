use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::engines::KvsEngine;
use crate::error::KvStoreError;
use crate::thread_pool::ThreadPool;
use crate::Result;
use serde_json::Deserializer;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Server for the Key/Value store.
pub struct KvsServer<E: KvsEngine, P: ThreadPool> {
    engine: E,
    pool: P,
}

impl<E: KvsEngine, P: ThreadPool> KvsServer<E, P> {
    pub fn new(engine: E, pool: P) -> Self {
        KvsServer { engine, pool }
    }

    pub fn run<A: ToSocketAddrs>(self, addr: A) -> Result<()> {
        for stream in TcpListener::bind(addr)?.incoming() {
            // Cloning the engine because each thread will have an Arc reference
            // to the source engine.
            let engine = self.engine.clone();

            // Spawns threads for each incoming tcp request.
            self.pool.spawn(move || match stream {
                Ok(stream) => {
                    if let Err(_e) = handle_stream(engine, stream) {
                        panic!("Error on serving cleint: {}");
                    }
                }
                Err(e) => panic!("Connection failed: {}", e),
            })
        }

        Ok(())
    }
}

pub fn handle_stream<E: KvsEngine>(engine: E, stream: TcpStream) -> Result<()> {
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
            Request::Set { key, value } => send_response!(match engine.set(key, value) {
                Ok(_) => SetResponse::Ok(()),
                Err(e) => SetResponse::Err(e.to_string()),
            }),
            Request::Get { key } => send_response!(match engine.get(key) {
                Ok(r) => GetResponse::Ok(r),
                Err(_) => GetResponse::Err(KvStoreError::KeyNotFoundError.to_string()),
            }),
            Request::Remove { key } => send_response!(match engine.remove(key) {
                Ok(_) => RemoveResponse::Ok(()),
                Err(e) => RemoveResponse::Err(e.to_string()),
            }),
        }
    }

    Ok(())
}
