use rmp_rpc::{Service, Value};
// use commandsold::Commands;
use containers::graph::GraphContainer;
use commands::initials::{import_paths, initialize_graph};
use commands::retrievals::{get_edge, get_vertex};

// Our server type
#[derive(Clone)]
pub struct Echo(pub GraphContainer);

// The Service trait defines how the server handles incoming requests and notifications.
impl Service for Echo 
{
    // This is the type of future we send back from `handle_request`. Since we have a response
    // available immediately, there's no need to send back a "genuine" future.
    type RequestFuture = Result<Value, Value>;

    // Define how the server handle requests.
    fn handle_request(&mut self, method: &str, params: &[Value]) -> Self::RequestFuture 
    {

        // Call the proper command according to the input trigger from the frontend
        match method 
        {
            "import" => import_paths(params[0].as_array().expect("expected array"), 
                        params[1].as_str().expect("expected string"), 
                        params[2].as_str().expect("expected string")),
            "graph" => initialize_graph(params[0].as_u64().expect("expected id"), &mut self.0),
            "get" => {
                let canvas_id: u8 = params[2].as_u64().expect("invalid canvas id") as u8;
                match params[0].as_str().expect("expected str") {
                    "edge" => get_edge(canvas_id, params[1].as_str().unwrap()), 
                    "vert" => get_vertex(canvas_id, params[1].as_str().unwrap()),
                    _ => Err("Could not get such object".into())
                }
            },
            _ => Err("Invalid method call".into())
        }
    }
    // TODO: Create a struct of response that always contains ids along with each info and return this to the frontend
    // Define how the server handle notifications.
    //
    // This server just prints the method in the console.
    fn handle_notification(&mut self, method: &str, _: &[Value]) {
        println!("{}", method);
    }
}





