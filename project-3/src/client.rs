use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::{KvStoreError, Result};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Key Value store client that reads and writes to a Key Value store server.
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connects to a server given an address.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;

        // Creates reference to the same stream but handled independently.
        let writer = reader.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    /// Sets a key value pair at the server.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set { key, value };

        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;

        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(s) => Err(KvStoreError::StringError(s)),
        }
    }

    /// Get a value according to a key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let request = Request::Get { key };

        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;

        let res = GetResponse::deserialize(&mut self.reader)?;
        match res {
            GetResponse::Ok(r) => Ok(r),
            GetResponse::Err(s) => Err(KvStoreError::StringError(s)),
        }
    }

    /// Removes a kv pair.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove { key };

        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;

        let res = RemoveResponse::deserialize(&mut self.reader)?;
        match res {
            RemoveResponse::Ok(r) => Ok(r),
            RemoveResponse::Err(e) => Err(KvStoreError::StringError(e)),
        }
    }
}
