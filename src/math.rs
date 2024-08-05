use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use serde::{Serialize, Deserialize};

const PI_OVER_ONE_EIGHTY : f32 = std::f32::consts::PI / 180.0_f32;
const ONE_EIGHTY_OVER_PI : f32 = 180.0_f32 / std::f32::consts::PI;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Degree(pub f32);
pub struct Radian(pub f32);

pub fn to_radians(d: Degree) -> Radian {
    Radian(d.0 * PI_OVER_ONE_EIGHTY)
}

pub fn to_degrees(r: Radian) -> Degree {
    Degree(r.0 * ONE_EIGHTY_OVER_PI)
}

impl Radian {
    fn sin(&self) -> f32 { self.0.sin() }
    fn cos(&self) -> f32 { self.0.cos() }
    fn tan(&self) -> f32 { self.0.tan() }
}

impl Degree {
    pub fn sin(&self) -> f32 { to_radians(*self).sin() }
    pub fn cos(&self) -> f32 { to_radians(*self).cos() }
    pub fn tan(&self) -> f32 { to_radians(*self).tan() }
}

#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32
}

pub fn in_range(range: Range, f: f32) -> bool {
    f < range.max && f > range.min
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Vec3 {
    dat: [f32; 3],
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

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/*
    [ 0,  1,  2,
      3,  4,  5,
      6,  7, 8]
*/
pub struct Mat3 {
    dat: [f32; 9],
}

impl Mat3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(xx: f32, xy: f32, xz: f32,
               yx: f32, yy: f32, yz: f32,
               zx: f32, zy: f32, zz: f32) -> Mat3 {
        Mat3{ dat: [xx, xy, xz, yx, yy, yz, zx, zy, zz] }
    }

    pub fn from_vecs(x: Vec3, y: Vec3, z: Vec3) -> Mat3 {
        Mat3{ dat: [x.x(), x.y(), x.z(),
                    y.x(), y.y(), y.z(),
                    z.x(), z.y(), z.z()] }
    }

    pub fn fill(v: f32) -> Mat3 {
        Mat3{ dat: [v; 9] }
    }

    pub fn identity() -> Mat3 {
        let mut m = Mat3::fill(0.0_f32);
        m.dat[0] = 1.0f32;
        m.dat[4] = 1.0f32;
        m.dat[8] = 1.0f32;
        m
    }
}

impl Index<usize> for Mat3 {
    type Output = [f32];

    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[(i * 3)..(i * 3) + 3]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.dat[(i * 3)..(i * 3) + 3]
    }
}

pub fn determinant(m: &Mat3) -> f32 {

    m.dat[0] * (m.dat[4] * m.dat[8] - m.dat[5] * m.dat[7]) -
    m.dat[1] * (m.dat[3] * m.dat[8] - m.dat[5] * m.dat[6]) +
    m.dat[2] * (m.dat[3] * m.dat[7] - m.dat[4] * m.dat[6])
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    dat: [f32; 4]
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4{ dat: [x, y, z, w] }
    }

    pub fn zeros() -> Vec4 {
        Vec4{ dat: [0.0; 4] }
    }

    pub fn from_vec3(v: Vec3, w: f32) -> Vec4 {
        Vec4{ dat: [ v.x(), v.y(), v.z(), w ] }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3{ dat: [self.dat[0], self.dat[1], self.dat[2]] }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[i]
    }
}

#[derive(Debug)]
pub struct Mat4 {
    dat: [f32; 16]
}

impl Index<usize> for Mat4 {
    type Output = [f32];

    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[(i * 4)..(i * 4) + 4]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.dat[(i * 4)..(i * 4) + 4]
    }
}

impl Mat4 {
    /*
    [ 0,  1,  2,  3,
      4,  5,  6,  7,
      8,  9, 10, 11,
     12, 13, 14, 15 ]
    */
    pub fn identity() -> Mat4 {
        let mut m = Mat4{ dat: [0.0; 16] };
        m.dat[0] = 1.0;
        m.dat[5] = 1.0;
        m.dat[10] = 1.0;
        m.dat[15] = 1.0;
        m
    }

    pub fn zeros() -> Mat4 {
        Mat4{ dat: [0.0; 16] }
    }

    pub fn translate(v: &Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.dat[3] = v.x();
        m.dat[7] = v.y();
        m.dat[11] = v.z();
        m
    }

    /// Create inverse translation matrix
    pub fn itranslate(v: &Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.dat[3] = -v.x();
        m.dat[7] = -v.y();
        m.dat[11] = -v.z();
        m
    }

    pub fn scale(v: &Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.dat[0] = v.x();
        m.dat[5] = v.y();
        m.dat[10] = v.z();
        m
    }

    /// Create inverse scaling matrix
    pub fn iscale(v: &Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.dat[0] = 1.0 / v.x();
        m.dat[5] = 1.0 / v.y();
        m.dat[10] = 1.0 / v.z();
        m
    }

    pub fn rotate_x(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[5] = ct;
        m.dat[6] = -st;
        m.dat[9] = st;
        m.dat[10] = ct;
        m
    }

    /// Create inverse rotation matrix about X-axis
    pub fn irotate_x(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[5] = ct;
        m.dat[6] = st;
        m.dat[9] = -st;
        m.dat[10] = ct;
        m
    }

    pub fn rotate_y(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[0] = ct;
        m.dat[2] = -st;
        m.dat[8] = st;
        m.dat[10] = ct;
        m
    }

    /// Create inverse rotation matrix about Y-axis
    pub fn irotate_y(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[0] = ct;
        m.dat[2] = st;
        m.dat[8] = -st;
        m.dat[10] = ct;
        m
    }

    pub fn rotate_z(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[0] = ct;
        m.dat[1] = -st;
        m.dat[4] = st;
        m.dat[5] = ct;
        m
    }

    /// Create inverse rotation matrix about Z-axis
    pub fn irotate_z(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[0] = ct;
        m.dat[1] = st;
        m.dat[4] = -st;
        m.dat[5] = ct;
        m
    }

    /*
    [ 0,  1,  2,  3,
      4,  5,  6,  7,
      8,  9, 10, 11,
     12, 13, 14, 15 ]
    */
    pub fn transpose(&self) -> Self {
        Mat4 { dat: [ self.dat[0], self.dat[4], self.dat[8], self.dat[12],
                      self.dat[1], self.dat[5], self.dat[9], self.dat[13], 
                      self.dat[2], self.dat[6], self.dat[10], self.dat[14],
                      self.dat[3], self.dat[7], self.dat[11], self.dat[15],
                    ]
            }
    } 

}

impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        let x = self[0][0] * v[0] + self[0][1] * v[1] + self[0][2] * v[2] + self[0][3] * v[3];
        let y = self[1][0] * v[0] + self[1][1] * v[1] + self[1][2] * v[2] + self[1][3] * v[3];
        let z = self[2][0] * v[0] + self[2][1] * v[1] + self[2][2] * v[2] + self[2][3] * v[3];
        let w = self[3][0] * v[0] + self[3][1] * v[1] + self[3][2] * v[2] + self[3][3] * v[3];
        Vec4{ dat: [x, y, z, w] }
    }
}

impl Mul for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Mat4 {
        let mut m = Mat4::zeros();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        m
    }
}
