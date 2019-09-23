extern crate structopt;
use kvs::{KvsServer, Result};
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
        Opt::Run { addr } => {
            println!(" [x] Running the KVS Server...");
            match addr {
                Some(a) => KvsServer::new(a).run()?,
                None => KvsServer::new(String::from(DEFAULT_LISTEN_ADDR)).run()?,
            }
        }
    };

    Ok(())
}
