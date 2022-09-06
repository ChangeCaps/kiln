#![deny(unsafe_op_in_unsafe_fn)]

mod error;
mod manifest;
mod render;
mod shader;
mod shader_processor;
mod show;
mod window;

use clap::{crate_authors, crate_version, Parser, Subcommand};
use show::Show;

#[derive(Subcommand)]
enum Command {
    Init,
    /// Show a kiln shader.
    Show(Show),
}

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {}
        Command::Show(show) => {
            show.run().unwrap();
        }
    }
}
