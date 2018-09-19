use commands::calcs::{LIMIT, create_uid_map, find_neighbors};
use uuid::Uuid;
use std::collections::{BTreeMap, HashMap};
use super::nodes::Node;
use statics;
use graphdener::{Datastore, Transaction, VertexQuery};

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

	pub fn get_graph(&self, id: u8) -> &Graph
	{
		self.0.get(&id).unwrap()	
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
    	// let mut nodes = &mut self.nodes;
	    let trans = statics::DATASTORE.transaction().unwrap();
	    let count = trans.get_vertex_count().unwrap();
	    let idx_map: HashMap<Uuid, usize>;
	    
	    // TODO: Set the start_id according to the last of previous graph
	    // if graph_id == 1 {
	    //     let start_id = ...
	    // }
	    let mut id: usize = 1;

	    if let Ok(x) = trans.get_vertices(&VertexQuery::All{ start_id: None, limit: LIMIT })
	    {
	        idx_map = create_uid_map(x, &mut self.nodes);
	    	find_neighbors(trans, &mut self.nodes, &idx_map);
	    }
	    else {
	        println!("No vertices found");
	    }
	}

	pub fn get_positions(&self) -> Vec<[f64; 2]>
	{
		self.nodes.iter().map(|x| x.pos.get()).collect()
	}
}