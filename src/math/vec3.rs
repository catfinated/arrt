use std::ops::{Add, Sub, Mul, Div, Index, Neg};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Vec3 {
    pub(super) dat: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{ dat: [x, y, z] }
    }

    pub fn fill(val: f32) -> Vec3 {
        Vec3{ dat: [val; 3] }
    }

    pub fn zeros() -> Vec3 {
        Vec3::fill(0.0)
    }

    pub fn ones() -> Vec3 {
        Vec3::fill(1.0)
    }

    pub fn x(&self) -> f32 {
        self.dat[0]
    }

    pub fn y(&self) -> f32 {
        self.dat[1]
    }

    pub fn z(&self) -> f32 {
        self.dat[2]
    }

    pub fn set_x(&mut self, x: f32) {
        self.dat[0] = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.dat[1] = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.dat[2] = z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { dat: [self.x() + other.x(),
                     self.y() + other.y(),
                     self.z() + other.z()] }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, f: f32) -> Self {
        Vec3 { dat: [self.x() + f,
                     self.y() + f,
                     self.z() + f] }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { dat: [self.x() - other.x(),
                     self.y() - other.y(),
                     self.z() - other.z()] }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, f: f32) -> Self {
        Vec3 { dat: [self.x() - f,
                     self.y() - f,
                     self.z() - f] }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 { dat: [self.x() * other.x(),
                     self.y() * other.y(),
                     self.z() * other.z()] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        Vec3 { dat: [self.x() * f,
                     self.y() * f,
                     self.z() * f] }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        let k = 1.0_f32 / f;
        self * k
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[i]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

pub fn sum(v: Vec3) -> f32 {
    v.x() + v.y() + v.z()
}

pub fn square(v: Vec3) -> Vec3 {
    v * v
}

pub fn length(v: Vec3) -> f32 {
    sum(square(v)).sqrt()
}

pub fn normalize(v: Vec3) -> Vec3 {
    v / length(v)
}

pub fn dot(v: Vec3, u: Vec3) -> f32 {
    sum(v * u)
}

pub fn cross(v: Vec3, u: Vec3) -> Vec3 {
    let x = v.y() * u.z() - v.z() * u.y();
    let y = v.z() * u.x() - v.x() * u.z(); //-(v.x * u.z - v.z * u.x);
    let z = v.x() * u.y() - v.y() * u.x();
    Vec3::new(x, y, z)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let r = 2.0_f32 * dot(n, v) * n - v;
    normalize(r)
}

pub fn refract(v: &Vec3, n: &Vec3, cos_theta_i: f32, eta: f32) -> Option<Vec3> {
    let f  = 1.0_f32 - (1.0_f32 - (cos_theta_i * cos_theta_i)) / (eta * eta);
    let mut result = None;

    if f >= 0.0_f32 {
        let wt = -(*v) / eta - (f.sqrt() - cos_theta_i / eta) * *n;
        result = Some(normalize(wt))
    }

    // total internal reflection !!
    result
}