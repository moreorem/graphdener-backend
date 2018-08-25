use uuid::Uuid;
use std::collections::HashMap;
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

pub fn import_vertices(path: &str, uuid_map: &mut HashMap<u32, Uuid>) -> io::Result<bool>
{
	let file = File::open(path)?;
	// TODO: Make regular expression customizable according to frontend input
	let re = Regex::new(r#"^(?P<id>\d+)\s+"(?P<label>[^"]*)"\s+"(?P<type>[^"]*)""#).unwrap();
	// Create temporary collection to handle import
	let mut relation_table = NodeImporter::new();
	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		let line_string = &String::from(line.unwrap());
		let caps = re.captures(line_string).unwrap();  // FIXME: Backend panics when unwrapping empty value
	
		// Distinguish id columns and store them separately
		let id_label_type: (u32, &str, &str) = (caps["id"].parse::<u32>().expect("expected digit"),	
												&caps["label"], 
												&caps["type"]);
		relation_table.update(id_label_type);
		
	}
	
	// FIXME: restore result to single and save uuid_map into struct outside of this file
	*uuid_map = relation_table.generate_id_map().unwrap();
	
	relation_table.generate_type_map();
	relation_table.create_vertices();
	
	initialize_spatial();

	Ok(true)
}

pub fn import_edges(path: &str, uuid_map: &HashMap<u32, Uuid>) -> io::Result<bool>
{
	let file = File::open(path)?;
	let from_to: (u32, u32);
	let t = 0;
	// Regular expression pattern for edge list
	// TODO: Make regular expression customizable according to frontend input
	// let re = Regex::new(r"(\d+)[ \t]+(\d+)").unwrap();

	let re = Regex::new(r#"^(?P<id>\d+)\s+(?P<source>\d+)\s+(?P<target>\d+)\s+"(?P<label>[^"]*)"\s+"(?P<type>[^"]*)"\s+(?P<weight>\d+)"#).unwrap();	
	// Create temporary collection to handle import
	let mut relation_table = EdgeImporter::new();

	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		let line_string = &String::from(line.unwrap());
		let caps = re.captures(line_string).unwrap(); // FIXME: Backend panics when unwrapping empty value

		// Distinguish id columns and store them separately
		let from_to: (u32, u32, u32, &str, &str, u8) = (caps["id"].parse::<u32>().expect("expected digit"),
														caps["source"].parse::<u32>().expect("expected digit"),
														caps["target"].parse::<u32>().expect("expected digit"),
														&caps["label"], &caps["type"],
														caps["weight"].parse::<u8>().expect("expected digit"));
		relation_table.update(from_to);
		
		// for loop to use on another thread
	}
	// Generate a hashmap to translate imported ids to uuids
	// relation_table.generate_id_map();
	// Create number of vertices as many as the variety of uuids
	relation_table.create_edges(&uuid_map);
	Ok(true)
}
