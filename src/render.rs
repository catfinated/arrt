use crate::scene::{Camera, Light, Scene, Surfel, Material, Object};
use crate::framebuffer::{Framebuffer, ColorRGB};
use crate::math::{normalize, dot, Vec3, Ray};
use crate::bvh::BVH;

use std::time::{Instant, Duration};

/// A 2d view plane coordinate
#[derive(Debug, Copy, Clone)]
struct XYCoord {
    x: f32,
    y: f32,
}

/// A super pixel capable of doing
/// 0, 1, or 2 levels of adaptive supersampling
#[derive(Debug)]
struct Pixel {
    j: u32,
    k: u32,
    stash: [[Option<ColorRGB>; 5]; 5]
}

impl Pixel {
    fn new(j: u32, k: u32) -> Self {
        Pixel { j, k, stash: [[None; 5]; 5]}
    }

    fn sample(&mut self, tracer: &mut RayTracer, framebuf: &Framebuffer, max_depth: u8) -> ColorRGB {
        self.stash[4][0] = Some(framebuf.get_color(self.j, self.k));
        self.stash[4][4] = Some(framebuf.get_color(self.j + 1, self.k));
        self.stash[0][4] = Some(framebuf.get_color(self.j + 1, self.k - 1));
        self.stash[0][0] = Some(framebuf.get_color(self.j, self.k - 1));
        let bottom_left = XYCoord{ x: self.j as f32, y: self.k as f32 };

        match max_depth {
            0 => self.subdivide(bottom_left, (4, 0), 4, tracer, 4),
            1 => self.subdivide(bottom_left, (4, 0), 4, tracer, 2),
            _ => self.subdivide(bottom_left,(4, 0), 4, tracer, 1),
        }
    }

    fn subdivide(&mut self, coord: XYCoord, bottom_left: (usize, usize), depth: usize, tracer: &mut RayTracer, min_depth: usize) -> ColorRGB {
        let off = depth;
        let adjust = depth as f32 / 4.0;

        let a = *self.stash[bottom_left.0][bottom_left.1]
            .get_or_insert_with(|| tracer.sample_coord(coord));

        let b = *self.stash[bottom_left.0][bottom_left.1 + off]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x + adjust, y: coord.y}));

        let e = *self.stash[bottom_left.0 - off][bottom_left.1 + off]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x + adjust, y: coord.y - adjust}));

        let d = *self.stash[bottom_left.0 - off][bottom_left.1]
            .get_or_insert_with(|| tracer.sample_coord(XYCoord{x: coord.x, y: coord.y - adjust}));

        let samples = [a, b, e, d];

        if !samples_differ(&samples) || depth == min_depth {
            average_color(&samples)
        } else {
            let d = depth / 2;
            let off = d;
            let adjust = depth as f32 / 4.0;

            let a = self.subdivide(coord, bottom_left, d, tracer, min_depth);

            let b = self.subdivide(XYCoord{x: coord.x + adjust, y: coord.y}, (bottom_left.0, bottom_left.1 + off), d, tracer, min_depth);

            let e = self.subdivide(XYCoord{x: coord.x + adjust, y: coord.y - adjust}, (bottom_left.0 - off, bottom_left.1 + off), d, tracer, min_depth);

            let d = self.subdivide(XYCoord{x: coord.x, y: coord.y - adjust}, (bottom_left.0 - off, bottom_left.1), d, tracer, min_depth);

            let samples = [a, b, e, d];
            average_color(&samples)
        }
    }
}

fn average_color(samples: &[ColorRGB; 4]) -> ColorRGB {

    let mut sum = ColorRGB{ r: 0.0, g: 0.0, b: 0.0 };
    for &sample in samples {
        sum += sample;
    }
    sum / 4.0
}

fn samples_differ(samples: &[ColorRGB; 4]) -> bool {
    colors_differ(&samples[0], &samples[1]) ||
        colors_differ(&samples[0], &samples[3]) ||
        colors_differ(&samples[2], &samples[3]) ||
        colors_differ(&samples[2], &samples[1])
}

