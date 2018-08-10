use std::fmt;


pub struct Spatial
{
	pos: [f64; 2],
	size: f32,
}



impl Spatial
{
	pub fn new() -> Spatial
	{
		Spatial { pos: [0.,0.], size: 1. }
	}

	pub fn update_pos(&mut self, d_x: f64, d_y: f64)
	{
		self.pos[0] += d_x;
		self.pos[1] += d_y;
	}

	pub fn update_size(&mut self, d_r: f32)
	{
		self.size += d_r;
	}
}



impl fmt::Display for Spatial
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        write!(f, "Coordinates(pos.x: {}, pos.y: {}),\nsize({})", 
        	self.pos[0], self.pos[1], self.size)
    }
}

impl fmt::Debug for Spatial {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.pos[..].fmt(formatter)
    }
}


#[cfg(test)]
#[test]
fn printing()
{
	let s = Spatial::new();
	// println!("{}",s);
	
}



