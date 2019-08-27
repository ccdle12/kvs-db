use std::net::TcpListener;

pub struct KvsServer {}

impl KvsServer {
    pub fn new() -> KvsServer {
        KvsServer {}
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:443").unwrap();

        for stream in listener.incoming() {
            println!("Hello, World");
        }
    }
}
