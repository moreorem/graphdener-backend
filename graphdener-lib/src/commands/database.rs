use graphdenerdb::VertexMetadata;
use graphdenerdb::{
    Datastore, Edge, EdgeKey, EdgeQuery, Error, Transaction, Type, Vertex, VertexQuery,
};
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

pub fn set_vertex_position(uuid: Option<Uuid>, position: [f64; 2]) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v: VertexQuery;

    if let Some(x) = uuid {
        v = VertexQuery::Vertices { ids: vec![x] };
    } else {
        v = VertexQuery::All {
            start_id: None,
            limit: LIMIT,
        };
    }
    trans.set_vertex_metadata(&v, "pos", &json!(position));
}

pub fn set_vertex_metadata(uuid: Option<Uuid>, data: (String, String)) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v: VertexQuery;

    if let Some(x) = uuid {
        v = VertexQuery::Vertices { ids: vec![x] };
    } else {
        v = VertexQuery::All {
            start_id: None,
            limit: LIMIT,
        };
    }

    trans.set_vertex_metadata(&v, &data.0, &json!(data.1));
}

pub fn get_vertex_metadata(uuid: Option<Uuid>, name: &str) -> Result<Vec<VertexMetadata>, Error>
{
    let trans = statics::DATASTORE.transaction().unwrap();
    let v: VertexQuery;

    if let Some(x) = uuid {
        v = VertexQuery::Vertices { ids: vec![x] };
    } else {
        v = VertexQuery::All {
            start_id: None,
            limit: LIMIT,
        };
    }
    trans.get_vertex_metadata(&v, name)
}

pub fn create_edges(
    target: Uuid,
    t: Type,
    source: Uuid,
    label: Option<String>,
    weight: Option<String>,
) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();

    let e = EdgeKey::new(target, t, source);

    trans.create_edge(&e);

    trans.set_edge_metadata(
        &EdgeQuery::Edges {
            keys: vec![e.clone()],
        },
        "label",
        &json!(label),
    );

    trans.set_edge_metadata(
        &EdgeQuery::Edges { keys: vec![e] },
        "weight",
        &json!(weight),
    );
}
