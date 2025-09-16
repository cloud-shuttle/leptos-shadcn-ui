//! # leptos-shadcn Documentation Automation
//!
//! Automated documentation generation system for leptos-shadcn-ui components.
//! Provides comprehensive API documentation, interactive galleries, and test reports.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod parser;
// pub mod generator;  // TODO: Implement
// pub mod gallery;  // TODO: Implement
pub mod templates;
// pub mod testing;  // TODO: Implement

/// Configuration for documentation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
    pub examples_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub generate_gallery: bool,
    pub generate_api_docs: bool,
    pub generate_test_reports: bool,
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::from("packages/leptos"),
            output_dir: PathBuf::from("docs/generated"),
            components_dir: PathBuf::from("packages/leptos"),
            examples_dir: PathBuf::from("examples"),
            templates_dir: PathBuf::from("docs/templates"),
            generate_gallery: true,
            generate_api_docs: true,
            generate_test_reports: true,
        }
    }
}

/// Component metadata extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    pub name: String,
    pub description: Option<String>,
    pub props: Vec<PropMetadata>,
    pub events: Vec<EventMetadata>,
    pub examples: Vec<ExampleMetadata>,
    pub file_path: PathBuf,
    pub tests: Vec<TestMetadata>,
    pub accessibility: AccessibilityInfo,
    pub performance: PerformanceInfo,
}

/// Property metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropMetadata {
    pub name: String,
    pub prop_type: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
    pub examples: Vec<String>,
}

/// Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub name: String,
    pub description: Option<String>,
    pub event_type: String,
    pub examples: Vec<String>,
}

/// Example code metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleMetadata {
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub category: String,
}

/// Test metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetadata {
    pub name: String,
    pub test_type: String, // unit, integration, e2e, performance
    pub description: Option<String>,
    pub coverage: Option<f64>,
}

/// Accessibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    pub wcag_level: String,
    pub keyboard_support: bool,
    pub screen_reader_support: bool,
    pub aria_attributes: Vec<String>,
}

/// Performance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInfo {
    pub render_time_ms: Option<f64>,
    pub bundle_size_kb: Option<f64>,
    pub memory_usage_mb: Option<f64>,
}

/// Generated documentation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDocs {
    pub components: Vec<ComponentMetadata>,
    pub gallery_html: String,
    pub api_docs_html: String,
    pub test_reports_html: String,
    pub generation_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Main documentation generator
pub struct DocGenerator {
    config: DocConfig,
    handlebars: handlebars::Handlebars<'static>,
}

impl DocGenerator {
    /// Create a new documentation generator
    pub fn new(config: DocConfig) -> Result<Self, DocError> {
        let handlebars = handlebars::Handlebars::new();
        
        // Register built-in helpers
        // TODO: Implement template helpers
        // handlebars.register_helper("format_code", Box::new(templates::format_code_helper));
        // handlebars.register_helper("markdown", Box::new(templates::markdown_helper));
        
        Ok(Self {
            config,
            handlebars,
        })
    }

    /// Generate complete documentation
    pub async fn generate(&self) -> Result<GeneratedDocs, DocError> {
        log::info!("Starting documentation generation...");
        
        // Parse components
        let components = self.parse_components().await?;
        log::info!("Parsed {} components", components.len());
        
        // Generate different documentation types
        let gallery_html = if self.config.generate_gallery {
            self.generate_gallery(&components).await?
        } else {
            String::new()
        };
        
        let api_docs_html = if self.config.generate_api_docs {
            self.generate_api_docs(&components).await?
        } else {
            String::new()
        };
        
        let test_reports_html = if self.config.generate_test_reports {
            self.generate_test_reports(&components).await?
        } else {
            String::new()
        };

        let docs = GeneratedDocs {
            components,
            gallery_html,
            api_docs_html,
            test_reports_html,
            generation_timestamp: chrono::Utc::now(),
        };

        // Write output files
        self.write_documentation(&docs).await?;
        
        log::info!("Documentation generation completed successfully");
        Ok(docs)
    }

    /// Parse components from source directory
    async fn parse_components(&self) -> Result<Vec<ComponentMetadata>, DocError> {
        let mut components = Vec::new();
        
        for entry in walkdir::WalkDir::new(&self.config.components_dir) {
            let entry = entry.map_err(|e| DocError::FileSystem(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Some(component) = parser::parse_component_file(path).await? {
                        components.push(component);
                    }
                }
            }
        }
        
        Ok(components)
    }

