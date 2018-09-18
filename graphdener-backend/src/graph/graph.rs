use std::collections::{BTreeMap, HashSet};
use super::nodes::Node;

pub struct GraphContainer
{
	graphs: BTreeMap<usize, Graph>
}


struct Graph
{
	nodes: BTreeMap<usize, Node>
}