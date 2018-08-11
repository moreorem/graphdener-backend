use std::fmt;


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Color(u8,u8,u8);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spatial
{
	pos: [f64; 2],
	size: f32,
	color: Color
}

impl Spatial
{
	pub fn new(pos: [f64; 2], size: f32, color: (u8, u8, u8)) -> Spatial
	{
		Spatial { pos, size, color: Color(color.0,color.1,color.2) }
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

	pub fn set_color(&mut self, c:(u8, u8, u8))
	{
		self.color = Color(c.0, c.1, c.2);
	}


}

impl Default for Spatial
{
	fn default() -> Self
	{
		Self { pos: [0.,0.], size: 1., color: Color(165, 0, 255) }
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





#[cfg(test)]
#[test]
fn printing()
{
	let s = Spatial::new();
	// println!("{}",s);
	
}



