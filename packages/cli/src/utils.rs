//! Utility functions for the CLI

use std::path::PathBuf;

/// Check if a path is a valid Rust project
pub fn is_rust_project(path: &PathBuf) -> bool {
    path.join("Cargo.toml").exists()
}

/// Get the project root directory
pub fn get_project_root(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let mut current = path.canonicalize()?;
    
    while !current.join("Cargo.toml").exists() {
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return Err(anyhow::anyhow!("No Cargo.toml found in parent directories"));
        }
    }
    
    Ok(current)
}
