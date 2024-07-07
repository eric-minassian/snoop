mod cli;
mod config;
mod error;

use std::process::exit;

use clap::Parser;
use cli::Args;
use config::run_command;

fn main() {
    let args = Args::parse();

    run_command(&args.root_path, args.package.as_deref(), &args.command).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });
}
