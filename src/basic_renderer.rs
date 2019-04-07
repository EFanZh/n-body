use crate::configuration::Color;
use crate::renderer::Renderer;
use cgmath::Vector2;
use itertools::izip;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BasicRenderer {
    canvas_context: CanvasRenderingContext2d,
    body_colors: Vec<String>,
    trail_widths: Vec<f64>,
}

impl BasicRenderer {
    pub fn new(
        canvas_context: CanvasRenderingContext2d,
        width: f64,
        height: f64,
        body_colors: Vec<Color>,
        trail_widths: Vec<f64>,
    ) -> BasicRenderer {
        canvas_context.set_global_composite_operation("screen").unwrap();
        canvas_context.set_fill_style(&JsValue::from_str("black"));
        canvas_context.fill_rect(-width * 0.5, -height * 0.5, width, height);

        BasicRenderer {
            canvas_context,
            body_colors: body_colors.iter().map(|c| c.to_rgba()).collect(),
            trail_widths,
        }
    }
}

impl Renderer for BasicRenderer {
    fn render(&mut self, position_histories: &[Vec<Vector2<f64>>]) {
        for (position_history, color, trail_width) in izip!(position_histories, &self.body_colors, &self.trail_widths) {
            if position_history.len() > 1 {
                self.canvas_context.set_stroke_style(&JsValue::from_str(&color));
                self.canvas_context.set_line_width(*trail_width);

                self.canvas_context.begin_path();

                let (first_position, rest_positions) = position_history.split_first().unwrap();

                self.canvas_context.move_to(first_position.x, first_position.y);

                for position in rest_positions {
                    self.canvas_context.line_to(position.x, position.y);
                }

                self.canvas_context.stroke();
            }
        }
    }
}
