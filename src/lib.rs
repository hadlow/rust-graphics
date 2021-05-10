extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

mod setup;
mod element;
mod vector;
mod particle;

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
    elements: Vec<particle::Particle>,
}

#[wasm_bindgen]
impl Client
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        let context = setup::create_context().unwrap();
        let mut elements: Vec<particle::Particle> = Vec::new();
        let particle: particle::Particle = particle::Particle::new(50.0, 50.0, 1.0, 2.0);

        elements.push(particle);

        Self
        {
            context: context,
            elements: elements,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>
    {
        self.context.clear_rect(0.0, 0.0, width.into(), height.into());

        for element in self.elements.iter_mut()
        {
            element.update(&self.context);
        }

        Ok(())
    }

    pub fn render(&mut self)
    {
        self.context.begin_path();

        for element in self.elements.iter_mut()
        {
            element.render(&self.context);
        }

        &self.context.stroke();
    }
}
