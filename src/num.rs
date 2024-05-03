use std::cmp::Ordering;

pub type Num = f64;
pub type Int = i32;

#[allow(dead_code)]
mod consts {
    use super::*;
    
    pub const PRECISION: Num = 0.000000000000001_f64;
    
    pub const ZERO: Num = 0.0;
    pub const ONE: Num = 1.0;
    pub const TWO: Num = 2.0;
    pub const NEGATIVE_ONE: Num = -1.0;
    pub const PI: Num = std::f64::consts::PI;
    pub const E: Num = std::f64::consts::E;
}

#[allow(dead_code)]
mod funcs {
    use super::*;
    
    pub fn sqrt(x: Num) -> Num {
        x.sqrt()
    }
    
    pub fn pow(x: Num, y: Num) -> Num {
        x.powf(y)
    }
    
    pub fn abs(x: Num) -> Num {
        x.abs()
    }
    
    pub fn max(x: Num, y: Num) -> Num {
        if let Ordering::Greater = y.total_cmp(&x) {
            y
        } else {
            x
        }
    }
    
    pub fn min(x: Num, y: Num) -> Num {
        if let Ordering::Greater = x.total_cmp(&y) {
            y
        } else {
            x
        }
    }
    
    pub fn square(x: Num) -> Num {
        x * x
    }
    
    pub fn round(x: Num) -> Num {
        x.round()
    }
    
    pub fn floor(x: Num) -> Num {
        x.floor()
    }
    
    pub fn ceil(x: Num) -> Num {
        x.ceil()
    }
    
    pub fn cos(x: Num) -> Num {
        x.cos()
    }
    
    pub fn sin(x: Num) -> Num {
        x.sin()
    }
    
    pub fn acos(x: Num) -> Num {
        x.acos()
    }
    
    pub fn asin(x: Num) -> Num {
        x.asin()
    }
    
    pub fn center(x: Num, y: Num) -> Num {
        (x + y) / num(2)
    }
}

pub use consts::*;
pub use funcs::*;

pub fn num<T>(x: T) -> Num
    where
        T: Into<Num>,
{
    //Num::from(x)
    x.into()
}


#[cfg(test)]
mod test {
    use super::*;
    
    
    #[test]
    fn test_num_calc() {
        let x = num(4);
        assert_eq!(pow(x, num(2)), num(16));
        assert_eq!(sqrt(x), num(2));
        assert_eq!(abs(num(-2)), num(2));
        assert_eq!(square(num(2)), x);
        assert!(sin(num(PI / num(6))) - num(0.5) < PRECISION);
        assert!(cos(num(PI / num(3))) - num(0.5) < PRECISION);
        assert!(acos(num(0.5)) - num(PI / num(3)) < PRECISION);
        assert!(asin(num(0.5)) - num(PI / num(3)) < PRECISION);
    }
}
