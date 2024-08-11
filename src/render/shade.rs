
use std::sync::Arc;

use crate::math::{Vec3, Ray, normalize, dot};
use crate::objects::{Surfel, Material};
use crate::lights::Light;

use super::{ColorRGB, RayTracer};

#[allow(dead_code)]
pub fn phong_shade(lights: &[Arc<dyn Light>], eye: &Vec3, surfel: &Surfel, material: &Material) -> ColorRGB {

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
    color
}

pub fn hall_shade(lights: &[Arc<dyn Light>], eye: &Vec3, surfel: &Surfel, material: &Material, tracer: &RayTracer) -> ColorRGB {
    let mut color = ColorRGB::black();

    let n = surfel.normal;
    let v = normalize(*eye - surfel.hit_point); // from P to viewer

    for light in lights.iter() {
        let l = normalize(light.direction_from(surfel.hit_point)); // from P to light
        let mut intensity = light.intensity_at(l); // for spot lights

        if dot(n, l) > 0.0_f32 {
            let ray = Ray{origin: surfel.hit_point, direction: l};
            intensity = tracer.shadow(&ray, intensity);
        }

        if intensity == 0.0_f32 {
            continue;
        }
        
        let n_dot_l = dot(n, l).max(0.0_f32);
        let h = normalize(l + v);
        let n_dot_h = dot(n, h).max(0.0_f32);
        let exp = n_dot_h.powf(material.shininess);

        // diffuse reflection from light sources
        // + kd * Ilj * Cd * cos(theta)
        // todo texture mapping
        let diffuse = intensity * light.diffuse() * material.kd * material.diffuse * n_dot_l;

        // specular reflection from light sources
        // + ks * Ilj * Cs * cos(phi)^n
        let specular = intensity * light.specular() * material.ks * material.specular * exp;

        color += diffuse + specular;
    }

    color
}