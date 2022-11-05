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
    let image = args.image.unwrap_or_else(|| PathBuf::from(args.scene.file_name().unwrap()).with_extension("png"));
    framebuf.save_image(&image);
}
