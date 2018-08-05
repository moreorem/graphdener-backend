use rmp_rpc::Value;
use io::{filehandling, relations};
use indradb::{Datastore, MemoryDatastore, RocksdbDatastore, Transaction, Type, EdgeKey, util::generate_uuid_v1};
use datastore::ProxyDatastore;
use statics;
use std::iter::Iterator;


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
        ////////
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

    pub fn create_vertex(v_type: &str) -> Result<Value, Value>
    {
        // relational::create_vertex
        println!("Creating vertex...");
        // let trans = statics::DATASTORE.transaction().unwrap();

        let msg = String::from("ok");
        
        Ok(Value::from(msg.to_string()))
    }

    pub fn load_edges() -> Result<Value, Value>
    {
        println!("Creating edge...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let edge_list_available: bool;

        // If there is an edge list or map imported
        // if edge_list_available
        // {

        // }
        let uuid_from = generate_uuid_v1(); // TESTING
        let uuid_to = generate_uuid_v1(); // TESTING
        let e = EdgeKey::new(uuid_from, Type::new("ege".to_string()).unwrap(), uuid_to);
        // Edge::new(e);
        let msg = trans.create_edge(&e);

        Ok(Value::from("msg.as_str()"))
    }

    pub fn get_positions() -> Result<Value, Value>
    {
        // Returns x,y positions for every node in the graph
        Ok(Value::from("test"))
    }

    pub fn get_connections() -> Result<Value, Value>
    {
        // Returns the edge list, adjacency matrix or adjacency list in order to draw the graph
        
        Ok(Value::from("test"))

    }

}