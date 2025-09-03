use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

mod commands;
mod utils;

use commands::{add, init, validate, list};

#[derive(Parser)]
#[command(name = "rust-shadcn")]
#[command(about = "CLI tool for managing Leptos shadcn/ui components")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Leptos project with shadcn/ui
    Init {
        /// Project directory
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Framework to use (currently only Leptos supported)
        #[arg(long, default_value = "leptos")]
        framework: String,
        
        /// Theme to use
        #[arg(long, default_value = "default")]
        theme: String,
    },
    
    /// Add a component to your project
    Add {
        /// Component name to add
        component: String,
        
        /// Framework to use (currently only Leptos supported)
        #[arg(long, default_value = "leptos")]
        framework: String,
        
        /// Project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// List available components
    List {
        /// Framework to filter by
        #[arg(long, default_value = "leptos")]
        framework: String,
        
        /// Show only completed components
        #[arg(long)]
        completed: bool,
        
        /// Show only missing components
        #[arg(long)]
        missing: bool,
    },
    
    /// Validate component quality and consistency
    Validate {
        /// Component name to validate (or 'all' for all components)
        #[arg(default_value = "all")]
        component: String,
        
        /// Project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Check component status and completion
    Status {
        /// Show detailed status information
        #[arg(long)]
        detailed: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Set up progress bar style
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} {wide_msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    
    match &cli.command {
        Commands::Init { path, framework, theme } => {
            if framework != "leptos" {
                eprintln!("{}", style("⚠️  Warning: Only Leptos framework is currently supported").yellow());
            }
            
            let pb = ProgressBar::new_spinner();
            pb.set_style(progress_style.clone());
            pb.set_message("Initializing Leptos shadcn/ui project...");
            
            init::init_project(path, framework, theme).await?;
            
            pb.finish_with_message("✅ Project initialized successfully!");
            println!("\n🎉 Your Leptos shadcn/ui project is ready!");
            println!("📁 Project directory: {}", path.display());
            println!("🔧 Framework: {}", framework);
            println!("🎨 Theme: {}", theme);
            println!("\nNext steps:");
            println!("  rust-shadcn add button --framework leptos");
            println!("  rust-shadcn add card --framework leptos");
        }
        
        Commands::Add { component, framework, path } => {
            if framework != "leptos" {
                eprintln!("{}", style("⚠️  Warning: Only Leptos framework is currently supported").yellow());
            }
            
            let pb = ProgressBar::new_spinner();
            pb.set_style(progress_style.clone());
            pb.set_message(format!("Adding {} component...", component));
            
            add::add_component(component, framework, path.clone()).await?;
            
            pb.finish_with_message(format!("✅ {} component added successfully!", component));
            println!("\n📦 Component '{}' has been added to your project", component);
            println!("📁 Location: {}/src/components/{}", path.display(), component);
            println!("\nTo use the component:");
            println!("  use crate::components::{}::{};", component, component);
        }
        
        Commands::List { framework, completed, missing } => {
            if framework != "leptos" {
                eprintln!("{}", style("⚠️  Warning: Only Leptos framework is currently supported").yellow());
            }
            
            list::list_components(framework, *completed, *missing).await?;
        }
        
        Commands::Validate { component, path } => {
            let pb = ProgressBar::new_spinner();
            pb.set_style(progress_style.clone());
            pb.set_message("Validating components...");
            
            validate::validate_components(component, path.clone()).await?;
            
            pb.finish_with_message("✅ Component validation completed!");
        }
        
        Commands::Status { detailed } => {
            let pb = ProgressBar::new_spinner();
            pb.set_style(progress_style.clone());
            pb.set_message("Checking component status...");
            
            let status = commands::status::get_status(*detailed).await?;
            pb.finish_with_message("✅ Status check completed!");
            
            println!("{}", status);
        }
    }
    
    Ok(())
}
