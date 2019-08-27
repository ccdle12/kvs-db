extern crate structopt;
use kvs::KvsServer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs-server", about = "The server cli for the kvs.")]
enum Opt {
    /// Runs the Key/Value Store server.
    #[structopt(name = "run")]
    Run {},
}

fn main() {
    match Opt::from_args() {
        Opt::Run {} => {
            println!(" [x] Running the KVS Server...");
            let kvs_server = KvsServer::new();
            kvs_server.run();
        }
    }
}
