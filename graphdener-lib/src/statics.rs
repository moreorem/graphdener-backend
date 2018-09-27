// use uuid::Uuid;
use graphdenerdb::MemoryDatastore;
use std::collections::{BTreeMap, HashMap};

lazy_static! {
    /// The underlying datastore
    // pub static ref DATASTORE: ProxyDatastore = datastore();
    pub static ref DATASTORE: MemoryDatastore = MemoryDatastore::default();
}
