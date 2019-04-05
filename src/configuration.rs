use crate::body::Body;
use crate::distributions::{Circle, Reciprocal};
use cgmath::{InnerSpace, Vector2};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug)]
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
            alpha: 192,
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

pub fn random_configuration() -> Configuration {
    let min_bodies = 2;
    let max_bodies = 5;
    let min_mass = 2.0f64.powf(4.0);
    let max_mass = 2.0f64.powf(20.0);
    let min_trail_width = 2.0f64.powf(-3.0);
    let max_trail_width = 2.0f64.powf(1.0);
    let position_radius = 2.0f64.powf(8.0);
    let velocity_radius = 2.0f64.powf(5.0);

    let mut rng = rand::rngs::OsRng::new().unwrap(); // TODO: Update to use `thread_rng()`.
    let mass_rng = Reciprocal::new(min_mass, max_mass);
    let position_rng = Circle::new(position_radius);
    let velocity_rng = Circle::new(velocity_radius);

    let mass_to_trail_width = |mass: f64| {
        let r = (mass / min_mass).log(max_mass / min_mass);

        min_trail_width + (max_trail_width - min_trail_width) * r
    };

    let mut bodies = (0..rng.gen_range(min_bodies - 1, max_bodies))
        .map(|_| {
            let mass = mass_rng.sample(&mut rng);
            let trail_width = mass_to_trail_width(mass);

            StyledBody {
                body: Body::new(mass, position_rng.sample(&mut rng), velocity_rng.sample(&mut rng)),
                color: rng.gen(),
                trail_width,
            }
        })
        .collect::<Vec<_>>();

    // Generate the last body to keep the system at the center of the canvas.

    bodies.push({
        let other_bodies_mass_vector = bodies
            .iter()
            .map(|b| b.body.mass * b.body.position)
            .sum::<Vector2<f64>>();

        let position = -other_bodies_mass_vector.normalize() * (position_radius * rng.gen::<f64>().sqrt());
        let mass = other_bodies_mass_vector.magnitude() / position.magnitude();

        let velocity = -bodies
            .iter()
            .map(|b| b.body.mass * b.body.velocity)
            .sum::<Vector2<f64>>()
            / mass;

        StyledBody {
            body: Body::new(mass, position, velocity),
            color: rng.gen(),
            trail_width: mass_to_trail_width(mass),
        }
    });

    Configuration {
        bodies,
        sample_frequency: 1_000_000.0,
        super_resolution: 2.0,
    }
}
