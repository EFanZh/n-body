use cgmath::Vector2;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, Event, HtmlCanvasElement, Window};

mod basic_renderer;
mod renderer;
mod utilities;

pub mod basic_universe;
pub mod body;
pub mod universe;

use crate::basic_renderer::BasicRenderer;
use crate::basic_universe::BasicUniverse;
use crate::body::Body;
use crate::renderer::Renderer;
use crate::universe::Universe;

fn run_animation_frame_loop<F: FnMut(f64) + 'static>(window: &Window, mut f: F) {
    fn do_request_animation_frame(window: &Window, closure_rc: &Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>) {
        window
            .request_animation_frame(closure_rc.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }

    let closure_rc_0 = Rc::new(RefCell::new(None));
    let closure_rc_1 = closure_rc_0.clone();

    // TODO: Update to use `Closure::new`.

    *closure_rc_1.borrow_mut() = Some(Closure::wrap(Box::new({
        let window = window.clone();

        move |timestamp| {
            f(timestamp);

            do_request_animation_frame(&window, &closure_rc_0);
        }
    }) as Box<FnMut(f64)>));

    do_request_animation_frame(&window, &closure_rc_1);
}

fn register_resize_event(window: &Window, canvas: HtmlCanvasElement, canvas_width: f64, canvas_height: f64) {
    let update_canvas = Closure::wrap(Box::new({
        let window = window.clone();

        move || {
            let view_width = window.inner_width().unwrap().as_f64().unwrap();
            let view_height = window.inner_height().unwrap().as_f64().unwrap();

            canvas
                .style()
                .set_property(
                    "transform",
                    &format!("scale({}, {})", canvas_width / view_width, canvas_height / view_height),
                )
                .unwrap();
        }
    }) as Box<dyn Fn()>);

    window
        .add_event_listener_with_callback("resize", update_canvas.as_ref().unchecked_ref())
        .unwrap();

    update_canvas.forget();

    window.dispatch_event(&Event::new("resize").unwrap()).unwrap();
}

fn run_and_render_universe<U: Universe, R: Renderer>(
    window: &Window,
    canvas_width: f64,
    canvas_height: f64,
    context: CanvasRenderingContext2d,
    mut universe: U,
    mut renderer: R,
) {
    renderer.reset_canvas_context(&context);

    run_animation_frame_loop(&window, move |timestamp| {
        renderer.render(timestamp, &context, canvas_width, canvas_height, &mut universe);
    });
}

// FOR DEBUGGING ====>

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// fn dbg<T: std::fmt::Debug>(o: T) {
//     log(&format!("{:?}", o));
// }

// FOR DEBUGGING <====

fn main(window: Window, document: Document) {
    let (canvas, context, canvas_width, canvas_height) = {
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let screen = window.screen().unwrap();
        let canvas_width = f64::from(screen.width().unwrap());
        let canvas_height = f64::from(screen.height().unwrap());
        let dpi = window.device_pixel_ratio();

        canvas.set_width((canvas_width * dpi).round() as _);
        canvas.set_height((canvas_height * dpi).round() as _);

        canvas.style().set_property("transform-origin", "center").unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.scale(dpi, dpi).unwrap();

        context.translate(canvas_width * 0.5, canvas_height * 0.5).unwrap();
        (canvas, context, canvas_width, canvas_height)
    };

    register_resize_event(&window, canvas, canvas_width, canvas_height);

    let universe = BasicUniverse::new(&[
        Body::new(20000.0, Vector2::new(0.0, 0.0), Vector2::new(2.0, 3.0)),
        Body::new(10000.0, Vector2::new(400.0, 0.0), Vector2::new(-4.0, -6.0)),
        Body::new(1000.0, Vector2::new(0.0, 200.0), Vector2::new(0.0, 0.0)),
    ]);

    let renderer = BasicRenderer::new();

    run_and_render_universe(&window, canvas_width, canvas_height, context, universe, renderer);
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if document.ready_state() == "complete" {
        main(window, document);
    } else {
        document
            .add_event_listener_with_callback(
                "DOMContentLoaded",
                Closure::once_into_js({
                    let document = document.clone();

                    move || main(window, document)
                })
                .unchecked_ref(),
            )
            .unwrap();
    }
}
