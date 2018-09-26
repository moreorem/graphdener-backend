use alg::forcedirected::force_directed;
use rmp_rpc::{Service, Value};
// use commandsold::Commands;
use alg::circular;
use commands::initials::{
    apply_circular, apply_force_directed, apply_random_pos, import_paths, initialize_graph,
    populate_graph,
};
use commands::retrievals::{get_adjacency, get_edge, get_node_type, get_pos, get_vertex};
use models::graph::GraphContainer;

// FIXME: Find a better place for Graph Container initialization
// Our server type
#[derive(Clone)]
pub struct Echo(pub GraphContainer);

// The Service trait defines how the server handles incoming requests and notifications.
impl Service for Echo {
    // TODO: Use only one Graph struct and repopulate it every time you change canvas
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
                params[2].as_bool().expect("expected bool"),
                params[3].as_map().expect("expected array"),
            ),
            "newgraph" => initialize_graph(&mut self.0),
            "populate" => populate_graph(params[0].as_u64().expect("expected id"), &mut self.0),
            "random" => {
                apply_random_pos(params[0].as_u64().expect("expected id"), &mut self.0, 0.02)
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
            "get" => {
                let canvas_id: u8 = params[2].as_u64().expect("invalid canvas id") as u8;
                match params[0].as_str().expect("expected str") {
                    "edge" => get_edge(canvas_id, params[1].as_str().unwrap()),
                    "vert" => get_vertex(canvas_id, params[1].as_str().unwrap()),
                    _ => Err("Could not get such object".into()),
                }
            }
            _ => Err("Invalid method call".into()),
        }
    }
    // TODO: Create a struct of response that always contains ids along with each info and return this to the frontend
    // Define how the server handle notifications.
    //
    // This server just prints the method in the console.
    fn handle_notification(&mut self, method: &str, _: &[Value]) {
        println!("Rust Backend: {}", method);
    }
}
