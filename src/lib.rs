extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

mod setup;

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client
{
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Client
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        let context = setup::create_context().unwrap();

        Self
        {
            context: context,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>
    {
        self.context.clear_rect(0.0, 0.0, width.into(), height.into());

        Ok(())
    }

    pub fn render(&self)
    {
        self.context.begin_path();
        self.context.stroke();
    }
}
