use crate::renderer::Renderer;
use crate::universe::Universe;

pub trait Scheduler: 'static {
    fn advance<U: Universe, R: Renderer>(&mut self, timestamp: f64, universe: &mut U, renderer: &mut R);
}
