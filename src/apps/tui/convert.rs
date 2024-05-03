use crate::num::{Num};
use crate::vector::{Vector, Vector2D};

pub type Point = Vector2D;

pub struct Converter {
    dx: Vector,
    dy: Vector,
}

impl Converter {
    pub fn new(x: (Num, Num), y: (Num, Num), z: (Num, Num)) -> Self {
        Self {
            dx: Vector::new(x.0, y.0, z.0),
            dy: Vector::new(x.1, y.1, z.1),
        }
    }
    
    pub fn convert(&self, v: Vector) -> Point {
        Point::new(v.dot_prod(self.dx), v.dot_prod(self.dy))
    }
}
