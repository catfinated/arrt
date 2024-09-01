use std::sync::Arc;
use std::time::{Instant, Duration};

use super::{ColorRGB, XYCoord};

use crate::scene::{Camera, Scene};
use crate::math::{Ray, Range, normalize, dot, reflect, refract};
use crate::objects::{Object, Surfel, Material};

/// Core ray tracer
pub struct RayTracer {
    scene: Scene,
    camera: Camera,
    objects: Vec<Arc<dyn Object>>
}

#[derive(Copy,Clone)]
pub struct TraceResult {
    ray_count: u32,
    hit_count: u32,
    trace_sum: Duration,
    trace_max: Duration,
}

/// Trace context which can track per thread execution metrics
#[derive(Copy, Clone)]
pub struct TraceContext<'tracer> {
    tracer: &'tracer RayTracer,
    pub result: TraceResult
}

impl TraceResult {
    pub fn new() -> Self {
        TraceResult {
            ray_count: 0,
            hit_count: 0,
            trace_sum: Duration::from_secs(0),
            trace_max: Duration::from_secs(0),
        }
    }

    pub fn combine(&self, rhs: &Self) -> Self {
        let mut trace_max = self.trace_max;
        if trace_max < rhs.trace_max {
            trace_max = rhs.trace_max
        }

        TraceResult {
            ray_count: self.ray_count + rhs.ray_count,
            hit_count: self.hit_count + rhs.hit_count,
            trace_sum: self.trace_sum + rhs.trace_sum,
            trace_max,
        }
    }

    pub fn print_stats(&self) {
        let mut hit_percent = 0.0;
        let mut avg_trace = Duration::from_secs(0);
        if self.ray_count > 0 {
            hit_percent = (self.hit_count as f32 / self.ray_count as f32) * 100.0_f32;
            avg_trace = self.trace_sum / self.ray_count;
        }
        println!("ray count: {}, hit count: {}, hit %: {:.2}, sum: {:?}, avg: {:?}, max: {:?}",
                 self.ray_count, self.hit_count, hit_percent, self.trace_sum, avg_trace, self.trace_max);
    }
}

impl<'tracer> TraceContext<'tracer> {
    pub fn new(tracer: &'tracer RayTracer) -> Self {
       TraceContext{tracer, result: TraceResult::new()}
    }

    pub fn sample_point(&mut self, j: usize, k: usize) -> ColorRGB {
        let ray = self.tracer.camera.ray_at(j as f32, k as f32);
        self.trace_ray(&ray)
    }

    pub fn sample_coord(&mut self, coord: XYCoord) -> ColorRGB {
        let ray = self.tracer.camera.ray_at(coord.x, coord.y);
        self.trace_ray(&ray)
    }

    fn trace_ray(&mut self, ray: &Ray) -> ColorRGB {
        self.result.ray_count += 1;
        let start = Instant::now();
        let (color, hit) = self.tracer.sample_ray(ray);
        let stop = Instant::now();
        let delta = stop - start;
        self.result.trace_sum += delta;
        if delta > self.result.trace_max {self.result.trace_max = delta; }
        if hit { self.result.hit_count += 1 }
        color
    }
}

impl RayTracer {
    pub fn new(scene: Scene) -> Self {
        let camera = scene.make_camera();
        let objects = scene.make_objects();
        RayTracer{scene,
                  camera,
                  objects}
    }

    fn trace_ray(&self, ray: &Ray) -> Option<Surfel> {
        let mut range = Range{ min: 0.025, max: f32::MAX };
        let mut surfel = None;

        for object in &self.objects {
            if let Some(surf) = object.intersect(ray, range) {
                range.max = surf.t;
                surfel = Some(surf);
            } 
        }
        surfel
    }

    pub fn sample_ray(&self, ray: &Ray) -> (ColorRGB, bool) {

        let max_depth = 5_u32;
        let surfel = self.trace_ray(ray);

        if ray.depth > max_depth {
            return (ColorRGB::black(), false);
        }

        match surfel {
            Some(surf) => {
                let material = self.scene.material_for_surfel(&surf);
                let color = self.shade(&surf, material, ray.depth);
                (color, true)
            }
            None => { (self.scene.bgcolor(), false) }
        }
    }

