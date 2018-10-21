use models::graph::Graph;
use models::nodes::Node;
use std::thread;
use std::time::Duration;
use std::time::Instant;

// TODO: Improve speed using arrayfire or threads
const MAX_DISPLACEMENT_SQUARED: f32 = 56.0;
// L = spring rest length
// K_r = repulsive force constant
// K_s = spring constant
// delta_t = time step

pub fn force_directed(graph: &mut Graph, l: f32, k_r: f32, k_s: f32, deltat: f32) -> () {
    // let instant = Instant::now();
    let mut nodes = graph.nodes.clone();

    // repulsion between all pairs
    repulsion(&mut nodes, k_r);
    // spring force between adjacent pairs
    spring(&mut nodes, k_s, l);

    // update positions
    update(nodes, deltat, graph);
    // println!("{:?}", instant.elapsed());
}

fn repulsion(mut nodes: &mut Vec<Node>, repulsive_force: f32) -> () {
    let n = nodes.len();
    // repulsion between all pairs
    for i1 in 0..n - 2 {
        let pos1 = nodes[i1].pos.get();
        for i2 in i1 + 1..n - 1 {
            let pos2 = nodes[i2].pos.get();

            let (dx, dy) = ((pos2[0] - pos1[0]) as f32, (pos2[1] - pos1[1]) as f32);
            if dx != 0.0 || dy != 0.0 {
                let distanceSquared = dx.powf(2.0) + dy.powf(2.0);
                let distance = distanceSquared.sqrt();

                let force = repulsive_force / distanceSquared;
                let fx = force * dx / distance;
                let fy = force * dy / distance;

                let force1 = nodes[i1].force.get();
                let force2 = nodes[i2].force.get();

                nodes[i1].force.set(force1.0 - fx, force1.1 - fy);
                nodes[i2].force.set(force2.0 + fx, force2.1 + fy);
            }
        }
    }
}

// Spring force between adjactent pairs
fn spring(nodes: &mut Vec<Node>, spring_constant: f32, spring_rest_length: f32) -> () {
    let n = nodes.len();
    let mut pos1: [f64; 2];
    let mut pos2: [f64; 2];
    let mut distance: f32;

    for i1 in 0..n - 1 {
        pos1 = nodes[i1].pos.get();
        if nodes[i1].neighbors.len() > 0 {
            for j in 0..nodes[i1].neighbors.len() - 1 {
                let i2 = nodes[i1].neighbors[j];
                pos2 = nodes[i2].pos.get();
                if i1 < i2 {
                    let (dx, dy) = ((pos2[0] - pos1[0]) as f32, (pos2[1] - pos1[1]) as f32);
                    if dx != 0.0 || dy != 0.0 {
                        distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                        let force = spring_constant * (distance - spring_rest_length);
                        let fx = force * dx / distance;
                        let fy = force * dy / distance;

                        let force1 = nodes[i1].force.get();
                        let force2 = nodes[i2].force.get();

                        nodes[i1].force.set(force1.0 + fx, force1.1 + fy);
                        nodes[i2].force.set(force2.0 - fx, force2.1 - fy);
                    }
                }
            }
        }
    }
}

// Update positions
fn update(mut nodes: Vec<Node>, deltat: f32, graph: &mut Graph) -> () {
    for (i, node) in nodes.iter_mut().enumerate() {
        let force = node.force.get();
        let (mut dx, mut dy) = (deltat * force.0, deltat * force.1);
        let displacement_squared = dx.powf(2.0) + dy.powf(2.0);
        if displacement_squared > MAX_DISPLACEMENT_SQUARED.into() {
            let s = (MAX_DISPLACEMENT_SQUARED / displacement_squared).sqrt();
            dx = dx * s;
            dy = dy * s;
        }
        graph
            .get_mut_node(i)
            .pos
            .set(node.pos.get()[0] + dx as f64, node.pos.get()[1] + dy as f64);
    }
}
