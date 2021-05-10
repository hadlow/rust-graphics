pub struct Vector
{
    pub x: f64,
    pub y: f64,
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

    pub fn add(&mut self, v: &Vector)
    {
        self.x = self.x + v.x;
        self.y = self.y + v.y;
    }
}