fn colors_differ(a: &ColorRGB, b: &ColorRGB) -> bool {
    let diff = *a - *b;
    const TOLERANCE: f32 = 0.05;
    diff.r.abs() > TOLERANCE || diff.g.abs() > TOLERANCE || diff.b.abs() > TOLERANCE

}

/// Core ray tracer
struct RayTracer {
    scene: Scene,
    camera: Camera,
    bvh: BVH<Object>,
    ray_count: u32,
    hit_count: u32,
    trace_sum: Duration,
    trace_max: Duration,

}

impl RayTracer {
    fn new(scene: Scene) -> Self {
        let camera = scene.make_camera();
        let bvh = scene.make_bvh();
        println!("bbox: {:?}", bvh.bbox);
        RayTracer{scene,
                  camera,
                  bvh,
                  ray_count: 0,
                  hit_count: 0,
                  trace_sum: Duration::from_secs(0),
                  trace_max: Duration::from_secs(0)}
    }

    fn sample_point(&mut self, j: u32, k: u32) -> ColorRGB {
        let ray = self.camera.ray_at(j as f32, k as f32);
        self.trace_ray(&ray)
    }

    fn sample_coord(&mut self, coord: XYCoord) -> ColorRGB {
        let ray = self.camera.ray_at(coord.x, coord.y);
        self.trace_ray(&ray)
    }

    fn trace_ray(&mut self, ray: &Ray) -> ColorRGB {
        self.ray_count += 1;
        let start = Instant::now();
        let surfel = self.bvh.intersect(&ray);

        let color = match surfel {
            Some(surf) => {
                self.hit_count += 1;
                let mut color = ColorRGB::black();
                let material = self.scene.material_for_surfel(&surf);
                let ambient = self.scene.ambient() * material.ka * material.ambient;

                for light in self.scene.lights().iter() {
                    color = color + phong_shade(light, &self.camera.eye, &surf, material);
                }

                color = color + ambient;
                color = color.clamp(0.0_f32, 1.0_f32);
                color
            }
            None => { self.scene.bgcolor() }
        };

        let stop = Instant::now();
        let delta = stop - start;
        self.trace_sum += delta;
        if delta > self.trace_max {self.trace_max = delta; }
        color
    }

    fn print_stats(&self) {
        let hit_percent = (self.hit_count as f32 / self.ray_count as f32) * 100.0_f32;
        let avg_trace = self.trace_sum / self.ray_count;
        println!("ray count: {}, hit count: {}, hit %: {:.2}, sum: {:?}, avg: {:?}, max: {:?}",
                 self.ray_count, self.hit_count, hit_percent, self.trace_sum, avg_trace, self.trace_max);
    }

}

fn phong_shade<T: Light>(light: &T, eye: &Vec3, surfel: &Surfel, material: &Material) -> ColorRGB {
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

    diffuse + specular
}

pub fn render_scene(scene: Scene, anti_aliasing_depth: u8) -> Framebuffer {

    println!("bg color {:?}", scene.bgcolor());
    let setup_start = Instant::now();
    let mut fb = Framebuffer::new(scene.width(), scene.height());
    let mut tracer = RayTracer::new(scene);
    let setup_end = Instant::now();
    println!("setup time: {:?}", setup_end - setup_start);

    let begin = Instant::now();

    // first render pass
    for j in 0..fb.width {
        for k in 0..fb.height {
            let color = tracer.sample_point(j, k);
            fb.set_color(j, k, &color);
        }
    }
    let trace_end = Instant::now();
    println!("total tracing time: {:?}", trace_end - begin);

    // anti-aliasing
    for j in 0..fb.width - 1 {
        for k in 1..fb.height {
            let mut pixel = Pixel::new(j, k);
            let color = pixel.sample(&mut tracer, &fb, anti_aliasing_depth);
            fb.set_color(j, k, &color);
        }
    }

    let render_end = Instant::now();
    tracer.print_stats();
    println!("total render time: {:?}", render_end - begin);
    fb
}
