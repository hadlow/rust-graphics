use std::f64;
use web_sys::*;

use crate::vector;

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
        let acceleration = vector::Vector::new(-0.001, 0.001);

        Self
        {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }

    pub fn update(&mut self, context: &CanvasRenderingContext2d, width: f32, height: f32)
    {
        self.velocity.add(&self.acceleration);
        self.position.add(&self.velocity);

        if (self.position.x > width.into()) || (self.position.x < 0.0)
        {
            self.velocity.x = self.velocity.x * -1.0;
        }

        if (self.position.y > height.into()) || (self.position.y < 0.0)
        {
            self.velocity.y = self.velocity.y * -1.0;
        }
    }

    pub fn render(&mut self, context: &CanvasRenderingContext2d)
    {
        context.begin_path();

        context.move_to(self.position.x, self.position.y);
        context.arc(self.position.x, self.position.y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();

        context.fill();
    }
}
