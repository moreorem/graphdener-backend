use graphdenerdb::{Datastore, Transaction, Vertex, VertexQuery};
use statics;
use std::f64::consts::PI;

fn angle(p: (f64, f64), N: u32) -> (f64, f64) {
    (p.0 * 2.0 * PI / f64::from(N), p.1 * 2.0 * PI / f64::from(N))
}

// compute average position of neighbors
pub fn barycenter_heuristic(node_list: Vec<Vertex>) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let pos: (f64, f64) = (0., 0.);

    for node in node_list.iter() {
        let v = VertexQuery::Vertices { ids: vec![node.id] };

        trans.set_vertex_metadata(&v, "pos", &json!([pos.0, pos.1]));

        let p = pos;

        // sum_x = cos(angle(p1))
        // sum_y = sin(angle(p1))
        // for j = 0 to node1.neighbors.length-1
        // i2 = node1.neighbors[j]
        // node2 = nodes[i2]
        // p2 = positionOfNode(i2)
        // sum_x = sum_x + cos(angle(p2))
        // sum_y = sum_y + sin(angle(p2))
        // orderedNodes[p1].average = angleOfVector(sum_x,sum_y)
        // // sort the array according to the values of average
        // sort( orderedNodes, comparator )
    }
}
