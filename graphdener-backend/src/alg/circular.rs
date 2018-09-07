use std::f64::consts::PI;

fn r_from_circumference(circumference: f64) -> f64
{
	circumference/(2.*PI)
}

fn polar_to_rect(theta: f64, radius: f64) -> (f64, f64)
{
	let y = radius * theta.sin();
	let x = radius * theta.cos();
	(x, y)
}

pub fn polygon(n: u32) ->  Vec<(f64, f64)>
{
	let point_size: f64 = 1.;
	let circumference = point_size * (n as f64)/PI;
	let sides = n;
	let degrees = 360./(sides as f64); // Find the angle between corners in degrees 
	let r = r_from_circumference(circumference);
	let mut result: Vec<(f64, f64)> = Vec::new();
	let mut theta = 0.;

	while theta < 360. 
	{
		let point = polar_to_rect(theta, r);
		result.append(&mut vec!(point));
		theta = theta + degrees;
	}
	result
}

#[cfg(test)]
mod tests {
    use alg::circular::polygon;
    #[test]
    fn test_poly()
    {  
        println!("{:?}", polygon(5))
    }
}