use std::io::prelude::*;
use std::net::TcpStream;

pub struct KvsClient {}

impl KvsClient {
    pub fn new() -> KvsClient {
        KvsClient {}
    }

    pub fn connect(&self) {
        let mut stream = TcpStream::connect("127.0.0.1:443").unwrap();

        let msg = b"Hello";
        stream.write(msg).unwrap();
    }
}
