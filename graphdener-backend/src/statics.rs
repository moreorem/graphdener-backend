use datastore::{datastore, ProxyDatastore};

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();
}