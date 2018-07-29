use std::io::{BufRead, BufReader, Result, Error};
use std::{fs::File, io};
use regex::Regex;
use super::relational::NodeRelations;
use uuid::Uuid;

pub fn import_edges(path: &str) -> io::Result<()>
{
	println!("Parsing file {}", path);

	let file = File::open(path)?;

	println!("{:?}", file);
	let mut edge_count:i32 = 0;
	// Regular expression pattern for edge list
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();
	let mut from_to: (i32, i32);

	let mut relation_table = NodeRelations::new();
	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			from_to = (caps.get(1).unwrap().as_str().parse::<i32>().unwrap(), caps.get(2).unwrap().as_str().parse::<i32>().unwrap());
			relation_table.update(from_to);
			// TODO: Create Vertex Uuids and edges concurrently
    		// TODO: Make a list with those numbers that correspond to vertex indices
    		// TODO: Create instantly that many UUIDs as the max of vertex indices
    		// TODO: If necessary do calculations for analysis about graph from here
		}
		edge_count += 1;
		
	}

	Ok(())
}


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

