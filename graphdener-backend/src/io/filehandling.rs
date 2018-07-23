use std::io::{BufRead, BufReader, Result, Error};
use std::fs::File;
use regex::Regex;


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

		if edge_count > 100
		{
			break;
		}
		
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

