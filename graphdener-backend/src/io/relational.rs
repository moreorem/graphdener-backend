use uuid::Uuid;


// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

enum ReprMethod
{
	EdgeList,
	AdjMatrix,
	AdjList,
}

pub struct NodeRelations<T>
{
	pub edge_list: Vec<(T, T)>,
	representation_type: ReprMethod


}

// Imported Ids and corresponding Uuids
pub struct IdUuid
{
	id: u32,
	uuid: Uuid
}

impl IdUuid
{
	fn new(len: u32) -> [IdUuid]
	{

	}
}


impl NodeRelations<i32>
{
	pub fn new() -> NodeRelations<i32>
	{
		// let el =  elist ;
		NodeRelations { edge_list: vec!(), representation_type: ReprMethod::EdgeList }
	}
	pub fn update(&mut self, conn: (i32, i32) )
	{
		self.edge_list.push(conn);
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

	fn get_edges(&self) -> Vec<(i32, i32)>
	{
		self.edge_list


	}

	
}

// Combine information about vertex connections and x,y positions and create a list of tuples that contain 
// (from_posx,from_posy,to_posx,to_posy)