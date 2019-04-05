use cgmath::Vector2;

pub fn cross(u: Vector2<f64>, v: Vector2<f64>) -> f64 {
    u.x * v.y - u.y * v.x
}
