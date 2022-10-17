use std::process;
use std::time::Instant;

use arrt::args::parse_cli;
use arrt::render::render_scene;
use arrt::scene::Scene;

fn main() {
    let args = parse_cli().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("cli args= {:?}", args);

    let scene = Scene::new(&args.scene);
    let framebuf = render_scene(&scene);

    let start = Instant::now();
    framebuf.save_image(&args.image);
    let stop = Instant::now();
    println!("save image time: {:?}", stop - start);
}
