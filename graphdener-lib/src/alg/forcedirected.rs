use models::graph::Graph;
use models::nodes::Node;

// TODO: Improve speed using arrayfire or threads
const MAX_DISPLACEMENT_SQUARED: f64 = 24.0;

// PENDING: Find where K_s is missing
pub fn force_directed(graph: &mut Graph, l: f32, k_r: f32, k_s: f32, delta_t: f32) -> () {
    // L = spring rest length
    // K_r = repulsive force constant
    // K_s = spring constant
    // delta_t = time step
    // let mut nodes = &mut graph.nodes;
    let mut nodes = graph.nodes.clone();
    let n = nodes.len();
    // initialize net forces
    println!("initializing net forces");
    for i in 0..n - 1 {
        nodes[i].force.set(0.0, 0.0);
    }

    // repulsion between all pairs
    repulsion(n, &mut nodes, k_r);

    // spring force between adjacent pairs
    spring(n, &mut nodes, k_s, l);

    // update positions
    update(n, &mut nodes, delta_t.into(), graph);
    println!("Done applying force directed algorithm.");
}

fn repulsion(n: usize, nodes: &mut Vec<Node>, repulsive_force: f32) -> () {
    // repulsion between all pairs
    for i1 in 0..n - 2 {
        let mut node1 = nodes[i1].clone();
        for i2 in i1 + 1..n - 1 {
            let mut node2 = nodes[i2].clone();

            let pos1 = node1.pos.get();
            let pos2 = node2.pos.get();

            let (dx, dy) = (pos2[0] - pos1[0], pos2[1] - pos1[1]); //node2.pos.x - node1.pos.x;
            if dx != 0.0 || dy != 0.0 {
                let distanceSquared = dx * dx + dy * dy;
                let distance = distanceSquared.sqrt();
                let force = repulsive_force as f64 / distanceSquared;
                let fx = force * dx / distance;
                let fy = force * dy / distance;

                let force1 = node1.force.get();
                let force2 = node2.force.get();

                node1.force.set(force1.0 - fx as f32, force1.1 - fy as f32);
                node2.force.set(force2.0 + fx as f32, force2.1 + fy as f32);
            }
        }
    }
}

fn spring(n: usize, nodes: &mut Vec<Node>, repulsive_force: f32, spring_rest_length: f32) -> () {
    for i1 in 0..n - 1 {
        let mut node1 = &mut nodes[i1].clone();
        for i2 in node1.neighbors.iter() {
            let mut node2 = &mut nodes[*i2];
            if i1 < *i2 {
                let pos1 = node1.pos.get();
                let pos2 = node2.pos.get();

                let (dx, dy) = (pos2[0] - pos1[0], pos2[1] - pos1[1]);

                if dx != 0.0 || dy != 0.0 {
                    let distance = (dx * dx + dy * dy).sqrt();
                    let force = repulsive_force as f64 * (distance - spring_rest_length as f64);
                    let fx = force * dx / distance;
                    let fy = force * dy / distance;

                    let force1 = node1.force.get();
                    let force2 = node2.force.get();

                    node1.force.set(force1.0 + fx as f32, force1.1 + fy as f32);
                    node2.force.set(force2.0 - fx as f32, force2.1 - fy as f32);
                }
            }
        }
    }
}

// Update positions
fn update(n: usize, nodes: &mut Vec<Node>, delta_t: f64, graph: &mut Graph) -> () {
    for i in 0..n - 1 {
        let mut node = &mut nodes[i];
        let force = node.force.get();
        let (mut dx, mut dy) = (delta_t * force.0 as f64, delta_t * force.1 as f64);

        let displacement_squared = dx * dx + dy * dy;
        if displacement_squared > MAX_DISPLACEMENT_SQUARED.into() {
            let s = (MAX_DISPLACEMENT_SQUARED / displacement_squared).sqrt();
            dx = dx * s;
            dy = dy * s;
        }

        let pos = node.pos.get();
        node.pos.set(pos[0] + dx, pos[1] + dy);
        graph.get_mut_node(i).pos.set(pos[0] + dx, pos[1] + dy);
    }
}
