//! Dependency fixer that converts published deps to workspace paths

use crate::ContractError;
use std::fs;
use std::path::Path;

pub struct DependencyFixer {
    workspace_root: String,
}

impl DependencyFixer {
    pub fn new(workspace_root: impl Into<String>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
        }
    }

    /// Fix main package dependencies by converting to workspace paths
    pub fn fix_main_package_dependencies(&self) -> Result<(), ContractError> {
        let main_cargo_path = format!("{}/packages/leptos-shadcn-ui/Cargo.toml", self.workspace_root);

        if !Path::new(&main_cargo_path).exists() {
            return Err(ContractError::ValidationError {
                message: format!("Main package Cargo.toml not found at {}", main_cargo_path),
            });
        }

        let content = fs::read_to_string(&main_cargo_path)
            .map_err(|e| ContractError::ValidationError {
                message: format!("Failed to read {}: {}", main_cargo_path, e),
            })?;

        let fixed_content = self.convert_published_to_workspace_deps(content)?;

        fs::write(&main_cargo_path, fixed_content)
            .map_err(|e| ContractError::ValidationError {
                message: format!("Failed to write {}: {}", main_cargo_path, e),
            })?;

        println!("✅ Fixed dependencies in {}", main_cargo_path);
        Ok(())
    }

    fn convert_published_to_workspace_deps(&self, content: String) -> Result<String, ContractError> {
        let mut fixed_content = content;

        // Define comprehensive dependency replacements
        let replacements = vec![
            // Basic components
            (r#"leptos-shadcn-button = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-button = { path = "../leptos/button", optional = true }"#),
            (r#"leptos-shadcn-input = { version = "0.6.1", optional = true }"#,
             r#"leptos-shadcn-input = { path = "../leptos/input", optional = true }"#),
            (r#"leptos-shadcn-label = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-label = { path = "../leptos/label", optional = true }"#),
            (r#"leptos-shadcn-checkbox = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-checkbox = { path = "../leptos/checkbox", optional = true }"#),
            (r#"leptos-shadcn-switch = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-switch = { path = "../leptos/switch", optional = true }"#),
            (r#"leptos-shadcn-card = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-card = { path = "../leptos/card", optional = true }"#),

            // Additional components
            (r#"leptos-shadcn-radio-group = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-radio-group = { path = "../leptos/radio-group", optional = true }"#),
            (r#"leptos-shadcn-select = { version = "0.7.0", optional = true }"#,
             r#"leptos-shadcn-select = { path = "../leptos/select", optional = true }"#),
            (r#"leptos-shadcn-select = { version = "0.8.0", optional = true }"#,
             r#"leptos-shadcn-select = { path = "../leptos/select", optional = true }"#),
            (r#"leptos-shadcn-textarea = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-textarea = { path = "../leptos/textarea", optional = true }"#),
            (r#"leptos-shadcn-separator = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-separator = { path = "../leptos/separator", optional = true }"#),
            (r#"leptos-shadcn-tabs = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-tabs = { path = "../leptos/tabs", optional = true }"#),
            (r#"leptos-shadcn-accordion = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-accordion = { path = "../leptos/accordion", optional = true }"#),
            (r#"leptos-shadcn-dialog = { version = "0.7.0", optional = true }"#,
             r#"leptos-shadcn-dialog = { path = "../leptos/dialog", optional = true }"#),
            (r#"leptos-shadcn-dialog = { version = "0.8.0", optional = true }"#,
             r#"leptos-shadcn-dialog = { path = "../leptos/dialog", optional = true }"#),
            (r#"leptos-shadcn-popover = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-popover = { path = "../leptos/popover", optional = true }"#),
            (r#"leptos-shadcn-tooltip = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-tooltip = { path = "../leptos/tooltip", optional = true }"#),
            (r#"leptos-shadcn-alert = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-alert = { path = "../leptos/alert", optional = true }"#),
            (r#"leptos-shadcn-badge = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-badge = { path = "../leptos/badge", optional = true }"#),
            (r#"leptos-shadcn-skeleton = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-skeleton = { path = "../leptos/skeleton", optional = true }"#),
            (r#"leptos-shadcn-progress = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-progress = { path = "../leptos/progress", optional = true }"#),
            (r#"leptos-shadcn-toast = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-toast = { path = "../leptos/toast", optional = true }"#),
            (r#"leptos-shadcn-table = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-table = { path = "../leptos/table", optional = true }"#),
            (r#"leptos-shadcn-calendar = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-calendar = { path = "../leptos/calendar", optional = true }"#),
            (r#"leptos-shadcn-date-picker = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-date-picker = { path = "../leptos/date-picker", optional = true }"#),
            (r#"leptos-shadcn-pagination = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-pagination = { path = "../leptos/pagination", optional = true }"#),
            (r#"leptos-shadcn-slider = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-slider = { path = "../leptos/slider", optional = true }"#),
            (r#"leptos-shadcn-toggle = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-toggle = { path = "../leptos/toggle", optional = true }"#),
            (r#"leptos-shadcn-carousel = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-carousel = { path = "../leptos/carousel", optional = true }"#),

            // Advanced components
            (r#"leptos-shadcn-form = { version = "0.7.0", optional = true }"#,
             r#"leptos-shadcn-form = { path = "../leptos/form", optional = true }"#),
            (r#"leptos-shadcn-form = { version = "0.8.0", optional = true }"#,
             r#"leptos-shadcn-form = { path = "../leptos/form", optional = true }"#),
            (r#"leptos-shadcn-combobox = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-combobox = { path = "../leptos/combobox", optional = true }"#),
            (r#"leptos-shadcn-command = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-command = { path = "../leptos/command", optional = true }"#),
            (r#"leptos-shadcn-input-otp = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-input-otp = { path = "../leptos/input-otp", optional = true }"#),
            (r#"leptos-shadcn-breadcrumb = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-breadcrumb = { path = "../leptos/breadcrumb", optional = true }"#),
            (r#"leptos-shadcn-navigation-menu = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-navigation-menu = { path = "../leptos/navigation-menu", optional = true }"#),
            (r#"leptos-shadcn-context-menu = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-context-menu = { path = "../leptos/context-menu", optional = true }"#),
            (r#"leptos-shadcn-dropdown-menu = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-dropdown-menu = { path = "../leptos/dropdown-menu", optional = true }"#),
            (r#"leptos-shadcn-menubar = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-menubar = { path = "../leptos/menubar", optional = true }"#),
            (r#"leptos-shadcn-hover-card = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-hover-card = { path = "../leptos/hover-card", optional = true }"#),
            (r#"leptos-shadcn-aspect-ratio = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-aspect-ratio = { path = "../leptos/aspect-ratio", optional = true }"#),
            (r#"leptos-shadcn-collapsible = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-collapsible = { path = "../leptos/collapsible", optional = true }"#),
            (r#"leptos-shadcn-scroll-area = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-scroll-area = { path = "../leptos/scroll-area", optional = true }"#),
            (r#"leptos-shadcn-sheet = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-sheet = { path = "../leptos/sheet", optional = true }"#),
            (r#"leptos-shadcn-drawer = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-drawer = { path = "../leptos/drawer", optional = true }"#),
            (r#"leptos-shadcn-alert-dialog = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-alert-dialog = { path = "../leptos/alert-dialog", optional = true }"#),
            (r#"leptos-shadcn-avatar = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-avatar = { path = "../leptos/avatar", optional = true }"#),
            (r#"leptos-shadcn-resizable = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-resizable = { path = "../leptos/resizable", optional = true }"#),
            (r#"leptos-shadcn-performance-audit = { version = "0.1.0", optional = true }"#,
             r#"leptos-shadcn-performance-audit = { path = "../../performance-audit", optional = true }"#),

            // Additional packages
            (r#"leptos-shadcn-error-boundary = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-error-boundary = { path = "../leptos/error-boundary", optional = true }"#),
            (r#"leptos-shadcn-lazy-loading = { version = "0.6.0", optional = true }"#,
             r#"leptos-shadcn-lazy-loading = { path = "../leptos/lazy-loading", optional = true }"#),
            (r#"leptos-shadcn-registry = { version = "0.1.0", optional = true }"#,
             r#"leptos-shadcn-registry = { path = "../leptos/registry", optional = true }"#),
        ];

        for (old_dep, new_dep) in replacements {
            if fixed_content.contains(old_dep) {
                fixed_content = fixed_content.replace(old_dep, new_dep);
                println!("🔄 Converted: {} -> workspace path", old_dep.split('=').next().unwrap_or("unknown").trim());
            }
        }

        Ok(fixed_content)
    }

    /// Update version to 0.8.0 across the package
    pub fn update_package_version(&self) -> Result<(), ContractError> {
        let main_cargo_path = format!("{}/packages/leptos-shadcn-ui/Cargo.toml", self.workspace_root);

        if !Path::new(&main_cargo_path).exists() {
            return Err(ContractError::ValidationError {
                message: format!("Main package Cargo.toml not found at {}", main_cargo_path),
            });
        }

        let content = fs::read_to_string(&main_cargo_path)
            .map_err(|e| ContractError::ValidationError {
                message: format!("Failed to read {}: {}", main_cargo_path, e),
            })?;

        let updated_content = content.replace(r#"version = "0.7.0""#, r#"version = "0.8.0""#);

        fs::write(&main_cargo_path, updated_content)
            .map_err(|e| ContractError::ValidationError {
                message: format!("Failed to write {}: {}", main_cargo_path, e),
            })?;

        println!("✅ Updated package version to 0.8.0");
        Ok(())
    }

    /// Validate that fixes were applied correctly
    pub fn validate_fixes(&self) -> Result<(), ContractError> {
        let main_cargo_path = format!("{}/packages/leptos-shadcn-ui/Cargo.toml", self.workspace_root);

        let content = fs::read_to_string(&main_cargo_path)
            .map_err(|e| ContractError::ValidationError {
                message: format!("Failed to read {}: {}", main_cargo_path, e),
            })?;

        // Check for remaining published versions
        let problematic_patterns = [
            r#"version = "0.6.0""#,
            r#"version = "0.6.1""#,
            r#"version = "0.7.0""#,
        ];

        for pattern in &problematic_patterns {
            if content.contains(pattern) && content.contains("leptos-shadcn-") {
                return Err(ContractError::ValidationError {
                    message: format!("Still found published dependency with {}", pattern),
                });
            }
        }

        // Check that workspace paths are used
        let required_paths = [
            r#"path = "../leptos/button""#,
            r#"path = "../leptos/input""#,
            r#"path = "../leptos/label""#,
        ];

        let mut found_paths = 0;
        for path in &required_paths {
            if content.contains(path) {
                found_paths += 1;
            }
        }

        if found_paths == 0 {
            return Err(ContractError::ValidationError {
                message: "No workspace paths found - fixes may not have been applied".to_string(),
            });
        }

        println!("✅ Validation passed: {} workspace paths found", found_paths);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_dependency_conversion() {
        let fixer = DependencyFixer::new("/test/path");

        let input = r#"leptos-shadcn-button = { version = "0.6.0", optional = true }"#;
        let result = fixer.convert_published_to_workspace_deps(input.to_string()).unwrap();

        assert!(result.contains(r#"path = "../leptos/button""#));
        assert!(!result.contains(r#"version = "0.6.0""#));
    }

    #[tokio::test]
    async fn test_dependency_fixer_creation() {
        let fixer = DependencyFixer::new("/test/workspace");
        assert_eq!(fixer.workspace_root, "/test/workspace");
    }

    #[tokio::test]
    async fn test_validation_fails_with_published_deps() {
        // Create temporary directory structure
        let temp_dir = tempdir().unwrap();
        let workspace_root = temp_dir.path().to_str().unwrap();

        // Create packages directory
        let packages_dir = temp_dir.path().join("packages").join("leptos-shadcn-ui");
        fs::create_dir_all(&packages_dir).unwrap();

        // Create Cargo.toml with published dependencies
        let cargo_content = r#"
[package]
name = "leptos-shadcn-ui"
version = "0.7.0"

[dependencies]
leptos-shadcn-button = { version = "0.6.0", optional = true }
"#;
        fs::write(packages_dir.join("Cargo.toml"), cargo_content).unwrap();

        let fixer = DependencyFixer::new(workspace_root);
        let result = fixer.validate_fixes();

        assert!(result.is_err(), "Validation should fail with published dependencies");
    }
}