use graph::graph::GraphContainer;
use uuid::Uuid;
use std::collections::{HashMap, BTreeMap};
use graphdener::MemoryDatastore;

lazy_static! {
    /// The underlying datastore
    // pub static ref DATASTORE: ProxyDatastore = datastore();
    pub static ref DATASTORE: MemoryDatastore = MemoryDatastore::default();
    pub static ref GRAPH: BTreeMap<usize, GraphContainer> = BTreeMap::new();
}

