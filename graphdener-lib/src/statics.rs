// use uuid::Uuid;
use graphdenerdb::MemoryDatastore;

lazy_static! {
    /// The underlying datastore
    // pub static ref DATASTORE: ProxyDatastore = datastore();
    pub static ref DATASTORE: MemoryDatastore = MemoryDatastore::default();



}
pub static RECOGNIZED_NAMES: [&str; 9] = [
    "n_id", "n_label", "n_type", "e_id", "e_from", "e_to", "e_type", "e_label", "e_weight",
];

pub static RECOGNIZED_TYPES: [&str; 9] = [
    "int", "meta", "str", "int", "int", "int", "str", "str", "dec",
];
