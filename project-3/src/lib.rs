//! A library for a TCP client and server to run a write-ahead-log kv store.

pub use server::KvsServer;

mod client;
mod server;
