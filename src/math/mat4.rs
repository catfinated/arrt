use std::ops::{Index, IndexMut, Mul};

use super::vec3::Vec3;
use super::vec4::Vec4;
use super::Degree;

#[derive(Debug)]
pub struct Mat4 {
    dat: [f32; 16],
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
        let mut m = Mat4 { dat: [0.0; 16] };
        m.dat[0] = 1.0;
        m.dat[5] = 1.0;
        m.dat[10] = 1.0;
        m.dat[15] = 1.0;
        m
    }

    pub fn zeros() -> Mat4 {
        Mat4 { dat: [0.0; 16] }
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
        m.dat[2] = st;
        m.dat[8] = -st;
        m.dat[10] = ct;
        m
    }

    /// Create inverse rotation matrix about Y-axis
    pub fn irotate_y(theta: Degree) -> Mat4 {
        let mut m = Mat4::identity();
        let ct = theta.cos();
        let st = theta.sin();
        m.dat[0] = ct;
        m.dat[2] = -st;
        m.dat[8] = st;
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
        Mat4 {
            dat: [
                self.dat[0],
                self.dat[4],
                self.dat[8],
                self.dat[12],
                self.dat[1],
                self.dat[5],
                self.dat[9],
                self.dat[13],
                self.dat[2],
                self.dat[6],
                self.dat[10],
                self.dat[14],
                self.dat[3],
                self.dat[7],
                self.dat[11],
                self.dat[15],
            ],
        }
    }
}

impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    #[allow(clippy::many_single_char_names)]
    fn mul(self, v: Vec4) -> Vec4 {
        let x = self[0][0] * v[0] + self[0][1] * v[1] + self[0][2] * v[2] + self[0][3] * v[3];
        let y = self[1][0] * v[0] + self[1][1] * v[1] + self[1][2] * v[2] + self[1][3] * v[3];
        let z = self[2][0] * v[0] + self[2][1] * v[1] + self[2][2] * v[2] + self[2][3] * v[3];
        let w = self[3][0] * v[0] + self[3][1] * v[1] + self[3][2] * v[2] + self[3][3] * v[3];
        Vec4 { dat: [x, y, z, w] }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{vec3::Vec3, vec4::Vec4, Degree};

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn identity_times_vec_is_identity() {
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let r = &Mat4::identity() * v;
        assert!(approx(r[0], 1.0) && approx(r[1], 2.0) && approx(r[2], 3.0));
    }

    #[test]
    fn translate_moves_point() {
        let t = Vec3::new(1.0, 2.0, 3.0);
        let r = &Mat4::translate(&t) * Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!(approx(r[0], 1.0) && approx(r[1], 2.0) && approx(r[2], 3.0));
    }

    #[test]
    fn translate_inverse_cancels() {
        let t = Vec3::new(5.0, -3.0, 2.0);
        let combined = &Mat4::translate(&t) * &Mat4::itranslate(&t);
        let r = &combined * Vec4::new(1.0, 2.0, 3.0, 1.0);
        assert!(approx(r[0], 1.0) && approx(r[1], 2.0) && approx(r[2], 3.0));
    }

    #[test]
    fn scale_stretches_vector() {
        let s = Vec3::new(2.0, 3.0, 4.0);
        let r = &Mat4::scale(&s) * Vec4::new(1.0, 1.0, 1.0, 0.0);
        assert!(approx(r[0], 2.0) && approx(r[1], 3.0) && approx(r[2], 4.0));
    }

    #[test]
    fn scale_inverse_cancels() {
        let s = Vec3::new(2.0, 3.0, 4.0);
        let combined = &Mat4::scale(&s) * &Mat4::iscale(&s);
        let r = &combined * Vec4::new(1.0, 1.0, 1.0, 0.0);
        assert!(approx(r[0], 1.0) && approx(r[1], 1.0) && approx(r[2], 1.0));
    }

    #[test]
    fn rotate_x_90_maps_y_to_z() {
        let r = &Mat4::rotate_x(Degree(90.0)) * Vec4::new(0.0, 1.0, 0.0, 0.0);
        assert!(approx(r[0], 0.0) && approx(r[1], 0.0) && approx(r[2], 1.0));
    }

    #[test]
    fn rotate_y_90_maps_z_to_x() {
        let r = &Mat4::rotate_y(Degree(90.0)) * Vec4::new(0.0, 0.0, 1.0, 0.0);
        assert!(approx(r[0], 1.0) && approx(r[1], 0.0) && approx(r[2], 0.0));
    }

    #[test]
    fn rotate_z_90_maps_x_to_neg_y() {
        let r = &Mat4::rotate_z(Degree(90.0)) * Vec4::new(1.0, 0.0, 0.0, 0.0);
        assert!(approx(r[0], 0.0) && approx(r[1], 1.0) && approx(r[2], 0.0));
    }

    #[test]
    fn transpose_of_rotation_is_inverse() {
        let rot = Mat4::rotate_z(Degree(45.0));
        let m = &rot * &rot.transpose();
        // R * R^T = I for rotation matrices
        assert!(approx(m[0][0], 1.0) && approx(m[1][1], 1.0) && approx(m[2][2], 1.0));
        assert!(approx(m[0][1], 0.0) && approx(m[1][0], 0.0));
    }
}
