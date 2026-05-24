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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    #[test]
    fn point_at_t0_is_origin() {
        let ray = Ray {
            origin: Vec3::new(4.0, 5.0, 6.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
            depth: 0,
        };
        let p = ray.point_at(0.0);
        assert!((p.x() - 4.0).abs() < 1e-5);
        assert!((p.y() - 5.0).abs() < 1e-5);
        assert!((p.z() - 6.0).abs() < 1e-5);
    }

    #[test]
    fn point_at_t1_along_x() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
            depth: 0,
        };
        let p = ray.point_at(1.0);
        assert!((p.x() - 1.0).abs() < 1e-5);
        assert!(p.y().abs() < 1e-5);
        assert!(p.z().abs() < 1e-5);
    }

    #[test]
    fn point_at_offset_origin() {
        let ray = Ray {
            origin: Vec3::new(1.0, 2.0, 3.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
            depth: 0,
        };
        let p = ray.point_at(3.0);
        assert!((p.x() - 1.0).abs() < 1e-5);
        assert!((p.y() - 5.0).abs() < 1e-5);
        assert!((p.z() - 3.0).abs() < 1e-5);
    }
}
