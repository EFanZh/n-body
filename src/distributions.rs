use cgmath::Vector2;
use rand::distributions::Distribution;
use rand::Rng;
use std::f64::consts::PI;

pub struct Reciprocal {
    low: f64,
    high: f64,
}

impl Reciprocal {
    pub fn new(low: f64, high: f64) -> Reciprocal {
        Reciprocal { low, high }
    }
}

impl Distribution<f64> for Reciprocal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = rng.gen();

        self.low.powf(1.0 - x) * self.high.powf(x)
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Distribution<Vector2<f64>> for Circle {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2<f64> {
        let length = self.radius * rng.gen::<f64>().sqrt();
        let angle = rng.gen_range(0.0, PI * 2.0);

        Vector2::new(length * angle.cos(), length * angle.sin())
    }
}
