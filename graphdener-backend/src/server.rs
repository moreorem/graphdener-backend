use graphdener::commands::distribute::{apply_circular, apply_force_directed, apply_random_pos};
use graphdener::commands::initials::{import_paths, initialize_graph, populate_graph};
use graphdener::commands::retrievals::{get_adjacency, get_node_type, get_pos, get_stat};
use graphdener::models::graph::GraphContainer;
use rmp_rpc::{Service, Value};

// Our server type
#[derive(Clone)]
pub struct Echo(pub GraphContainer);

// The Service trait defines how the server handles incoming requests and notifications.
impl Service for Echo {
    // This is the type of future we send back from `handle_request`. Since we have a response
    // available immediately, there's no need to send back a "genuine" future.
    type RequestFuture = Result<Value, Value>;

    // Define how the server handle requests.
    fn handle_request(&mut self, method: &str, params: &[Value]) -> Self::RequestFuture {
        // Call the proper command according to the input trigger from the frontend
        match method {
            // TODO: create function that returns the max graph id
            "import" => import_paths(
                params[0].as_array().expect("expected array"),
                params[1].as_array().expect("expected array"),
                params[2].as_map().expect("expected dictionary"),
            ),
            "newgraph" => initialize_graph(&mut self.0),
            "populate" => populate_graph(params[0].as_u64().expect("expected id"), &mut self.0),
            "random" => {
                apply_random_pos(params[0].as_u64().expect("expected id") as u8, &mut self.0)
            }
            "diralg" => apply_force_directed(
                params[0].as_u64().expect("expected id"),
                &mut self.0,
                params[1].as_array().expect("expected array"),
            ),
            "ciralg" => apply_circular(params[0].as_u64().expect("expected id"), &mut self.0),
            "getnpos" => get_pos(params[0].as_u64().expect("expected id"), &self.0),
            "getadj" => get_adjacency(params[0].as_u64().expect("expected id"), &self.0),
            "getntype" => get_node_type(params[0].as_u64().expect("expected id"), &self.0),
            "getstat" => get_stat(),
            _ => Err("Invalid method call".into()),
        }
    }
    // Define how the server handle notifications.
    //
    // This server just prints the method in the console.
    fn handle_notification(&mut self, method: &str, _: &[Value]) {
        println!("Rust Backend: {}", method);
    }
}
