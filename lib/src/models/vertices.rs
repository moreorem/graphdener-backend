use super::types::Type;
use super::spatial::Spatial;
use util::generate_uuid_v1;
use uuid::Uuid;

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: Uuid,

    /// The type of the vertex.
    #[serde(rename = "type")]
    pub t: Type,

    // NEW, add x,y positions in the surface
    // pub pos: [f64; 2],
    pub spatial: Spatial,
    // NEW, add label to add a name for every node
    pub label: String
}

impl Vertex {
    /// Creates a new vertex with an ID generated via UUIDv1. These vertex IDs
    /// are trivially guessable and consequently less secure, but likely index
    /// better depending on the datastore. This method is suggested unless you
    /// need vertex IDs to not be trivially guessable.
    ///
    /// # Arguments
    ///
    /// * `t` - The type of the vertex.
    pub fn new(t: Type) -> Self {
        Self::with_id(generate_uuid_v1(), t, Spatial::default())
    }

    /// Creates a new vertex with a specified id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `t` - The type of the vertex.
    /// * `s` - The spatial info of the vertex.
    pub fn with_id(id: Uuid, t: Type, s: Spatial) -> Self {
        Vertex { 
                id: id,
                t: t,
                spatial: s,
                label: String::from("label") // Default value until it gets implemented to receive input label.unwrap_or("".to_string()) 
            }
    }

}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {}
