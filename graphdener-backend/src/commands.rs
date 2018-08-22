use rmp_rpc::Value;
use rand::prelude::*;

use io::filehandling;
use graphdener::{Datastore, Transaction, Type, EdgeKey, VertexQuery, Vertex, util::generate_uuid_v1};
// use datastore::ProxyDatastore;
use statics;
use std::iter::Iterator;
use uuid::Uuid;


// Here declare the functions that are going to be executed on the server
pub struct Commands;

impl Commands
{

    // Improved import function to accept an array of paths
    pub fn import_paths(path: &Vec<Value>) -> Result<Value, Value>
    {
        // Define path of edgelist
        let node_list_path = path[0].as_str();
        let edge_list_path = path[1].as_str();
        
        // Define path of communities
        // -------------------------

        // Count number of paths to import
        let paths_number = path.iter().count();
        
        let msg = format!("{}{}{}", "Imported ", paths_number, " paths");

        // Parse file to filehandling function

        // Handle the possibility of not setting a node filepath
        if let Some(node_list_path) = node_list_path {
            filehandling::import_vertices(node_list_path);
        }
        if let Some(edge_list_path) = edge_list_path {
            filehandling::import_edges(edge_list_path);
        }
        Ok(Value::from(msg))
    }

    
    pub fn create_vertex(v_type: &str) -> Result<Value, Value>
    {
        // relational::create_vertex
        println!("Creating vertex...");
        // let trans = statics::DATASTORE.transaction().unwrap();

        let msg = String::from("ok");
        
        Ok(Value::from(msg.to_string()))
    }

    // TODO: Make Getter trait for every object
  
    // Returns specific info about a set or all of the vertices that exist in the database to the frontend
    pub fn get_vertex(v_id: &[Value], info_type: &str) -> Result<Value, Value>
    {
        println!("{:?}", v_id);
        let trans = statics::DATASTORE.transaction().unwrap();
        let v: VertexQuery;
        let mut v_id_list: Vec<Uuid> = Vec::new();

        for item in v_id.iter()
        {
            v_id_list.push(<Uuid>::parse_str(item.as_str().unwrap()).unwrap());
        }

        println!("{:#?}", v_id_list);
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
        let response = trans.get_vertices(&v).unwrap();

        Ok(Commands::vert_info(info_type, response))
    }


    fn vert_info(info_type: &str, response: Vec<Vertex>) -> Value
    {
        // map all of the vectors in the response to one vector
        let r_iter = response.iter();
           
        // return the array of specific detail type for all of the selected vertices according to the command
        match info_type
        {
            "type" => Value::Array( r_iter.map( |x| Value::from(x.t.0.to_owned()) ).collect() ),
            "pos" => Value::Array(Commands::get_attribute("pos")),
            "size" => Value::Array(Commands::get_attribute("size")),
            "color" => Value::Array(Commands::get_attribute("color")),
            "label" => Value::Array(Commands::get_attribute("label")),
            _ => Value::from("error")

        }
    }

    // general getter that leads to specific objects
    pub fn get_object(obj: &str) -> Result<Value, Value>
    {
        let r = match obj {
            "edge" => Value::Boolean(Commands::get_edges().unwrap()),
            "vert" => Value::Boolean(Commands::get_vert().unwrap()),
            _ => Value::Boolean(false)
        };
        Ok(Value::from("e") ) // FIXME: this
    }


    // make a getter only for edge types, weight, direction and fromto
    fn get_edges() -> Result<bool, bool>
    {
        println!("Creating edge...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let edge_list_available: bool;

        Ok(true)
    }


    fn get_vert() -> Result<bool,bool>
    {
        Ok(true)
    }

    // Returns the edge list, adjacency matrix or adjacency list in order to draw the graph
    pub fn get_connections() -> Result<Value, Value>
    {
        Ok(Value::from("test"))
    }

    // Returns one of the attributes that reside in the metadata map of each vertex
    fn get_attribute(kind: &str) -> Vec<Value>
    {
        Commands::set_random_pos();
        
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

    pub fn update(field: &str, values: &[Value]) -> Result<Value, Value>
    {
        match field {
            "pos" => Commands::set_random_pos(),
            _ => panic!("unknown attribute")

        }

        Ok(Value::from("ok"))
    }

    fn set_random_pos()
    {
        let mut rng = thread_rng();

        let trans = statics::DATASTORE.transaction().unwrap();
        let mut x: f64;
        let mut y: f64;

        for vert in trans.get_vertices(&VertexQuery::All{ start_id: None, limit: 10000000 }).unwrap().iter()
        {
            x = rng.gen();
            y = rng.gen();
            let v = VertexQuery::Vertices{ ids: vec!(vert.id) };
            trans.set_vertex_metadata(&v, "pos", &json!([x, y]));
            // trans.get_vertex_metadata(&v, "pos").unwrap();
            // TESTME: use random values for position in order to print the nodes on the canvas
           
        }
    }

    
}

