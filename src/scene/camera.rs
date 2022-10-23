use serde::{Serialize, Deserialize};

use crate::math::{cross, normalize, Degree, Ray, Vec3};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub eye: Vec3,
    pub up: Vec3,
    pub look_at: Vec3,
    pub dist: f32,
    pub fov: Degree
}

pub struct Camera {
    pub eye: Vec3,
    top_left: Vec3,
    xv: Vec3,
    yv: Vec3,
    sj: f32,
    sk: f32,
    hres: f32,
    vres: f32
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

        println!("{:?}", config.eye);
        println!("{:?}", zv);
        println!("{:?}", vup);
        println!("{:?}", xv);
        println!("{:?}", yv);
        println!("{}", sj);
        println!("{}", sk);
        println!("{:?}", top_left);
        println!("{}", config.fov.0);
        println!("{}", theta.0);
        println!("{}", h);
        println!("{}", config.dist);

        Camera {
            eye: config.eye,
            top_left,
            xv,
            yv,
            sj,
            sk,
            hres,
            vres
        }
    }

    pub fn ray_at(&self, j: u32, k: u32) -> Ray {
        let jf = j as f32;
        let kf = k as f32;

        let v = (self.top_left -
            self.sj * (jf / (self.hres - 1.0_f32)) * self.xv -
            self.sk * (kf / (self.vres - 1.0_f32)) * self.yv) -
            self.eye;

        Ray{origin: self.eye, direction: normalize(v)}
    }
}
