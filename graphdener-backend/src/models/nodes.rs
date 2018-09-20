#[derive(Copy, Clone, Debug)]
pub struct Position {
	x: f64,
	y: f64
}

impl Position {
	fn new(x: f64, y: f64) -> Position
	{
		Position {x, y}
	}

	pub fn get(&self) -> [f64; 2]
	{
		[self.x, self.y]
	}

	pub fn set(&mut self, x: f64, y: f64)
	{
		self.x = x;
		self.y = y; 
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Force {
	x: f32,
	y: f32,
}

impl Force {
	fn new(x: f32, y: f32) -> Force
	{
		Force {x, y}
	}
	
	pub fn get(&self) -> (f32, f32 )
	{
		(self.x, self.y)
	}

	pub fn set(&mut self, x: f32, y: f32)
	{
		self.x = x;
		self.y = y; 
	}
}

#[derive(Clone, Debug)]
pub struct Node {
	pub id: usize,
	pub pos: Position,
	pub force: Force,
	pub neighbors: Vec<usize>,
	pub typ: String
}

impl Node
{
	pub fn new(id: usize, position: (f64, f64), neighbors: Option<Vec<usize>>, typ: String) -> Node
	{
		Node 
		{
			id, 
			pos: Position::new(position.0, position.1),
			force: Force::new(0., 0.),
			neighbors: if let Some(x) = neighbors {
				x
			}
			else {
			    Vec::new()
			},
			typ
		}
	}

	pub fn get_type(&self) -> String
	{
		self.typ.clone()
	}

}