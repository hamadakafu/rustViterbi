use std::f64::consts::{E, PI};
use rand_core::{RngCore, SeedableRng};
use rand::Rng;

pub fn box_muller() -> f64 {
    // let mut rng = sfmt::SFMT::from_entropy();
    // let (u1, u2): (f64, f64) = (rng.gen_range(0., 1.), rng.gen_range(0., 1.));
    let (u1, u2): (f64, f64) = (rand::random(), rand::random());
    // dbg!(u1, u2);
    (-2.0 * u1.log(E)).sqrt() * (2.0 * PI * u2).cos()
}
