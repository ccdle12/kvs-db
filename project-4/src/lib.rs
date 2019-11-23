//! A library for a TCP client and server to run a write-ahead-log kv store.

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
extern crate sled;

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvStoreError, Result};
pub use server::KvsServer;
pub use thread_pool::{NaiveThreadPool, ThreadPool};

mod client;
mod common;
mod engines;
mod error;
mod server;
pub mod thread_pool;
