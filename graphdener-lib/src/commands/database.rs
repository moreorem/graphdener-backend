use graphdenerdb::{Datastore, Error, Transaction, Vertex, VertexQuery};
use statics;
use uuid::Uuid;

pub const LIMIT: u32 = 10000000;
pub const EDGE_LIMIT: u32 = 100;

pub fn count() -> u64 {
    let trans = statics::DATASTORE.transaction().unwrap();
    trans.get_vertex_count().unwrap()
}

pub fn get_graph_vertices(start_id: Option<Uuid>) -> Result<Vec<Vertex>, Error> {
    let trans = statics::DATASTORE.transaction().unwrap();
    trans.get_vertices(&VertexQuery::All {
        start_id: None,
        limit: LIMIT,
    })
}

pub fn get_vertex_neighbors(uuid: Uuid) -> Result<Vec<Vertex>, Error> {
    let trans = statics::DATASTORE.transaction().unwrap();
    trans.get_vertices(
        &VertexQuery::Vertices { ids: vec![uuid] }
            .outbound_edges(None, None, None, None, EDGE_LIMIT)
            .inbound_vertices(EDGE_LIMIT),
    )
}

// pub fn get_vertex_types(uuid: Uuid) -> Result<Vec<Vertex>, Error>
// {
//  let trans = statics::DATASTORE.transaction().unwrap();
//  trans.get_vertex_metadata(&VertexQuery::Vertices{ ids: vec!(uuid) }.outbound_edges(None, None, None, None, EDGE_LIMIT).inbound_vertices(EDGE_LIMIT))
// }
