use super::super::alg::{circular, forcedirected, random};
use models::graph::GraphContainer;
use rmp_rpc::Value;

pub fn apply_circular(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    println!("Applying Circular Distribution...");
    let id: u8 = id as u8;
    let n = container.get_graph(id).unwrap().count();
    let positions = circular::polygon(n as u32);
    // container.get_mut_graph(id).set_positions(positions);

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

pub fn apply_random_pos(id: u8, container: &mut GraphContainer) -> Result<Value, Value> {
    let g = container.get_mut_graph(id);

    let min_distance = 0.002;
    Ok(Value::from(random::random_pos(g, min_distance)))
}
