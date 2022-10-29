use std::path::PathBuf;

use clap::Parser;

#[derive(Default, Debug, Parser)]
pub struct CliArgs {
    #[arg(short, long)]
    pub scene: PathBuf,
    #[arg(short, long)]
    pub image: Option<PathBuf>,
    #[arg(short = 'S', long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(0..3))]
    pub sampling_depth: u8

}
