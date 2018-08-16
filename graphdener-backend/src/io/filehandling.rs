use std::io::{BufRead, BufReader, Result, Error};
use std::{fs::File, io};
use regex::Regex;
use io::importer::{initialize_spatial, EdgeImporter, NodeImporter};
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
	let from_to: (u32, u32);

	// Regular expression pattern for edge list
	// TODO: Make regular expression customizable according to frontend input
	let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();

	// Create temporary collection to handle import
	let mut relation_table = EdgeImporter::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		for caps in re.captures_iter(&String::from(line.unwrap())) 
		{
			// Distinguish id columns and store them separately
			let from_to = (caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
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
	// TODO: Make regular expression customizable according to frontend input
	let re = Regex::new(r#"^(?P<id>\d+)\s+"(?P<label>[^"]*)"\s+"(?P<type>[^"]*)""#).unwrap();
	// Create temporary collection to handle import
	let mut relation_table = NodeImporter::new();
	println!("{:?}", re);
	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		let line_string = &String::from(line.unwrap());
		let caps = re.captures(line_string).unwrap();
		// TODO: Cleanup
		// TODO: Set elements into tuple

		// Distinguish id columns and store them separately
		let id_label_type: (u32, &str, &str) = (caps["id"].parse::<u32>().expect("expected digit"),	
												&caps["label"], 
												&caps["type"]);
		relation_table.update(id_label_type);
		
	}
	relation_table.generate_id_map();

	relation_table.generate_type_map();
	relation_table.create_vertices();
	Ok(true)
}
