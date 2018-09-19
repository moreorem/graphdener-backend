use rmp_rpc::Value;
use super::super::io::filehandling;
use super::super::containers::graph::GraphContainer;

// Improved import function to accept an array of paths
pub fn import_paths(path: &Vec<Value>, patternN: &str, patternE: &str) -> Result<Value, Value>
{
    println!("Importing: {} and {}", path[0], path[1]);
    // PENDING: Handle possibility of having only edgelist file
    // Define paths of files to parse
    let node_list_path = path[0].as_str();
    let edge_list_path = path[1].as_str();
    // Define regular expressions
    let mut node_pattern: String = String::from("");
    let mut edge_pattern: String = String::from("");
    node_pattern.push_str(&format!(r#"{}"#, patternN));
    edge_pattern.push_str(&format!(r#"{}"#, patternE));
    let format = [ &node_pattern[..], &edge_pattern[..] ];

    // Call filehandling method
    filehandling::import_files(node_list_path.unwrap(), edge_list_path.unwrap(), &format);
    Ok(Value::from("paths imported"))
}

// Initializes new graph. Activates when we want to draw an extra graph on a new canvas
pub fn initialize_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value>
{
	let id: u8 = id as u8;
	container.add_graph(id);
    Ok(Value::from(id))
}

pub fn populate_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value>
{
    let id: u8 = id as u8;
    container.get_mut_graph(id).populate();
    println!("{:?}", container.0.get(&id)); // TESTME: are nodes full?
    Ok(Value::from(id))
}