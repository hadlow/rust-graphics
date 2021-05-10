use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn create_context() -> Result<web_sys::CanvasRenderingContext2d, JsValue>
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("app").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, 0.0, 1.0);

    Ok(context)
}
