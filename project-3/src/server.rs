use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// Server for the Key/Value store.
pub struct KvsServer {}

impl KvsServer {
    pub fn new() -> KvsServer {
        KvsServer {}
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:443").unwrap();

        // BYTE array of 5 bytes.
        let mut buffer = [0_u8; 5];
        for stream in listener.incoming() {
            stream.unwrap().read(&mut buffer);
            println!("buffer: {:?}", String::from_utf8_lossy(&buffer));
        }
    }
}
