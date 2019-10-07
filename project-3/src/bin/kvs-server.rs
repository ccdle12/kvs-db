extern crate structopt;
use kvs::{KvStore, KvsServer, Result};
use std::env;
use structopt::StructOpt;

/// Default listening address for the server - 127.0.0.1:4000.
const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:4000";

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs-server", about = "The server cli for the kvs.")]
enum Opt {
    /// Runs the Key/Value Store server.
    #[structopt(name = "run")]
    Run {
        #[structopt(
            help = "The ip address and port as <IP:PORT> the server is serving from, runs the server on default settings if left blank."
        )]
        addr: Option<String>,
    },
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Run { addr } => match addr {
            Some(a) => {
                println!(" [x] Serving the KVS Server at {}", a);
                KvsServer::new(KvStore::open(&env::current_dir()?)?).run(a)?;
            }
            None => {
                println!(" [x] Serving the KVS Server at {}", DEFAULT_LISTEN_ADDR);
                KvsServer::new(KvStore::open(&env::current_dir()?)?)
                    .run(String::from(DEFAULT_LISTEN_ADDR))?
            }
        },
    };

    Ok(())
}
