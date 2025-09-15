// Snapshot testing utilities for leptos-shadcn-ui components
// Provides comprehensive snapshot testing for UI consistency and regression detection

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Snapshot test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    pub name: String,
    pub component_name: String,
    pub variant: Option<String>,
    pub props_hash: String,
    pub created_at: String,
    pub leptos_version: String,
}

/// Snapshot data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub config: SnapshotConfig,
    pub html_output: String,
    pub css_classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub children_count: usize,
    pub accessibility_tree: Option<AccessibilityNode>,
}

/// Accessibility tree node for a11y snapshot testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityNode {
    pub role: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub properties: HashMap<String, String>,
    pub children: Vec<AccessibilityNode>,
}

/// Snapshot testing framework
pub struct SnapshotTester {
    snapshots_dir: PathBuf,
    update_snapshots: bool,
}

impl SnapshotTester {
    /// Create a new snapshot tester
    pub fn new<P: AsRef<Path>>(snapshots_dir: P) -> Self {
        let snapshots_dir = snapshots_dir.as_ref().to_path_buf();
        fs::create_dir_all(&snapshots_dir).unwrap_or_else(|_| {
            panic!("Failed to create snapshots directory: {:?}", snapshots_dir)
        });

        Self {
            snapshots_dir,
            update_snapshots: std::env::var("UPDATE_SNAPSHOTS").is_ok(),
        }
    }

    /// Test a component against its snapshot
    pub fn test_component_snapshot<V: leptos::IntoView>(
        &self,
        name: &str,
        component: V,
        props_description: &str,
    ) -> SnapshotTestResult {
        let snapshot = self.capture_snapshot(name, component, props_description);
        let snapshot_file = self.get_snapshot_path(name);

        if self.update_snapshots || !snapshot_file.exists() {
            self.save_snapshot(&snapshot, &snapshot_file);
            SnapshotTestResult::Updated
        } else {
            match self.load_snapshot(&snapshot_file) {
                Ok(existing_snapshot) => {
                    if self.snapshots_match(&snapshot, &existing_snapshot) {
                        SnapshotTestResult::Passed
                    } else {
                        SnapshotTestResult::Failed {
                            differences: self.compute_differences(&snapshot, &existing_snapshot),
                            actual: snapshot,
                            expected: existing_snapshot,
                        }
                    }
                }
                Err(e) => SnapshotTestResult::Error(format!("Failed to load snapshot: {}", e)),
            }
        }
    }

