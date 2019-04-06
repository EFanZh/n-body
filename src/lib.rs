use rand::random;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

mod basic_renderer;
mod configuration;
mod distributions;
mod renderer;
mod utilities;

pub mod basic_universe;
pub mod body;
pub mod universe;

use crate::basic_renderer::BasicRenderer;
use crate::basic_universe::BasicUniverse;
use crate::configuration::{random_configuration, Configuration};
use crate::renderer::Renderer;
use crate::universe::Universe;

fn build_universe_and_renderer(configuration: Configuration) -> (impl Universe, impl Renderer) {
    let universe = BasicUniverse::new(&configuration.bodies.iter().map(|b| b.body.clone()).collect::<Vec<_>>());

    let renderer = BasicRenderer::new(
        configuration.bodies.iter().map(|b| b.color.clone()).collect(),
        configuration.bodies.iter().map(|b| b.trail_width).collect(),
        configuration.sample_frequency,
        &universe,
    );

    (universe, renderer)
}

fn run_animation_frame_loop<F: FnMut(f64) + 'static>(window: &Window, mut f: F) {
    fn do_request_animation_frame(window: &Window, f: &Closure<FnMut(f64)>) {
        window.request_animation_frame(f.as_ref().unchecked_ref()).unwrap();
    }

    let closure_rc_0 = Rc::new(RefCell::new(None));
    let closure_rc_1 = closure_rc_0.clone();

    // TODO: Update to use `Closure::new`.

    *closure_rc_1.borrow_mut() = Some(Closure::wrap(Box::new({
        let window = window.clone();

        move |timestamp| {
            do_request_animation_frame(&window, closure_rc_0.borrow().as_ref().unwrap());

            f(timestamp);
        }
    }) as _));

    do_request_animation_frame(&window, closure_rc_1.borrow().as_ref().unwrap());
}

fn run_and_render_universe<U: Universe, R: Renderer>(
    window: &Window,
    canvas_width: f64,
    canvas_height: f64,
    context: CanvasRenderingContext2d,
    mut universe: U,
    mut renderer: R,
) {
    renderer.initialize_canvas_context(&context, canvas_width, canvas_height, &mut universe);

    run_animation_frame_loop(&window, move |timestamp| {
        renderer.render(timestamp, &context, canvas_width, canvas_height, &mut universe);
    });
}

fn main(window: Window, document: Document, configuration: Configuration) {
    let (context, canvas_width, canvas_height) = {
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let screen = window.screen().unwrap();
        let canvas_width = f64::from(screen.width().unwrap());
        let canvas_height = f64::from(screen.height().unwrap());
        let dpi = window.device_pixel_ratio() * configuration.super_resolution;

        canvas.set_width((canvas_width * dpi).round() as _);
        canvas.set_height((canvas_height * dpi).round() as _);

        let style = canvas.style();

        style.set_property("width", &format!("{}px", canvas_width)).unwrap();
        style.set_property("height", &format!("{}px", canvas_height)).unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.scale(dpi, dpi).unwrap();
        context.translate(canvas_width * 0.5, canvas_height * 0.5).unwrap();

        (context, canvas_width, canvas_height)
    };

    let (universe, renderer) = build_universe_and_renderer(configuration);

    run_and_render_universe(&window, canvas_width, canvas_height, context, universe, renderer);
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let configuration = random_configuration(random());

    if document.ready_state() == "loading" {
        document
            .add_event_listener_with_callback(
                "DOMContentLoaded",
                Closure::once_into_js({
                    let document = document.clone();

                    move || main(window, document, configuration)
                })
                .unchecked_ref(),
            )
            .unwrap();
    } else {
        main(window, document, configuration);
    }
}
