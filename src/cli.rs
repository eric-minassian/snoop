use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    // Command to run
    pub command: String,

    // Package to run
    pub package: Option<String>,

    /// Workspace / Package Root Path
    #[clap(short, long, default_value = ".")]
    pub root_path: PathBuf,
}