    /// Calculate light intensity due to shadowing
    fn shadow_intensity(&self, ray: &Ray, light_intensity: f32) -> f32 {
        let mut range = Range{min: 0.001_f32, max: f32::MAX};
        let mut intensity = light_intensity;

        for object in &self.objects {
            if let Some(surf) = object.intersect(ray, range) {
                let material = self.scene.material_for_surfel(&surf);
                if material.kt > 0.0_f32 {
                    intensity *= 0.5;
                    range.min = surf.t;
                } else {   
                    return 0.0_f32;
                }
            } 
        }

        intensity
    }

    /// Apply shading to the given surface and material
    /// Uses Hall/phong model 
    fn shade(&self, 
        surfel: &Surfel, 
        material: &Material, 
        curr_depth: u32) -> ColorRGB {

        let mut color = ColorRGB::black();

        let mut n = normalize(surfel.normal);
        let v = normalize(self.camera.eye - surfel.hit_point); // from P to viewer
        let mut visible_lights = Vec::new();

        for light in self.scene.lights().iter() {
            let l = normalize(light.direction_from(surfel.hit_point)); // from P to light
            let mut intensity = light.intensity_at(l); // for spot lights

            // shadows
            if dot(n, l) > 0.0_f32 { // hit pint faces towards light
                let ray = Ray{origin: surfel.hit_point + (0.01_f32 *n), direction: l, depth: curr_depth};
                intensity = self.shadow_intensity(&ray, intensity);
            }

            if intensity == 0.0_f32 {
                continue;
            }

            visible_lights.push(light.clone());

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

        // reflections
        let mut reflected_color = ColorRGB::black();

        if material.kr > 0.0_f32 { // material is reflective
            let r = reflect(v, n);
            let reflected = Ray{origin: surfel.hit_point + (surfel.n_offset * n), direction: r, depth: curr_depth + 1};
            let reflected_intensity = self.sample_ray(&reflected).0;
            // specular reflection from other surfaces
            // + kr * Ir * Cs
            reflected_color += material.kr * reflected_intensity * material.specular;
            color += reflected_color;
        }

        // refractions
        if material.kt > 0.0_f32 {
            let mut eta = material.ior;
            let mut cos_theta_i = dot(n, v);

            if cos_theta_i < 0.0_f32 {
                cos_theta_i = -cos_theta_i;
                n = normalize(-n);
                eta = 1.0_f32 / eta;
            }

            if let Some(t) = refract(&v, &n, cos_theta_i, eta) {
                let transmitted = Ray{origin: surfel.hit_point + (surfel.n_offset * n), direction: t, depth: curr_depth + 1};
                let it = self.sample_ray(&transmitted).0;
                color += material.kt * it * material.transmissive;

                for light in &visible_lights {
                    let l = normalize(light.direction_from(surfel.hit_point));
                    let cos_alpha = dot(t, l).max(0.0_f32);
                    let f = cos_alpha.powf(material.highlight);
                    let il = light.intensity_at(l);
                    color += material.kt * il * light.specular() * material.transmissive * f;
                }
   
            } else {
                // option 1 - return reflective color
                //let it = reflected_color;

                // option 2 - shot an internal reflect ray
                let r = reflect(v, n);
                let ray = Ray{origin: surfel.hit_point + (surfel.n_offset * n), direction: r, depth: curr_depth + 1};

                // option 3 - make T = -V
                //let ray = Ray{origin: surfel.hit_point + (surfel.n_offset * n), direction: -v, depth: curr_depth + 1};

                // 2 or 3
                let it = self.sample_ray(&ray).0;
                color += material.kt * it * material.transmissive;
            }
        }

        // ambient lighting
        let ambient = self.scene.ambient() * material.ka * material.ambient;
        color += ambient;
        color.clamp(0.0_f32, 1.0_f32)
    }

}

