use super::super::alg::circular;
use super::super::alg::forcedirected;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use super::database;
use super::retrievals::get_pos;
use io::filehandling::PatternFormat;
use rand::prelude::*;
use rmp_rpc::Value;
// Improved import function to accept an array of paths
pub fn import_paths(
    path: &Vec<Value>,
    patterns: &Vec<Value>,
    is_single_path: bool,
    col_names: &Vec<Value>,
) -> Result<Value, Value> {
    println!("Importing: {} and {}", path[0], path[1]);
    // Define paths of files to parse
    if is_single_path {
        let unified_list_path = path[0].as_str();
        let mut unified_pattern: String = String::from("");
        unified_pattern.push_str(&format!(r#"{}"#, patterns[0]));
        let format = [&unified_pattern[..]];
        // Call filehandling method
        filehandling::import_files(
            unified_list_path.unwrap(),
            "",
            PatternFormat::Unified(format),
            is_single_path,
        );
    } else {
        let node_list_path = path[0].as_str();
        let edge_list_path = path[1].as_str();
        // Define regular expressions
        let mut node_pattern: String = String::from("");
        let mut edge_pattern: String = String::from("");
        node_pattern.push_str(&format!("{}", patterns[0]));
        edge_pattern.push_str(&format!("{}", patterns[1]));
        let format = [&node_pattern[..], &edge_pattern[..]];

        // Call filehandling method
        filehandling::import_files(
            node_list_path.unwrap(),
            edge_list_path.unwrap(),
            PatternFormat::Dual(format),
            is_single_path,
        );
    }

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

pub fn apply_circular(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    println!("Applying Circular Distribution...");
    let id: u8 = id as u8;
    let n = container.get_graph(id).unwrap().count();
    let positions = circular::polygon(n as u32);
    container.get_mut_graph(id).set_positions(positions, None);

    Ok(Value::from(id))
}

pub fn apply_force_directed(
    id: u64,
    container: &mut GraphContainer,
    parameters: &Vec<Value>,
) -> Result<Value, Value> {
    let params = parameters.into_iter().map(|x| x.as_f64().unwrap() as f32);
    let p: Vec<f32> = params.collect();
    println!("Applying Force Directed Distribution...");
    println!("{}", id);
    let id: u8 = id as u8;
    let mut graph = &mut container.get_mut_graph(id);
    forcedirected::force_directed(&mut graph, p[0], p[1], p[2], p[3]);

    Ok(Value::from(id))
}

pub fn apply_random_pos(
    id: u64,
    container: &mut GraphContainer,
    min_distance: f64,
) -> Result<Value, Value> {
    fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powf(2.) + (p1.1 - p2.1).powf(2.)).sqrt()
    }
    let id: u8 = id as u8;
    let n = container.get_graph(id).unwrap().count();
    let mut rng = thread_rng();
    let mut positions = Vec::with_capacity(n);
    let spread = (n as f64).log10();
    println!("{}", spread);

    while positions.len() < n {
        let mut p1 = rng.gen::<(f64, f64)>();
        let mut p2 = rng.gen::<(f64, f64)>();
        p1 = (p1.0 * spread, p1.1 * spread);
        p2 = (p2.0 * spread, p2.1 * spread);

        if positions.iter().all(|&p2| distance(p1, p2) > min_distance) {
            positions.push(p1);
            positions.push(p2);
        }
    }
    container.get_mut_graph(id).set_positions(positions, None);
    Ok(Value::from(id))
}
