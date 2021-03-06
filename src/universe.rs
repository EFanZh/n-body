use crate::body::Body;
use crate::utilities::cross;
use cgmath::{InnerSpace, Vector2};
use itertools::Itertools;

pub trait Universe: 'static {
    fn advance(&mut self, time: f64);
    fn get_bodies(&self) -> &[Body];

    fn get_mass(&self) -> f64 {
        self.get_bodies().iter().map(|b| b.mass).sum()
    }

    fn get_energy(&self) -> f64 {
        let bodies = self.get_bodies();

        let mut e: f64 = bodies.iter().map(|b| b.mass * b.velocity.magnitude2()).sum();

        e *= 0.5;

        let negative_potential_energy: f64 = bodies
            .iter()
            .tuple_combinations()
            .map(|(b_i, b_j)| b_i.mass * b_j.mass / (b_j.position - b_i.position).magnitude())
            .sum();

        e -= negative_potential_energy;

        e
    }

    fn get_center_of_mass(&self) -> Vector2<f64> {
        self.get_bodies()
            .iter()
            .map(|b| b.mass * b.position)
            .sum::<Vector2<f64>>()
            / self.get_mass()
    }

    fn get_momentum(&self) -> Vector2<f64> {
        self.get_bodies().iter().map(|b| b.mass * b.velocity).sum()
    }

    fn get_angular_momentum(&self) -> f64 {
        self.get_bodies()
            .iter()
            .map(|b| cross(b.position, b.mass * b.velocity))
            .sum()
    }
}
