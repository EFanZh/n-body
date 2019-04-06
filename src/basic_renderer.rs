use crate::configuration::Color;
use crate::renderer::Renderer;
use crate::universe::Universe;
use cgmath::{InnerSpace, Vector2};
use itertools::izip;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BasicRenderer {
    body_colors: Vec<String>,
    trail_widths: Vec<f64>,
    sample_frequency: f64,
    step_size: f64,                             // Pre-calculated.
    sampled: u64,                               // State.
    last_timestamp: f64,                        // State.
    start_time: f64,                            // State.
    position_histories: Vec<Vec<Vector2<f64>>>, // Shared Buffer.
}

impl BasicRenderer {
    pub fn new<U: Universe>(
        body_colors: Vec<Color>,
        trail_widths: Vec<f64>,
        sample_frequency: f64,
        universe: &U,
    ) -> BasicRenderer {
        BasicRenderer {
            body_colors: body_colors.iter().map(|c| c.to_rgba()).collect(),
            trail_widths,
            sample_frequency: sample_frequency / 1000.0,
            step_size: sample_frequency.recip(),
            sampled: 0,
            last_timestamp: 0.0,
            start_time: 0.0,
            position_histories: universe.get_bodies().iter().map(|b| vec![b.position]).collect(),
        }
    }
}

impl Renderer for BasicRenderer {
    fn render<U: Universe>(
        &mut self,
        mut timestamp: f64,
        context: &CanvasRenderingContext2d,
        _width: f64,
        _height: f64,
        universe: &mut U,
    ) {
        timestamp -= self.start_time;

        // Adjust timestamp for long time running.

        let ellapsed_time = timestamp - self.last_timestamp;

        if ellapsed_time > 1000.0 {
            self.start_time += ellapsed_time - 1000.0;

            timestamp = self.last_timestamp + 1000.0;
        }

        // Collect the required drawing actions.

        let target_samples = (self.sample_frequency * timestamp) as _;

        for _ in self.sampled..target_samples {
            universe.advance(self.step_size);

            for (position_history, position) in self
                .position_histories
                .iter_mut()
                .zip(universe.get_bodies().iter().map(|b| b.position))
            {
                if (position - position_history.last().unwrap()).magnitude2() >= 1.0 {
                    position_history.push(position);
                }
            }
        }

        // Do actual drawings.

        for (position_history, color, trail_width) in
            izip!(&self.position_histories, &self.body_colors, &self.trail_widths)
        {
            if position_history.len() > 1 {
                context.set_stroke_style(&JsValue::from_str(&color));
                context.set_line_width(*trail_width);

                context.begin_path();

                let (first_position, rest_positions) = position_history.split_first().unwrap();

                context.move_to(first_position.x, first_position.y);

                for position in rest_positions {
                    context.line_to(position.x, position.y);
                }

                context.stroke();
            }
        }

        // Discard old histories.

        for position_history in &mut self.position_histories {
            let last_index = position_history.len() - 1;

            position_history.swap(0, last_index);
            position_history.truncate(1);
        }

        self.sampled = target_samples;
        self.last_timestamp = timestamp;
    }

    fn initialize_canvas_context<U: Universe>(
        &self,
        context: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        _universe: &mut U,
    ) {
        context.set_global_composite_operation("screen").unwrap();
        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(-width * 0.5, -height * 0.5, width, height);
    }
}
