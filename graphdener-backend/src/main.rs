extern crate regex;
// server.rs externs
extern crate indradb;
extern crate futures;
extern crate rmp_rpc;
extern crate tokio_core;

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate uuid;

use server::Echo;

use std::net::SocketAddr;

use futures::Stream;
use rmp_rpc::serve;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

mod server;
mod statics;
mod datastore;
mod io;
mod commands;

enum FileExtension
{
	Txt,
	Csv
}

enum ColumnSeparator
{
	Comma,
	Whitespace
}


fn main() 
{
	println!("running server...");
	let addr: SocketAddr = "127.0.0.1:54321".parse().unwrap();
    // Create a tokio event loop.
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Create a listener to listen for incoming TCP connections.
    let server = TcpListener::bind(&addr, &handle)
        .unwrap()
        .incoming()
        // Each time the listener finds a new connection, start up a server to handle it.
        .for_each(move |(stream, _addr)| {
            serve(stream, Echo, handle.clone())
        });

    // Run the server on the tokio event loop. This is blocking. Press ^C to stop
    core.run(server).unwrap();
}

