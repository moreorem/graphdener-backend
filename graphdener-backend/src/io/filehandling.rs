use uuid::Uuid;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::{fs::File, io};
use regex::{Regex, Captures};
use io::importer::{initialize_spatial, EdgeImporter, NodeImporter};
use statics;

// TODO: Create Vertex Uuids and edges concurrently
// TODO: Create instantly that many UUIDs as the max of vertex indices
// TODO: Receive column declarations about which part is what (ex. source,target, label, type, weight)
// TODO: If necessary do calculations for analysis about graph from here
// TODO: Add file format error handling
// TODO: If there is only one file, then generate ids from edge relations

// PENDING: Use it!
enum LineType<'a> {
	NodeLine(u32, &'a str, &'a str),
	EdgeLine(u32, u32, u32, &'a str, &'a str, u8),
}

enum Importer {
	NodeImporter,
	EdgeImporter
}


fn process_n_line(line: String, caps: &Captures, relation_table: &mut NodeImporter ) -> ()
{
	let id_label_type: (u32, &str, &str) = (caps["id"].parse::<u32>().expect("expected digit"),	
										&caps["label"], 
										&caps["type"]);
	relation_table.update(id_label_type);
}

fn process_e_line(line: String, caps: &Captures, relation_table: &mut EdgeImporter ) -> ()
{
	let from_to: (u32, u32, u32, &str, &str, u8) = (caps["id"].parse::<u32>().expect("expected digit"),
														caps["source"].parse::<u32>().expect("expected digit"),
														caps["target"].parse::<u32>().expect("expected digit"),
														&caps["label"], &caps["type"],
														caps["weight"].parse::<u8>().expect("expected digit"));
	relation_table.update(from_to);
}

fn import_vertices(path: &str, uuid_map: &mut HashMap<u32, Uuid>) -> io::Result<bool>
{
	let file = File::open(path).expect("There was a problem reading the vertices file.");
	// TODO: Make regular expression customizable according to frontend input
	let re = Regex::new(r#"^(?P<id>\d+)\s+"(?P<label>[^"]*)"\s+"(?P<type>[^"]*)""#).unwrap();
	// Create temporary collection to handle import
	let mut relation_table = NodeImporter::new();
	println!("Parsing file {}", path);

	for line in BufReader::new(file).lines()
	{
		let line_text = line.unwrap();
		let line_string = String::from(line_text);
		let caps: Captures;
		let id_label_type: (u32, &str, &str);
		
		// Handle lines that are empty or do not fit in the expression
		if let Some(x) = re.captures(&line_string) {
			caps = re.captures(&line_string).unwrap();
			// Parse the line into the relation table
			process_n_line(line_string.to_owned(), &caps, &mut relation_table);
		}
		else {
			continue;
		}
	}
	
	*uuid_map = relation_table.generate_id_map().unwrap();
	
	relation_table.generate_type_map();
	relation_table.create_vertices();
	
	initialize_spatial();

	Ok(true)
}

fn import_edges(path: &str, uuid_map: &HashMap<u32, Uuid>) -> io::Result<bool>
{
	let file = File::open(path).expect("There was a problem reading the edges file.");
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
		let line_text = line.unwrap();
		let line_string = &String::from(line_text);
		let caps;

		// Handle lines that are empty or do not fit in the expression
		if let Some(x) = re.captures(line_string) {
			caps = re.captures(line_string).unwrap();	
			process_e_line(line_string.to_owned(), &caps, &mut relation_table);

		}
		else {
			continue;
		}
	}
	// Create number of vertices as many as the variety of uuids
	relation_table.create_edges(&uuid_map);
	Ok(true)
}

pub fn import_files(vert_path: &str, edge_path: &str) -> Result<(), &'static str>
{
	let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
	let e: bool;
	let v: bool;

	// Handle the possibility of not setting a node filepath
    v = import_vertices(vert_path, &mut uuid_map).unwrap();
    e = import_edges(edge_path, &uuid_map).unwrap();
    if e && v {
    	Ok(())
    }
    else {
    	Err("There was an error with one or both of the files")
    }
}