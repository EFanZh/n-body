use crate::body::Body;
use crate::universe::Universe;
use cgmath::{InnerSpace, Vector2};
use itertools::Itertools;

pub struct BasicUniverse {
    bodies: Vec<Body>,
    acceleration_buffer: Vec<Vector2<f64>>,
}

impl BasicUniverse {
    pub fn new(bodies: &[Body]) -> BasicUniverse {
        BasicUniverse {
            bodies: bodies.to_vec(),
            acceleration_buffer: vec![Vector2 { x: 0.0, y: 0.0 }; bodies.len()],
        }
    }
}

fn pow_negative_one_half(x: f64) -> f64 {
    (x.sqrt() * x).recip()
}

impl Universe for BasicUniverse {
    fn advance(&mut self, time: f64) {
        for a in &mut self.acceleration_buffer {
            *a = Vector2 { x: 0.0, y: 0.0 };
        }

        for ((i, body_i), (j, body_j)) in self.bodies.iter().enumerate().tuple_combinations() {
            let distance = body_j.position - body_i.position;
            let magnitude = pow_negative_one_half(distance.magnitude2());

            self.acceleration_buffer[i] += distance * (body_j.mass * magnitude);
            self.acceleration_buffer[j] -= distance * (body_i.mass * magnitude);
        }

        for (body, acceleration) in self.bodies.iter_mut().zip(&self.acceleration_buffer) {
            body.velocity += acceleration * time;
            body.position += body.velocity * time;
        }
    }

    fn get_bodies(&self) -> &[Body] {
        &self.bodies
    }
}

#[cfg(test)]
mod tests {
    use super::{pow_negative_one_half, BasicUniverse};
    use crate::body::Body;
    use crate::universe::Universe;
    use cgmath::{InnerSpace, Vector2};

    #[test]
    fn check_pow_negative_one_half() {
        let mut i = 1.0e-6;

        while i < 1.0e6 {
            assert!((pow_negative_one_half(i) - i.powf(-1.5)).abs() < 1e-6);

            i *= 1.0001;
        }
    }

    fn get_taylor_universe() -> BasicUniverse {
        BasicUniverse::new(&[
            Body::new(2.0, Vector2::new(3.0, 5.0), Vector2::new(7.0, 11.0)),
            Body::new(13.0, Vector2::new(19.0, 19.0), Vector2::new(23.0, 29.0)),
            Body::new(31.0, Vector2::new(37.0, 41.0), Vector2::new(43.0, 47.0)),
            Body::new(53.0, Vector2::new(59.0, 61.0), Vector2::new(67.0, 71.0)),
        ])
    }

    #[test]
    fn energy() {
        let mut universe = get_taylor_universe();
        let e0 = universe.get_energy();

        let expected_energy = 18821413.0 / 58.0
            - f64::sqrt(2699449.0 / 884.0)
            - f64::sqrt(162409.0 / 808.0)
            - f64::sqrt(2809.0 / 1568.0)
            - f64::sqrt(961.0 / 613.0)
            - f64::sqrt(169.0 / 113.0);

        assert!((e0 - expected_energy).abs() < 1.0e-10);

        for _ in 0..1000000 {
            universe.advance(0.000001);
        }

        let e1 = universe.get_energy();

        assert!((e1 - expected_energy).abs() < 1.0e-4);
    }

    #[test]
    fn center_of_mass() {
        let mut universe = get_taylor_universe();
        let momentum_0 = universe.get_momentum();

        let center_of_mass_0 = universe.get_center_of_mass();
        let expected_center_of_mass_0 = Vector2::new(503.0 / 11.0, 529.0 / 11.0);

        assert_eq!(center_of_mass_0, expected_center_of_mass_0);

        for _ in 0..1000000 {
            universe.advance(0.000001);
        }

        let center_of_mass_1 = universe.get_center_of_mass();
        let expected_center_of_mass_1 = expected_center_of_mass_0 + momentum_0 / universe.get_mass();

        assert!((center_of_mass_1 - expected_center_of_mass_1).magnitude2() < 1.0e-23);
    }

    #[test]
    fn momentum() {
        let mut universe = get_taylor_universe();
        let m0 = universe.get_momentum();

        let expected_momentum = Vector2::new(5197.0, 5619.0);

        assert_eq!(m0, expected_momentum);

        for _ in 0..1000000 {
            universe.advance(0.000001);
        }

        let m1 = universe.get_momentum();

        assert!((m1 - expected_momentum).magnitude2() < 1.0e-18);
    }

    #[test]
    fn angular_momentun() {
        let mut universe = get_taylor_universe();
        let m0 = universe.get_angular_momentum();

        let expected_angular_momentum = 6140.0;

        assert_eq!(m0, expected_angular_momentum);

        for _ in 0..1000000 {
            universe.advance(0.000001);
        }

        let m1 = universe.get_angular_momentum();

        assert!((m1 - expected_angular_momentum).abs() < 1.0e-8);
    }
}
