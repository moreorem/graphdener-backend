use std::io::{BufRead, BufReader, Result, Error};
use std::{fs::File, io};
use regex::Regex;
use io::importer::EdgeImporter;
use statics;

// TODO: Create Vertex Uuids and edges concurrently
// TODO: Create instantly that many UUIDs as the max of vertex indices
// TODO: Receive column declarations about which part is what (ex. source,target, label, type, weight)
// TODO: If necessary do calculations for analysis about graph from here
// TODO: Add file format error handling
// TODO: If there is only one file, then generate ids from edge relations


pub fn import_edges(path: &str) -> io::Result<bool>
{
	let file = File::open(path)?;


	// Regular expression pattern for edge list
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();
	let mut from_to: (u32, u32);

	// Create temporary collection to handle import
	let mut relation_table = EdgeImporter::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			// Distinguish id columns and store them separately
			from_to = (caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
			relation_table.update(from_to);


		}
		// for loop to use on another thread
	}
	// Generate a hashmap to translate imported ids to uuids
	relation_table.generate_id_map();
	// Create number of vertices as many as the variety of uuids
	relation_table.create_vertices(Some(&String::from("ego")));
	initialize_spatial();
	Ok(true)
}

pub fn import_vertices(path: &str) -> io::Result<bool>
{
	let file = File::open(path)?;
	let re = Regex::new(r"(\d+)[ \t]+(\d+)[ \t]+(\d+)").unwrap();

	let mut id_label_type: (u32, String);

	// Create temporary collection to handle import
	let mut relation_table = NodeImporter::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			// Distinguish id columns and store them separately
			id_label_type = (caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
							caps.get(2).unwrap().as_str().unwrap(),
							caps.get(3).unwrap().as_str().unwrap());
			relation_table.update(id_label_type);
		}
		// for loop to use on another thread
	}
}
