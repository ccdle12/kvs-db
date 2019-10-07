use crate::common::{GetResponse, Request, SetResponse};
use crate::{KvStoreError, Result};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;
        // Creates reference to the same stream but handled independently.
        let writer = reader.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // 1. create a set request given the string.
        // 2. Serialize the request to the BufWriter using serde since it will
        //    be in json.
        // 3. Then we are going to flush, sending the buffer  to the writer.
        // 4. Read and deserialize the response from the reader.
        // 5. Send the response back to the client.
        let request = Request::Set { key, value };
        serde_json::to_writer(&mut self.writer, &request)?;
        self.writer.flush()?;
        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(s) => Err(KvStoreError::StringError(s)),
        }
    }

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
}
