use alg::random::af::RandomEngineType::PHILOX_4X32_10;
use models::graph::Graph;
use rand::prelude::*;
extern crate arrayfire as af;
use self::af::Dim4;
use self::af::RandomEngine;
use self::af::{randn, randu};

// TODO: Improve Randomizer with faster code
pub fn random_pos(graph: &mut Graph, min_distance: f64) -> &str {
    fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        ((p1.0 - p2.0).powf(2.) + (p1.1 - p2.1).powf(2.)).sqrt()
    }
    let n = graph.count();
    println!("{}", n);
    let mut rng = thread_rng();
    // let mut positions = Vec::with_capacity(n);
    let spread = (n as f64).log10();
    println!("{}", spread);
    let mut posx: Vec<f64> = Vec::new();
    let mut posy: Vec<f64> = Vec::new();

    // TODO: Make only one array of nx2
    let multiplier: af::Array<f64> = randn::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * 50; //af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));

    let positionx: af::Array<f64> = randu::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * &multiplier; //af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));
    let positiony: af::Array<f64> = randu::<f64>(Dim4::new(&[1, n as u64, 1, 1])) * &multiplier;
    // * af::Array::new(&[1.002], Dim4::new(&[1, n as u64, 1, 1]));?

    posx.resize(positionx.elements(), 0.0);
    posy.resize(positiony.elements(), 0.0);

    positionx.host(&mut posx);
    positiony.host(&mut posy);

    // println!("{:?} ", posx);

    // while positions.len() < n {
    //     let mut p1 = rng.gen::<(f64, f64)>();
    //     let mut p2 = rng.gen::<(f64, f64)>();
    //     p1 = (p1.0 * spread, p1.1 * spread);
    //     p2 = (p2.0 * spread, p2.1 * spread);

    //     if positions.iter().all(|&p2| distance(p1, p2) > min_distance) {
    //         positions.push(p1);
    //         positions.push(p2);
    //     }
    // }
    // println!("{:?}", );
    graph.set_positions((posx, posy));
    "applied random pos"
}
