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

    pub fn subtract(&mut self, v: &Vector)
    {
        self.x = self.x - v.x;
        self.y = self.y - v.y;
    }
    
    pub fn multiply(&mut self, n: f64)
    {
        self.x = self.x * n;
        self.y = self.y * n;
    }

    pub fn divide(&mut self, n: f64)
    {
        self.x = self.x / n;
        self.y = self.y / n;
    }

    pub fn magnitude(&self) -> f64
    {
        (self.x * self.x).sqrt() + (self.y + self.y).sqrt()
    }

    pub fn normalize(&mut self)
    {
        let magnitude: f64 = self.magnitude();

        if magnitude != 0.0
        {
            self.divide(magnitude);
        }
    }
}
