use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CliArgs {
    pub scene: PathBuf,
    pub image: PathBuf,
}

impl CliArgs {
    pub fn new(args: &[String]) -> Result<CliArgs, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let scene = PathBuf::from(&args[1]);
        let image = if args.len() > 2 {
            PathBuf::from(&args[2])
        } else {
            Path::new(scene.file_name().unwrap()).with_extension("png")
        };
        Ok(CliArgs{ scene, image })
    }
}

pub fn parse_cli() -> Result<CliArgs, &'static str> {
    let args: Vec<String> = env::args().collect();
    CliArgs::new(&args)
}
