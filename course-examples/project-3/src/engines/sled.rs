use super::KvsEngine;
use crate::Result;
use sled::Db;

pub struct SledKvsEngine {
    engine: Db,
    // engine: Tree,
}

impl SledKvsEngine {
    pub fn new() -> Result<SledKvsEngine> {
        Ok(SledKvsEngine {
            engine: Db::open("default_db")?,
        })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.engine.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self
            .engine
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.engine.remove(key.as_bytes())?;
        Ok(())
    }
}
