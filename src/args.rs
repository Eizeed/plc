use std::path::PathBuf;

use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'e', long = "extension", num_args = 1..)]
    pub extensions: Option<Vec<String>>,

    #[arg(short = 'p', long = "path")]
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short = 'h', long = "hidden")]
    pub hidden: bool,

    #[arg(short = 'd', long = "docs")]
    pub docs: bool,

    #[arg(short = 'c', long = "comments")]
    pub comments: bool,

    #[arg(short = 'f', long = "fixme")]
    pub fixme: bool,

    #[arg(short = 't', long = "todo")]
    pub todo: bool,

    #[arg(short = 'u', long = "units")]
    pub units: bool,
}
