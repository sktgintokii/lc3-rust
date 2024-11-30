use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'i', long = "image")]
    pub image_path: PathBuf,
}
