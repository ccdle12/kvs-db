# Key Value Store

## Background

A key value store project. Currently implements a write ahead log with a client
server cli tool found at [bin](src/bin/)

Further reading:

<https://en.wikipedia.org/wiki/Write-ahead_logging>

## Project Structure

- [course examples](course-examples/) - My archived entries for each project in the PingCap course
- [src](src/) - The main area of the project
- [tests](tests/) - Integration tests

## Modules and Files

- [bin](src/bin/) - Contains the cli files
- [engines](src/engines/) - Key Value store implementation and trait for the DB Engine
- [client](src/client.rs/) - Client API implementation, used in `kvs-client` cli
- [common](src/common.rs/) - Enums used for serialization between DB and request
- [error](src/error.rs/) - Errors for the KVS project
- [lib](src/lib.rs/) - Entry point for the project as a library 
- [server](src/server.rs/) - Server API implementation, used in `kvs-server` cli

## Tests

```sh
cargo test
```
