use crate::{
    body::Body,
    num::{Int, Num},
    vector::{Vector, Vector2D as Point},
};
use super::convert::Converter;
use crate::controller::Controller;

pub struct BodyContext<'a> {
    pub pos: Point,
    pub depth: Num,
    pub body: &'a Body,
}
pub struct View {
    width: usize,
    height: usize,
    center: Point,
    converter: Converter,
}

impl View {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            center: Point::new((width / 2) as Int, (height / 2) as Int),
            converter: Converter::new((1., 0.), (0.5, 0.5), (0., 1.)),
        }
    }
    
    pub fn get_depth(&self, pos: Vector) -> Num {
        pos.z()  //TODO
    }
    
    pub fn parse(&self, controller: &Controller) -> Vec<BodyContext> {
        let mut bodies: Vec<_> = controller.iter().map(
            |x| BodyContext {
                body: x,
                depth: self.get_depth(x.pos),
                pos: self.converter.convert(x.pos),
            }).collect();
        bodies.sort_by(|x, y| x.depth.partial_cmp(&y.depth).unwrap());
        bodies
    }
}

