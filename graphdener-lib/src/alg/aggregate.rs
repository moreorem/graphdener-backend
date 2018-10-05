use models::graph::Graph;
use models::nodes::Node;

// fn create_arrowhead(A: [f64;2], B: [f64;2], v1: &mut [f64;2], v2: &mut [f64;2]) -> ()
// {
//     let w = 1.;
//     let h: f64 = w * (0.8660254037844386467637); // sqrt(3)/2
//     let mut U: [f64;2];
//     let mag = ((B[0] - A[0]).powf(2.0) + (B[1] - A[1]).powf(2.0)).sqrt();

//     let u0 = (B[0] - A[0])/(mag);
//     let u1 = (B[1] - A[1])/(mag);
//     U = [u0, u1];
//     let V: [f64;2] = [-U[1], U[0]];
//     *v1 = [B[0] - h*U[0] + w*V[0], B[1] - h*U[1] + w*V[1]];
//     *v2 = [B[0] - h*U[0] - w*V[0], B[1] - h*U[1] - w*V[1]];
// }

//     fn get_adj_list() -> Vec<Value>
//     {
//         let trans = statics::DATASTORE.transaction().unwrap();
//         let mut idx_map: HashMap<Uuid, usize> = HashMap::new();
//         let i = 0;
//         let v = VertexQuery::All{ start_id: None, limit: 1000000000 };
//         let t = trans.get_vertex_metadata(&v, "pos").unwrap();

//         // Create index map in order to create the adjacency list next
//         for (i, x) in t.iter().enumerate()
//         {
//             idx_map.insert(x.id, i);
//         }
//         let draft_edges = trans.get_edges(&VertexQuery::All{start_id: None, limit: 1000000}
//                 .outbound_edges(None, None, None, None, 1000000)).unwrap();

//         draft_edges.iter().map(|x| Value::Array([
//                                     Value::from(*idx_map.get(&x.key.outbound_id).unwrap()),
//                                     Value::from(*idx_map.get(&x.key.inbound_id).unwrap())
//                                     ].to_vec()))
//                                     .collect()
//     }


pub fn particle_swarm_optimization(mut graph: &mut Graph) -> () {
	for i in graph.nodes.iter() { // particle i = 1, ..., S do
      
   		Initialize the particle's position with a uniformly distributed random vector: xi ~ U(blo, bup)
   		Initialize the particle's best known position to its initial position: pi ← xi
   		if f(pi) < f(g) {
   			update the swarm's best known  position: g ← pi
   		}
   		Initialize the particle's velocity: vi ~ U(-|bup-blo|, |bup-blo|)
   	}
	while {//a termination criterion is not met do:
   		for i in graph.nodes.iter() { //each particle i = 1, ..., S do
      		for i in range(0..2) { //each dimension d = 1, ..., n do
         		Pick random numbers: rp, rg ~ U(0,1)
         		Update the particle's velocity: vi,d ← ω vi,d + φp rp (pi,d-xi,d) + φg rg (gd-xi,d)
         	}
      		Update the particle's position: xi ← xi + vi
      		if f(xi) < f(pi) {
         		Update the particle's best known position: pi ← xi
         		if f(pi) < f(g) {
            		Update the swarm's best known position: g ← pi
            	}
            }
        }
    }
}