use cgmath::Vector2;

#[derive(Clone)]
pub struct Body {
    pub mass: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

impl Body {
    pub fn new(mass: f64, position: Vector2<f64>, velocity: Vector2<f64>) -> Body {
        Body {
            mass,
            position,
            velocity,
        }
    }
}
