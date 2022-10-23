use crate::scene::{Light, Scene, Surfel, Material};
use crate::framebuffer::{Framebuffer, ColorRGB};
use crate::math::{normalize, dot, Vec3};

use std::time::{Instant, Duration};

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

pub fn render_scene(scene: &Scene) -> Framebuffer {

    let setup_start = Instant::now();
    let mut fb = Framebuffer::new(scene.width(), scene.height());
    let camera = scene.make_camera();
    let bvh = scene.make_bvh();
    let setup_end = Instant::now();
    println!("setup time: {:?}", setup_end - setup_start);
    println!("bbox: {:?}", bvh.bbox);

    let mut count = 0;
    let mut sum = Duration::from_secs(0);
    let mut max = Duration::from_secs(0);
    let mut hit_count = 0;

    let begin = Instant::now();

    for j in 0..scene.width() {
        for k in 0..scene.height() {
            let ray = camera.ray_at(j, k);

            count += 1;
            let start = Instant::now();
            let surfel = bvh.intersect(&ray);
            let stop = Instant::now();
            let delta = stop - start;
            sum += delta;
            if delta > max { max = delta; }

            match surfel {
                Some(surf) => {
                    hit_count += 1;
                    let mut color = ColorRGB::black();
                    let material = scene.material_for_surfel(&surf);
                    let ambient = scene.ambient() * material.ka * material.ambient;

                    for light in scene.lights().iter() {
                        color = color + phong_shade(light, &camera.eye, &surf, material);
                    }

                    color = color + ambient;
                    color = color.clamp(0.0_f32, 1.0_f32);
                    fb.set_color(j, k, &color);
                }
                None => { fb.set_color(j, k, &scene.bgcolor()); }
            }
        }
    }

    let end = Instant::now();
    let hit_percent = (hit_count as f32 / count as f32) * 100.0_f32;

    println!("total tracing time: {:?}", end - begin);
    println!("ray count: {}, hit count: {}, hit %: {:.2}, sum: {:?}, avg: {:?}, max: {:?}",
             count, hit_count, hit_percent, sum, sum / count, max);

    fb

}
