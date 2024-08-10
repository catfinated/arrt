use std::path::PathBuf;

use clap::Parser;

use arrt::args::CliArgs;
use arrt::render::render_with_args;

fn main() {
    let args = CliArgs::parse();
    println!("cli args= {:?}", args);
    let framebuf = render_with_args(&args);
    let image = args.image.unwrap_or_else(|| PathBuf::from(args.scene.file_name().unwrap()).with_extension("png"));
    framebuf.save_image(&image);
}
