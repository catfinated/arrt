use serde::{Serialize, Deserialize};

use super::framebuffer::ColorRGB;
use super::math::*;
use super::objects::Surfel;
use super::scene::Scene;

pub trait Light {
    fn direction_from(&self, from: Vec3) -> Vec3;
    fn intensity_at(&self, at: Vec3) -> f32;
    fn ambient(&self) -> ColorRGB;
    fn diffuse(&self) -> ColorRGB;
    fn specular(&self) -> ColorRGB;
}

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    position: Vec3,
    ambient: ColorRGB,
    diffuse: ColorRGB,
    specular: ColorRGB,
}

impl PointLight {
    pub fn new(position: Vec3,
               ambient: ColorRGB,
               diffuse: ColorRGB,
               specular: ColorRGB) -> PointLight {
        PointLight{ position, ambient, diffuse, specular }
    }
}

impl Light for PointLight {
    fn direction_from(&self, from: Vec3) -> Vec3 {
        self.position - from
    }

    fn intensity_at(&self, _at: Vec3) -> f32 {
        1.0_f32
    }

    fn ambient(&self) -> ColorRGB {
        self.ambient
    }

    fn diffuse(&self) -> ColorRGB {
        self.diffuse
    }

    fn specular(&self) -> ColorRGB {
        self.specular
    }
}

pub fn phong_shade<T: Light>(light: &T, surfel: &Surfel, scene: &Scene) -> ColorRGB {
    let material = &scene.materials[surfel.material_id];
    let n = surfel.normal;
    let l = normalize(light.direction_from(surfel.hit_point)); // from P to light
    let v = normalize(scene.camera.eye - surfel.hit_point);  // from P to viewer
    let n_dot_l = dot(n, l).max(0.0_f32);
    let r = normalize((2.0_f32 * n_dot_l * n) - l);
    let r_dot_v = dot(r, v).max(0.0_f32);

    let exp = r_dot_v.powf(material.shininess);
    let il = light.intensity_at(l); // for spot lights
    let diffuse = il * light.diffuse() * material.kd * material.diffuse * n_dot_l;
    let specular = il * light.specular() * material.ks * material.specular * exp;

    diffuse + specular
}
