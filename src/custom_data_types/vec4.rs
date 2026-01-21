use std::ops::{Add, Div, Mul, Sub, Neg};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, rhs: Vec4) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w 
    }

    pub fn cross(self, rhs: Vec4) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0.0,
        }
    }

    pub fn magnitude_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.magnitude()
    }
}
/*
impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Vec4) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
*/
impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Vec4 {
        if rhs == 0.0 {
            panic!("Division by 0 inside Vec4 Div Trait\n");
        }

        let inv = 1.0 / rhs;

        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
            w: self.w * inv,
        }
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
         }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}