use rmp_rpc::Value;
use super::super::io::filehandling;
use super::super::containers::graph::{GraphContainer, Graph};

pub struct Initializer;

// Improved import function to accept an array of paths
pub fn import_paths(path: &Vec<Value>, patternN: &str, patternE: &str) -> Result<Value, Value>
{
    println!("{:?}", path);
    // Define path of edgelist
    let node_list_path = path[0].as_str();
    let edge_list_path = path[1].as_str();
    // Define regular expressions
    let mut node_pattern: String = String::from("");
    let mut edge_pattern: String = String::from("");
    node_pattern.push_str(&format!(r#"{}"#, patternN));
    edge_pattern.push_str(&format!(r#"{}"#, patternE));
    let format = [ &node_pattern[..], &edge_pattern[..] ];

    filehandling::import_files(node_list_path.unwrap(), edge_list_path.unwrap(), &format);
    Ok(Value::from("paths imported"))
}


pub fn initialize_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value>
{
	let id: u8 = id as u8;
    println!("{:?}", container);
	container.add_graph(id);
    Ok(Value::from("new graph created"))

}

