use uuid::Uuid;
use std::collections::HashMap;
use graphdener::util;
use graphdener::{Datastore, Transaction, Type, EdgeKey, EdgeQuery, Vertex, VertexQuery, EdgeDirection};
use statics;
// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

// TODO: Simplify importers. Only write to database and use graphcontainer afterwards to create Uuid_map and edgelists

pub struct EdgeImporter
{
	pub edge_list: Vec<(u32, u32, u32, String, String, u8)>,
	uuid_map: HashMap<u32, Uuid>
}

impl EdgeImporter
{
	pub fn new() -> EdgeImporter
	{
		EdgeImporter { edge_list: Vec::new(), uuid_map: HashMap::new() }
	}

	pub fn update(&mut self, conn: (u32, u32, u32, &str, &str, u8) )
	{
		let a = (conn.0, conn.1, conn.2, conn.3.to_string(), conn.4.to_string(), conn.5);
		self.edge_list.push(a);
	}

	// Use this method only when there is only an edge list file
	pub fn create_edges(&self, uuid_map: &HashMap<u32, Uuid>) -> ()
	{
        println!("Storing edges to database...");
        let trans = statics::DATASTORE.transaction().unwrap();

        // iterate over every uuid in the hashmap and create each unique node into the db
        for val in self.edge_list.iter()
        {
        	// uuid_list.push(val);
        	let target = uuid_map.get(&val.2).unwrap();
        	let source = uuid_map.get(&val.1).unwrap();
        	let t = Type::new(val.4.to_owned()).unwrap();
        	let l = &val.3;
        	let w = &val.5;
        	let e = EdgeKey::new(*target, t, *source);

        	trans.create_edge(&e);
        	trans.set_edge_metadata(&EdgeQuery::Edges{keys: vec!(e.clone())}, "label", &json!(l));
        	trans.set_edge_metadata(&EdgeQuery::Edges{keys: vec!(e)}, "weight", &json!(w));

        }
	}

}

pub struct NodeImporter
{
	node_list: Vec<(u32, String, String)>,
	type_map: HashMap<String, Vec<Uuid>>,
	uuid_map: HashMap<u32, Uuid> // PENDING: Deprecated delete if sure
}

impl NodeImporter
{
	pub fn new() -> NodeImporter
	{
		NodeImporter {	node_list: Vec::new(),
						type_map: HashMap::new(),
						uuid_map: HashMap::new()
					 }
	}

	pub fn update(&mut self, id_label_type: (u32, &str, &str) )
	{
		let a = (id_label_type.0, id_label_type.1.to_string(), id_label_type.2.to_string());
		self.node_list.push(a);
	}

	pub fn generate_id_map(&mut self) -> Result<HashMap<u32, Uuid>, bool>
	{
		let mut a: Vec<u32> = Vec::new();
		let mut uuid_map: HashMap<u32, Uuid> = HashMap::new();

		for tup in &self.node_list
		{
			a.push(tup.0);
		}

		// probably would be faster if map function is used
		a.sort();
		a.dedup();

		for element in a.into_iter()
		{
			let uuid = util::generate_uuid_v1();
			uuid_map.insert(element, uuid);
		}

		// Update instance map
		self.uuid_map = uuid_map.clone();
		Ok(uuid_map)
	}

	pub fn generate_type_map(&mut self) -> Result<bool, bool>
	{
		// Create a hashmap with type as key and vector of uuids as the values that belong to that type
		for tup in &self.node_list
		{
			let t = tup.2.clone();
			let id = tup.0.clone();
			let uuid = *self.uuid_map.get(&id).unwrap();
			let mut last_entry: Vec<Uuid>;

			// Search type_map for a type that has been read from node_list. If it exists already
			// push the next id into its value vector
			if let Some(x) = &self.type_map.get(&t) {
				last_entry = x.to_vec();
			}
			else {
				last_entry = Vec::new();
			}

			last_entry.push(uuid);

			&self.type_map.insert(t, last_entry);
		}
		Ok(true)
	}

	pub fn create_vertices(&self) -> ()
	{
        println!("Storing vertices to database...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut v: Vertex;

        let mut uuid_list: Vec<&Uuid> = Vec::new();

        // iterate over every uuid in the hashmap and create each unique node into the db
        for (key,val) in self.type_map.iter()
        {
        	for uuid in val.iter()
        	{
	        	v = Vertex::with_id(*uuid, Type::new(key
											    .to_string())
											    .unwrap());

    	    	let msg = trans.create_vertex(&v);
    	    }
        }
		let msg = trans.get_vertex_count();
	}

}

// Initializes the vertex attributes to be changed in every frame
pub fn initialize_spatial()
{
    let trans = statics::DATASTORE.transaction().unwrap();
    let v = VertexQuery::All{ start_id: None, limit: 1000000000 };
    trans.set_vertex_metadata(&v, "pos", &json!([0., 0.]));
    trans.set_vertex_metadata(&v, "size", &json!(1.));
    trans.set_vertex_metadata(&v, "color", &json!((165,0,255)));
    trans.set_vertex_metadata(&v, "label", &json!(""));

}