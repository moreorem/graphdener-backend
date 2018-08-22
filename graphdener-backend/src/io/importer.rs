use graphdener::util::generate_uuid_v1;
use uuid::Uuid;
use std::collections::HashMap;
use graphdener::util;
use serde_json::Value;
use graphdener::{Datastore, Transaction, Type, Edge, EdgeKey, Vertex, VertexQuery};
use statics;
// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights
pub struct EdgeImporter
{
	pub edge_list: Vec<(u32, u32)>,
	uuid_map: HashMap<u32, Uuid>
}

impl EdgeImporter
{
	pub fn new() -> EdgeImporter
	{
		EdgeImporter { edge_list: Vec::new(), uuid_map: HashMap::new() }
	}

	pub fn update(&mut self, conn: (u32, u32) )
	{
		self.edge_list.push(conn);
	}

	// create a map to translate imported ids into uuids
	// Use this only when the data are imported only from an edge list file
	pub fn generate_id_map(&mut self) -> Result<bool, bool>
	{
		let mut a: Vec<u32> = Vec::new();

		for tup in &self.edge_list
		{
			a.push(tup.0);
			a.push(tup.1);
		}

		// probably would be faster if map function is used
		a.sort();
		a.dedup();

		for element in a.into_iter()
		{
			self.uuid_map.insert(element, util::generate_uuid_v1());
		}

		println!("IdMap: {:#?}", self.uuid_map);
		Ok(true)
	}

	// Use this method only when there is only an edge list file
	pub fn create_vertices(&self, vertex_type: Option<&String>) -> ()
	{
        println!("Storing vertices to database...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut v: Vertex;

        let mut uuid_list: Vec<&Uuid> = Vec::new();

        // iterate over every uuid in the hashmap and create each unique node into the db
        for val in self.uuid_map.values()
        {
        	uuid_list.push(val);
        	v = Vertex::with_id(*val, Type::new(vertex_type
												.unwrap_or(&String::from("unknown"))
											    .to_string())
											    .unwrap());

        	let msg = trans.create_vertex(&v);

        }

		let msg = trans.get_vertex_count();
        println!("{:?}", msg);
	}

	
}
// Combine information about vertex connections and x,y positions and create a list of tuples that contain 
// (from_posx,from_posy,to_posx,to_posy)

pub struct NodeImporter
{
	node_list: Vec<(u32, String, String)>,
	type_map: HashMap<String, Vec<Uuid>>,
	uuid_map: HashMap<u32, Uuid>
}

impl NodeImporter
{
	pub fn new() -> NodeImporter
	{
		NodeImporter {	node_list: Vec::new(),
						type_map: HashMap::new(),
						uuid_map: HashMap::new() }
	}

	pub fn update(&mut self, id_label_type: (u32, &str, &str) )
	{
		let a = (id_label_type.0, id_label_type.1.to_string(), id_label_type.2.to_string());
		self.node_list.push(a);
	}

	pub fn generate_id_map(&mut self) -> Result<bool, bool>
	{
		let mut a: Vec<u32> = Vec::new();

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
			&self.uuid_map.insert(element, uuid);
		}
		Ok(true)
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


		println!("TypeMap: {:#?}", &self.type_map);
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
        println!("{:?}", msg);
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