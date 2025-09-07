//! Standard prop definitions and validation for leptos-shadcn-ui components

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{StandardVariant, StandardSize, ApiIssue, TestResult};

/// Core props that every component must support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreProps {
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub disabled: Option<bool>,
}

impl Default for CoreProps {
    fn default() -> Self {
        Self {
            id: None,
            class: None,
            style: None,
            disabled: Some(false),
        }
    }
}

/// Styling props for visual components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StylingProps {
    pub variant: Option<StandardVariant>,
    pub size: Option<StandardSize>,
    pub color: Option<String>,
    pub theme: Option<String>,
}

impl Default for StylingProps {
    fn default() -> Self {
        Self {
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            color: None,
            theme: None,
        }
    }
}

/// Accessibility props for interactive components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityProps {
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_labelledby: Option<String>,
    pub role: Option<String>,
    pub tabindex: Option<i32>,
}

impl Default for AccessibilityProps {
    fn default() -> Self {
        Self {
            aria_label: None,
            aria_describedby: None,
            aria_labelledby: None,
            role: None,
            tabindex: None,
        }
    }
}

/// Form-specific props
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormProps {
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub readonly: Option<bool>,
    pub autocomplete: Option<String>,
}

impl Default for FormProps {
    fn default() -> Self {
        Self {
            name: None,
            placeholder: None,
            required: Some(false),
            readonly: Some(false),
            autocomplete: None,
        }
    }
}

/// Component props validation trait
pub trait PropsValidation {
    /// Validate that props conform to standards
    fn validate_props(&self) -> Result<(), Vec<ApiIssue>>;
    
    /// Get list of required props for this component
    fn required_props() -> Vec<&'static str>;
    
    /// Get list of optional props for this component  
    fn optional_props() -> Vec<&'static str>;
    
    /// Test prop handling compliance
    fn test_prop_compliance(&self) -> TestResult;
}

/// Standard prop validation implementation
impl PropsValidation for CoreProps {
    fn validate_props(&self) -> Result<(), Vec<ApiIssue>> {
        let mut issues = Vec::new();

        // Validate ID format if present
        if let Some(id) = &self.id {
            if !is_valid_html_id(id) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "id".to_string(),
                    expected: "valid HTML ID".to_string(),
                    actual: id.clone(),
                });
            }
        }

        // Validate CSS class format if present
        if let Some(class) = &self.class {
            if !is_valid_css_class(class) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "class".to_string(),
                    expected: "valid CSS class name(s)".to_string(),
                    actual: class.clone(),
                });
            }
        }

        // Validate inline styles if present
        if let Some(style) = &self.style {
            if !is_valid_css_style(style) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "style".to_string(),
                    expected: "valid CSS style declarations".to_string(),
                    actual: style.clone(),
                });
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }

    fn required_props() -> Vec<&'static str> {
        vec![] // Core props are all optional
    }

    fn optional_props() -> Vec<&'static str> {
        vec!["id", "class", "style", "disabled"]
    }

    fn test_prop_compliance(&self) -> TestResult {
        match self.validate_props() {
            Ok(()) => TestResult::passed("Core props validation passed"),
            Err(issues) => TestResult::failed(format!(
                "Core props validation failed: {} issues", 
                issues.len()
            )).with_detail("issues", serde_json::to_value(issues).unwrap_or_default()),
        }
    }
}

impl PropsValidation for StylingProps {
    fn validate_props(&self) -> Result<(), Vec<ApiIssue>> {
        let mut issues = Vec::new();

        // Validate color format if present
        if let Some(color) = &self.color {
            if !is_valid_color_value(color) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "color".to_string(),
                    expected: "valid CSS color value".to_string(),
                    actual: color.clone(),
                });
            }
        }

        // Validate theme name if present
        if let Some(theme) = &self.theme {
            if !is_valid_theme_name(theme) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "theme".to_string(),
                    expected: "valid theme name".to_string(),
                    actual: theme.clone(),
                });
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }

    fn required_props() -> Vec<&'static str> {
        vec![] // Styling props are typically optional
    }

    fn optional_props() -> Vec<&'static str> {
        vec!["variant", "size", "color", "theme"]
    }

    fn test_prop_compliance(&self) -> TestResult {
        match self.validate_props() {
            Ok(()) => TestResult::passed("Styling props validation passed"),
            Err(issues) => TestResult::failed(format!(
                "Styling props validation failed: {} issues",
                issues.len()
            )).with_detail("issues", serde_json::to_value(issues).unwrap_or_default()),
        }
    }
}

