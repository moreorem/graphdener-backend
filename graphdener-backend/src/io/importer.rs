use indradb::util::generate_uuid_v1;
use uuid::Uuid;
use std::collections::HashMap;
use indradb::util;
use serde_json::Value;
use indradb::{Datastore, Transaction, Type, Edge, EdgeKey, Vertex, VertexQuery};
use statics;

// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

enum ReprMethod
{
	EdgeList,
	AdjMatrix,
	AdjList,
}

pub struct NodeRelations
{
	pub edge_list: Vec<(u32, u32)>,
	representation_type: ReprMethod,
	uuid_map: HashMap<u32, Uuid>

}


impl NodeRelations
{
	pub fn new() -> NodeRelations
	{
		// let el =  elist ;
		NodeRelations { edge_list: Vec::new(), representation_type: ReprMethod::EdgeList, uuid_map: HashMap::new() }
	}


	pub fn update(&mut self, conn: (u32, u32) )
	{
		self.edge_list.push(conn);
	}


	// create a map to translate imported ids into uuids
	pub fn generate_id_map(&mut self) -> Result<bool, bool> //Vec<(u32, Uuid)>
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


	pub fn create_vertices(&self, vertex_type: Option<&String>) -> ()
	{
        println!("Storing vertices to database...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut v: Vertex;
        // let mut e: EdgeKey;

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


	fn get_type(&self) -> &str
	{
		match self.representation_type
		{
			ReprMethod::EdgeList => "Edge List",
			ReprMethod::AdjMatrix => "Adjacency Matrix",
			ReprMethod::AdjList => "Adjacency List"
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

	// UNFINISHED
	pub fn create_edges() -> ()
	{
		 // for pair in self.edge_list.iter()
        // {
        // 	println!("{:?}", pair);
	       //  let uuid_from = self.uuid_map.get(&pair.0).unwrap();
	       //  let uuid_to = self.uuid_map.get(&pair.1).unwrap();
        // 	println!("{:?} - {:?}", uuid_from, uuid_to);

	        // TODO repeat for every node in the list
	        // TODO try serializing all queries in one transaction
	        // v = Vertex::with_id(uuid_list,



	        // 	*self.uuid_map.get(&pair.0).unwrap(), Type::new(vertex_type
	        // 													.unwrap_or(&String::from("unknown"))
									// 						    .to_string())
									// 						    .unwrap());
	        

	        // let edge_type = Type::new(String::from("unknown")).unwrap();
	        // e = EdgeKey::new(*uuid_from, edge_type, *uuid_to);

	        // trans.create_edge(&e);
	    
	    // }
	}
}

// Combine information about vertex connections and x,y positions and create a list of tuples that contain 
// (from_posx,from_posy,to_posx,to_posy)