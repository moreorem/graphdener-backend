use super::super::alg::{circular, forcedirected, random};
use models::graph::GraphContainer;
use rmp_rpc::Value;

pub fn apply_circular(id: u64, container: &mut GraphContainer) -> Result<Value, Value> {
    let id: u8 = id as u8;
    let n = container.get_graph(id).unwrap().count();
    let positions = circular::polygon(n as u32);
    container.get_mut_graph(id).set_positions(positions);

    Ok(Value::from(id))
}

pub fn apply_force_directed(
    id: u64,
    container: &mut GraphContainer,
    parameters: &Vec<Value>,
) -> Result<Value, Value> {
    let params = parameters.into_iter().map(|x| x.as_f64().unwrap() as f32);
    let p: Vec<f32> = params.collect();
    let id: u8 = id as u8;
    let mut graph = &mut container.get_mut_graph(id);
    forcedirected::force_directed(&mut graph, p[0], p[1], p[2], p[3]);

    Ok(Value::from(id))
}

pub fn apply_random_uniform(
    id: u8,
    container: &mut GraphContainer,
    spread_factor: u64,
    distribution: &str,
) -> Result<Value, Value> {
    let g = container.get_mut_graph(id);
    let min_distance = 0.002;
    match distribution {
        "uniform" => Ok(Value::from(random::random_uniform(
            g,
            min_distance,
            spread_factor as u8,
        ))),
        "normalized" => Ok(Value::from(random::random_normalized(
            g,
            min_distance,
            spread_factor as u8,
        ))),
        _ => Err(Value::from("Unknown distribution")),
    }
}
