use models::graph::Graph;
use models::nodes::Node;

const MAX_DISPLACEMENT_SQUARED: f64 = 24.0;
//mut nodes: &mut Vec<Node>
pub fn force_directed(mut graph: &mut Graph, L: f32, K_r: f32, K_s: f32, delta_t: f32) -> ()
{
	// L = spring rest length
	// K_r = repulsive force constant
	// K_s = spring constant
	// delta_t = time step
	// let mut nodes = &mut graph.nodes;
	let mut nodes = graph.nodes.clone();
	let N = nodes.len();
	// initialize net forces
	println!("initializing net forces");
	for i in 0..N-1
	{
		nodes[i].force.set(0.0, 0.0);
	}
	
	// repulsion between all pairs
	repulsion(N, &mut nodes, K_r);

	// spring force between adjacent pairs
	spring(N, &mut nodes, K_r, L);

	// update positions
	update(N, &mut nodes, delta_t.into(), graph);
	println!("Done applying force directed algorithm.");
}

fn repulsion(N: usize, nodes: &mut Vec<Node>, repulsive_force: f32 ) -> ()
{
	// repulsion between all pairs
	for i1 in 0..N-2
	{
		let mut node1 = nodes[i1].clone();
		for i2 in i1+1..N-1
		{
 			let mut node2 = nodes[i2].clone();

 			let pos1 = node1.pos.get();
 			let pos2 = node2.pos.get();

 			let (dx, dy) = (pos2[0] - pos1[0], pos2[1] - pos1[1]);  //node2.pos.x - node1.pos.x;
 			if dx != 0.0 || dy != 0.0
 			{
				let distanceSquared = dx*dx + dy*dy;
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

fn spring(N: usize, nodes: &mut Vec<Node>, repulsive_force: f32, spring_rest_length: f32) -> ()
{
	for i1 in 0..N-1
	{
		let mut node1 = &mut nodes[i1].clone();
		for i2 in node1.neighbors.iter() //0..node1.neighbors.len() - 1
		{
	 		let mut node2 = &mut nodes[*i2];
	 		if i1 < *i2 
	 		{
	 			let pos1 = node1.pos.get();
 				let pos2 = node2.pos.get();

 				let (dx, dy) = (pos2[0] - pos1[0], pos2[1] - pos1[1]);

				if dx != 0.0 || dy != 0.0
				{
					let distance = (dx*dx + dy*dy).sqrt();
					let force = repulsive_force as f64 * ( distance - spring_rest_length as f64);
					let fx = force * dx / distance;
					let fy = force * dy / distance;
					
					let force1 = node1.force.get();
					let force2 = node2.force.get();

					node1.force.set(force1.0 + fx as f32, force1.1 + fy as f32);
					node2.force.set(force2.0 - fx as f32, force2.1 - fy as f32);
					
					// node1.force.x = node1.force.x + fx as f32;
					// node1.force.y = node1.force.y + fy as f32;
					// node2.force.x = node2.force.x - fx as f32;
					// node2.force.y = node2.force.y - fy as f32;
				}
			}
		}
	}
}

// Update positions
fn update(N: usize, nodes: &mut Vec<Node>, delta_t: f64, graph: &mut Graph) -> ()
{
	for i in 0..N-1
	{
		let mut node = &mut nodes[i];
		let force = node.force.get();
		let (dx, dy) = (delta_t * force.0 as f64, delta_t * force.1 as f64);

		let displacementSquared = dx*dx + dy*dy;
		if displacementSquared > MAX_DISPLACEMENT_SQUARED.into()
		{
	 		let s = (MAX_DISPLACEMENT_SQUARED / displacementSquared).sqrt();
	 		let dx = dx * s;
	 		let dy = dy * s;
	 	}

	 	let pos = node.pos.get();
	 	node.pos.set(pos[0] + dx, pos[1] + dy);
	 	graph.get_mut_node(i).pos.set(pos[0] + dx, pos[1] + dy);
 		println!("AFTER: {:?}", pos);
	}

}


// trait Aglorithm(&self) -> Vec<node>;