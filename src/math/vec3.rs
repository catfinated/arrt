use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Vec3 {
    pub(super) dat: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { dat: [x, y, z] }
    }

    pub fn fill(val: f32) -> Vec3 {
        Vec3 { dat: [val; 3] }
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
        Vec3 {
            dat: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, f: f32) -> Self {
        Vec3 {
            dat: [self.x() + f, self.y() + f, self.z() + f],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            dat: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, f: f32) -> Self {
        Vec3 {
            dat: [self.x() - f, self.y() - f, self.z() - f],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            dat: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        Vec3 {
            dat: [self.x() * f, self.y() * f, self.z() * f],
        }
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

#[allow(clippy::many_single_char_names)]
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
    let f = 1.0_f32 - (1.0_f32 - (cos_theta_i * cos_theta_i)) / (eta * eta);
    let mut result = None;

    if f >= 0.0_f32 {
        let wt = -(*v) / eta - (f.sqrt() - cos_theta_i / eta) * *n;
        result = Some(normalize(wt));
    }

    // total internal reflection !!
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn add() {
        let c = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert_eq!([c.x(), c.y(), c.z()], [5.0, 7.0, 9.0]);
    }

    #[test]
    fn sub() {
        let c = Vec3::new(4.0, 5.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert_eq!([c.x(), c.y(), c.z()], [3.0, 3.0, 3.0]);
    }

    #[test]
    fn scalar_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0) * 2.0;
        assert_eq!([v.x(), v.y(), v.z()], [2.0, 4.0, 6.0]);
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1.0, -2.0, 3.0);
        assert_eq!([v.x(), v.y(), v.z()], [-1.0, 2.0, -3.0]);
    }

    #[test]
    fn dot_product() {
        assert_eq!(
            dot(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0)),
            32.0
        );
    }

    #[test]
    fn dot_orthogonal_is_zero() {
        assert_eq!(dot(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)), 0.0);
    }

    #[test]
    fn cross_basis_vectors() {
        let z = cross(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert!(approx(z.x(), 0.0) && approx(z.y(), 0.0) && approx(z.z(), 1.0));
    }

    #[test]
    fn length_3_4_5() {
        assert!(approx(length(Vec3::new(3.0, 4.0, 0.0)), 5.0));
    }

    #[test]
    fn normalize_is_unit() {
        let n = normalize(Vec3::new(3.0, 4.0, 0.0));
        assert!(approx(length(n), 1.0));
        assert!(approx(n.x(), 0.6) && approx(n.y(), 0.8));
    }

    #[test]
    fn reflect_along_normal() {
        // v == n: reflected direction equals incident direction
        let v = normalize(Vec3::new(0.0, 1.0, 0.0));
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = reflect(v, n);
        assert!(approx(r.y(), 1.0));
    }

    #[test]
    fn reflect_flips_tangent() {
        // v at 45° to normal: tangential component flips sign
        let v = normalize(Vec3::new(1.0, 1.0, 0.0));
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = reflect(v, n);
        assert!(r.x() < 0.0 && r.y() > 0.0);
    }

    #[test]
    fn refract_total_internal_reflection() {
        // cos_theta_i = 0 (grazing), eta = 0.5 => sin²/η² > 1
        let v = normalize(Vec3::new(1.0, 0.0, 0.0));
        let n = Vec3::new(0.0, 1.0, 0.0);
        assert!(refract(&v, &n, 0.0, 0.5).is_none());
    }

    #[test]
    fn refract_normal_incidence_no_bending() {
        // v is the view vector (from hit point toward viewer), n is surface normal.
        // With v == n (straight-on) and eta=1, transmitted ray goes straight through (-y).
        let v = Vec3::new(0.0, 1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let t = refract(&v, &n, 1.0, 1.0).unwrap();
        assert!(approx(t.y(), -1.0));
    }
}
