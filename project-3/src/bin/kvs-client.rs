extern crate structopt;
use std::env;
use structopt::StructOpt;

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

fn main() {
    match Opt::from_args() {
        Opt::Set {
            key,
            value,
            address,
        } => {
            // TODO (ccdle12):
            // 1. Parse the address and return error if not an address.
            // 2. Do this for all instances of address.
            // 3. Address needs to be either IPV4 or IPV6 compatible.
            let server_address = parse_server_address(address);

            std::process::exit(0);
        }
        Opt::Get { key, address } => {
            std::process::exit(0);
        }
        Opt::Remove { key, address } => {
            std::process::exit(0);
        }
    }
}

fn parse_server_address(address: Option<String>) -> String {
    match address {
        Some(a) => return a,
        None => return String::from("127.0.0.1:4000"),
    };
}
