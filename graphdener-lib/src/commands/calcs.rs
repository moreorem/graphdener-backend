use super::super::alg::forcedirected as fdir;
use super::database;
use commands::database::LIMIT;
use graphdenerdb::{Datastore, Transaction, Vertex, VertexQuery};
use models::graph;
use models::graph::GraphContainer;
use models::nodes::Node;
use rmp_rpc::Value;
use statics;
use std::collections::HashMap;
use uuid::Uuid;

// pub fn get_adj_list() -> Vec<Value>
//     {
//         let trans = statics::DATASTORE.transaction().unwrap();
//         let mut idx_map: HashMap<Uuid, usize> = HashMap::new();
//         let i = 0;
//         let v = VertexQuery::All{ start_id: None, limit: LIMIT };
//         let t = trans.get_vertex_metadata(&v, "pos").unwrap(); // FIXME: Prefer to use just uuid query

//         // Create index map in order to create the adjacency list next
//         for (i, x) in t.iter().enumerate()
//         {
//             idx_map.insert(x.id, i);
//         }
//         let draft_edges = trans.get_edges(&VertexQuery::All{start_id: None, limit: LIMIT}
//                 .outbound_edges(None, None, None, None, LIMIT)).unwrap();

//         draft_edges.iter().map(|x| Value::Array([
//                                     Value::from(*idx_map.get(&x.key.outbound_id).unwrap()),
//                                     Value::from(*idx_map.get(&x.key.inbound_id).unwrap())
//                                     ].to_vec()))
//                                     .collect()
//     }

pub fn create_uid_map(vertices: Vec<Vertex>, nodes: &mut Vec<Node>) -> HashMap<Uuid, usize> {
    let count = vertices.len();
    // Map to translate Uuids to Ids
    let mut idx_map: HashMap<Uuid, usize> = HashMap::with_capacity(count as usize);

    // First create Uuid to Id translation map
    for (idx, vert) in vertices.iter().enumerate() {
        idx_map.insert(vert.id, idx);

        // Create Node struct for current node without neighbors but with type and random pos
        let node = Node::new(idx, (0.0, 0.0), None, String::from(vert.t.0.clone()));
        nodes.insert(idx, node);
    }
    idx_map
}

pub fn find_neighbors(
    nodes: &mut Vec<Node>,
    idx_map: &HashMap<Uuid, usize>,
) -> Result<bool, &'static str> {
    let mut surrounding_verts: Vec<Vertex>;
    // Iterate again to find neighbors for every node
    for (uuid, id) in idx_map.iter() {
        // PENDING: Move error checking to database
        // Find neighbors for current node
        if let Ok(x) = database::get_vertex_neighbors(*uuid) {
            // Ignore absence of neighbors
            if x.len() == 0 {
                continue;
            }
            surrounding_verts = x;
        } else {
            return Err("problem getting neighbors");
        }

        let neighbors: Vec<usize> = surrounding_verts
            .iter()
            .map(|x| *idx_map.get(&x.id).unwrap())
            .collect();
        nodes[*id].neighbors = neighbors;
    }
    Ok(true)
}
