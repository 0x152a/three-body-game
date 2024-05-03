use crate::num::{sqrt, square, cos, num, sin, center, Num, ZERO};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Vector(Num, Num, Num);

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "({:.2}, {:.2}, {:.2})", self.0, self.1, self.2)
        } else {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Eq for Vector {}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: Into<Num>> Mul<T> for Vector {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let rhs: Num = rhs.into();
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Into<Num>> MulAssign<T> for Vector {
    fn mul_assign(&mut self, rhs: T) {
        let rhs: Num = rhs.into();
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl<T: Into<Num>> Div<T> for Vector {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let rhs: Num = rhs.into();
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T: Into<Num>> DivAssign<T> for Vector {
    fn div_assign(&mut self, rhs: T) {
        let rhs: Num = rhs.into();
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[allow(dead_code)]
impl Vector {
    pub fn x_axis() -> Vector {
        Vector(1., 0., 0.)
    }
    
    pub fn y_axis() -> Vector {
        Vector(0., 1., 0.)
    }
    
    pub fn z_axis() -> Vector {
        Vector(0., 0., 1.)
    }
    
    pub fn origin() -> Self {
        Self::new(0, 0, 0)
    }
    
    pub fn new<TX: Into<Num>, TY: Into<Num>, TZ: Into<Num>>(x: TX, y: TY, z: TZ) -> Self {
        Self(x.into(), y.into(), z.into())
    }
    pub fn from_tuple<TX: Into<Num>, TY: Into<Num>, TZ: Into<Num>>(x: (TX, TY, TZ)) -> Self {
        Self(x.0.into(), x.1.into(), x.2.into())
    }
    
    pub fn module(&self) -> Num {
        sqrt(square(self.0) + square(self.1) + square(self.2))
    }
    
    pub fn dot_prod(&self, rhs: &Self) -> Num {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    
    pub fn cross_prod(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == ZERO && self.1 == ZERO && self.2 == ZERO
    }
    
    pub fn is_parallel(&self, rhs: &Self) -> bool {
        self.is_zero() || self.cross_prod(rhs).is_zero()
    }
    
    pub fn is_vertical(&self, rhs: &Self) -> bool {
        self.dot_prod(rhs) == ZERO
    }
    
    pub fn unit(self) -> Self {
        self / self.module()
    }
    
    pub fn distance(self, other: Self) -> Num {
        (self - other).module()
    }
    
    
    pub fn x(&self) -> Num {
        self.0
    }
    
    pub fn y(&self) -> Num {
        self.1
    }
    
    pub fn z(&self) -> Num {
        self.2
    }
    
    pub fn rotate(&self, axis: &Self, theta: Num) -> Self {
        // Rote $self over $theta about $axis, using Rodrigues’ Rotation Formula
        let axis = axis.unit();
        let cos_theta = cos(theta);
        *self * cos_theta
            + axis * (self.dot_prod(&axis) * (num(1) - cos_theta))
            + axis.cross_prod(self) * sin(theta)
    }
    
    pub fn to_tuple(&self) -> (Num, Num, Num) {
        (self.0, self.1, self.2)
    }
    
    pub fn center_of(&self, other: &Vector) -> Vector {
        Vector(center(self.0, other.0), center(self.1, other.1), center(self.2, other.2))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector2D(Num, Num);

impl Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("({}, {})", self.0, self.1).as_str());
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Eq for Vector2D {}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Self;
    fn sub(self, rhs: Vector2D) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Vector2D> for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: Into<Num>> Mul<T> for Vector2D {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let rhs: Num = rhs.into();
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Into<Num>> MulAssign<T> for Vector2D {
    fn mul_assign(&mut self, rhs: T) {
        let rhs: Num = rhs.into();
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl<T: Into<Num>> Div<T> for Vector2D {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let rhs: Num = rhs.into();
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Into<Num>> DivAssign<T> for Vector2D {
    fn div_assign(&mut self, rhs: T) {
        let rhs: Num = rhs.into();
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

#[allow(dead_code)]
impl Vector2D {
    pub fn x_axis(&self) -> Vector2D {
        Vector2D(1., 0.)
    }
    
    pub fn y_axis(&self) -> Vector2D {
        Vector2D(0., 1.)
    }
    
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
    
    pub fn new<TX: Into<Num>, TY: Into<Num>>(x: TX, y: TY) -> Self {
        Self(x.into(), y.into())
    }
    
    pub fn from_tuple<TX: Into<Num>, TY: Into<Num>>(x: (TX, TY)) -> Self {
        Self(x.0.into(), x.1.into())
    }
    
    pub fn module(&self) -> Num {
        sqrt(square(self.0) + square(self.1))
    }
    
    pub fn dot_prod(&self, rhs: &Self) -> Num {
        self.0 * rhs.0 + self.1 * rhs.1
    }
    
    pub fn is_zero(&self) -> bool {
        self.0 == ZERO && self.1 == ZERO
    }
    
    pub fn is_parallel(&self, rhs: &Self) -> bool {
        self.is_zero() || self.0 * rhs.1 == self.1 * rhs.0
    }
    
    pub fn is_vertical(&self, rhs: &Self) -> bool {
        self.dot_prod(rhs) == ZERO
    }
    
    pub fn unit(self) -> Self {
        self / self.module()
    }
    
    pub fn distance(self, other: Self) -> Num {
        (self - other).module()
    }
    
    
    pub fn x(&self) -> Num {
        self.0
    }
    
    pub fn y(&self) -> Num {
        self.1
    }
    
    
    pub fn to_tuple(&self) -> (Num, Num) {
        (self.0, self.1)
    }
    
    pub fn rotate(&self, theta: Num) -> Self {
        Self(cos(theta) * self.0 - sin(theta) * self.1, cos(theta) * self.1 + sin(theta) * self.0)
    }
    
    pub fn center_of(&self, other: &Vector2D) -> Vector2D {
        Vector2D(center(self.0, other.0), center(self.1, other.1))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::num::num;
    
    #[test]
    fn test_vector() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(5, 4, 2);
        
        let c = Vector::new(10, 8, 4);
        let d = a.cross_prod(&b);
        assert_eq!(a + b, Vector::new(6, 6, 5));
        assert_eq!(d, Vector::new(-8, 13, -6));
        assert_eq!(a.is_parallel(&b), false);
        assert_eq!(b.is_parallel(&c), true);
        assert_eq!(b.is_vertical(&d), true);
        assert_eq!(a.is_vertical(&d), true);
        assert_eq!(a.unit().is_vertical(&d), true);
        assert_eq!(Vector::new(1, 2, 2).module(), num(3));
        
        let mut x = Vector::new(1, 2, -3);
        x *= -2;
        assert_eq!(x, Vector::new(-2, -4, 6));
        x /= num(2);
        assert_eq!(x, Vector::new(-1, -2, 3));
        assert_eq!(x.cross_prod(&(x * 1.5)), Vector::new(0, 0, 0));
        assert_eq!(Vector::new(0, -1, 0).distance(Vector::origin()), num(1));
        assert_eq!(Vector::new(1, 0, 0).cross_prod(&Vector::new(0, 1, 0)), Vector::new(0, 0, 1));
    }
    
    #[test]
    fn test_vector2d() {
        let a = Vector2D::new(1, 2);
        let b = Vector2D::new(5, 6);
        
        let c = Vector2D::new(10, 12);
        assert_eq!(a + b, Vector2D::new(6, 8));
        assert_eq!(a.is_parallel(&b), false);
        assert_eq!(b.is_parallel(&c), true);
        assert_eq!(Vector2D::new(3, 4).module(), num(5));
        
        let mut x = Vector2D::new(1, 2);
        x *= -2;
        assert_eq!(x, Vector2D::new(-2, -4));
        x /= num(2);
        assert_eq!(x, Vector2D::new(-1, -2));
        assert_eq!(Vector2D::new(0, -1).distance(Vector2D::origin()), num(1));
    }
}
