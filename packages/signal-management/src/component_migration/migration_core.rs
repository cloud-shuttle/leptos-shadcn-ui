//! Component Migration Core Module
//! 
//! This module provides the core migration utilities for tracking component migration progress.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Migration status for tracking component migration progress
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationStatus {
    /// Whether all components have been successfully migrated
    pub all_migrated: bool,
    /// Number of components successfully migrated
    pub migrated_count: usize,
    /// Number of components that failed migration
    pub failed_count: usize,
}

impl Default for MigrationStatus {
    fn default() -> Self {
        Self {
            all_migrated: false,
            migrated_count: 0,
            failed_count: 46, // Total number of components in the workspace
        }
    }
}

/// Component migration utilities
pub struct ComponentMigrator {
    /// Track migration status
    status: ArcRwSignal<MigrationStatus>,
    /// Track which components have been migrated
    migrated_components: ArcRwSignal<Vec<String>>,
}

impl ComponentMigrator {
    /// Create a new component migrator
    pub fn new() -> Self {
        Self {
            status: ArcRwSignal::new(MigrationStatus::default()),
            migrated_components: ArcRwSignal::new(Vec::new()),
        }
    }

    /// Get current migration status
    pub fn status(&self) -> ArcRwSignal<MigrationStatus> {
        self.status.clone()
    }

    /// Get list of migrated components
    pub fn migrated_components(&self) -> ArcRwSignal<Vec<String>> {
        self.migrated_components.clone()
    }

    /// Mark a component as migrated
    pub fn mark_migrated(&self, component_name: &str) {
        let mut components = self.migrated_components.get();
        if !components.contains(&component_name.to_string()) {
            components.push(component_name.to_string());
            self.migrated_components.set(components);
            
            // Update status
            let mut status = self.status.get();
            status.migrated_count += 1;
            status.failed_count = 46 - status.migrated_count;
            status.all_migrated = status.migrated_count == 46;
            self.status.set(status);
        }
    }

    /// Check if a component has been migrated
    pub fn is_migrated(&self, component_name: &str) -> bool {
        self.migrated_components.get().contains(&component_name.to_string())
    }

    /// Get migration progress percentage
    pub fn progress_percentage(&self) -> f64 {
        let status = self.status.get();
        (status.migrated_count as f64 / 46.0) * 100.0
    }

    /// Reset migration status
    pub fn reset(&self) {
        self.status.set(MigrationStatus::default());
        self.migrated_components.set(Vec::new());
    }

    /// Get remaining components to migrate
    pub fn remaining_components(&self) -> Vec<String> {
        let all_components = vec![
            // Core Form Components (12)
            "button", "input", "label", "checkbox", "switch", "radio-group",
            "select", "textarea", "form", "combobox", "command", "input-otp",
            
            // Layout Components (8)
            "card", "separator", "tabs", "accordion", "collapsible", 
            "scroll-area", "aspect-ratio", "resizable",
            
            // Overlay Components (7)
            "dialog", "popover", "tooltip", "alert-dialog", "sheet", 
            "drawer", "hover-card",
            
            // Navigation Components (5)
            "breadcrumb", "navigation-menu", "context-menu", 
            "dropdown-menu", "menubar",
            
            // Feedback & Status (9)
            "alert", "badge", "skeleton", "progress", "toast", 
            "table", "calendar", "date-picker", "pagination",
            
            // Interactive Components (4)
            "slider", "toggle", "carousel", "avatar",
            
            // Development & Utilities (1)
            "error-boundary",
        ];
        
        let migrated = self.migrated_components.get();
        all_components.into_iter()
            .filter(|component| !migrated.contains(&component.to_string()))
            .map(|s| s.to_string())
            .collect()
    }

    /// Get migration statistics
    pub fn get_statistics(&self) -> MigrationStatistics {
        let status = self.status.get();
        let migrated = self.migrated_components.get();
        
        MigrationStatistics {
            total_components: 46,
            migrated_count: status.migrated_count,
            failed_count: status.failed_count,
            remaining_count: 46 - status.migrated_count,
            progress_percentage: self.progress_percentage(),
            migrated_components: migrated,
            is_complete: status.all_migrated,
        }
    }
}

impl Default for ComponentMigrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Migration statistics for detailed reporting
#[derive(Debug, Clone, PartialEq)]
pub struct MigrationStatistics {
    pub total_components: usize,
    pub migrated_count: usize,
    pub failed_count: usize,
    pub remaining_count: usize,
    pub progress_percentage: f64,
    pub migrated_components: Vec<String>,
    pub is_complete: bool,
}

impl MigrationStatistics {
    /// Get a summary string of migration progress
    pub fn summary(&self) -> String {
        format!(
            "Migration Progress: {}/{} components migrated ({:.1}%)",
            self.migrated_count,
            self.total_components,
            self.progress_percentage
        )
    }

    /// Check if migration is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Get remaining work percentage
    pub fn remaining_percentage(&self) -> f64 {
        100.0 - self.progress_percentage
    }
}
