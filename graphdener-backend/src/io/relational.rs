// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

pub struct NodeRelations<T>
{
	pub elist: Vec<(T, T)>,

}


impl NodeRelations<i32>
{
	pub fn new() -> NodeRelations<i32>
	{
		// let el =  elist ;
		NodeRelations { elist: vec!() }
	}
	pub fn update(&mut self, con: (i32, i32) )
	{
		self.elist.push((con))
	}
}