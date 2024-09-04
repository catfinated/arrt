use serde::{Serialize, Deserialize};

use crate::math::{Degree, Mat4, Vec3};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub translate: Vec3,
    pub rotate: Vec3,
    pub scale: Vec3,
}

impl Transform {
     /// Create combined transformation matrix
     pub fn mat4(&self) -> Mat4 {
         let t = Mat4::translate(&self.translate);
         let s = Mat4::scale(&self.scale);
         let rx = Mat4::rotate_x(Degree(self.rotate.x()));
         let ry = Mat4::rotate_y(Degree(self.rotate.y()));
         let rz = Mat4::rotate_z(Degree(self.rotate.z()));
         let r = &(&rx * &ry) * &rz;
         &(&t * &r) * &s
     }
 
     /// Create inverse transformation matrix
     pub fn inverse(&self) -> Mat4 {
         let t = Mat4::itranslate(&self.translate);
         let s = Mat4::iscale(&self.scale);
         let rx = Mat4::irotate_x(Degree(self.rotate.x()));
         let ry = Mat4::irotate_y(Degree(self.rotate.y()));
         let rz = Mat4::irotate_z(Degree(self.rotate.z()));
         let r = &(&rz * &ry) * &rx;
         &(&s * &r) * &t
     }
 }
 
 impl Default for Transform {
    fn default() -> Self {
        Self { translate: Vec3::zeros(),
               rotate: Vec3::zeros(),
               scale: Vec3::ones() }
    }
}
