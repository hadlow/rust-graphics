extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>
    {
        Ok(())
    }

    pub fn render(&self)
    {
        
    }
}
