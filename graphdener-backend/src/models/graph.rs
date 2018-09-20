use commands::calcs::{create_uid_map, find_neighbors};
use commands::database;
use uuid::Uuid;
use std::collections::{BTreeMap, HashMap};
use super::nodes::Node;

#[derive(Clone, Debug)]
pub struct GraphContainer(pub BTreeMap<u8, Graph>);

impl GraphContainer
{
	pub fn default() -> GraphContainer
	{
		Self {
			0: BTreeMap::new()
		}
	}

	pub fn add_graph(&mut self, id: u8)
	{
		self.0.entry(id).or_insert(Graph::new());
	}

	pub fn get_mut_graph(&mut self, id: u8) -> &mut Graph
	{
		self.0.get_mut(&id).unwrap()
	}

	pub fn get_graph(&self, id: u8) -> Result<&Graph, String>
	{
		if let Some(x) = self.0.get(&id) {
			Ok(x)
		}
		else {
			Err(format!("Cannot find any graphs with id {}", id))
		}

	}
}

#[derive(Clone, Debug)]
pub struct Graph
{
	idx_map: HashMap<Uuid, usize>,
	nodes: Vec<Node>
}

impl Graph {
	pub fn new() -> Graph
	{
		Graph {
			idx_map: HashMap::new(),
			nodes: Vec::new()
		}
	}

	pub fn populate(&mut self)
	{
	    let count = database::count();
	    let idx_map: HashMap<Uuid, usize>;
	    
	    // TODO: Set the start_id according to the last of previous graph
	    // if graph_id == 1 {
	    //     let start_id = ...
	    // }
	    let mut id: usize = 1;

	    // PENDING: Move error checking to database
	    if let Ok(x) = database::get_graph_vertices(None) 
	    {
	        idx_map = create_uid_map(x, &mut self.nodes);
	    	find_neighbors(&mut self.nodes, &idx_map);
	    }
	    else {
	        println!("No vertices found");
	    }
	}

	pub fn get_positions(&self) -> Vec<[f64; 2]>
	{
		self.nodes.iter().map(|x| x.pos.get()).collect()
	}

	pub fn get_types(&self) -> Vec<String>
	{
		self.nodes.iter().map(|x| x.get_type()).collect()
	}

	pub fn get_adj_list(&self) -> Vec<[u64; 2]>
	{
		let mut list: Vec<[u64; 2]> = Vec::new();
		let mut a: [u64; 2] = [0, 0];

		// TODO: Add weights in output or create a separate method for that
		for (id, node) in self.nodes.iter().enumerate()
		{
			for neighbor in node.neighbors.iter() {
				a[0] = id as u64;
				a[1] = neighbor.clone() as u64;
				list.append(&mut vec![a]);
			}
			
		}
		list
	}

	pub fn modify_nodes(&mut self) -> &mut Vec<Node>
	{
		&mut self.nodes
	}

	pub fn count(&self) -> usize
	{
		self.nodes.len()
	}

}