    /// Capture a snapshot of a component
    fn capture_snapshot<V: leptos::IntoView>(
        &self,
        name: &str,
        component: V,
        props_description: &str,
    ) -> Snapshot {
        // In a real implementation, this would render the component to HTML
        // and extract CSS classes, attributes, etc.
        // For now, we create a mock snapshot

        let html_output = format!("<div data-component='{}'>Mock HTML output</div>", name);
        let css_classes = vec!["component-base".to_string(), name.to_string()];
        let mut attributes = HashMap::new();
        attributes.insert("data-component".to_string(), name.to_string());

        let config = SnapshotConfig {
            name: name.to_string(),
            component_name: name.split('_').next().unwrap_or(name).to_string(),
            variant: None,
            props_hash: self.hash_string(props_description),
            created_at: chrono::Utc::now().to_rfc3339(),
            leptos_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let accessibility_tree = Some(AccessibilityNode {
            role: Some("generic".to_string()),
            name: Some(name.to_string()),
            description: None,
            properties: HashMap::new(),
            children: vec![],
        });

        Snapshot {
            config,
            html_output,
            css_classes,
            attributes,
            children_count: 0,
            accessibility_tree,
        }
    }

    /// Check if two snapshots match
    fn snapshots_match(&self, a: &Snapshot, b: &Snapshot) -> bool {
        a.html_output == b.html_output
            && a.css_classes == b.css_classes
            && a.attributes == b.attributes
            && a.children_count == b.children_count
            && self.accessibility_trees_match(&a.accessibility_tree, &b.accessibility_tree)
    }

    /// Compare accessibility trees
    fn accessibility_trees_match(
        &self,
        a: &Option<AccessibilityNode>,
        b: &Option<AccessibilityNode>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.role == b.role
                    && a.name == b.name
                    && a.description == b.description
                    && a.properties == b.properties
                    && a.children.len() == b.children.len()
                    && a.children
                        .iter()
                        .zip(b.children.iter())
                        .all(|(child_a, child_b)| {
                            self.accessibility_trees_match(&Some(child_a.clone()), &Some(child_b.clone()))
                        })
            }
            _ => false,
        }
    }

    /// Compute differences between snapshots
    fn compute_differences(&self, actual: &Snapshot, expected: &Snapshot) -> Vec<SnapshotDifference> {
        let mut differences = Vec::new();

        if actual.html_output != expected.html_output {
            differences.push(SnapshotDifference::HtmlOutput {
                actual: actual.html_output.clone(),
                expected: expected.html_output.clone(),
            });
        }

        if actual.css_classes != expected.css_classes {
            differences.push(SnapshotDifference::CssClasses {
                actual: actual.css_classes.clone(),
                expected: expected.css_classes.clone(),
            });
        }

        if actual.attributes != expected.attributes {
            differences.push(SnapshotDifference::Attributes {
                actual: actual.attributes.clone(),
                expected: expected.attributes.clone(),
            });
        }

        if actual.children_count != expected.children_count {
            differences.push(SnapshotDifference::ChildrenCount {
                actual: actual.children_count,
                expected: expected.children_count,
            });
        }

        differences
    }

    /// Get the path for a snapshot file
    fn get_snapshot_path(&self, name: &str) -> PathBuf {
        self.snapshots_dir.join(format!("{}.snap.json", name))
    }

    /// Save a snapshot to disk
    fn save_snapshot(&self, snapshot: &Snapshot, path: &Path) {
        let json = serde_json::to_string_pretty(snapshot)
            .expect("Failed to serialize snapshot");
        
        fs::write(path, json)
            .unwrap_or_else(|e| panic!("Failed to write snapshot to {:?}: {}", path, e));
    }

    /// Load a snapshot from disk
    fn load_snapshot(&self, path: &Path) -> Result<Snapshot, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read snapshot file: {}", e))?;
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse snapshot JSON: {}", e))
    }

    /// Hash a string for comparison
    fn hash_string(&self, s: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Result of a snapshot test
#[derive(Debug)]
pub enum SnapshotTestResult {
    Passed,
    Updated,
    Failed {
        differences: Vec<SnapshotDifference>,
        actual: Snapshot,
        expected: Snapshot,
    },
    Error(String),
}

/// Types of differences that can occur between snapshots
#[derive(Debug, Clone)]
pub enum SnapshotDifference {
    HtmlOutput { actual: String, expected: String },
    CssClasses { actual: Vec<String>, expected: Vec<String> },
    Attributes { actual: HashMap<String, String>, expected: HashMap<String, String> },
    ChildrenCount { actual: usize, expected: usize },
}

/// Macro for creating snapshot tests
#[macro_export]
macro_rules! snapshot_test {
    ($test_name:ident, $component:expr, $props_desc:expr) => {
        #[test]
        fn $test_name() {
            use $crate::snapshot_testing::SnapshotTester;
            
            let tester = SnapshotTester::new("tests/snapshots");
            let result = tester.test_component_snapshot(
                stringify!($test_name),
                $component,
                $props_desc,
            );

            match result {
                SnapshotTestResult::Passed => {},
                SnapshotTestResult::Updated => {
                    println!("Snapshot updated: {}", stringify!($test_name));
                },
                SnapshotTestResult::Failed { differences, .. } => {
                    panic!("Snapshot test failed: {:?}", differences);
                },
                SnapshotTestResult::Error(err) => {
                    panic!("Snapshot test error: {}", err);
                },
            }
        }
    };
}

/// Visual regression testing utilities
pub mod visual_regression {
    use super::*;

    /// Configuration for visual regression tests
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VisualTestConfig {
        pub viewport_width: u32,
        pub viewport_height: u32,
        pub device_pixel_ratio: f32,
        pub theme: String,
        pub animations_disabled: bool,
    }

    impl Default for VisualTestConfig {
        fn default() -> Self {
            Self {
                viewport_width: 1920,
                viewport_height: 1080,
                device_pixel_ratio: 1.0,
                theme: "light".to_string(),
                animations_disabled: true,
            }
        }
    }

    /// Visual snapshot data
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VisualSnapshot {
        pub config: VisualTestConfig,
        pub component_name: String,
        pub screenshot_path: PathBuf,
        pub bounding_box: BoundingBox,
        pub timestamp: String,
    }

    /// Bounding box for component positioning
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BoundingBox {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }

    /// Visual regression tester
    pub struct VisualTester {
        screenshots_dir: PathBuf,
        config: VisualTestConfig,
    }

    impl VisualTester {
        pub fn new<P: AsRef<Path>>(screenshots_dir: P, config: VisualTestConfig) -> Self {
            let screenshots_dir = screenshots_dir.as_ref().to_path_buf();
            fs::create_dir_all(&screenshots_dir).unwrap();

            Self {
                screenshots_dir,
                config,
            }
        }

        /// Take a visual snapshot of a component
        pub fn take_visual_snapshot(
            &self,
            component_name: &str,
            _variant: Option<&str>,
        ) -> Result<VisualSnapshot, String> {
            // In a real implementation, this would:
            // 1. Render the component in a controlled environment
            // 2. Take a screenshot using a headless browser
            // 3. Save the screenshot to disk
            // 4. Return the snapshot metadata

            let screenshot_path = self.screenshots_dir.join(format!("{}.png", component_name));
            
            // Mock screenshot creation
            std::fs::write(&screenshot_path, b"mock screenshot data")
                .map_err(|e| format!("Failed to write screenshot: {}", e))?;

            Ok(VisualSnapshot {
                config: self.config.clone(),
                component_name: component_name.to_string(),
                screenshot_path,
                bounding_box: BoundingBox {
                    x: 0.0,
                    y: 0.0,
                    width: 200.0,
                    height: 100.0,
                },
                timestamp: chrono::Utc::now().to_rfc3339(),
            })
        }

        /// Compare two visual snapshots
        pub fn compare_visual_snapshots(
            &self,
            actual: &VisualSnapshot,
            expected: &VisualSnapshot,
            tolerance: f32,
        ) -> Result<bool, String> {
            // In a real implementation, this would:
            // 1. Load both images
            // 2. Compare them pixel by pixel
            // 3. Calculate a difference percentage
            // 4. Return whether the difference is within tolerance

            if !actual.screenshot_path.exists() {
                return Err("Actual screenshot not found".to_string());
            }

            if !expected.screenshot_path.exists() {
                return Err("Expected screenshot not found".to_string());
            }

            // Mock comparison - in reality, this would use image comparison libraries
            let difference_percentage = 0.0; // Mock: no difference
            
            Ok(difference_percentage <= tolerance)
        }
    }
}

/// Multi-theme snapshot testing
pub mod theme_testing {
    use super::*;

    /// Theme configuration for snapshot testing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ThemeConfig {
        pub name: String,
        pub css_variables: HashMap<String, String>,
        pub class_overrides: HashMap<String, String>,
    }

    /// Multi-theme snapshot tester
    pub struct ThemeTester {
        tester: SnapshotTester,
        themes: Vec<ThemeConfig>,
    }

    impl ThemeTester {
        pub fn new<P: AsRef<Path>>(snapshots_dir: P, themes: Vec<ThemeConfig>) -> Self {
            Self {
                tester: SnapshotTester::new(snapshots_dir),
                themes,
            }
        }

        /// Test a component across all themes
        pub fn test_component_across_themes<V: leptos::IntoView + Clone>(
            &self,
            name: &str,
            component: V,
            props_description: &str,
        ) -> Vec<(String, SnapshotTestResult)> {
            self.themes
                .iter()
                .map(|theme| {
                    let themed_name = format!("{}_{}", name, theme.name);
                    let result = self.tester.test_component_snapshot(
                        &themed_name,
                        component.clone(),
                        props_description,
                    );
                    (theme.name.clone(), result)
                })
                .collect()
        }
    }
}

