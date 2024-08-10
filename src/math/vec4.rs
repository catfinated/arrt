use std::ops::Index;

use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub(super) dat: [f32; 4]
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

    pub fn to_vec3(self) -> Vec3 {
        Vec3{ dat: [self.dat[0], self.dat[1], self.dat[2]] }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[i]
    }
}
