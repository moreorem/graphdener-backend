extern crate regex;
// server.rs externs
extern crate graphdener;
extern crate futures;
extern crate rmp_rpc;
extern crate tokio_core;
extern crate rand; // TESTME: for random x,y
#[macro_use]
extern crate lazy_static;
#[macro_use]
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
    // TODO: Add console argument to be used as address
	println!("running server...");
	let addr: SocketAddr = "127.0.0.1:6000".parse().unwrap();
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

