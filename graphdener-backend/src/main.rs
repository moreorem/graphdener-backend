extern crate time;
extern crate indradb;

use std::fs::File;
use std::io::{BufRead, BufReader, Result, Error};
use time::PreciseTime;
use indradb::util;

fn main() {
    let a = import_f("/home/orestes/Workspace/src/github.com/moreorem/graphdener-backend/test.txt");
    println!("{:?}", a);
    println!("{:?}", util::generate_uuid_v1())
}

fn import_f(path: &str) -> Result<()>
{
	let file = File::open(path)?;
	let mut line_numbers:i32 = 0;

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

	for line in BufReader::new(file).lines()
	{
		edge_count += 1;
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