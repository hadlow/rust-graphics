pub struct Vector
{
    x: f64,
    y: f64,
}

impl Vector
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Self
        {
            x: x,
            y: y,
        }
    }
}
