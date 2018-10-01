use graphdenerdb::VertexMetadata;
use graphdenerdb::{
    Datastore, Edge, EdgeDirection, EdgeKey, EdgeQuery, Error, Transaction, Type, Vertex,
    VertexQuery,
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

pub fn get_graph_edges(start_id: Option<Uuid>) -> Result<Vec<Edge>, Error> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let e = EdgeQuery::Pipe {
        vertex_query: Box::new(VertexQuery::All {
            start_id: None,
            limit: LIMIT,
        }),
        converter: EdgeDirection::Outbound,
        limit: LIMIT,
        low_filter: None,
        high_filter: None,
        type_filter: None,
    };
    trans.get_edges(&e)
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

pub fn get_vertex_metadata(uuid: Option<Uuid>, name: &str) -> Result<Vec<VertexMetadata>, Error> {
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

pub fn create_edges(source: Uuid, t: String, target: Uuid) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let tp = Type::new(t).unwrap();
    let e = EdgeKey::new(target, tp, source);

    trans.create_edge(&e);
}

pub fn create_vertices(pair: (Uuid, String)) -> Result<bool, Error> {
    let trans = statics::DATASTORE.transaction().unwrap();
    let v = Vertex::with_id(pair.0, Type::new(pair.1).unwrap());
    trans.create_vertex(&v)
}

pub fn set_edge_metadata(from: Uuid, t: String, to: Uuid, data: (String, String)) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let e = EdgeQuery::Edges {
        keys: vec![EdgeKey::new(to, Type::new(t).unwrap(), from)],
    };

    trans.set_edge_metadata(&e, &data.0, &json!(data.1));
}

pub fn get_edge_metadata(uuid: Option<Uuid>, data: String, typ: Option<String>) -> () {
    let trans = statics::DATASTORE.transaction().unwrap();
    let edgetype: String;

    let e = EdgeQuery::Pipe {
        vertex_query: Box::new(VertexQuery::All {
            start_id: None,
            limit: LIMIT,
        }),
        converter: EdgeDirection::Outbound,
        limit: LIMIT,
        low_filter: None,
        high_filter: None,
        type_filter: None,
    };

    trans.get_edge_metadata(&e, &data);
}
