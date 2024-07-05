mod cli;
mod config;

use clap::Parser;
use cli::Args;
use config::{load_json, PackageManifest, Workspace};

fn main() {
    let args = Args::parse();

    let config: Workspace = load_json(&args.config);
    let package = config
        .packages
        .iter()
        .find(|p| p.name == args.package)
        .unwrap_or_else(|| {
            eprintln!("Package not found: {}", args.package);
            std::process::exit(1);
        });

    let package_config_path = package.root.join("snoop.json");
    let package_config: PackageManifest = load_json(&package_config_path);
    let command = package_config
        .commands
        .iter()
        .find(|c| c.name == args.command)
        .unwrap_or_else(|| {
            eprintln!("Command not found: {}", args.command);
            std::process::exit(1);
        });

    println!("Running command: {}", command.command);
}
