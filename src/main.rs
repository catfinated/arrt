use std::env;
use std::path::Path;
use std::process;
use std::time::{Instant, Duration};

use arrt::framebuffer::{Framebuffer, ColorRGB};
use arrt::scene::{Scene, Camera, Config};
use arrt::lights::phong_shade;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let ofpath = Path::new(Path::new(&config.filename).file_name().unwrap()).with_extension("png");
    println!("Scene file: '{}'", config.filename);
    println!("Output file: {:?}", ofpath);

    let setup_start = Instant::now();
    let scene = Scene::new(&config);

    let mut fb = Framebuffer::new(scene.width, scene.height);
    let camera = Camera::new(scene.camera, scene.width as f32, scene.height as f32);
    let setup_end = Instant::now();
    println!("setup time: {:?}", setup_end - setup_start);
    println!("bbox: {:?}", scene.bvh.bbox);

    let mut count = 0;
    let mut sum = Duration::from_secs(0);
    let mut max = Duration::from_secs(0);
    let mut hit_count = 0;

    let begin = Instant::now();

    for j in 0..scene.width {
        for k in 0..scene.height {
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
                    for light in scene.lights.iter() {
                        color = color + phong_shade(light, &surf, &scene);
                    }

                    let mat = &scene.materials[surf.material_id];
                    let ambient = scene.ambient * mat.ka * mat.ambient;
                    color = color + ambient;
                    color = color.clamp(0.0_f32, 1.0_f32);
                    fb.set_color(j, k, &color);
                }
                None => { fb.set_color(j, k, &scene.bgcolor); }
            }
        }
    }

    let end = Instant::now();

    let hit_percent = (hit_count as f32 / count as f32) * 100.0_f32;

    println!("total tracing time: {:?}", end - begin);
    println!("ray count: {}, hit count: {}, hit %: {:.2}, sum: {:?}, avg: {:?}, max: {:?}",
             count, hit_count, hit_percent, sum, sum / count, max);

    let begin2 = Instant::now();
    fb.save_image(&ofpath);
    let end2 = Instant::now();

    println!("save image time: {:?}", end2 - begin2);
}
