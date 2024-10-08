use std::ops::{Index, IndexMut};
/*
    [ 0,  1,  2,
      3,  4,  5,
      6,  7, 8]
*/
pub struct Mat3 {
    pub(super) dat: [f32; 9],
}

impl Mat3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(xx: f32, xy: f32, xz: f32,
               yx: f32, yy: f32, yz: f32,
               zx: f32, zy: f32, zz: f32) -> Mat3 {
        Mat3{ dat: [xx, xy, xz, yx, yy, yz, zx, zy, zz] }
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