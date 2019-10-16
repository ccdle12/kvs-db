extern crate structopt;
use kvs::{KvsClient, Result};
use structopt::StructOpt;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:4000";

// TODO (ccdle12):
// 1. Implement command to print the version

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs", about = "A Key/Value store CLI")]
enum Opt {
    /// Sets a string key/value pair
    #[structopt(name = "set")]
    Set {
        #[structopt(short = "k", help = "The key string of the key/value pair")]
        key: String,

        #[structopt(short = "v", help = "The value string of the key/value pair")]
        value: String,

        #[structopt(short = "a", help = "The server address as IP:PORT")]
        address: Option<String>,
    },

    /// Gets a string value according to passed string key
    #[structopt(name = "get")]
    Get {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(short = "a", help = "The server address as IP:PORT")]
        address: Option<String>,
    },

    /// Removes the string key/value pair according to the passed string key
    #[structopt(name = "rm")]
    Remove {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(short = "a", help = "The server address as IP:PORT")]
        address: Option<String>,
    },
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Set {
            key,
            value,
            address,
        } => {
            let addr: String = parse_server_address(address);
            KvsClient::connect(addr)?.set(key, value)?;
            std::process::exit(0);
        }
        Opt::Get { key, address } => {
            let addr: String = parse_server_address(address);
            let res = KvsClient::connect(addr)?.get(key);
            match res {
                Ok(v) => println!("{}", v.unwrap()),
                Err(_) => println!("key not found"),
            }
            std::process::exit(0);
        }
        Opt::Remove { key, address } => {
            let addr: String = parse_server_address(address);
            let res = KvsClient::connect(addr)?.remove(key);
            match res {
                Err(e) => println!("{}", e),
                _ => println!(""),
            }
            std::process::exit(0);
        }
    }

    fn parse_server_address(address: Option<String>) -> String {
        match address {
            Some(a) => return a,
            None => return String::from(DEFAULT_LISTEN_ADDR),
        };
    }
}
