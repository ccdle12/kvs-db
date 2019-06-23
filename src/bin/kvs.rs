#[macro_use]
extern crate clap;
use clap::App;

use kvs::KvStore;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // TEMP:
    let mut kv_store = KvStore::new();

    // Match on get.
    if let Some(key) = m.value_of("get") {
        kv_store.get(key.to_string());
    }

    // Match on rm.
    if let Some(key) = m.value_of("remove") {
        kv_store.remove(key.to_string());
    }

    // Match on set.
    if let Some(mut kv) = m.values_of("set") {
        let key = kv.next().unwrap().to_string();
        let value = kv.next().unwrap().to_string();

        kv_store.set(key, value);
    }

    // TEMP: just exits on eror for now.
    std::process::exit(1);
}
