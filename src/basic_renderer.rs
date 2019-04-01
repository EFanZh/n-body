use crate::renderer::Renderer;
use crate::universe::Universe;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BasicRenderer {}

impl BasicRenderer {
    pub fn new() -> BasicRenderer {
        BasicRenderer {}
    }
}

impl Renderer for BasicRenderer {
    fn render<U: Universe>(
        &mut self,
        _timestamp: f64,
        context: &CanvasRenderingContext2d,
        _width: f64,
        _height: f64,
        universe: &mut U,
    ) {
        for _ in 0..10 {
            for _ in 0..100_00 {
                universe.advance(0.00001);
            }

            for body in universe.get_bodies() {
                context.fill_rect(body.position.x - 0.5, body.position.y - 0.5, 1.0, 1.0);
            }
        }
    }

    fn initialize_canvas_context(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str("black"));
    }
}
