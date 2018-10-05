use super::database;
use graphdenerdb::Vertex;
use models::nodes::Node;
use std::collections::HashMap;
use uuid::Uuid;

pub fn create_uid_map(vertices: Vec<Vertex>, nodes: &mut Vec<Node>) -> HashMap<Uuid, usize> {
    let count = vertices.len();
    // Map to translate Uuids to Ids
    let mut idx_map: HashMap<Uuid, usize> = HashMap::with_capacity(count as usize);

    // First create Uuid to Id translation map
    for (idx, vert) in vertices.iter().enumerate() {
        idx_map.insert(vert.id, idx);

        // Create Node struct for current node without neighbors but with type and random pos
        let node = Node::new(idx, (0.0, 0.0), None, String::from(vert.t.0.clone()));
        nodes.insert(idx, node);
    }
    idx_map
}

pub fn find_neighbors(
    nodes: &mut Vec<Node>,
    idx_map: &HashMap<Uuid, usize>,
) -> Result<bool, &'static str> {
    let mut surrounding_verts: Vec<Vertex>;
    // Iterate again to find neighbors for every node
    for (uuid, id) in idx_map.iter() {
        // Find neighbors for current node
        if let Ok(x) = database::get_vertex_neighbors(*uuid) {
            // Ignore absence of neighbors
            if x.len() == 0 {
                continue;
            }
            surrounding_verts = x;
        } else {
            return Err("problem getting neighbors");
        }

        let neighbors: Vec<usize> = surrounding_verts
            .iter()
            .map(|x| *idx_map.get(&x.id).unwrap())
            .collect();
        nodes[*id].neighbors = neighbors;
    }
    Ok(true)
}
