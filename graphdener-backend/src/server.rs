use rmp_rpc::{Service, Value};
use std::iter::Iterator;
use indradb::{Datastore, MemoryDatastore, RocksdbDatastore, Transaction, Type};


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
            "conc" => Methods::concatenate(params[0].as_str().expect("expected str"), params[1].as_str().expect("expected str")),
            "import" => Methods::import_paths(params[0].as_array().expect("expected array")),
            "init" => Methods::initialize(params[0].as_str().expect("expected str")), //TODO receive trigger of choice from client
            "c_vert" => Methods::create_vertex(),
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
        Ok(Value::from(msg))
    }

    fn concatenate(a: &str, b: &str) -> Result<Value, Value>
    {
        let s: Value = Value::from(a.to_owned() + b);
        Ok(s)
    }

    fn initialize(datastore_type: &str) -> Result<Value, Value>
    {
        let msg = "hello";
        println!("Initializing database...");
        // let mut db: dbt;
        let dbtype = match datastore_type { 
            "rocksdb" => DatastoreType::RocksDB, 
            _ => DatastoreType::Memory
        }; 

        if let DatastoreType::RocksDB = dbtype {
            let db = &RocksdbDatastore::new("localhost:8888", Some(5));
        }
        else {
            let db = &MemoryDatastore::default();
        }

        Ok(Value::from(msg))
    }

    fn create_vertex() -> Result<Value, Value>
    {
        let msg = "";
        println!("Creating vertex...");
        let mut db = MemoryDatastore::default();
        let trans = db.transaction().unwrap();

        trans.create_vertex_from_type(Type::new("egg".to_string()).unwrap());
        println!("{:?}", trans);
        Ok(Value::from(msg))
    }

}

enum DatastoreType
{
    Memory,
    RocksDB
}

struct Dbase
{
    mem: MemoryDatastore,
    roc: RocksdbDatastore
}


