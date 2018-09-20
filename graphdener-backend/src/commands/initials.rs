use rmp_rpc::Value;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use super::super::alg::circular;

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
    Ok(Value::from(id))
}

pub fn apply_circular(id: u64, container: &mut GraphContainer) -> Result<Value, Value>
{
    let id: u8 = id as u8;
    let n = container.get_graph(id).unwrap().count();
    let positions = circular::polygon(n as u32);
    let mut nodes = container.get_mut_graph(id).modify_nodes();
    for (i,node) in nodes.iter().enumerate()
    {
        let (x, y) = (positions[i].0, positions[i].1);
        node.clone().pos.set(x, y);
    }
    Ok(Value::from(id))
}

