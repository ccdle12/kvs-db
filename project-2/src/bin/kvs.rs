extern crate structopt;
use kvs::{KvStore, Result};
use std::env;
use structopt::StructOpt;

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
    },

    /// Gets a string value according to passed string key
    #[structopt(name = "get")]
    Get {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,
    },

    /// Removes the string key/value pair according to the passed string key
    #[structopt(name = "rm")]
    Remove {
        #[structopt(help = "The key string of the key/value pair")]
        key: String,
    },
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Set { key, value } => {
            let mut kv_store = KvStore::open(env::current_dir()?)?;
            kv_store.set(key, value)?;

            std::process::exit(0);
        }
        Opt::Get { key } => {
            let kv_store = KvStore::open(env::current_dir()?)?;
            eprintln!("unimplemented");

            std::process::exit(1);
        }
        Opt::Remove { key } => {
            let mut kv_store = KvStore::open(env::current_dir()?)?;
            let _result = kv_store.remove(key);

            std::process::exit(0);
        }
    }
}
