use rmp_rpc::Value;
use io::filehandling;
use indradb::{Datastore, MemoryDatastore, RocksdbDatastore, Transaction, Type, EdgeKey, VertexQuery, Vertex, util::generate_uuid_v1};
use datastore::ProxyDatastore;
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
        let edge_list_path = path[0].as_str();

        // Define path of communities
        // -------------------------

        // Count number of paths to import
        let paths_number = path.iter().count();
        
        let msg = format!("{}{}{}", "Imported ", paths_number, " paths");

        // Parse file to filehandling function
        filehandling::import_edges(edge_list_path.unwrap());

        Ok(Value::from(msg))
    }

    pub fn initialize(datastore_type: &str) -> Result<Value, Value>
    {
        let msg = "hello";
        println!("Initializing database...");
       
        if datastore_type == "rocksdb" {
            let datastore = RocksdbDatastore::new("localhost:8888", Some(5))
                            .expect("Expected to be able to create a RocksDB datastore");
            ProxyDatastore::Rocksdb(datastore);
        }
        else {
            let datastore = MemoryDatastore::default();
            ProxyDatastore::Memory(datastore);
        }

        Ok(Value::from(msg))
    }

    // pub fn create_vertex(v_type: &str) -> Result<Value, Value>
    // {
    //     // relational::create_vertex
    //     println!("Creating vertex...");
    //     // let trans = statics::DATASTORE.transaction().unwrap();

    //     let msg = String::from("ok");
        
    //     Ok(Value::from(msg.to_string()))
    // }

  
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
            v = VertexQuery::All{start_id: None, limit: 100};
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
            "position" => Value::Array( r_iter.map(|x| Value::Array(vec![Value::from(x.pos[0]), Value::from(x.pos[1])]) ).collect() ) ,
            "type" => Value::Array( r_iter.map( |x| Value::from(x.t.0.to_owned()) ).collect() ),
            // "label" => Value::Array( r_iter.map( |x| Value::from(x.label.to_owned().unwrap()) ).collect() ),
            _ => Value::from("error")

        }
    }

    pub fn load_edges() -> Result<Value, Value>
        {
            println!("Creating edge...");
            let trans = statics::DATASTORE.transaction().unwrap();
            let edge_list_available: bool;

            let uuid_from = generate_uuid_v1(); // TESTING
            let uuid_to = generate_uuid_v1(); // TESTING
            let e = EdgeKey::new(uuid_from, Type::new("ege".to_string()).unwrap(), uuid_to);
            // Edge::new(e);
            let msg = trans.create_edge(&e);

            Ok(Value::from("msg.as_str()"))
        }

    pub fn get_connections() -> Result<Value, Value>
    {
        // Returns the edge list, adjacency matrix or adjacency list in order to draw the graph
        Ok(Value::from("test"))

    }

}