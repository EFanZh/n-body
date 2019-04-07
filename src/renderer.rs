use cgmath::Vector2;

pub trait Renderer: 'static {
    fn render(&mut self, position_histories: &[Vec<Vector2<f64>>]);
}
