extern crate structopt;

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

fn main() {
    match Opt::from_args() {
        Opt::Set { key, value } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Opt::Get { key } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Opt::Remove { key } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
    }
}
