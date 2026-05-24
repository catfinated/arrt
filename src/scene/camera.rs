use serde::{Deserialize, Serialize};

use crate::math::{cross, normalize, Degree, Ray, Vec3};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub eye: Vec3,     // camera location O
    pub up: Vec3,      // camera view up vector Vup
    pub look_at: Vec3, // camera view out direction Zv
    pub dist: f32,     // distance to image plane
    pub fov: Degree,   // field of view
}

pub struct Camera {
    pub eye: Vec3,
    top_left: Vec3,
    xv: Vec3,
    yv: Vec3,
    sj: f32,
    sk: f32,
    hres: f32,
    vres: f32,
}

impl Camera {
    pub fn new(config: &CameraConfig, hres: f32, vres: f32) -> Camera {
        let zv = normalize(config.look_at - config.eye);
        let vup = normalize(config.up);
        // right handed coordinates
        let xv = normalize(cross(vup, zv));
        let yv = normalize(cross(zv, xv));

        let theta = Degree(config.fov.0 / 2.0_f32);
        let h = config.dist * theta.tan();
        let sj = 2.0_f32 * h;
        let sk = sj * (vres / hres);

        let top_left = config.eye + config.dist * zv + (sj / 2.0_f32) * xv + (sk / 2.0_f32) * yv;

        log::debug!("eye: {:?}", config.eye);
        log::debug!("zv:  {zv:?}");
        log::debug!("vup: {vup:?}");
        log::debug!("xv:  {xv:?}");
        log::debug!("yv:  {yv:?}");
        log::debug!("sj:  {sj}");
        log::debug!("sk:  {sk}");
        log::debug!("top left {top_left:?}");
        log::debug!("fov:   {}", config.fov.0);
        log::debug!("theta: {}", theta.0);
        log::debug!("h:     {h}");
        log::debug!("dist:  {}", config.dist);

        Camera {
            eye: config.eye,
            top_left,
            xv,
            yv,
            sj,
            sk,
            hres,
            vres,
        }
    }

    pub fn ray_at(&self, jf: f32, kf: f32) -> Ray {
        let v = (self.top_left
            - self.sj * (jf / (self.hres - 1.0_f32)) * self.xv
            - self.sk * (kf / (self.vres - 1.0_f32)) * self.yv)
            - self.eye;

        Ray {
            origin: self.eye,
            direction: normalize(v),
            depth: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    fn simple_camera() -> Camera {
        let config = CameraConfig {
            eye: Vec3::new(0.0, 0.0, 3.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            dist: 1.0,
            fov: Degree(60.0),
        };
        Camera::new(&config, 512.0, 512.0)
    }

    #[test]
    fn center_ray_points_toward_scene() {
        let cam = simple_camera();
        let ray = cam.ray_at(255.5, 255.5);
        assert!(ray.direction.z() < -0.99);
    }

    #[test]
    fn ray_direction_is_normalized() {
        let cam = simple_camera();
        let ray = cam.ray_at(0.0, 0.0);
        let len_sq = ray.direction.x() * ray.direction.x()
            + ray.direction.y() * ray.direction.y()
            + ray.direction.z() * ray.direction.z();
        assert!((len_sq.sqrt() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn ray_origin_is_eye() {
        let cam = simple_camera();
        let ray = cam.ray_at(100.0, 200.0);
        assert!((ray.origin.x() - 0.0).abs() < 1e-5);
        assert!((ray.origin.y() - 0.0).abs() < 1e-5);
        assert!((ray.origin.z() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn left_and_right_edge_rays_are_symmetric() {
        let cam = simple_camera();
        let left = cam.ray_at(0.0, 255.5);
        let right = cam.ray_at(511.0, 255.5);
        assert!((left.direction.x() + right.direction.x()).abs() < 1e-4);
    }
}
