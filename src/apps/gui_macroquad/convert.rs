use crate::num::{Num, num};
use crate::vector::{Vector, Vector2D};

#[derive(Debug)]
pub struct ConverterData {
    pub center: Vector,
    pub theta_x: Num,
    pub unit_length: Num,
}

impl Default for ConverterData {
    fn default() -> Self {
        Self {
            center: Vector::new(1, 1, 0.8).unit(),
            theta_x: num(0),
            unit_length: num(200),
        }
    }
}


pub struct Converter {
    x: Vector,
    y: Vector,
    z: Vector,
}

#[allow(dead_code)]
impl Converter {
    pub fn convert(&self, v: Vector) -> (Vector2D, Num) {
        (Vector2D::new(-v.dot_prod(&self.x), v.dot_prod(&self.y)), v.dot_prod(&self.z))
    }
    
    pub fn x_axis(&self) -> &Vector {
        &self.x
    }
    
    pub fn y_axis(&self) -> &Vector {
        &self.y
    }
    
    pub fn z_axis(&self) -> &Vector {
        &self.z
    }
    
    pub fn new(data: &ConverterData) -> Self {
        let z_axis = data.center.unit();
        let x_base = if z_axis.x() == num(0) && z_axis.y() == num(0) {
            Vector::new(1, 0, 0)
        } else {
            data.center.cross_prod(&Vector::z_axis())
        };
        let x_axis = if data.theta_x == num(0) {
            x_base
        } else {
            // Rote the X axis over theta about the z axis
            x_base.rotate(&z_axis, data.theta_x)
        };
        let x_axis = x_axis * data.unit_length;
        let y_axis = z_axis.cross_prod(&x_axis);
        Self {
            x: x_axis,
            y: y_axis,
            z: z_axis,
        }
    }
}
