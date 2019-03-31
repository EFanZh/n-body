use cgmath::{InnerSpace, Vector2};

pub fn squared_norm(v: Vector2<f64>) -> f64 {
    v.dot(v)
}

pub fn norm(v: Vector2<f64>) -> f64 {
    squared_norm(v).sqrt()
}

pub fn cross(u: Vector2<f64>, v: Vector2<f64>) -> f64 {
    u.x * v.y - u.y * v.x
}
