pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&self, key: String, val: String) {
        panic!()
    }

    pub fn get(&self, req: String) -> Option<String> {
        eprintln!("unimplemented");
        std::process::exit(1);
    }

    pub fn remove(&self, key: String) {
        panic!()
    }
}
