extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use rand::Rng;
use web_sys::*;

mod setup;
mod element;
mod vector;
mod particle;

#[wasm_bindgen]
pub struct Client
{
    context: CanvasRenderingContext2d,
    elements: Vec<particle::Particle>,
    mouse_x: f32,
    mouse_y: f32,
    mousedown: bool,
}

#[wasm_bindgen]
impl Client
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        let canvas: web_sys::HtmlCanvasElement = setup::find_canvas("app");
        let context = setup::create_context(&canvas).unwrap();

        let mut elements: Vec<particle::Particle> = Vec::new();

        for _ in 1..11
        {
            let particle: particle::Particle = particle::Particle::new(rand::thread_rng().gen_range(0, 500) as f64, rand::thread_rng().gen_range(0, 500) as f64);

            elements.push(particle);
        }

        Self
        {
            context: context,
            elements: elements,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mousedown: false,
        }
    }

    pub fn update(&mut self, height: f32, width: f32) -> Result<(), JsValue>
    {
        for element in self.elements.iter_mut()
        {
            element.update(&self.context, width, height, self.mouse_x, self.mouse_y);
        }

        Ok(())
    }

    pub fn render(&mut self, height: f32, width: f32)
    {
        self.context.clear_rect(0.0, 0.0, width.into(), height.into());

        for element in self.elements.iter_mut()
        {
            element.render(&self.context);
        }
    }

    pub fn update_mouse(&mut self, mouse_x: f32, mouse_y: f32)
    {
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }

    pub fn update_mousedown(&mut self, mousedown: bool)
    {
        self.mousedown = mousedown;
    }
}
