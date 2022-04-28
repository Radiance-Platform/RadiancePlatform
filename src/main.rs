use std::path::PathBuf;
use clap::Parser;

pub mod game;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Full or relative path to the root folder containing the configuration files
    #[clap(short, long, parse(from_os_str), value_name = "FILEPATH")]
    config_path: PathBuf,

    /// Whether or not to compile the files into the binary
    #[clap(long)]
    compile: bool,
}

fn main() {
    // Make sure we have arguments or compiled-in
    // TODO: Some way to compile config files in, might need a custom build script or something
    let args = Args::parse();
    // Debug printing
    println!("path: {:?}", args.config_path);
    println!("compile: {:?}", args.compile);

    let mut g = game::Game::initialize(args.config_path);
    g.start();
}