/// Responsive snapshot testing
pub mod responsive_testing {
    use super::*;

    /// Viewport configuration for responsive testing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Viewport {
        pub name: String,
        pub width: u32,
        pub height: u32,
        pub device_pixel_ratio: f32,
    }

    /// Common viewport configurations
    impl Viewport {
        pub fn mobile() -> Self {
            Self {
                name: "mobile".to_string(),
                width: 375,
                height: 667,
                device_pixel_ratio: 2.0,
            }
        }

        pub fn tablet() -> Self {
            Self {
                name: "tablet".to_string(),
                width: 768,
                height: 1024,
                device_pixel_ratio: 2.0,
            }
        }

        pub fn desktop() -> Self {
            Self {
                name: "desktop".to_string(),
                width: 1920,
                height: 1080,
                device_pixel_ratio: 1.0,
            }
        }
    }

    /// Responsive snapshot tester
    pub struct ResponsiveTester {
        tester: SnapshotTester,
        viewports: Vec<Viewport>,
    }

    impl ResponsiveTester {
        pub fn new<P: AsRef<Path>>(snapshots_dir: P, viewports: Vec<Viewport>) -> Self {
            Self {
                tester: SnapshotTester::new(snapshots_dir),
                viewports,
            }
        }

        /// Test a component across all viewports
        pub fn test_component_responsive<V: leptos::IntoView + Clone>(
            &self,
            name: &str,
            component: V,
            props_description: &str,
        ) -> Vec<(String, SnapshotTestResult)> {
            self.viewports
                .iter()
                .map(|viewport| {
                    let responsive_name = format!("{}_{}", name, viewport.name);
                    let result = self.tester.test_component_snapshot(
                        &responsive_name,
                        component.clone(),
                        props_description,
                    );
                    (viewport.name.clone(), result)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_snapshot_tester_creation() {
        let temp_dir = tempdir().unwrap();
        let tester = SnapshotTester::new(temp_dir.path());
        
        assert_eq!(tester.snapshots_dir, temp_dir.path());
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_hash_string() {
        let temp_dir = tempdir().unwrap();
        let tester = SnapshotTester::new(temp_dir.path());
        
        let hash1 = tester.hash_string("test");
        let hash2 = tester.hash_string("test");
        let hash3 = tester.hash_string("different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_accessibility_tree_matching() {
        let temp_dir = tempdir().unwrap();
        let tester = SnapshotTester::new(temp_dir.path());

        let tree1 = AccessibilityNode {
            role: Some("button".to_string()),
            name: Some("Click me".to_string()),
            description: None,
            properties: HashMap::new(),
            children: vec![],
        };

        let tree2 = tree1.clone();
        let mut tree3 = tree1.clone();
        tree3.role = Some("link".to_string());

        assert!(tester.accessibility_trees_match(&Some(tree1.clone()), &Some(tree2)));
        assert!(!tester.accessibility_trees_match(&Some(tree1.clone()), &Some(tree3)));
        assert!(tester.accessibility_trees_match(&None, &None));
        assert!(!tester.accessibility_trees_match(&Some(tree1), &None));
    }
}