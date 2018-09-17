use graphdener::Edge;
use std::collections::HashMap;
use rmp_rpc::Value;
use rand::prelude::*;

use alg::barycenterordering as bary;
use alg::circular as cir;
use alg::forcedirected as fdir;

use io::filehandling;
use graphdener::{Datastore, Transaction, EdgeKey, VertexQuery, Vertex};
use statics;
use std::iter::Iterator;
use uuid::Uuid;
// use std::num::{sqrt, pow};

enum Unit<Info>
{
    EdgeUnit(Info),
    VertUnit(Info)
}

pub trait Info<Unit>{
    fn get() -> Value
    {
        let n = [3;1];
        Value::Array(vec!(Value::from(1)))
    }
}    
    
// Here declare the functions that are going to be executed on the server
pub struct Commands;

impl Commands
{
    // TODO: Implement uuid_map as a struct in models
    // Improved import function to accept an array of paths
    pub fn import_paths(path: &Vec<Value>, patternN: &str, patternE: &str) -> Result<Value, Value>
    {

        println!("{:?}", path);
        // Define path of edgelist
        let node_list_path = path[0].as_str();
        let edge_list_path = path[1].as_str();
        // Define regular expressions
        let mut node_pattern: String = String::from("");
        let mut edge_pattern: String = String::from("");
        node_pattern.push_str(&format!(r#"{}"#, patternN));
        edge_pattern.push_str(&format!(r#"{}"#, patternE));
        // r#"^(?P<id>\d+)\s+(?P<source>\d+)\s+(?P<target>\d+)\s+"(?P<label>[^"]*)"\s+"(?P<type>[^"]*)"\s+(?P<weight>\d+)"#
        let format = [ &node_pattern[..], &edge_pattern[..] ];

        filehandling::import_files(node_list_path.unwrap(), edge_list_path.unwrap(), &format);
        
        Ok(Value::from("paths imported"))
    }


    // Returns specific info about a set or all of the vertices that exist in the database to the frontend
    pub fn get_vertex(canvas_id: u8, info_type: &str) -> Result<Value, Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let v: VertexQuery;

        v = VertexQuery::All{ start_id: None, limit: 1000000000 };
        println!("all vertices");
    
        // In this case the msg variable is of type model::Vertex. It has to be broken into the struct items to be used
        let draft_info = trans.get_vertices(&v).unwrap();
        println!("asked about {}", &info_type);
        Ok(Commands::vert_info(info_type, draft_info))
    }

    fn vert_info(info_type: &str, draft_model: Vec<Vertex>) -> Value
    {
        // map all of the vectors in the response to one vector
        let r_iter = draft_model.iter();
           
        // return the array of specific detail type for all of the selected vertices according to the command
        match info_type
        {
            "type" => Value::Array( r_iter.map( |x| Value::from(x.t.0.to_owned()) ).collect() ),
            "pos" => Value::Array(Commands::get_v_attribute("pos")),
            "size" => Value::Array(Commands::get_v_attribute("size")),
            "color" => Value::Array(Commands::get_v_attribute("color")),
            "label" => Value::Array(Commands::get_v_attribute("label")),
            _ => Value::from(format!("No such info: {}", info_type ))

        }
    }

    // make a getter only for edge types, weight, direction and fromto
    pub fn get_edge(canvas_id: u8, info_type: &str) -> Result<Value, Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let edge_list_available: bool;

        let draft_info = trans.get_edges(&VertexQuery::All{start_id: None, limit: 1000000}
                                        .outbound_edges(None, None, None, None, 1000000) )
                                        .unwrap();
        Ok(Commands::edge_info(info_type, draft_info))
    }
 
    fn edge_info(info_type: &str, draft_model: Vec<Edge>) -> Value
    {
        // map all of the vectors in the response to one vector
        let r_iter = draft_model.iter();
           
        // return the array of specific detail type for all of the selected vertices according to the command
        match info_type
        {
            "type" => Value::Array( r_iter.map( |x| Value::from(x.key.t.0.to_owned() ) ).collect() ),
            "pos" => Value::Array(Commands::get_adj_list() ),
            "label" => Value::Array(Commands::get_e_attribute("label")),
            "weight" => Value::Array(Commands::get_e_attribute("weight")),
            _ => Value::from("error")
        }
    }

    // Returns one of the attributes that reside in the metadata map of each vertex
    fn get_v_attribute(kind: &str) -> Vec<Value>
    {
        // Commands::set_random_pos(); // TESTME: Delete afterwards
        Commands::use_algorithm(); // TESTME: Delete afterwards

        let trans = statics::DATASTORE.transaction().unwrap();
        let v = VertexQuery::All{ start_id: None, limit: 1000000000 };

        let t = match kind
        {
            "pos" => trans.get_vertex_metadata(&v, "pos").unwrap(),
            "size" => trans.get_vertex_metadata(&v, "size").unwrap(),
            "color" => trans.get_vertex_metadata(&v, "color").unwrap(),
            "label" => trans.get_vertex_metadata(&v, "label").unwrap(),
            _ => vec!()
        };
   
        t.iter().map(|x| Value::from(x.value.to_string())).collect() // TODO: Find a way to return a float instead of string
    }

    fn get_e_attribute(kind: &str) -> Vec<Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let e = &VertexQuery::All{start_id: None, limit: 100000000}
                            .outbound_edges(None, None, None, None, 1000000000);
        let t = match kind
        {
            "weight" => trans.get_edge_metadata(&e, "weight").unwrap(),
            "label" => trans.get_edge_metadata(&e, "label").unwrap(),
            _ => vec!()
        };

        t.iter().map(|x| Value::from(x.value.to_string())).collect() // TODO: Find a way to return a float instead of string        
    }

    // TODO: Convert to algorithm caller later
    // TODO: Change name to more appropriate
    fn set_random_pos()
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut x: f64;
        let mut y: f64;

        let node_list = trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap();
        let vert_num = node_list.len() as u32;

        let mut positions = cir::polygon(vert_num).into_iter();

        for vert in trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            let (x,y) = positions.next().unwrap();
            let v = VertexQuery::Vertices{ ids: vec!(vert.id) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
        }
    }

