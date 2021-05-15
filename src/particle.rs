extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::f64;
use web_sys::*;

use crate::vector;

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Particle
{
    pub position: vector::Vector,
    pub velocity: vector::Vector,
    pub acceleration: vector::Vector,
}

impl Particle
{
    pub fn new(position_x: f64, position_y: f64) -> Self
    {
        let position = vector::Vector::new(position_x, position_y);
        let velocity = vector::Vector::new(0.0, 0.0);
        let acceleration = vector::Vector::new(0.0, 0.0);

        Self
        {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }

    pub fn update(&mut self, context: &CanvasRenderingContext2d, width: f32, height: f32, mouse_x: f32, mouse_y: f32)
    {
        let mut mouse = vector::Vector::new(mouse_x.into(), mouse_y.into());
        let mut direction = vector::Vector::new(self.position.x, self.position.y);

        mouse.subtract(&direction);
        mouse.normalize();
        mouse.multiply(0.5);

        self.acceleration = mouse;

        self.velocity.add(&self.acceleration);
        self.velocity.limit(10.0);
        self.position.add(&self.velocity);
    }

    pub fn render(&mut self, context: &CanvasRenderingContext2d)
    {
        context.begin_path();

        context.move_to(self.position.x, self.position.y);
        context.arc(self.position.x, self.position.y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();

        context.fill();
        
        /*
        context.set_font("11px Arial");
        context.fill_text(&format!("{}", self.acceleration.x)[..], self.position.x + 12.0, self.position.y);
        context.fill_text(&format!("{}", self.acceleration.y)[..], self.position.x + 12.0, self.position.y + 12.0);
        context.fill_text(&format!("{}", self.acceleration.magnitude())[..], self.position.x + 12.0, self.position.y + 24.0);
        */
    }
}
