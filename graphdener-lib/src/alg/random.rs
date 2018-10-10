use af::Dim4;
use af::{randn, randu, Array};
use models::graph::Graph;

// TODO: Improve Randomizer with faster code
pub fn random_pos(graph: &mut Graph, min_distance: f64, spread_factor: u8) -> &str {
    fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powf(2.) + (p1.1 - p2.1).powf(2.)).sqrt()
    }
    let n = graph.count();
    let mut posx: Vec<f64> = Vec::with_capacity(n);
    let mut posy: Vec<f64> = Vec::with_capacity(n);

    // TODO: Make only one array of nx2
    let l = n as f64;
    // let multiplier: Array<f64> = randu::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * l.ln(); //af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));

    let positionx: Array<f64> =
        randu::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * l.ln() * spread_factor; //&multiplier; //af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));
    let positiony: Array<f64> =
        randu::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * l.ln() * spread_factor; //&multiplier;
                                                                                // * af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));

    posx.resize(positionx.elements(), 0.0);
    posy.resize(positiony.elements(), 0.0);

    positionx.host(&mut posx);
    positiony.host(&mut posy);
    let positions: Vec<(f64, f64)> = posx.into_iter().zip(posy).collect();

    graph.set_positions(positions);
    "applied random pos"
}
