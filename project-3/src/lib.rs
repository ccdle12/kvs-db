//! A library for a TCP client and server to run a write-ahead-log kv store.

extern crate failure;
#[macro_use]
extern crate failure_derive;

pub use client::KvsClient;
pub use error::{KvStoreError, Result};
pub use server::KvsServer;

mod client;
mod common;
mod error;
mod server;
