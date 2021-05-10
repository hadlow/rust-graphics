use web_sys::*;

use crate::vector;

pub trait Element
{
    fn update(&self, context: &CanvasRenderingContext2d);

    fn render(&self, context: &CanvasRenderingContext2d);
}
