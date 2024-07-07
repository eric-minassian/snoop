use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use crate::error::Result;

pub fn generate_hash(paths: &[&str]) -> Result<String> {
    let mut hasher = Sha256::new();

    for dir in paths {
        visit_paths(Path::new(dir), &mut hasher)?;
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn visit_paths(dir: &Path, hasher: &mut Sha256) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            visit_paths(&path, hasher)?;
        }
    } else if dir.is_file() {
        let metadata = dir.metadata()?;
        let file_info = format!(
            "{:?}|{}|{}",
            dir,
            metadata.len(),
            metadata
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        );
        hasher.update(file_info);
    }

    Ok(())
}
