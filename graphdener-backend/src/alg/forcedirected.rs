use graphdener::Vertex;

struct Position {
	x: f64,
	y: f64
}

struct Force {
	x: f32,
	y: f32,
}

pub struct Node {
	pos: Position,
	force: Force,
	neighbors: Vec<Node>,
}

const MAX_DISPLACEMENT_SQUARED: f64 = 4.0;

pub fn force_directed(mut nodes: Vec<Node>) -> ()
{
	let L = 2.0; // spring rest length
	let K_r = 2.0; // repulsive force constant
	let K_s = 2.0; // spring constant
	let delta_t = 2.0; // time step
	let N = nodes.len();
	// initialize net forces
	for i in 0..N-1
	{
		nodes[i].force.x = 0.0;
		nodes[i].force.y = 0.0;
	}
	
	// repulsion between all pairs
	repulsion(N, &mut nodes, K_r);

	// spring force between adjacent pairs
	spring(N, &mut nodes, K_r, L);

	// update positions
	update(N, &mut nodes, delta_t);
}




fn repulsion(N: usize, nodes: &mut [Node], repulsive_force: f32 ) -> ()
{
	// repulsion between all pairs
	for i1 in 0..N-2
	{
		let mut node1 = &mut nodes[i1];
		for i2 in i1+1..N-1
		{
 			let mut node2 = nodes[i2];
 			let dx = node2.pos.x - node1.pos.x;
 			let dy = node2.pos.y - node1.pos.y;
 			if dx != 0.0 || dy != 0.0
 			{
				let distanceSquared = dx*dx + dy*dy;
				let distance = distanceSquared.sqrt();
				let force = repulsive_force as f64 / distanceSquared;
				let fx = force * dx / distance;
				let fy = force * dy / distance;
				node1.force.x = node1.force.x - fx as f32;
				node1.force.y = node1.force.y - fy as f32;
				node2.force.x = node2.force.x + fx as f32;
				node2.force.y = node2.force.y + fy as f32;
			}
		}
	}
}

fn spring(N: usize, nodes: &mut [Node], repulsive_force: f32, spring_rest_length: f32) -> ()
{
	for i1 in 0..N-1
	{
		let mut node1 = &mut nodes[i1];
		for j in 0..node1.neighbors.len() - 1
		{
	 		let i2 = &node1.neighbors[j]; // Get node by Uuid
	 		let mut node2 = i2;
	 		if i1 < j // was i2
	 		{
	 			let dx = node2.pos.x - node1.pos.x;
	 			let dy = node2.pos.y - node1.pos.y;
				if dx != 0.0 || dy != 0.0
				{
					let distance = (dx*dx + dy*dy).sqrt();
					let force = repulsive_force as f64 * ( distance - spring_rest_length as f64);
					let fx = force * dx / distance;
					let fy = force * dy / distance;
					node1.force.x = node1.force.x + fx as f32;
					node1.force.y = node1.force.y + fy as f32;
					node2.force.x = node2.force.x - fx as f32;
					node2.force.y = node2.force.y - fy as f32;
				}
			}
		}
	}
}

fn update(N: usize, nodes: &mut [Node], delta_t: f64) -> ()
{
	for i in 0..N-1
	{
		let mut node = &mut nodes[i];
		let dx = delta_t * node.force.x as f64;
		let dy = delta_t * node.force.y as f64;
		let displacementSquared = dx*dx + dy*dy;
		if displacementSquared > MAX_DISPLACEMENT_SQUARED.into()
		{
	 		let s = (MAX_DISPLACEMENT_SQUARED / displacementSquared).sqrt();
	 		let dx = dx * s;
	 		let dy = dy * s;
	 	}
	 	node.pos.x = node.pos.x + dx;
		node.pos.y = node.pos.y + dy;
	}
}