use std::time::Instant;
use std::path::PathBuf;

use clap::Parser;

use arrt::args::CliArgs;
use arrt::render::render_scene;
use arrt::scene::Scene;

fn main() {
    let args = CliArgs::parse();
    println!("cli args= {:?}", args);

    let scene = Scene::new(&args.scene);
    let framebuf = render_scene(scene, args.sampling_depth);

    let start = Instant::now();
    let image = args.image.unwrap_or(PathBuf::from(args.scene.file_name().unwrap()).with_extension("png"));
    framebuf.save_image(&image);
    let stop = Instant::now();
    println!("save image time: {:?}", stop - start);
}
