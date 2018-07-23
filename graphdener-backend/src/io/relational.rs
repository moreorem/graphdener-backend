// Contains one or more ways of temporarily storing node relations. It usually contains an edge list, directions, or even weights

pub struct NodeRelations
{
	pub elist: Vec<()>,

}


impl NodeRelations
{
	pub fn new() -> NodeRelations
	{
		// let el =  elist ;
		NodeRelations { elist: vec!() }
	}
	pub fn update(&mut self, con: (i32, i32) )
	{
		self.elist.push(())
	}
}