    fn use_algorithm() -> ()
    {
        let mut rng = thread_rng();
        // Translate Uuids to Ids

        let trans = statics::DATASTORE.transaction().unwrap();
        let count = trans.get_vertex_count().unwrap();

        let mut idx_map: HashMap<Uuid, usize> = HashMap::with_capacity(count as usize);
        let mut x: f64;
        let mut y: f64;
        let mut id: usize = 1;
        let mut nodes: Vec<fdir::Node> = Vec::new();

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
            let node = fdir::Node::new(id, (x,y), None);
            nodes.push(node);

        }

        // Iterate again to find neighbors for every node
        for (uuid, id) in idx_map.iter() // trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            // Find neighbors
            // PENDING: Cover case of not having any neighbors
            let surrounding_verts = trans.get_vertices(&VertexQuery::Vertices{ ids: vec!(*uuid) }.outbound_edges(None, None, None, None, 100).inbound_vertices(100)).unwrap();
            let neighbors: Vec<usize> = surrounding_verts.iter().map(|x| *idx_map.get(&x.id).unwrap() ).collect();
            nodes[*id].neighbors = neighbors;
        }

        let nd = fdir::force_directed(nodes);

        for (uuid, id) in idx_map.iter() //trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            let (x,y) = nd[*id].pos.get();
            let v = VertexQuery::Vertices{ ids: vec!(*uuid) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
        }

    }

    fn get_adj_list() -> Vec<Value>
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
        println!("{:?}", idx_map);
        let draft_edges = trans.get_edges(&VertexQuery::All{start_id: None, limit: 1000000}
                .outbound_edges(None, None, None, None, 1000000)).unwrap();
        
        draft_edges.iter().map(|x| Value::Array([
                                    Value::from(*idx_map.get(&x.key.outbound_id).unwrap()), 
                                    Value::from(*idx_map.get(&x.key.inbound_id).unwrap())
                                    ].to_vec())) 
                                    .collect()
    }
    
}

fn create_arrowhead(A: [f64;2], B: [f64;2], v1: &mut [f64;2], v2: &mut [f64;2]) -> ()
{
    let w = 1.;
    let h: f64 = w * (0.8660254037844386467637); // sqrt(3)/2
    let mut U: [f64;2]; 
    let mag = ((B[0] - A[0]).powf(2.0) + (B[1] - A[1]).powf(2.0)).sqrt();

    let u0 = (B[0] - A[0])/(mag);
    let u1 = (B[1] - A[1])/(mag);
    U = [u0, u1];
    let V: [f64;2] = [-U[1], U[0]];
    *v1 = [B[0] - h*U[0] + w*V[0], B[1] - h*U[1] + w*V[1]];
    *v2 = [B[0] - h*U[0] - w*V[0], B[1] - h*U[1] - w*V[1]];
}




#[cfg(test)]
mod tests {
use commands::create_arrowhead;
use commands::Commands;
    #[test]
    fn test_arrow()
    {  
        let mut v1: [f64;2] = [-2.0, 3.0];
        let mut v2: [f64;2] = [-2.0, 3.0];
        create_arrowhead([1.0,1.0],[-2.0,3.0], &mut v1, &mut v2);
        println!("{:?}//{:?}",v1, v2);
        panic!("{:?}//{:?}",v1, v2);
        // println!("{:?}", Commands::get_adj_list(IDX_MAP))
    }
}