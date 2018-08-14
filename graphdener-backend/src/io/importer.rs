use indradb::util::generate_uuid_v1;
use uuid::Uuid;
use std::collections::HashMap;
use indradb::util;
use serde_json::Value;
use indradb::{Datastore, Transaction, Type, Edge, EdgeKey, Vertex, VertexQuery};
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

		println!("{:#?}", self.uuid_map);
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
	node_list: Vec<(u32, &str, &str)>,
	label_list: Vec<String>
	uuid_map: HashMap<u32, Uuid>
}

impl NodeImporter
{
	fn get_type(&self) -> &str
	{
		match self.representation_type
		{
			ReprMethod::EdgeList => "Edge List",
			ReprMethod::AdjMatrix => "Adjacency Matrix",
			ReprMethod::AdjList => "Adjacency List"
		}
	}

	pub fn update(&mut self, id_label_type: (u32, &str, &str) )
	{
		self.node_list.push(id_label_type);
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

		for element in a.into_iter()
		{
			self.uuid_map.insert(element, util::generate_uuid_v1());
		}

		println!("{:#?}", self.uuid_map);
		Ok(true)
	}

	pub fn generate_type_map(&mut self) -> Result<bool, bool>
	{
		let mut a: Vec<u32> = Vec::new();

		for tup in &self.type_list
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

		println!("{:#?}", self.uuid_map);
		Ok(true)
	}
}

pub fn initialize_spatial(&self)
{
    let trans = statics::DATASTORE.transaction().unwrap();
    let v = VertexQuery::All{ start_id: None, limit: 1000000000 };
    trans.set_vertex_metadata(&v, "pos", &json!([0.,0.]));
    trans.set_vertex_metadata(&v, "size", &json!(1.));
    trans.set_vertex_metadata(&v, "color", &json!((165,0,255)));
}