use datastore::{datastore, ProxyDatastore};
use io::relations::NodeRelations;

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();
}