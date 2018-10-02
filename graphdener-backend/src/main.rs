// server.rs externs
extern crate futures;
extern crate graphdener;
extern crate graphdenerdb;
extern crate rmp_rpc;
extern crate tokio_core;

use futures::Stream;
use rmp_rpc::serve;
use server::Echo;
use std::net::SocketAddr;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use graphdener::models::graph::GraphContainer;

mod server;

fn main() {
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
            serve(stream, Echo(GraphContainer::default()), handle.clone())
        });

    // Run the server on the tokio event loop. This is blocking. Press ^C to stop
    core.run(server).unwrap();
}
