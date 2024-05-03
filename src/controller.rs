use std::slice::{Iter, IterMut};
use crate::body::{BodyLike, BodyId};
use crate::num::{num, square, Num};

pub struct Controller<B: BodyLike> {
    bodies: Vec<B>,
    time: Num,
}

pub enum IterStatus<T> {
    Done(T),
    Continue,
}

#[allow(dead_code)]
impl<B: BodyLike> Controller<B> {
    pub fn new(bodies: Vec<B>) -> Self {
        Self { bodies, time: num(0) }
    }
    
    pub fn update(&mut self, delta: Num) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let a = self.bodies[i];
                let b = self.bodies[j];
                let tmp = *a.pos() - *b.pos();
                let module = tmp.module();
                let fac = square(module);
                let tmp = tmp / module;
                *self.bodies[i].speed_mut() += tmp * (-delta * b.mass() / fac);
                *self.bodies[j].speed_mut() += tmp * (delta * a.mass() / fac);
            }
        }
        for a in self.bodies.iter_mut() {
            let pos = *a.speed() * delta;
            *a.pos_mut() += pos;
        }
        self.time += delta;
    }
    
    pub fn total_seconds(&self) -> Num {
        self.time
    }
    
    pub fn len(&self) -> usize {
        self.bodies.len()
    }
    
    pub fn iter(&self) -> Iter<B> {
        self.bodies.iter()
    }
    
    pub fn with_each_other<T, F: FnMut(&B, &B) -> IterStatus<T>>(&self, mut f: F) -> Option<T> {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let a = &self.bodies[i];
                let b = &self.bodies[j];
                if let IterStatus::Done(x) = f(a, b) {
                    return Some(x);
                }
            }
        }
        
        None
    }
    
    pub fn iter_mut(&mut self) -> IterMut<B> {
        self.bodies.iter_mut()
    }
    
    pub fn get_body(&self, id: BodyId) -> Option<&B> {
        for i in self.bodies.iter() {
            if *i.id() == id {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use crate::auto_impl_body;
    use crate::vector::Vector;
    use crate::num::{sqrt, Int};
    use super::*;
    
    const COUNT: Int = 10000_000;
    const DIFF: f64 = 0.0001;
    const DELTA: f64 = 0.00001;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Body {
        id: BodyId,
        pos: Vector,
        speed: Vector,
        mass: Num,
    }
    auto_impl_body!(pos, speed, mass, id, ());
    
    #[test]
    fn test_con_2() {
        let mass = num(4);
        let bodies = vec![
            Body {
                pos: Vector::new(0, 1, 0),
                speed: Vector::new(-1, 0, 0),
                mass,
                id: 0,
            },
            Body {
                pos: Vector::new(0, -1, 0),
                speed: Vector::new(1, 0, 0),
                mass,
                id: 1,
            },
        ];
        test_con_round(bodies, num(1));
    }
    
    #[test]
    fn test_con_3() {
        let sqrt_3 = sqrt(num(3));
        let mass = num(8) * sqrt_3;
        let bodies = vec![
            Body {
                pos: Vector::new(0, 2, 0),
                speed: Vector::new(-2, 0, 0),
                mass,
                id: 0,
            },
            Body {
                pos: Vector::new(sqrt_3, -1, 0),
                speed: Vector::new(1, sqrt_3, 0),
                mass,
                id: 1,
            },
            Body {
                pos: Vector::new(-sqrt_3, -1, 0),
                speed: Vector::new(1, -sqrt_3, 0),
                mass,
                id: 2,
            },
        ];
        test_con_round(bodies, num(2));
    }
    
    fn test_con_round(bodies: Vec<Body>, r: Num) {
        let mut con = Controller::new(bodies);
        let t = num(DELTA);
        for _ in 0..COUNT {
            con.update(t);
            for a in &con.bodies {
                if a.pos.distance(Vector::origin()) - r >= num(DIFF) {
                    println!(
                        "{}, {}, {}",
                        a.pos,
                        a.speed,
                        a.pos.distance(Vector::origin())
                    );
                    println!("Expect: {}+-{}", r, DIFF);
                    panic!("Not in round!");
                }
            }
        }
    }
    
    #[test]
    fn test_con_4() {
        let mass = num(4) * (num(2) * sqrt(num(2)) - num(1)) / num(7);
        let bodies = vec![
            Body {
                pos: Vector::new(0, 1, 0),
                speed: Vector::new(-1, 0, 0),
                mass,
                id: 0,
            },
            Body {
                pos: Vector::new(0, -1, 0),
                speed: Vector::new(1, 0, 0),
                mass,
                id: 1,
            },
            Body {
                pos: Vector::new(-1, 0, 0),
                speed: Vector::new(0, -1, 0),
                mass,
                id: 2,
            },
            Body {
                pos: Vector::new(1, 0, 0),
                speed: Vector::new(0, 1, 0),
                mass,
                id: 3,
            },
        ];
        test_con_round(bodies, num(1));
    }
    
    #[test]
    fn test_con_double() {
        let bodies = vec![
            Body {
                pos: Vector::new(-1, 0, 0),
                speed: Vector::new(0, -1, 0),
                mass: num(48),
                id: 0,
            },
            Body {
                pos: Vector::new(3, 0, 0),
                speed: Vector::new(0, 3, 0),
                mass: num(16),
                id: 1,
            },
        ];
        let mut con = Controller::new(bodies);
        let t = num(DELTA);
        for _ in 0..COUNT {
            con.update(t);
            assert!(con.bodies[0].pos.distance(Vector::origin()) - num(1) < num(DIFF));
            assert!(con.bodies[1].pos.distance(Vector::origin()) - num(3) < num(DIFF));
        }
    }
}
