use std::io::{BufRead, BufReader, Result, Error};
use std::{fs::File, io};
use regex::Regex;
use io::importer::NodeRelations;
use statics;

pub fn import_edges(path: &str) -> io::Result<bool>
{
	let file = File::open(path)?;

	// TODO: add file format error handling

	// Regular expression pattern for edge list
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();
	let mut from_to: (u32, u32);

	// Create temporary collection to handle import
	let mut relation_table = NodeRelations::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			// Distinguish id columns and store them separately
			from_to = (caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
			relation_table.update(from_to);

			// TODO: Create Vertex Uuids and edges concurrently
    		// TODO: Make a list with those numbers that correspond to vertex indices
    		// TODO: Create instantly that many UUIDs as the max of vertex indices
    		// TODO: If necessary do calculations for analysis about graph from here
		}
		// for loop to use on another thread

	}
	// Generate a hashmap to translate imported ids to uuids
	relation_table.generate_id_map();
	// Create number of vertices as many as the variety of uuids
	relation_table.create_vertices(Some(&String::from("ego")));
	relation_table.initialize_spatial();
	Ok(true)
}

