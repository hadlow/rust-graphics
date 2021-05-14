use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn create_context(canvas: &web_sys::HtmlCanvasElement) -> Result<web_sys::CanvasRenderingContext2d, JsValue>
{
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, 0.0, 1.0);

    Ok(context)
}

pub fn find_canvas(id: &str) -> web_sys::HtmlCanvasElement
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}
