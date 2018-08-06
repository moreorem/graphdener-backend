use std::io::{BufRead, BufReader, Result, Error};
use std::{fs::File, io};
use regex::Regex;
use io::relations::NodeRelations;
use statics;

pub fn import_edges(path: &str) -> io::Result<bool>
{
	println!("Parsing file {}", path);

	let file = File::open(path)?;

	println!("{:?}", file);
	let mut edge_count:i32 = 0;
	// Regular expression pattern for edge list
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();
	let mut from_to: (u32, u32);

	let mut relation_table = NodeRelations::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			from_to = (caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
			relation_table.update(from_to);


			// add id.0 and id.1 to map if they don't exist
			// id_uuid_table.insert(from_to.0, next_uuid());

			// TODO: Create Vertex Uuids and edges concurrently
    		// TODO: Make a list with those numbers that correspond to vertex indices
    		// TODO: Create instantly that many UUIDs as the max of vertex indices
    		// TODO: If necessary do calculations for analysis about graph from here
		}
		edge_count += 1;
		// for loop to use on another thread

	}
	relation_table.generate_id_map();
	relation_table.create_vert_edges(Some(&String::from("pea")));

	Ok(true)
}


fn import_circles(path: &str) -> Result<bool>
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
	Ok(true)
}

fn import_communities(path: &str) -> Result<bool>
{
	Ok(true)
}