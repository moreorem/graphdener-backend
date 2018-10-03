use super::super::alg::circular;
use super::super::alg::forcedirected;
use super::super::io::filehandling;
use super::super::models::graph::GraphContainer;
use super::database;
use super::retrievals::get_pos;
use io::pattern::InitPattern;
use rand::prelude::*;
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

// TODO: Improve Randomizer with faster code
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
