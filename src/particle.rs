use web_sys::*;

use crate::element;
use crate::vector;

pub struct Particle
{
    position: vector::Vector,
    velocity: vector::Vector,
}

impl element::Element for Particle
{
    fn new(position: vector::Vector, velocity: vector::Vector) -> Self
    {
        let position = position;
        let velocity = velocity;

        Self
        {
            position: position,
            velocity: velocity,
        }
    }

    fn update(&self, context: &CanvasRenderingContext2d)
    {
        
    }

    fn render(&self, context: &CanvasRenderingContext2d)
    {

    }
}
