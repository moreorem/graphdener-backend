use std::collections::{BTreeMap, HashSet};
use super::nodes::Node;

#[derive(Clone, Debug)]
pub struct GraphContainer(BTreeMap<u8, Graph>);

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
		self.0.insert(id, Graph::new());
	}
}

#[derive(Clone, Debug)]
pub struct Graph
{
	id: u8,
	nodes: BTreeMap<usize, Node>
}

impl Graph {
	pub fn new() -> Graph
	{
		Graph {
			id: 1, // FIXME: Receive iteration of ids
			nodes: BTreeMap::new()
		}
	}
}