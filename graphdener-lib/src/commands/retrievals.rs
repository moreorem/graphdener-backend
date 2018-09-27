use models::graph::GraphContainer;
// use commands::calcs::{get_adj_list};
use graphdenerdb::{Datastore, Edge, Transaction, Vertex, VertexQuery};
use rmp_rpc::Value;
use statics;

pub struct Retriever;

// TODO: Execute this command concurrently with the algorithm update
// NEW ORIGIN
pub fn get_pos(graph_id: u64, container: &GraphContainer) -> Result<Value, Value> {
    let id = graph_id as u8;
    println!("{}", id);
    // Get positions from Graph struct with specific id
    let graph = container.get_graph(id).expect("Err");
    let positions = graph.get_positions();
    let nodepos: Vec<Value> = positions
        .iter()
        .map(|x| Value::Array(vec![Value::from(x[0]), Value::from(x[1])]))
        .collect();

    Ok(Value::from(nodepos))
}

// NEW ORIGIN
pub fn get_adjacency(graph_id: u64, container: &GraphContainer) -> Result<Value, Value> {
    let id = graph_id as u8;

    let graph = container.get_graph(id).expect("Err");
    let adjacency = graph.get_adj_list();

    let adj_list: Vec<Value> = adjacency
        .iter()
        .map(|ab| Value::Array(vec![Value::from(ab[0]), Value::from(ab[1])]))
        .collect();

    Ok(Value::from(adj_list))
}

// NEW ORIGIN
pub fn get_node_type(graph_id: u64, container: &GraphContainer) -> Result<Value, Value> {
    let id = graph_id as u8;

    let graph = container.get_graph(id).expect("Err");
    let types = graph.get_types();

    let typelist: Vec<Value> = types
        .iter()
        .map(|ab| Value::Array(vec![(ab.as_str().into())]))
        .collect();

    Ok(Value::from(typelist))
}

// Returns specific info about a set or all of the vertices that exist in the database to the frontend
pub fn get_vertex(canvas_id: u8, info_type: &str) -> Result<Value, Value> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v: VertexQuery;
    v = VertexQuery::All {
        start_id: None,
        limit: 1000000000,
    };

    // In this case the msg variable is of type model::Vertex. It has to be broken into the struct items to be used
    let draft_info = trans.get_vertices(&v).unwrap();
    println!("asked about {}", &info_type);
    Ok(vert_info(info_type, draft_info))
}

fn vert_info(info_type: &str, draft_model: Vec<Vertex>) -> Value {
    // map all of the vectors in the response to one vector
    let r_iter = draft_model.iter();

    // return the array of specific detail type for all of the selected vertices according to the command
    match info_type {
        "type" => Value::Array(r_iter.map(|x| Value::from(x.t.0.to_owned())).collect()),
        "pos" => Value::Array(get_v_attribute("pos")),
        "size" => Value::Array(get_v_attribute("size")),
        "color" => Value::Array(get_v_attribute("color")),
        "label" => Value::Array(get_v_attribute("label")),
        _ => Value::from(format!("No such info: {}", info_type)),
    }
}

// make a getter only for edge types, weight, direction and fromto
pub fn get_edge(canvas_id: u8, info_type: &str) -> Result<Value, Value> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let edge_list_available: bool;

    let draft_info = trans
        .get_edges(
            &VertexQuery::All {
                start_id: None,
                limit: 1000000,
            }.outbound_edges(None, None, None, None, 1000000),
        ).unwrap();
    Ok(edge_info(info_type, draft_info))
}

fn edge_info(info_type: &str, draft_model: Vec<Edge>) -> Value {
    // map all of the vectors in the response to one vector
    let r_iter = draft_model.iter();

    // return the array of specific detail type for all of the selected vertices according to the command
    match info_type {
        "type" => Value::Array(r_iter.map(|x| Value::from(x.key.t.0.to_owned())).collect()),
        // "pos" => Value::Array(get_adj_list() ),
        "label" => Value::Array(get_e_attribute("label")),
        "weight" => Value::Array(get_e_attribute("weight")),
        _ => Value::from("error"),
    }
}

// Returns one of the attributes that reside in the metadata map of each vertex
fn get_v_attribute(kind: &str) -> Vec<Value> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v = VertexQuery::All {
        start_id: None,
        limit: 1000000000,
    };

    let t = match kind {
        "pos" => trans.get_vertex_metadata(&v, "pos").unwrap(),
        "size" => trans.get_vertex_metadata(&v, "size").unwrap(),
        "color" => trans.get_vertex_metadata(&v, "color").unwrap(),
        "label" => trans.get_vertex_metadata(&v, "label").unwrap(),
        _ => vec![],
    };

    t.iter().map(|x| Value::from(x.value.to_string())).collect() // TODO: Find a way to return a float instead of string
}

fn get_e_attribute(kind: &str) -> Vec<Value> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let e = &VertexQuery::All {
        start_id: None,
        limit: 100000000,
    }.outbound_edges(None, None, None, None, 1000000000);
    let t = match kind {
        "weight" => trans.get_edge_metadata(&e, "weight").unwrap(),
        "label" => trans.get_edge_metadata(&e, "label").unwrap(),
        _ => vec![],
    };

    t.iter().map(|x| Value::from(x.value.to_string())).collect() // TODO: Find a way to return a float instead of string
}
