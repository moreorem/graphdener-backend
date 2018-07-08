extern crate time;
extern crate indradb;
extern crate regex;
extern crate pyo3;

// server.rs externs
extern crate futures;
extern crate rmp_rpc;
extern crate tokio_core;

use server::Echo;
use futures::future::Ok;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Error};
use time::PreciseTime;
use indradb::util;
use regex::Regex;
use regex::Captures;
use pyo3::prelude::*;


use std::net::SocketAddr;

use futures::Stream;
use rmp_rpc::{serve, Service, Value};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

mod server;

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
// fn main() -> PyResult<()> {
// 	let gil = Python::acquire_gil();
//     let py = gil.python();
//     let sys = py.import("sys")?;
//     let version: String = sys.get("version")?.extract()?;

// 	let locals = PyDict::new(py);
//     locals.set_item("os", py.import("os")?)?;
//     let user: String = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))?.extract()?;

//     println!("Hello {}, I'm Python {}", user, version);
//     Ok(())
//     // let a = import_edges("/home/orestes/Workspace/src/github.com/moreorem/graphdener-backend/test.txt");
//     // println!("{:?}", a);
//     // println!("{:?}", util::generate_uuid_v1())
// }




fn import_circles(path: &str) -> Result<()>
{
	let file = File::open(path)?;
	let mut line_numbers:i32 = 0;
	// Regular expression pattern for circles
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();


	for line in BufReader::new(file).lines()
	{

		line_numbers += 1;
		
	}
	println!("Number of lines: {}", line_numbers);
	Ok(())
}


fn import_edges(path: &str) -> Result<()>
{
	let file = File::open(path)?;
	let mut edge_count:i32 = 0;
	// Regular expression pattern for edge list
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();
	
	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
    		println!("Node {} is connected to node {}",
            		caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()
            		);
    		// TODO: Store each number as int to variables and send them to be created as vertices
    		// TODO: Make a list with those numbers that correspond to vertex indeces
    		// TODO: Create instantly that many UUIDs as the max of vertex indices
    		// caps.get(1).unwrap().as_str().parse::<i32>();
		}
		edge_count += 1;

		util::generate_uuid_v1();
		// TEST purposes. Stop early
		if edge_count > 100
		{
			break;
		}
		
	}
	Ok(())
}


#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn read_speed()
	{
	}
}