impl PropsValidation for AccessibilityProps {
    fn validate_props(&self) -> Result<(), Vec<ApiIssue>> {
        let mut issues = Vec::new();

        // Validate ARIA role if present
        if let Some(role) = &self.role {
            if !is_valid_aria_role(role) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "role".to_string(),
                    expected: "valid ARIA role".to_string(),
                    actual: role.clone(),
                });
            }
        }

        // Validate tabindex range if present
        if let Some(tabindex) = self.tabindex {
            if !(-1..=32767).contains(&tabindex) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "tabindex".to_string(),
                    expected: "integer between -1 and 32767".to_string(),
                    actual: tabindex.to_string(),
                });
            }
        }

        // Check for ARIA labeling consistency
        if self.aria_label.is_none() && self.aria_labelledby.is_none() {
            issues.push(ApiIssue::AccessibilityViolation {
                rule: "ARIA_LABELING".to_string(),
                description: "Interactive components should have either aria-label or aria-labelledby".to_string(),
            });
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }

    fn required_props() -> Vec<&'static str> {
        vec![] // Will vary by component type
    }

    fn optional_props() -> Vec<&'static str> {
        vec!["aria_label", "aria_describedby", "aria_labelledby", "role", "tabindex"]
    }

    fn test_prop_compliance(&self) -> TestResult {
        match self.validate_props() {
            Ok(()) => TestResult::passed("Accessibility props validation passed"),
            Err(issues) => TestResult::failed(format!(
                "Accessibility props validation failed: {} issues",
                issues.len()
            )).with_detail("issues", serde_json::to_value(issues).unwrap_or_default()),
        }
    }
}

impl PropsValidation for FormProps {
    fn validate_props(&self) -> Result<(), Vec<ApiIssue>> {
        let mut issues = Vec::new();

        // Validate name format if present
        if let Some(name) = &self.name {
            if !is_valid_form_name(name) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "name".to_string(),
                    expected: "valid form field name".to_string(),
                    actual: name.clone(),
                });
            }
        }

        // Validate autocomplete value if present
        if let Some(autocomplete) = &self.autocomplete {
            if !is_valid_autocomplete_value(autocomplete) {
                issues.push(ApiIssue::InvalidPropType {
                    prop: "autocomplete".to_string(),
                    expected: "valid autocomplete token".to_string(),
                    actual: autocomplete.clone(),
                });
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }

    fn required_props() -> Vec<&'static str> {
        vec![] // Form props are typically optional except in specific contexts
    }

    fn optional_props() -> Vec<&'static str> {
        vec!["name", "placeholder", "required", "readonly", "autocomplete"]
    }

    fn test_prop_compliance(&self) -> TestResult {
        match self.validate_props() {
            Ok(()) => TestResult::passed("Form props validation passed"),
            Err(issues) => TestResult::failed(format!(
                "Form props validation failed: {} issues",
                issues.len()
            )).with_detail("issues", serde_json::to_value(issues).unwrap_or_default()),
        }
    }
}

/// Comprehensive prop validation for complete component props
#[derive(Debug, Clone)]
pub struct ComponentPropsValidator {
    pub core: CoreProps,
    pub styling: Option<StylingProps>,
    pub accessibility: Option<AccessibilityProps>,
    pub form: Option<FormProps>,
}

impl ComponentPropsValidator {
    pub fn new() -> Self {
        Self {
            core: CoreProps::default(),
            styling: None,
            accessibility: None,
            form: None,
        }
    }

    pub fn with_styling(mut self, styling: StylingProps) -> Self {
        self.styling = Some(styling);
        self
    }

    pub fn with_accessibility(mut self, accessibility: AccessibilityProps) -> Self {
        self.accessibility = Some(accessibility);
        self
    }

    pub fn with_form_props(mut self, form: FormProps) -> Self {
        self.form = Some(form);
        self
    }

