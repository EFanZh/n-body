use crate::body::Body;
use crate::distributions::{Circle, Reciprocal};
use cgmath::Vector2;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn to_rgba(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red,
            self.green,
            self.blue,
            f64::from(self.alpha) / 255.0
        )
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let [red, green, blue] = rng.gen::<[u8; 3]>();

        Color {
            red,
            green,
            blue,
            alpha: 96,
        }
    }
}

pub struct StyledBody {
    pub body: Body,
    pub color: Color,
    pub trail_width: f64,
}

pub struct Configuration {
    pub bodies: Vec<StyledBody>,
    pub sample_frequency: f64,
    pub super_resolution: f64,
}

fn normalize_bodies(mut bodies: Vec<StyledBody>) -> Vec<StyledBody> {
    let mass = bodies.iter().map(|b| b.body.mass).sum();
    let center_of_mass = bodies.iter().map(|b| b.body.mass * b.body.position).sum::<Vector2<_>>() / mass;
    let velocity = bodies.iter().map(|b| b.body.mass * b.body.velocity).sum::<Vector2<_>>() / mass;

    for body in &mut bodies {
        body.body.position -= center_of_mass;
        body.body.velocity -= velocity;
    }

    bodies
}

pub fn random_configuration(seed: u64) -> Configuration {
    let min_bodies = 2;
    let max_bodies = 5;
    let min_mass = 2.0f64.powf(4.0);
    let max_mass = 2.0f64.powf(24.0);
    let min_trail_width = 2.0f64.powf(-3.0);
    let max_trail_width = 2.0f64.powf(1.0);
    let position_radius = 2.0f64.powf(8.0);
    let velocity_radius = 2.0f64.powf(5.0);

    let mut rng = StdRng::seed_from_u64(seed);
    let mass_rng = Reciprocal::new(min_mass, max_mass);
    let position_rng = Circle::new(position_radius);
    let velocity_rng = Circle::new(velocity_radius);

    let mass_to_trail_width = |mass: f64| {
        let r = (mass.log(min_mass) - 1.0) / (max_mass.log(min_mass) - 1.0);

        (max_trail_width - min_trail_width).mul_add(r, min_trail_width)
    };

    let bodies = normalize_bodies(
        (0..rng.gen_range(min_bodies, max_bodies + 1))
            .map(|_| {
                let mass = mass_rng.sample(&mut rng);
                let trail_width = mass_to_trail_width(mass);

                StyledBody {
                    body: Body::new(mass, position_rng.sample(&mut rng), velocity_rng.sample(&mut rng)),
                    color: rng.gen(),
                    trail_width,
                }
            })
            .collect(),
    );

    Configuration {
        bodies,
        sample_frequency: 1_000_000.0,
        super_resolution: 2.0,
    }
}
