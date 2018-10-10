use models::graph::Graph;
use models::nodes::Node;

// TODO: Improve speed using arrayfire or threads
const MAX_DISPLACEMENT_SQUARED: f32 = 56.0;
// L = spring rest length
// K_r = repulsive force constant
// K_s = spring constant
// delta_t = time step

pub fn force_directed(graph: &mut Graph, l: f32, k_r: f32, k_s: f32, deltat: f32) -> () {
    let mut nodes = graph.nodes.clone();

    // repulsion between all pairs
    repulsion(&mut nodes, k_r);
    // spring force between adjacent pairs
    spring(&mut nodes, k_s, l);

    // update positions
    update(&mut nodes, deltat, graph);
    println!("Done applying force directed algorithm.");
}

fn repulsion(mut nodes: &mut Vec<Node>, repulsive_force: f32) -> () {
    let n = nodes.len();
    // repulsion between all pairs
    for i1 in 0..n - 2 {
        // let mut node1 = &mut nodes[i1];
        for i2 in i1 + 1..n - 1 {
            // let mut node2 = &mut nodes[i2];

            let pos1 = nodes[i1].pos.get();
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
    let mut node1: Node;
    let mut node2: Node;
    let mut pos1: [f64; 2];
    let mut pos2: [f64; 2];
    let mut distance: f32;

    for i1 in 0..n - 1 {
        // node1 = nodes[i1].clone();
        for i2 in nodes[i1].clone().neighbors.iter() {
            // node2 = nodes[*i2].clone();
            if i1 < *i2 {
                pos1 = nodes[i1].pos.get();
                pos2 = nodes[*i2].pos.get();

                let (dx, dy) = ((pos2[0] - pos1[0]) as f32, (pos2[1] - pos1[1]) as f32);

                if dx != 0.0 || dy != 0.0 {
                    distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                    let force = spring_constant * (distance - spring_rest_length);
                    let fx = force * dx / distance;
                    let fy = force * dy / distance;

                    let force1 = nodes[i1].force.get();
                    let force2 = nodes[*i2].force.get();

                    nodes[i1].force.set(force1.0 + fx, force1.1 + fy);
                    nodes[*i2].force.set(force2.0 - fx, force2.1 - fy);
                }
            }
        }
    }
}

// Update positions
fn update(nodes: &mut Vec<Node>, deltat: f32, graph: &mut Graph) -> () {
    let n = nodes.len();
    for i in 0..n - 1 {
        let mut node = &mut nodes[i];
        let force = node.force.get();
        let (mut dx, mut dy) = (deltat * force.0, deltat * force.1);

        let displacement_squared = dx.powf(2.0) + dy.powf(2.0);
        if displacement_squared > MAX_DISPLACEMENT_SQUARED.into() {
            let s = (MAX_DISPLACEMENT_SQUARED / displacement_squared).sqrt();
            dx = dx * s;
            dy = dy * s;
        }

        let pos = node.pos.get();
        node.pos.set(pos[0] + dx as f64, pos[1] + dy as f64);
        graph
            .get_mut_node(i)
            .pos
            .set(node.pos.get()[0], node.pos.get()[1]);
    }
}
