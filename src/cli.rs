use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Config file
    #[clap(short, long, default_value = "snoop.json")]
    pub config: PathBuf,

    // Package to run
    #[clap()]
    pub package: String,

    // Command to run
    #[clap()]
    pub command: String,
}
