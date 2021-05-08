use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn create_context() -> Result<web_sys::CanvasRenderingContext2d, JsCast>
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("app").unwrap();

    let canvas: web_sys::HtmlCanvasElement
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear_depth(1.0);

    Ok(context);
}
