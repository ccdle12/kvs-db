#[macro_use]
extern crate clap;
use clap::App;

use kvs::KvStore;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // TEMP:
    let kv_store = KvStore::new();

    // Match on get.
    if let Some(key) = m.value_of("get") {
        kv_store.get(key.to_string());
    }

    // Match on rm.
    if let Some(key) = m.value_of("remove") {
        kv_store.remove(key.to_string());
    }

    // TEMP: just exits on eror for now.
    std::process::exit(1);
}
