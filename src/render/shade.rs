
use std::sync::Arc;

use crate::math::{Vec3, normalize, dot};
use crate::objects::{Surfel, Material};
use crate::lights::Light;

use super::ColorRGB;

#[allow(dead_code)]
pub fn phong_shade(lights: &[Arc<dyn Light>], eye: &Vec3, ambient: ColorRGB, surfel: &Surfel, material: &Material) -> ColorRGB {

    let mut color = ColorRGB::black();

    for light in lights.iter() {
        let n = surfel.normal;
        let l = normalize(light.direction_from(surfel.hit_point)); // from P to light
        let v = normalize(*eye - surfel.hit_point);  // from P to viewer
        let n_dot_l = dot(n, l).max(0.0_f32);
        let r = normalize((2.0_f32 * n_dot_l * n) - l);
        let r_dot_v = dot(r, v).max(0.0_f32);

        let exp = r_dot_v.powf(material.shininess);
        let il = light.intensity_at(l); // for spot lights
        let diffuse = il * light.diffuse() * material.kd * material.diffuse * n_dot_l;
        let specular = il * light.specular() * material.ks * material.specular * exp;

        color += diffuse + specular;
    }
    
    let ambient = ambient * material.ka * material.ambient;
    color += ambient;
    color.clamp(0.0_f32, 1.0_f32)
}
