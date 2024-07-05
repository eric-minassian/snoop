use std::{fs::File, io::BufReader, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug)]
pub struct PackageManifest {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    pub commands: Vec<Command>,
}

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub name: String,
    pub root: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    pub packages: Vec<PackageInfo>,
}

pub fn load_json<T>(path: &PathBuf) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open file: {}", e);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}", e);
        std::process::exit(1);
    })
}