    /// Validate all props sections
    pub fn validate_all(&self) -> Result<(), Vec<ApiIssue>> {
        let mut all_issues = Vec::new();

        // Validate core props
        if let Err(issues) = self.core.validate_props() {
            all_issues.extend(issues);
        }

        // Validate styling props if present
        if let Some(ref styling) = self.styling {
            if let Err(issues) = styling.validate_props() {
                all_issues.extend(issues);
            }
        }

        // Validate accessibility props if present
        if let Some(ref accessibility) = self.accessibility {
            if let Err(issues) = accessibility.validate_props() {
                all_issues.extend(issues);
            }
        }

        // Validate form props if present
        if let Some(ref form) = self.form {
            if let Err(issues) = form.validate_props() {
                all_issues.extend(issues);
            }
        }

        if all_issues.is_empty() {
            Ok(())
        } else {
            Err(all_issues)
        }
    }

    /// Generate comprehensive compliance test result
    pub fn test_comprehensive_compliance(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        
        match self.validate_all() {
            Ok(()) => {
                let mut details = HashMap::new();
                details.insert("core_props".to_string(), serde_json::json!(true));
                
                if self.styling.is_some() {
                    details.insert("styling_props".to_string(), serde_json::json!(true));
                }
                if self.accessibility.is_some() {
                    details.insert("accessibility_props".to_string(), serde_json::json!(true));
                }
                if self.form.is_some() {
                    details.insert("form_props".to_string(), serde_json::json!(true));
                }

                TestResult::passed("Comprehensive props validation passed")
                    .with_timing(start_time.elapsed().as_millis() as u64)
                    .with_detail("validated_sections", serde_json::to_value(details).unwrap_or_default())
            }
            Err(issues) => {
                TestResult::failed(format!(
                    "Comprehensive props validation failed: {} total issues",
                    issues.len()
                )).with_timing(start_time.elapsed().as_millis() as u64)
                  .with_detail("all_issues", serde_json::to_value(issues).unwrap_or_default())
            }
        }
    }
}

impl Default for ComponentPropsValidator {
    fn default() -> Self {
        Self::new()
    }
}

// Validation helper functions
fn is_valid_html_id(id: &str) -> bool {
    let regex = regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    regex.is_match(id) && !id.is_empty()
}

fn is_valid_css_class(class: &str) -> bool {
    // Allow multiple classes separated by spaces
    class.split_whitespace()
        .all(|c| {
            let regex = regex::Regex::new(r"^[a-zA-Z_-][a-zA-Z0-9_-]*$").unwrap();
            regex.is_match(c)
        })
}

fn is_valid_css_style(style: &str) -> bool {
    // Basic CSS validation - check for property: value; patterns
    let regex = regex::Regex::new(r"^(\s*[a-zA-Z-]+\s*:\s*[^;]+;\s*)*$").unwrap();
    regex.is_match(style)
}

fn is_valid_color_value(color: &str) -> bool {
    // Support hex, rgb, rgba, hsl, hsla, and named colors
    let patterns = [
        r"^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$", // hex
        r"^rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)$", // rgb
        r"^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*[0-1]?\.?\d*\s*\)$", // rgba
        r"^hsl\(\s*\d+\s*,\s*\d+%?\s*,\s*\d+%?\s*\)$", // hsl
        r"^hsla\(\s*\d+\s*,\s*\d+%?\s*,\s*\d+%?\s*,\s*[0-1]?\.?\d*\s*\)$", // hsla
        r"^[a-zA-Z]+$", // named colors
    ];

    patterns.iter().any(|pattern| {
        regex::Regex::new(pattern).unwrap().is_match(color)
    })
}

fn is_valid_theme_name(theme: &str) -> bool {
    let valid_themes = ["light", "dark", "auto", "high-contrast"];
    valid_themes.contains(&theme) || {
        // Allow custom theme names following naming convention
        let regex = regex::Regex::new(r"^[a-z][a-z0-9-]*$").unwrap();
        regex.is_match(theme)
    }
}

fn is_valid_aria_role(role: &str) -> bool {
    let valid_roles = [
        "alert", "alertdialog", "application", "article", "banner", "button",
        "checkbox", "columnheader", "combobox", "complementary", "contentinfo",
        "dialog", "directory", "document", "form", "grid", "gridcell", "group",
        "heading", "img", "link", "list", "listbox", "listitem", "log", "main",
        "marquee", "math", "menu", "menubar", "menuitem", "menuitemcheckbox",
        "menuitemradio", "navigation", "note", "option", "presentation",
        "progressbar", "radio", "radiogroup", "region", "row", "rowgroup",
        "rowheader", "scrollbar", "search", "separator", "slider", "spinbutton",
        "status", "tab", "tablist", "tabpanel", "textbox", "timer", "toolbar",
        "tooltip", "tree", "treegrid", "treeitem"
    ];
    
    valid_roles.contains(&role)
}

