#![deny(missing_docs)]
//! A simple Key/Value store.

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

pub use error::{KvStoreError, Result};
pub use kv::KvStore;

mod error;
mod kv;
