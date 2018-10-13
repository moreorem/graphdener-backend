use super::super::alg::circular;
use super::super::alg::forcedirected;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use rmp_rpc::Value;

// Improved import function to accept an array of paths
pub fn import_paths(
    path: &Vec<Value>,
    patterns: &Vec<Value>,
    // col_names: &Vec<(Value, Value)>,
) -> Result<Value, Value> {
    // Convert path and pattern to their respective types
    let converted_path = [path[0].as_str().unwrap(), path[1].as_str().unwrap()];
    let converted_pattern = [patterns[0].as_str().unwrap(), patterns[1].as_str().unwrap()];

    // Send information to filehandling
    filehandling::import_files(converted_path, converted_pattern);

    Ok(Value::from(true))
}

// Initializes new graph. Activates when we want to draw an extra graph on a new canvas
pub fn initialize_graph(container: &mut GraphContainer) -> Result<Value, Value> {
    let id = container.add_graph();
    Ok(Value::from(id))
}

pub fn populate_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    let id: u8 = id as u8;
    container.get_mut_graph(id).populate();
    Ok(Value::from(id))
}

pub fn kill_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    if let Ok(x) = container.remove_graph(id as u8) {
        Ok(Value::from(id))
    } else {
        Err(Value::from("Could not delete this graph"))
    }
}
