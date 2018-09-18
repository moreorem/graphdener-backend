use containers::nodes::Node;
use uuid::Uuid;
use rmp_rpc::Value;
use std::collections::HashMap;
use statics;
use super::super::alg::forcedirected as fdir;
use rand::prelude::*;
use graphdener::{VertexQuery, Datastore, Transaction};

pub fn get_adj_list() -> Vec<Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut idx_map: HashMap<Uuid, usize> = HashMap::new();
        let i = 0;
        let v = VertexQuery::All{ start_id: None, limit: 1000000000 };
        let t = trans.get_vertex_metadata(&v, "pos").unwrap(); // FIXME: Prefer to use just uuid query

        // Create index map in order to create the adjacency list next
        for (i, x) in t.iter().enumerate()
        {
            idx_map.insert(x.id, i);
        }
        let draft_edges = trans.get_edges(&VertexQuery::All{start_id: None, limit: 1000000}
                .outbound_edges(None, None, None, None, 1000000)).unwrap();
        
        draft_edges.iter().map(|x| Value::Array([
                                    Value::from(*idx_map.get(&x.key.outbound_id).unwrap()), 
                                    Value::from(*idx_map.get(&x.key.inbound_id).unwrap())
                                    ].to_vec())) 
                                    .collect()
    }
    

pub fn use_algorithm() -> ()
    {
        let mut rng = thread_rng();
        // Translate Uuids to Ids

        let trans = statics::DATASTORE.transaction().unwrap();
        let count = trans.get_vertex_count().unwrap();

        let mut idx_map: HashMap<Uuid, usize> = HashMap::with_capacity(count as usize);
        let mut x: f64;
        let mut y: f64;
        let mut id: usize = 1;
        let mut nodes: Vec<Node> = Vec::new();

        // First create Uuid to Id translation map
        for (id, vert) in trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter().enumerate()
        {
            idx_map.insert(vert.id, id);

            // Create random positions to begin with
            x = rng.gen();
            y = rng.gen();
            let v = VertexQuery::Vertices{ ids: vec!(vert.id) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
            // Create Node struct for current node
            let node = Node::new(id, (x,y), None);
            nodes.insert(id, node);

        }

        // Iterate again to find neighbors for every node
        for (uuid, id) in idx_map.iter() // trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            // Find neighbors
            let surrounding_verts = trans.get_vertices(&VertexQuery::Vertices{ ids: vec!(*uuid) }.outbound_edges(None, None, None, None, 100).inbound_vertices(100)).unwrap();
            let neighbors: Vec<usize> = surrounding_verts.iter().map(|x| *idx_map.get(&x.id).unwrap() ).collect();
            nodes[*id].neighbors = neighbors;

        }

        let nd = fdir::force_directed(nodes, 0.1, 0.1, 0.2, 2.0);

        for (uuid, id) in idx_map.iter() //trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            let (x,y) = nd[*id].pos.get();
            let v = VertexQuery::Vertices{ ids: vec!(*uuid) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
        }

    }