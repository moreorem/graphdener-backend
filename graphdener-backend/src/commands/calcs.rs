use containers::graph::GraphContainer;
use containers::nodes::Node;
use containers::graph;
use uuid::Uuid;
use rmp_rpc::Value;
use std::collections::HashMap;
use statics;
use super::super::alg::forcedirected as fdir;
use rand::prelude::*;
use graphdener::{VertexQuery, Datastore, Transaction, Vertex, MemoryTransaction};

pub const LIMIT: u32 = 10000000;

pub fn get_adj_list() -> Vec<Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut idx_map: HashMap<Uuid, usize> = HashMap::new();
        let i = 0;
        let v = VertexQuery::All{ start_id: None, limit: LIMIT };
        let t = trans.get_vertex_metadata(&v, "pos").unwrap(); // FIXME: Prefer to use just uuid query

        // Create index map in order to create the adjacency list next
        for (i, x) in t.iter().enumerate()
        {
            idx_map.insert(x.id, i);
        }
        let draft_edges = trans.get_edges(&VertexQuery::All{start_id: None, limit: LIMIT}
                .outbound_edges(None, None, None, None, LIMIT)).unwrap();
        
        draft_edges.iter().map(|x| Value::Array([
                                    Value::from(*idx_map.get(&x.key.outbound_id).unwrap()), 
                                    Value::from(*idx_map.get(&x.key.inbound_id).unwrap())
                                    ].to_vec())) 
                                    .collect()
    }
    


pub fn create_uid_map(vertices: Vec<Vertex>, nodes: &mut Vec<Node>) -> HashMap<Uuid, usize>
{
    let count = vertices.len();
    // Map to translate Uuids to Ids
    let mut idx_map: HashMap<Uuid, usize> = HashMap::with_capacity(count as usize);
    // call rng
    let mut rng = thread_rng();
    let mut x: f64;
    let mut y: f64;
    // First create Uuid to Id translation map
    for (idx, vert) in vertices.iter().enumerate()
    {
        idx_map.insert(vert.id, idx);

        // Create random positions to begin with
        x = rng.gen();
        y = rng.gen();
        
        // Create Node struct for current node without neighbors
        let node = Node::new(idx, (x,y), None);
        nodes.insert(idx, node);
    }
    idx_map
}

pub fn find_neighbors(trans: MemoryTransaction, nodes: &mut Vec<Node>, idx_map: &HashMap<Uuid, usize>) -> Result<bool, &'static str>
{
    let mut surrounding_verts: Vec<Vertex>;
    // Iterate again to find neighbors for every node
    for (uuid, id) in idx_map.iter()
    {
        // Find neighbors for current node
        if let Ok(x) = trans.get_vertices(&VertexQuery::Vertices{ ids: vec!(*uuid) }.outbound_edges(None, None, None, None, 100).inbound_vertices(100))
        {
            // Ignore absence of neighbors
            if x.len() == 0 {
                continue;
            }
            surrounding_verts = x;
        }
        else {
            return Err("problem getting neighbors")
        }
        // FIXME: Remove or handle surrounding_verts empty values
        let neighbors: Vec<usize> = surrounding_verts.iter().map(|x| *idx_map.get(&x.id).unwrap() ).collect();
        println!("{:?}", neighbors);
        nodes[*id].neighbors = neighbors;
        println!("{:?}", nodes);

    }
    Ok(true)
}


// pub fn use_algorithm() -> ()
//     {
//         match alg {

//         }
//         let nd = fdir::force_directed(nodes, 0.1, 0.1, 0.2, 2.0);

//         for (uuid, id) in idx_map.iter() //trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
//         {
//             let (x,y) = nd[*id].pos.get();
//             let v = VertexQuery::Vertices{ ids: vec!(*uuid) };
//             trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
//         }

//     }

    // TODO: Make a database updater to be runned after every pause of actions
    // TODO: Separate methods into initial, repeating, backend