    /// Generate interactive component gallery
    async fn generate_gallery(&self, _components: &[ComponentMetadata]) -> Result<String, DocError> {
        // TODO: Implement gallery generation
        Ok("<html><body>Gallery placeholder</body></html>".to_string())
    }

    /// Generate API documentation
    async fn generate_api_docs(&self, _components: &[ComponentMetadata]) -> Result<String, DocError> {
        // TODO: Implement API docs generation
        Ok("<html><body>API docs placeholder</body></html>".to_string())
    }

    /// Generate test reports
    async fn generate_test_reports(&self, _components: &[ComponentMetadata]) -> Result<String, DocError> {
        // TODO: Implement test reports generation
        Ok("<html><body>Test reports placeholder</body></html>".to_string())
    }

    /// Write documentation to output directory
    async fn write_documentation(&self, docs: &GeneratedDocs) -> Result<(), DocError> {
        tokio::fs::create_dir_all(&self.config.output_dir)
            .await
            .map_err(DocError::FileSystem)?;

        if !docs.gallery_html.is_empty() {
            let gallery_path = self.config.output_dir.join("gallery.html");
            tokio::fs::write(&gallery_path, &docs.gallery_html)
                .await
                .map_err(DocError::FileSystem)?;
        }

        if !docs.api_docs_html.is_empty() {
            let api_path = self.config.output_dir.join("api.html");
            tokio::fs::write(&api_path, &docs.api_docs_html)
                .await
                .map_err(DocError::FileSystem)?;
        }

        if !docs.test_reports_html.is_empty() {
            let test_path = self.config.output_dir.join("test-reports.html");
            tokio::fs::write(&test_path, &docs.test_reports_html)
                .await
                .map_err(DocError::FileSystem)?;
        }

        // Write metadata JSON
        let metadata_path = self.config.output_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(docs)
            .map_err(DocError::Serialization)?;
        tokio::fs::write(&metadata_path, metadata_json)
            .await
            .map_err(DocError::FileSystem)?;

        Ok(())
    }
}

/// Documentation generation errors
#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Template error: {0}")]
    Template(#[from] handlebars::RenderError),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Walk directory error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_doc_generator_creation() {
        let temp_dir = tempdir().unwrap();
        let config = DocConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let generator = DocGenerator::new(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_component_metadata_serialization() {
        let component = ComponentMetadata {
            name: "TestComponent".to_string(),
            description: Some("A test component".to_string()),
            props: vec![
                PropMetadata {
                    name: "variant".to_string(),
                    prop_type: "String".to_string(),
                    description: Some("Component variant".to_string()),
                    default_value: Some("default".to_string()),
                    required: false,
                    examples: vec!["primary".to_string(), "secondary".to_string()],
                }
            ],
            events: vec![],
            examples: vec![],
            file_path: PathBuf::from("test.rs"),
            tests: vec![],
            accessibility: AccessibilityInfo {
                wcag_level: "AA".to_string(),
                keyboard_support: true,
                screen_reader_support: true,
                aria_attributes: vec!["aria-label".to_string()],
            },
            performance: PerformanceInfo {
                render_time_ms: Some(12.5),
                bundle_size_kb: Some(3.2),
                memory_usage_mb: Some(0.5),
            },
        };

        let json = serde_json::to_string(&component).unwrap();
        let deserialized: ComponentMetadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(component.name, deserialized.name);
        assert_eq!(component.props.len(), deserialized.props.len());
    }

    #[test]
    fn test_doc_config_default() {
        let config = DocConfig::default();
        
        assert!(config.generate_gallery);
        assert!(config.generate_api_docs);
        assert!(config.generate_test_reports);
        assert_eq!(config.source_dir, PathBuf::from("packages/leptos"));
    }

    #[tokio::test]
    async fn test_empty_components_generation() {
        let temp_dir = tempdir().unwrap();
        let config = DocConfig {
            output_dir: temp_dir.path().to_path_buf(),
            components_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let generator = DocGenerator::new(config).unwrap();
        
        // Should handle empty directory gracefully
        let result = generator.generate().await;
        assert!(result.is_ok());
        
        let docs = result.unwrap();
        assert!(docs.components.is_empty());
    }
}