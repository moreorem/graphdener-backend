use rmp_rpc::{Service, Value};
use commands::Commands;

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
            "import" => Commands::import_paths(params[0].as_array().expect("expected array")),
            "init" => Commands::initialize(params[0].as_str().expect("expected str")), //TODO receive trigger of choice from client
            "c_vert" => Commands::create_vertex(params[0].as_str().expect("expected str")),
            "c_edge" => Commands::load_edges(),
            "get_conns" => Commands::get_connections(),
            "get_vert" => Commands::get_vertex(params[0].as_array().expect("expected array"), params[1].as_str().expect("expected str")),
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





