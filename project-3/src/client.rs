use crate::Result;
use std::net::{TcpStream, ToSocketAddrs};

// TODO(ccdle12): Currently storing server address as string.
// Maybe better as a built in class or custom class?
pub struct KvsClient {}

impl KvsClient {
    pub fn new() -> KvsClient {
        KvsClient {}
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> Result<()> {
        let mut stream = TcpStream::connect(addr)?;
        Ok(())
    }
}
