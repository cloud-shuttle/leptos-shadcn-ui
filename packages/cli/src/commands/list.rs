use console::style;
use shadcn_registry::{registry_ui::UI, schema::FrameworkName};

/// List available components for the specified framework
pub async fn list_components(framework: &str, completed: bool, missing: bool) -> anyhow::Result<()> {
    if framework != "leptos" {
        eprintln!("{}", style("⚠️  Only Leptos framework is currently supported").yellow());
        return Ok(());
    }
    
    let registry = UI.get(&FrameworkName::Leptos).ok_or_else(|| {
        anyhow::anyhow!("Leptos registry not found")
    })?;
    
    // Get all available components from the complete registry
    let all_components = get_all_components();
    let leptos_components: std::collections::HashSet<_> = registry
        .iter()
        .map(|item| item.name.clone())
        .collect();
    
    let mut missing_components: Vec<_> = all_components
        .iter()
        .filter(|name| !leptos_components.contains(*name))
        .collect();
    missing_components.sort();
    
    let mut completed_components: Vec<_> = all_components
        .iter()
        .filter(|name| leptos_components.contains(*name))
        .collect();
    completed_components.sort();
    
    println!("{}", style("=== Leptos shadcn/ui Components ===").bold().blue());
    println!();
    
    if completed {
        println!("{}", style("✅ Completed Components:").bold().green());
        for component in &completed_components {
            println!("  • {}", style(component).green());
        }
        println!();
    } else if missing {
        println!("{}", style("❌ Missing Components:").bold().red());
        for component in &missing_components {
            println!("  • {}", style(component).red());
        }
        println!();
    } else {
        // Show summary
        println!("{}", style("📊 Component Status:").bold());
        println!("  Total Components: {}", all_components.len());
        println!("  Completed: {} ({:.1}%)", 
            completed_components.len(), 
            (completed_components.len() as f64 / all_components.len() as f64) * 100.0
        );
        println!("  Missing: {} ({:.1}%)", 
            missing_components.len(), 
            (missing_components.len() as f64 / all_components.len() as f64) * 100.0
        );
        println!();
        
        println!("{}", style("✅ Completed Components:").bold().green());
        for component in &completed_components {
            println!("  • {}", style(component).green());
        }
        println!();
        
        println!("{}", style("❌ Missing Components:").bold().red());
        for component in &missing_components {
            println!("  • {}", style(component).red());
        }
        println!();
    }
    
    println!("{}", style("💡 Usage:").bold().yellow());
    println!("  rust-shadcn add <component> --framework leptos");
    println!("  rust-shadcn list --completed");
    println!("  rust-shadcn list --missing");
    
    Ok(())
}

/// Get all available shadcn/ui components
fn get_all_components() -> Vec<String> {
    vec![
        // Form & Input Components
        "button".to_string(), "checkbox".to_string(), "radio-group".to_string(),
        "select".to_string(), "combobox".to_string(), "form".to_string(),
        "input".to_string(), "label".to_string(), "textarea".to_string(),
        "slider".to_string(), "switch".to_string(), "toggle".to_string(),
        
        // Navigation Components
        "navigation-menu".to_string(), "menubar".to_string(), "tabs".to_string(),
        "breadcrumb".to_string(), "pagination".to_string(), "command".to_string(),
        "context-menu".to_string(),
        
        // Overlay Components
        "dialog".to_string(), "alert-dialog".to_string(), "sheet".to_string(),
        "drawer".to_string(), "dropdown-menu".to_string(), "popover".to_string(),
        "tooltip".to_string(), "toast".to_string(),
        
        // Layout Components
        "accordion".to_string(), "collapsible".to_string(), "resizable".to_string(),
        "scroll-area".to_string(), "separator".to_string(), "sidebar".to_string(),
        "aspect-ratio".to_string(),
        
        // Display Components
        "alert".to_string(), "avatar".to_string(), "badge".to_string(),
        "card".to_string(), "calendar".to_string(), "progress".to_string(),
        "skeleton".to_string(), "table".to_string(),
        
        // Advanced Components
        "carousel".to_string(), "chart".to_string(), "data-table".to_string(),
        "date-picker".to_string(), "hover-card".to_string(), "input-otp".to_string(),
        "sonner".to_string(), "typography".to_string(),
    ]
}
