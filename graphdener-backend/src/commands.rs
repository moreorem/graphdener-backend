use rmp_rpc::Value;
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

    // pub fn initialize(datastore_type: &str) -> Result<Value, Value>
    // {
    //     println!("Initializing database...");
    //     let msg: Value;

    //     if datastore_type == "rocksdb" {
    //         let datastore = RocksdbDatastore::new("localhost:8888", Some(5))
    //                         .expect("Expected to be able to create a RocksDB datastore");
    //         ProxyDatastore::Rocksdb(datastore);
    //         msg = Value::from("RocksDB Datastore");
    //     }
    //     else {
    //         let datastore = MemoryDatastore::default();
    //         ProxyDatastore::Memory(datastore);
    //         msg = Value::from("Memory Datastore");
    //     }

    //     Ok(Value::from(msg))
    // }

    pub fn create_vertex(v_type: &str) -> Result<Value, Value>
    {
        // relational::create_vertex
        println!("Creating vertex...");
        // let trans = statics::DATASTORE.transaction().unwrap();

        let msg = String::from("ok");
        
        Ok(Value::from(msg.to_string()))
    }

  
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
            // "position" => Value::Array( r_iter.map(|x| Value::Array(vec![Value::from(x.pos[0]), Value::from(x.pos[1])]) ).collect() ) ,
            "type" => Value::Array( r_iter.map( |x| Value::from(x.t.0.to_owned()) ).collect() ),
            "pos" => Value::Array(Commands::get_spatial("pos")),
            "size" => Value::Array(Commands::get_spatial("size")),
            "color" => Value::Array(Commands::get_spatial("color")),
            // "label" => Value::Array( r_iter.map( |x| Value::from(x.label.to_owned().unwrap()) ).collect() ),
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
        Ok(Value::from("e") )// TEMPORARY
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

    pub fn get_connections() -> Result<Value, Value>
    {
        // Returns the edge list, adjacency matrix or adjacency list in order to draw the graph
        Ok(Value::from("test"))
    }

    fn get_spatial(kind: &str) -> Vec<Value>
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        let v = VertexQuery::All{ start_id: None, limit: 1000000000 };
        let t = match kind
        {
            "pos" => trans.get_vertex_metadata(&v, "pos").unwrap(),
            "size" => trans.get_vertex_metadata(&v, "size").unwrap(),
            "color" => trans.get_vertex_metadata(&v, "color").unwrap(),
            _ => vec!()
        };

        t.iter().map(|x| Value::from(x.value.to_string())).collect()

    }

    pub fn update(field: &str, values: &[Value]) -> Result<Value, Value>
    {
        match field {
            "pos" => Commands::set_pos(values),
            _ => panic!("unknown attribute")

        }

        Ok(Value::from("ok"))
    }

    fn set_pos(v: &[Value])
    {
        let trans = statics::DATASTORE.transaction().unwrap();
        // trans.get_vertex_metadata(&v, "pos").unwrap();
    }

    
}

