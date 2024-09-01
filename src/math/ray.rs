use super::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub depth: u32,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }
}