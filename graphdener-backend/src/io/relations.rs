use indradb::util::generate_uuid_v1;
use uuid::Uuid;
use std::collections::HashMap;
use indradb::util;
use indradb::{Datastore, Transaction, Type, Edge, EdgeKey, Vertex};
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
		NodeRelations { edge_list: vec!(), representation_type: ReprMethod::EdgeList, uuid_map: HashMap::new() }
	}
	pub fn update(&mut self, conn: (u32, u32) )
	{
		self.edge_list.push(conn);
	}

	// create a map to translate imported ids into uuids
	pub fn generate_id_map(&mut self) -> Result<&str, &str> //Vec<(u32, Uuid)>
	{
		let mut a: Vec<u32> = vec!();
		// let mut b: Vec<u32> = vec!();

		for tup in self.edge_list.iter()
		{
			a.push(tup.0);
			a.push(tup.1);
		}

		// probably would be faster if map function is used
		// a.append(&mut b);
		a.sort();
		a.dedup();

		for element in a.into_iter()
		{
			self.uuid_map.insert(element, util::generate_uuid_v1());
		}

		println!("{:#?}", self.uuid_map);
		Ok("Id to Uuid mapping created")
	}

	pub fn create_vert_edges(&self, vertex_type: Option<&String>) -> ()
	{
        println!("Creating edge...");
        let trans = statics::DATASTORE.transaction().unwrap();
        let mut v: Vertex;
        let mut e: EdgeKey;

        for pair in self.edge_list.iter()
        {
        	println!("{:?}", pair);
	        let uuid_from = self.uuid_map.get(&pair.0).unwrap();
	        let uuid_to = self.uuid_map.get(&pair.1).unwrap();
        	println!("{:?} - {:?}", uuid_from, uuid_to);

	        // TODO repeat for every node in the list
	        // TODO try serializing all queries in one transaction
	        v = Vertex::with_id(*self.uuid_map.get(&pair.0).unwrap(), Type::new(vertex_type
	        													.unwrap_or(&String::from("unknown"))
															    .to_string())
															    .unwrap());
	        let msg = trans.create_vertex(&v);

	        println!("{:?}", v);

	        // let edge_type = Type::new(String::from("unknown")).unwrap();
	        // e = EdgeKey::new(*uuid_from, edge_type, *uuid_to);

	        // trans.create_edge(&e);
			let msg = trans.get_vertex_count();
	        println!("{:?}", msg);

	    }
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

	// fn get_edges(&self) -> Vec<(i32, i32)>
	// {
	// 	self.edge_list


	// }

	
}

// Combine information about vertex connections and x,y positions and create a list of tuples that contain 
// (from_posx,from_posy,to_posx,to_posy)