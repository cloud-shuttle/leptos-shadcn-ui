//! System information gathering for performance testing context

use crate::{SystemInfo, PerfTestError};

/// Gather comprehensive system information for performance context
pub fn gather_system_info() -> Result<SystemInfo, PerfTestError> {
    let system = sysinfo::System::new_all();
    
    let os = format!("{} {}", 
        std::env::consts::OS, 
        sysinfo::System::kernel_version().unwrap_or_else(|| "unknown".to_string())
    );
    
    let cpu_model = system.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    let cpu_cores = system.cpus().len();
    let memory_total_mb = system.total_memory() / 1024 / 1024;
    
    let rust_version = get_rust_version();
    let leptos_version = get_leptos_version();
    
    Ok(SystemInfo {
        os,
        cpu_model,
        cpu_cores,
        memory_total_mb,
        rust_version,
        leptos_version,
    })
}

/// Get Rust version information
fn get_rust_version() -> String {
    std::process::Command::new("rustc")
        .args(&["--version"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|version| version.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Get Leptos version from Cargo.toml or environment
fn get_leptos_version() -> String {
    // Try to get from environment first (set during build)
    std::env::var("LEPTOS_VERSION")
        .unwrap_or_else(|_| "0.8.0".to_string()) // Default fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gather_system_info() {
        let info = gather_system_info().unwrap();
        
        assert!(!info.os.is_empty());
        assert!(!info.cpu_model.is_empty());
        assert!(info.cpu_cores > 0);
        assert!(info.memory_total_mb > 0);
        assert!(!info.rust_version.is_empty());
        assert!(!info.leptos_version.is_empty());
    }

    #[test]
    fn test_get_rust_version() {
        let version = get_rust_version();
        assert!(!version.is_empty());
        // Should contain "rustc" and a version number
        assert!(version.contains("rustc") || version.contains("."));
    }

    #[test]
    fn test_get_leptos_version() {
        let version = get_leptos_version();
        assert!(!version.is_empty());
        // Should be a valid version format
        assert!(version.chars().any(|c| c.is_ascii_digit()));
    }
}