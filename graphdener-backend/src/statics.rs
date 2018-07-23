use datastoremode::{datastore, ProxyDatastore};

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();
}