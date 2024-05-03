use crate::body::{BodyLike, BodyId};
use crate::num::{abs, num, Num, floor, min};
use crate::vector::{Vector, Vector2D};

use super::basic::{AppContext, Point};
use super::convert::{Converter, ConverterData};
use std::default::Default;

pub struct View {
    width: Num,
    height: Num,
    converter: Converter,
    converter_data: ConverterData,
}

#[allow(dead_code)]
impl View {
    pub fn new(width: Num, height: Num) -> Self {
        let width = floor(abs(width) / num(2));
        let height = floor(abs(height) / num(2));
        let converter_data = ConverterData::default();
        let converter = Converter::new(&converter_data);
        Self {
            width,
            height,
            converter,
            converter_data,
        }
    }
    
    pub fn get_x(&self, point: &Point) -> Num {
        point.pos.x() + self.width
    }
    
    pub fn get_y(&self, point: &Point) -> Num {
        point.pos.y() + self.height
    }
    
    pub fn center(&self) -> Vector2D {
        Vector2D::new(self.width, self.height)
    }
    
    pub fn unit_size(&self) -> Num {
        self.converter_data.unit_length
    }
    
    pub fn half_width(&self) -> Num {
        self.width
    }
    
    pub fn half_height(&self) -> Num {
        self.height
    }
    
    pub fn convert(&self, v: Vector) -> (Vector2D, Num) {
        self.converter.convert(v)
    }
    
    fn refresh_converter(&mut self) {
        // println!("New converter from data: {:?}", self.converter_data);
        self.converter = Converter::new(&self.converter_data);
    }
    
    pub fn rotate_view(&mut self, delta: (Num, Num)) {
        // println!("Rotate: {:?}", delta);
        let center = self.converter_data.center
            .rotate(&Vector::new(0, 0, -1), delta.0)
            .rotate(self.converter.x_axis(), delta.1)
            // .unit()
            ;
        self.converter_data.unit_length += delta.1 * 100.;
        self.converter_data.center = center;
        self.refresh_converter();
    }
    
    pub fn refresh(&mut self, width: Num, height: Num) {
        self.width = floor(width / num(2));
        self.height = floor(height / num(2));
    }
    
    pub fn reset_view(&mut self) {
        self.converter_data = Default::default();
        self.refresh_converter();
    }
    
    pub fn zoom(&mut self, scale: Num) {
        self.converter_data.unit_length *= scale;
        self.refresh_converter();
    }
    
    pub fn parse(&self, context: &AppContext) -> Vec<(Point, BodyId)> {
        let mut points: Vec<_> =
            context.controller
                .iter()
                .map(
                    |x| {
                        let (pos, depth) = self.converter.convert(*(x.pos()));
                        (
                            Point {
                                body_id: *x.id(),
                                pos,
                                color: x.color(),
                                radius: *x.mass() * context.config.radius_factor
                                    + min(depth * context.config.depth_factor, context.config.depth_max),
                                depth,
                            },
                            *x.id()
                        )
                    })
                // .filter(
                //     |(x, _)|
                //         abs(x.pos.x()) <= self.width + x.radius.
                //             && abs(x.pos.y()) <= self.height + x.radius.
                // )
                .collect();
        points.sort_by(|x, y| x.0.depth.partial_cmp(&y.0.depth).unwrap());
        // points.iter().for_each(|(x, _)| println!("{:?}", x));
        points
    }
}