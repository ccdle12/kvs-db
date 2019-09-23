use crate::Result;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// TODO(ccdle12): currently using listening address as a string.
// Maybe should be a custom class or an existing address class.
/// Server for the Key/Value store.
pub struct KvsServer {
    listening_address: String,
}

impl KvsServer {
    pub fn new(listening_address: String) -> KvsServer {
        KvsServer { listening_address }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.listening_address)?;

        // BYTE array of 5 bytes.
        let mut buffer = [0_u8; 5];
        for stream in listener.incoming() {
            stream?.read(&mut buffer);
            println!("buffer: {:?}", String::from_utf8_lossy(&buffer));
        }

        Ok(())
    }
}
