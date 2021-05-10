use std::f64;
use web_sys::*;

use crate::vector;

pub struct Particle
{
    pub position: vector::Vector,
    pub velocity: vector::Vector,
}

impl Particle
{
    pub fn new(position_x: f64, position_y: f64, velocity_x: f64, velocity_y: f64) -> Self
    {
        let position = vector::Vector::new(position_x, position_y);
        let velocity = vector::Vector::new(velocity_x, velocity_y);

        Self
        {
            position: position,
            velocity: velocity,
        }
    }

    pub fn update(&mut self, context: &CanvasRenderingContext2d)
    {
        self.position.x = self.position.x + self.velocity.x;
        self.position.y = self.position.y + self.velocity.y;
    }

    pub fn render(&mut self, context: &CanvasRenderingContext2d)
    {
        context.move_to(self.position.x, self.position.y);
        context
            .arc(self.position.x, self.position.y, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
    }
}
