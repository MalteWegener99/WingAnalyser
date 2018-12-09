#[derive(Debug, Copy, Clone)]
pub struct Vec2{
    pub x: f64,
    pub y: f64,
}

impl Vec2{
    pub fn add(&self, other: &Vec2) -> Vec2{
        Vec2{x: self.x + other.x, y: self.y + other.y}
    }

    pub fn substract(&self, other: &Vec2) -> Vec2{
        Vec2{x: self.x - other.x, y: self.y - other.y}
    }

    pub fn scale(&self, other: f64) -> Vec2{
        Vec2{x: self.x*other, y: self.y*other}
    }

    pub fn magnitude(&self) -> f64{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }

    pub fn unitvec(&self) -> Vec2{
        let x = self.x/self.magnitude();
        let y = self.y/self.magnitude();
        Vec2{x: x, y: y}
    }
}