extern crate log;
extern crate rustc_hex as hex;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate jsonrpc_core;
#[macro_use]
extern crate jsonrpc_macros;
extern crate chain;
extern crate db;
extern crate jsonrpc_http_server;
extern crate keys;
extern crate miner;
extern crate network;
extern crate p2p;
extern crate primitives;
extern crate script as global_script;
extern crate serialization as ser;
extern crate storage;
extern crate sync;
extern crate tokio_core;
extern crate verification;
extern crate script;

pub mod rpc_server;
pub mod v1;

pub use jsonrpc_core::{Compatibility, Error, MetaIoHandler};
pub use jsonrpc_http_server::tokio_core::reactor::Remote;

pub use jsonrpc_http_server::Server;
pub use rpc_server::start_http;
