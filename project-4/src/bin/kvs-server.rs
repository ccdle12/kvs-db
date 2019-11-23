#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate structopt;

use kvs::{KvStore, KvsEngine, KvsServer, NaiveThreadPool, Result, SledKvsEngine, ThreadPool};
use log::LevelFilter;
use std::env;
use std::net::SocketAddr;
use structopt::StructOpt;

/// Default listening address for the server.
const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

/// Runs the Key/Value Store server.
#[derive(Debug, StructOpt)]
#[structopt(name = "kvs-server", about = "The server cli for the kvs.")]
struct Opt {
    #[structopt(
        long,
        help = "Sets the listening adress",
        value_name = "IP:PORT",
        raw(default_value = "DEFAULT_LISTEN_ADDR"),
        parse(try_from_str)
    )]
    addr: SocketAddr,

    #[structopt(
        short = "e",
        long,
        help = "Sets the engine for the key value store",
        value_name = "ENGINE-NAME",
        raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,
}

// Wraps the enum as a clap enum. Implements the function ::variants().
// Allows the enum to be used in the struct to use the enum as a cli value.
arg_enum! {
  #[allow(non_camel_case_types)]
  #[derive(Debug)]
  enum Engine {
    kvs,
    sled,
  }
}

fn main() -> Result<()> {
    // Apparently uses the log crate as a facade.
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::from_args();
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Listening on {}", opt.addr);

    let pool = NaiveThreadPool::new(0)?;
    match opt.engine.unwrap_or(DEFAULT_ENGINE) {
        Engine::kvs => run_with_engine(KvStore::open(&env::current_dir()?)?, pool, opt.addr),
        Engine::sled => run_with_engine(SledKvsEngine::new()?, pool, opt.addr),
    }
}

/// Internal helper function that runs a KvsServer given the trait KvsEngine
/// and runs the server. Purely for readability in the main function.
fn run_with_engine<E: KvsEngine, P: ThreadPool>(
    engine: E,
    pool: P,
    addr: SocketAddr,
) -> Result<()> {
    let server = KvsServer::new(engine, pool);
    server.run(addr)
}
