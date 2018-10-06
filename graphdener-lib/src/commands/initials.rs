use super::super::alg::circular;
use super::super::alg::forcedirected;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use io::pattern::InitPattern;
use rmp_rpc::Value;

// Improved import function to accept an array of paths
pub fn import_paths(
    path: &Vec<Value>,
    patterns: &Vec<Value>,
    col_names: &Vec<(Value, Value)>,
) -> Result<Value, Value> {
    // Convert received rpc values to the corresponding data types
    let mut names: Vec<(&str, &str)> = Vec::with_capacity(col_names.len());
    for pair in col_names.iter() {
        names.push((pair.0.as_str().unwrap(), pair.1.as_str().unwrap()));
    }

    let converted_path = path.iter().map(|x| x.as_str().unwrap()).collect();
    let converted_pattern = patterns.iter().map(|x| x.as_str().unwrap()).collect();

    // Store import info to the corresponding struct
    let import_info = InitPattern::create_import(converted_path, converted_pattern, names);
    filehandling::import_files(import_info);

    Ok(Value::from("paths imported"))
}

// Initializes new graph. Activates when we want to draw an extra graph on a new canvas
pub fn initialize_graph(container: &mut GraphContainer) -> Result<Value, Value> {
    let next_id = container.get_next_id();
    container.add_graph(next_id);
    Ok(Value::from(next_id))
}

pub fn populate_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    let id: u8 = id as u8;
    container.get_mut_graph(id).populate();
    Ok(Value::from(id))
}

// PENDING: Implement killgraph
pub fn kill_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    Ok(Value::from("ok"))
}
