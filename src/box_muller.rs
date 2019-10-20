use std::f64::consts::{E, PI};
use rand_core::{RngCore, SeedableRng};
use rand::Rng;

pub fn box_muller() -> f64 {
    let (u1, u2): (f64, f64) = (rand::random(), rand::random());
    (-2.0 * u1.log(E)).sqrt() * (2.0 * PI * u2).cos()
}
