use serde::{Serialize, Deserialize};

use crate::math::{cross, normalize, Degree, Ray, Vec3};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub eye: Vec3, // camera location O
    pub up: Vec3, // camera view up vector Vup
    pub look_at: Vec3, // camera view out direction Zv
    pub dist: f32, // distance to image plane
    pub fov: Degree // field of view
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

        println!("eye: {:?}", config.eye);
        println!("zv:  {:?}", zv);
        println!("vup: {:?}", vup);
        println!("xv:  {:?}", xv);
        println!("yv:  {:?}", yv);
        println!("sj:  {}", sj);
        println!("sk:  {}", sk);
        println!("top left {:?}", top_left);
        println!("fov:   {}", config.fov.0);
        println!("theta: {}", theta.0);
        println!("h:     {}", h);
        println!("dist:  {}", config.dist);

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

    pub fn ray_at(&self, jf: f32, kf: f32) -> Ray {
        let v = (self.top_left -
            self.sj * (jf / (self.hres - 1.0_f32)) * self.xv -
            self.sk * (kf / (self.vres - 1.0_f32)) * self.yv) -
            self.eye;

        Ray{origin: self.eye, direction: normalize(v), depth: 0}
    }
}
