use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};

use serde::Deserialize;

use crate::error::{Error, Result};

pub const CONFIG_FILE: &str = "snoop.json";
pub const SNOOP_VERSION: &str = env!("CARGO_PKG_VERSION_MAJOR");

#[derive(Deserialize, Debug)]
pub struct CacheConfig {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CacheStrategy {
    Cached {
        cache: bool,
        #[serde(rename = "cacheConfig")]
        cache_config: CacheConfig,
    },
    Uncached {
        cache: bool,
    },
}

#[derive(Deserialize, Debug)]
pub struct ShellCommand {
    pub name: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    #[serde(flatten)]
    pub cache_strategy: CacheStrategy,
}

#[derive(Deserialize, Debug)]
pub struct PackageManifest {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    pub commands: Vec<ShellCommand>,
}

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    pub packages: Vec<PackageInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Config {
    Workspace(Workspace),
    Package(PackageManifest),
}

fn load_json<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn run_command(root: &Path, package_name: Option<&str>, command: &str) -> Result<()> {
    let root_config_path = root.join(CONFIG_FILE);
    let root_config: Config = load_json(&root_config_path)?;

    let package_manifest = match root_config {
        Config::Workspace(workspace) => {
            let package_name = package_name.ok_or(Error::MissingPackageName)?;

            let package_info = workspace
                .packages
                .into_iter()
                .find(|p| p.name == package_name)
                .ok_or_else(|| Error::PackageNotFound(package_name.to_string()))?;

            let package_manifest_path = root.join(package_info.path).join(CONFIG_FILE);
            load_json(&package_manifest_path)?
        }
        Config::Package(package_manifest) => package_manifest,
    };

    let shell_command = package_manifest
        .commands
        .into_iter()
        .find(|c| c.name == command)
        .ok_or_else(|| Error::CommandNotFound(command.to_string()))?;

    Command::new(shell_command.command)
        .args(shell_command.args.unwrap_or_default())
        .status()
        .map_err(Error::CommandExecutionFailed)?;

    Ok(())
}
