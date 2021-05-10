use web_sys::*;

use crate::vector;

pub trait Element
{
    fn new(position: vector::Vector, velocity: vector::Vector) -> Self;

    fn update(&self, context: &CanvasRenderingContext2d);

    fn render(&self, context: &CanvasRenderingContext2d);
}
