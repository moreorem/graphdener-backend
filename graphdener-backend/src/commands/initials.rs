use super::super::alg::circular;
use super::super::alg::forcedirected;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use super::retrievals::get_pos;
use rand::distributions::{Poisson, Standard, StudentT, Uniform};
use rand::prelude::*;
use rmp_rpc::Value;

// Improved import function to accept an array of paths
pub fn import_paths(path: &Vec<Value>, patternN: &str, patternE: &str) -> Result<Value, Value> {
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
    let format = [&node_pattern[..], &edge_pattern[..]];

    // Call filehandling method
    filehandling::import_files(node_list_path.unwrap(), edge_list_path.unwrap(), &format);
    Ok(Value::from("paths imported"))
}

// Initializes new graph. Activates when we want to draw an extra graph on a new canvas
// FIXME: Make a check on whether receiving an already existing id
pub fn initialize_graph(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    let id: u8 = id as u8;
    container.add_graph(id);
    Ok(Value::from(id))
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
    let id: u8 = id as u8;
    let mut graph = &mut container.get_mut_graph(id);
    forcedirected::force_directed(&mut graph, p[0], p[1], p[2], p[3]);

    Ok(Value::from(id))
}

// pub fn apply_random_pos(id: u64, container: &mut GraphContainer) -> Result<Value, Value>
// {
//     println!("Applying Random Distribution...");
//     let id: u8 = id as u8;
//     // Call Rng
//     let mut rng = thread_rng();
//     let mut x: f64;
//     let mut y: f64;

//     let n = container.get_graph(id).unwrap().count();
//     let mut positions: Vec<(f64, f64)> = Vec::with_capacity(n);

//     let positions = thread_rng().sample_iter(&Standard).take(n)
//                              .collect::<Vec<(f64, f64)>>();

//     container.get_mut_graph(id).set_positions(positions, None);

//     Ok(Value::from(id))
// }

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

    while positions.len() < n {
        let mut p1 = rng.gen::<(f64, f64)>();
        let mut p2 = rng.gen::<(f64, f64)>();
        p1 = (p1.0 * 10.0, p1.1 * 10.0);
        p2 = (p2.0 * 10.0, p2.1 * 10.0);

        if positions.iter().all(|&p2| distance(p1, p2) > min_distance) {
            positions.push(p1);
            positions.push(p2);
        }
    }
    container.get_mut_graph(id).set_positions(positions, None);
    Ok(Value::from(id))
}
