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
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(help = "The value string of the key/value pair")]
        value: String,

        #[structopt(
            long,
            help = "The server address as IP:PORT",
            raw(default_value = "DEFAULT_LISTEN_ADDR")
        )]
        addr: String,
    },

    /// Gets a string value according to passed string key
    #[structopt(name = "get")]
    Get {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(
            long,
            help = "The server address as IP:PORT",
            raw(default_value = "DEFAULT_LISTEN_ADDR")
        )]
        addr: String,
    },

    /// Removes the string key/value pair according to the passed string key
    #[structopt(name = "rm")]
    Remove {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,

        #[structopt(
            long,
            help = "The server address as IP:PORT",
            raw(default_value = "DEFAULT_LISTEN_ADDR")
        )]
        addr: String,
    },
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Set { key, value, addr } => {
            KvsClient::connect(addr)?.set(key, value)?;

            std::process::exit(0);
        }

        Opt::Get { key, addr } => {
            let res = KvsClient::connect(addr)?.get(key);
            match res {
                Ok(v) => match v {
                    Some(v) => println!("{}", v),
                    None => println!("Key not found"),
                },
                Err(e) => println!("{}", e),
            }
            std::process::exit(0);
        }

        Opt::Remove { key, addr } => KvsClient::connect(addr)?.remove(key),
    }
}