fn is_valid_form_name(name: &str) -> bool {
    let regex = regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    regex.is_match(name) && !name.is_empty()
}

fn is_valid_autocomplete_value(autocomplete: &str) -> bool {
    let valid_values = [
        "on", "off", "name", "honorific-prefix", "given-name", "additional-name",
        "family-name", "honorific-suffix", "nickname", "email", "username",
        "new-password", "current-password", "one-time-code", "organization-title",
        "organization", "street-address", "address-line1", "address-line2",
        "address-line3", "address-level4", "address-level3", "address-level2",
        "address-level1", "country", "country-name", "postal-code", "cc-name",
        "cc-given-name", "cc-additional-name", "cc-family-name", "cc-number",
        "cc-exp", "cc-exp-month", "cc-exp-year", "cc-csc", "cc-type",
        "transaction-currency", "transaction-amount", "language", "bday",
        "bday-day", "bday-month", "bday-year", "sex", "tel", "tel-country-code",
        "tel-national", "tel-area-code", "tel-local", "tel-extension", "impp",
        "url", "photo"
    ];
    
    valid_values.contains(&autocomplete)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_props_validation() {
        let valid_props = CoreProps {
            id: Some("valid-id".to_string()),
            class: Some("valid-class another-class".to_string()),
            style: Some("color: red; font-size: 14px;".to_string()),
            disabled: Some(false),
        };

        assert!(valid_props.validate_props().is_ok());
    }

    #[test]
    fn test_invalid_core_props() {
        let invalid_props = CoreProps {
            id: Some("123-invalid".to_string()), // starts with number
            class: Some("invalid@class".to_string()), // invalid character
            style: Some("invalid-css".to_string()), // missing colon and semicolon
            disabled: Some(false),
        };

        let result = invalid_props.validate_props();
        assert!(result.is_err());
        
        let issues = result.unwrap_err();
        assert_eq!(issues.len(), 3); // id, class, and style issues
    }

    #[test]
    fn test_styling_props_validation() {
        let valid_props = StylingProps {
            variant: Some(StandardVariant::Primary),
            size: Some(StandardSize::Lg),
            color: Some("#ff0000".to_string()),
            theme: Some("dark".to_string()),
        };

        assert!(valid_props.validate_props().is_ok());
    }

    #[test]
    fn test_accessibility_props_validation() {
        let valid_props = AccessibilityProps {
            aria_label: Some("Button label".to_string()),
            role: Some("button".to_string()),
            tabindex: Some(0),
            ..Default::default()
        };

        assert!(valid_props.validate_props().is_ok());
    }

    #[test]
    fn test_form_props_validation() {
        let valid_props = FormProps {
            name: Some("email".to_string()),
            autocomplete: Some("email".to_string()),
            required: Some(true),
            ..Default::default()
        };

        assert!(valid_props.validate_props().is_ok());
    }

    #[test]
    fn test_comprehensive_validator() {
        let validator = ComponentPropsValidator::new()
            .with_styling(StylingProps::default())
            .with_accessibility(AccessibilityProps {
                aria_label: Some("Test".to_string()),
                ..Default::default()
            });

        assert!(validator.validate_all().is_ok());
    }

    #[test]
    fn test_color_validation() {
        assert!(is_valid_color_value("#ff0000"));
        assert!(is_valid_color_value("#fff"));
        assert!(is_valid_color_value("rgb(255, 0, 0)"));
        assert!(is_valid_color_value("rgba(255, 0, 0, 0.5)"));
        assert!(is_valid_color_value("red"));
        
        assert!(!is_valid_color_value("#gg0000"));
        assert!(!is_valid_color_value("invalid-color"));
    }

    #[test]
    fn test_aria_role_validation() {
        assert!(is_valid_aria_role("button"));
        assert!(is_valid_aria_role("dialog"));
        assert!(is_valid_aria_role("navigation"));
        
        assert!(!is_valid_aria_role("invalid-role"));
        assert!(!is_valid_aria_role("custom"));
    }
}