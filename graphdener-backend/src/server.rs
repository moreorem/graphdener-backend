use rmp_rpc::{Service, Value};
use std::iter::Iterator;
use indradb::{Datastore, MemoryDatastore, RocksdbDatastore, Transaction, Type, EdgeKey, Edge, util::generate_uuid_v1};
use datastoremode::ProxyDatastore;
use io::filehandling;
use statics;

// Our server type
#[derive(Clone)]
pub struct Echo;


// The Service trait defines how the server handles incoming requests and notifications.
impl Service for Echo 
{
    // This is the type of future we send back from `handle_request`. Since we have a response
    // available immediately, there's no need to send back a "genuine" future.
    type RequestFuture = Result<Value, Value>;

    // Define how the server handle requests.
    fn handle_request(&mut self, method: &str, params: &[Value]) -> Self::RequestFuture 
    {
        match method 
        {
            "sum" => Methods::sum(params[0].as_u64().expect("expected u64"), params[1].as_u64().expect("expected u64")),
            "import" => Methods::import_paths(params[0].as_array().expect("expected array")),
            "init" => Methods::initialize(params[0].as_str().expect("expected str")), //TODO receive trigger of choice from client
            "c_vert" => Methods::create_vertex(),
            "c_edge" => Methods::load_edges(),

            _ => Err("invalid argument".into())
        }
        
    }
  
    // Define how the server handle notifications.
    //
    // This server just prints the method in the console.
    fn handle_notification(&mut self, method: &str, _: &[Value]) {
        println!("{}", method);
    }
}


// Here declare the functions that are going to be executed on the server
struct Methods;

impl Methods
{
    fn sum(a: u64, b: u64) -> Result<Value, Value>
    {
        let c: Value = Value::from(a + b);
        Ok(c)
    }   

    // Improved import function to accept an array of paths
    fn import_paths(path: &Vec<Value>) -> Result<Value, Value>
    {
        let edge_list_path: String = path[0].to_string();
        println!("Edge list path is: {}", edge_list_path);
        let paths_number = path.iter().count();

        let msg = format!("{}{}{}", "Imported ", paths_number, " paths");

        // Parse file to filehandling function
        filehandling::import_edges(&edge_list_path);
        Ok(Value::from(msg))
    }

    fn initialize(datastore_type: &str) -> Result<Value, Value>
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

    fn create_vertex() -> Result<Value, Value>
    {
        println!("Creating vertex...");
        let trans = statics::DATASTORE.transaction().unwrap();

        let msg = trans.create_vertex_from_type(Type::new("egg".to_string()).unwrap());
        Ok(Value::from(msg.unwrap().to_string()))
    }

    fn load_edges() -> Result<Value, Value>
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

}


