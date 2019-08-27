use std::io::prelude::*;
use std::net::TcpStream;

pub struct KvsClient {}

impl KvsClient {
    pub fn new() -> KvsClient {
        KvsClient {}
    }

    pub fn connect(&self) {
        let mut stream = TcpStream::connect("127.0.0.1:443").unwrap();

        stream.write(&[3]).unwrap();
    }
}
