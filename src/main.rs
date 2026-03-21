mod installer;
mod rules;
mod scan;

use clap::{Parser, Subcommand};
use crate::rules::PackageType;

#[derive(Parser)]
#[command(name = "tdgzi")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Inspect { file: String },
}

fn main() {
    println!("Hello, world!");
}
