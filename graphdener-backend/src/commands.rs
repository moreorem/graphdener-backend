use graphdener::Edge;
use std::collections::HashMap;
use rmp_rpc::Value;
use rand::prelude::*;

use io::filehandling;
use graphdener::{Datastore, Transaction, Type, EdgeKey, VertexQuery, Vertex};
// use datastore::ProxyDatastore;
use statics;
use std::iter::Iterator;
use uuid::Uuid;


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
    pub fn import_paths(path: &Vec<Value>) -> Result<Value, Value>
    {
        println!("{:?}", path);
        // Define path of edgelist
        let node_list_path = path[0].as_str();
        let edge_list_path = path[1].as_str();
       
        // Count number of paths to import
        let paths_number = path.iter().count();
        
        let msg = format!("{}{}{}", "Imported ", paths_number, " paths");

        let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();
        // Parse file to filehandling function

        // Handle the possibility of not setting a node filepath
        if let Some(node_list_path) = node_list_path {
            filehandling::import_vertices(node_list_path, &mut uuid_map);
        }
        if let Some(edge_list_path) = edge_list_path {
            filehandling::import_edges(edge_list_path, &uuid_map);
        }
        Ok(Value::from(msg))
    }

    // TODO: Make Getter trait for every object
  
    // general getter that leads to specific objects
    pub fn get_object(obj: &str, info: &str) -> Result<Value, Value>
    {
        let r = match obj {
            "edge" => Commands::get_edge(&[], info).unwrap(), //Value::Boolean(Commands::get_edge().unwrap()),
            "vert" => Commands::get_vertex(&[], info).unwrap(), //Commands::get_vertex(&vec!(Value::from(2)), "pos"),
            _ => Value::from("Error")
        };
        Ok(r)
    }

    // Returns specific info about a set or all of the vertices that exist in the database to the frontend
    fn get_vertex(v_id: &[Value], info_type: &str) -> Result<Value, Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let v: VertexQuery;
        let mut v_id_list: Vec<Uuid> = Vec::new();

        for item in v_id.iter()
        {
            v_id_list.push(<Uuid>::parse_str(item.as_str().unwrap()).unwrap());
        }

        // FIXME: Simplify vertex query, remove some conditionals
        if v_id.len() > 0
        {
            v = VertexQuery::Vertices{ ids: v_id_list };
            println!("some uuids");
        }
        else
        {
            v = VertexQuery::All{ start_id: None, limit: 1000000000 };
            println!("all vertices");
        }
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
    fn get_edge(v_id: &[Value], info_type: &str) -> Result<Value, Value>
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
        Commands::set_random_pos(); // TESTME: Delete afterwards
        
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
    fn set_random_pos()
    {
        let mut rng = thread_rng();

        let trans = statics::DATASTORE.transaction().unwrap();
        let mut x: f64;
        let mut y: f64;

        for vert in trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 1000000 }).unwrap().iter()
        {
            x = rng.gen();
            y = rng.gen();
            let v = VertexQuery::Vertices{ ids: vec!(vert.id) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
            // trans.get_vertex_metadata(&v, "pos").unwrap();
            // TESTME: use random values for position in order to print the nodes on the canvas
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
#[cfg(test)]
mod tests {
    use commands::Commands;
    #[test]
    fn test_pos()
    {  
        // println!("{:?}", Commands::get_adj_list(IDX_MAP))
    }
}