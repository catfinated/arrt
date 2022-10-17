use crate::scene::{Camera, Light, Scene, Surfel};
use crate::framebuffer::{Framebuffer, ColorRGB};
use crate::math::{normalize, dot};

use std::time::{Instant, Duration};

fn phong_shade<T: Light>(light: &T, surfel: &Surfel, scene: &Scene) -> ColorRGB {
    let material = &scene.materials()[surfel.material_id];
    let n = surfel.normal;
    let l = normalize(light.direction_from(surfel.hit_point)); // from P to light
    let v = normalize(scene.camera().eye - surfel.hit_point);  // from P to viewer
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
    let camera = Camera::new(scene.camera(), scene.width() as f32, scene.height() as f32);
    let setup_end = Instant::now();
    println!("setup time: {:?}", setup_end - setup_start);
    println!("bbox: {:?}", scene.bvh.bbox);

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
            let surfel = scene.intersect(&ray);
            let stop = Instant::now();
            let delta = stop - start;
            sum += delta;
            if delta > max { max = delta; }

            match surfel {
                Some(surf) => {
                    hit_count += 1;
                    let mut color = ColorRGB::black();
                    for light in scene.lights().iter() {
                        color = color + phong_shade(light, &surf, scene);
                    }

                    let mat = &scene.materials()[surf.material_id];
                    let ambient = scene.ambient() * mat.ka * mat.ambient;
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
