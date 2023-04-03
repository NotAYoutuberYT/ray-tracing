use crate::vector3::Vector3;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f64::consts::PI;

pub fn random_normal_distribution(rng: &mut ThreadRng) -> f64 {
    let theta = 2.0 * PI * rng.gen::<f64>();
    let rho = (-2.0 * rng.gen::<f64>().ln()).sqrt();
    rho * theta.cos()
}

pub fn random_direction(rng: &mut ThreadRng) -> Vector3 {
    Vector3::new(
        random_normal_distribution(rng),
        random_normal_distribution(rng),
        random_normal_distribution(rng),
    )
    .normalized()
}
