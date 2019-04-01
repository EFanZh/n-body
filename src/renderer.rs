use crate::universe::Universe;
use web_sys::CanvasRenderingContext2d;

pub trait Renderer: 'static {
    fn render<U: Universe>(
        &mut self,
        timestamp: f64,
        context: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        universe: &mut U,
    );

    fn initialize_canvas_context(&self, context: &CanvasRenderingContext2d);
}
