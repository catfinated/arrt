use std::ops::Deref;
use std::sync::Arc;
use std::time::{Instant, Duration};

use super::{ColorRGB, XYCoord};
use super::shade::phong_shade;

use crate::scene::{Camera, Scene};
use crate::math::{Ray, Range};
use crate::objects::{Object, Surfel};

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
        let mut range = Range{ min: 1e-6, max: f32::MAX };
        let mut surfel = None;

        for object in &self.objects {
            if let Some(surf) = object.intersect(ray, range) {
                    range.max = surf.t;
                    surfel = Some(surf);
            } 
        }
        surfel
    }

    fn sample_ray(&self, ray: &Ray) -> (ColorRGB, bool) {

        let surfel = self.trace_ray(ray);

        match surfel {
            Some(surf) => {
                let mut color = ColorRGB::black();
                let material = self.scene.material_for_surfel(&surf);
                let ambient = self.scene.ambient() * material.ka * material.ambient;

                for light in self.scene.lights().iter() {
                    color += phong_shade(light.deref(), &self.camera.eye, &surf, material);
                }

                color += ambient;
                color = color.clamp(0.0_f32, 1.0_f32);
                (color, true)
            }
            None => { (self.scene.bgcolor(), false) }
        }
    }
}

