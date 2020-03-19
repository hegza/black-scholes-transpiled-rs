use black_scholes::*;
use itertools::izip;
use rand::Rng;
use savefile::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::iter::repeat_with;

fn main() {
    let start = std::time::Instant::now();

    let n = 144000000 / 2;
    // Generated using code like: std::iter::repeat_with(|| rng.clone().gen_range(0.025, 0.075)).take(n).collect::<Vec<f64>>()
    let S: Vec<f64> = load_file("data/S.bin", 0).unwrap();
    let K: Vec<f64> = load_file("data/K.bin", 0).unwrap();
    let T: Vec<f64> = load_file("data/T.bin", 0).unwrap();
    let r: Vec<f64> = load_file("data/r.bin", 0).unwrap();
    let sigma: Vec<f64> = load_file("data/sigma.bin", 0).unwrap();

    let mut outputs: Vec<f64> = vec![0.; n];

    for (out, (S, K, T, r, sigma)) in outputs.iter_mut().zip(izip!(S, K, T, r, sigma)) {
        *out = euro_vanilla_put(S, K, T, r, sigma)
    }

    let duration = std::time::Instant::now() - start;
    println!("N was {}", n);
    println!("Took {:?}", duration);
}
