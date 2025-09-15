#!/usr/bin/env rust-script
//! TDD Expansion Tool
//! 
//! This binary applies TDD principles to other packages in the workspace,
//! ensuring consistent quality and testing standards across all packages.

use anyhow::Result;
use leptos_shadcn_contract_testing::tdd_expansion::TddExpansionManager;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("scan");

    // Get workspace root (assume we're running from workspace root)
    let workspace_root = env::current_dir()?;
    
    let mut manager = TddExpansionManager::new(workspace_root);

    match command {
        "scan" => {
            println!("🔍 Scanning workspace for packages needing TDD implementation...");
            
            let packages_needing_tdd = manager.scan_workspace()?;
            
            if packages_needing_tdd.is_empty() {
                println!("✅ All packages already have adequate TDD implementation!");
            } else {
                println!("📋 Found {} packages needing TDD implementation:", packages_needing_tdd.len());
                for package in &packages_needing_tdd {
                    println!("  - {}", package);
                }
                
                println!("\n💡 Run 'cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion apply' to implement TDD for all packages");
            }
        }
        
        "apply" => {
            println!("🧪 Applying TDD principles to workspace packages...");
            
            // First scan to identify packages
            let _packages = manager.scan_workspace()?;
            
            // Apply TDD to all identified packages
            manager.apply_tdd_to_workspace()?;
            
            println!("✅ TDD implementation complete!");
        }
        
        "apply-package" => {
            let package_name = args.get(2)
                .ok_or_else(|| anyhow::anyhow!("Package name required for apply-package command"))?;
            
            println!("🧪 Applying TDD to package: {}", package_name);
            
            // Scan workspace first
            let _packages = manager.scan_workspace()?;
            
            // Apply TDD to specific package
            manager.generate_tdd_implementation(package_name)?;
            
            println!("✅ TDD implementation complete for {}", package_name);
        }
        
        "report" => {
            println!("📊 Generating TDD implementation report...");
            
            // Scan workspace
            let _packages = manager.scan_workspace()?;
            
            // Generate report
            let report = manager.generate_implementation_report()?;
            
            // Save report to file
            let report_path = "tdd_implementation_report.md";
            std::fs::write(&report_path, &report)?;
            
            println!("📄 Report saved to: {}", report_path);
            println!("\n{}", report);
        }
        
        "validate" => {
            println!("✅ Validating TDD implementation across workspace...");
            
            // Scan workspace
            let packages = manager.scan_workspace()?;
            
            if packages.is_empty() {
                println!("✅ All packages have adequate TDD implementation!");
                std::process::exit(0);
            } else {
                println!("❌ {} packages still need TDD implementation:", packages.len());
                for package in &packages {
                    println!("  - {}", package);
                }
                std::process::exit(1);
            }
        }
        
        "help" | "--help" | "-h" => {
            print_help();
        }
        
        _ => {
            eprintln!("❌ Unknown command: {}", command);
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_help() {
    println!("TDD Expansion Tool - Apply TDD principles to workspace packages");
    println!();
    println!("Usage: cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion <command> [args]");
    println!();
    println!("Commands:");
    println!("  scan              Scan workspace for packages needing TDD implementation");
    println!("  apply             Apply TDD to all packages that need it");
    println!("  apply-package     Apply TDD to a specific package");
    println!("  report            Generate TDD implementation report");
    println!("  validate          Validate TDD implementation (exit code 0 if all good)");
    println!("  help              Show this help message");
    println!();
    println!("Examples:");
    println!("  cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion scan");
    println!("  cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion apply");
    println!("  cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion apply-package leptos-shadcn-button");
    println!("  cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion report");
    println!("  cargo run --package leptos-shadcn-contract-testing --bin tdd_expansion validate");
}
