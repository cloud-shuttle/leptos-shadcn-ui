//! TDD-driven dependency fixer executable
//! This binary applies the dependency fixes identified by our contract tests

use leptos_shadcn_contract_testing::{DependencyFixer, ContractError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), ContractError> {
    println!("🚀 Starting TDD-driven dependency remediation...");

    // Get the workspace root - we're running from workspace root
    let workspace_root = env::current_dir()
        .map_err(|e| ContractError::ValidationError {
            message: format!("Failed to get current directory: {}", e),
        })?
        .to_string_lossy()
        .to_string();

    println!("📍 Workspace root: {}", workspace_root);

    let fixer = DependencyFixer::new(workspace_root);

    // Phase 1: Update package version
    println!("\n📝 Phase 1: Updating package version to 0.8.0...");
    fixer.update_package_version()?;

    // Phase 2: Fix main package dependencies
    println!("\n🔧 Phase 2: Converting published deps to workspace paths...");
    fixer.fix_main_package_dependencies()?;

    // Phase 3: Validate fixes
    println!("\n✅ Phase 3: Validating fixes...");
    fixer.validate_fixes()?;

    println!("\n🎉 TDD-driven dependency remediation completed successfully!");
    println!("📋 Next steps:");
    println!("   1. Run: cargo nextest run --package leptos-shadcn-contract-testing");
    println!("   2. Run: cargo build --workspace");
    println!("   3. Verify all components compile correctly");

    Ok(())
}