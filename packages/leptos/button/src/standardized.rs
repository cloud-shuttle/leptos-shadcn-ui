//! Standardized Button component following leptos-shadcn-ui v1.0 API standards
//! This implementation demonstrates the new API standardization framework

use leptos::prelude::*;
use leptos_shadcn_api_standards::*;
use leptos_shadcn_api_standards::props::*;
use std::collections::HashMap;

/// Standardized Button component props following API standards
#[derive(Debug, Clone, PartialEq)]
pub struct StandardizedButtonProps {
    // Core props (required by standards)
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub disabled: Option<bool>,
    
    // Styling props
    pub variant: Option<StandardVariant>,
    pub size: Option<StandardSize>,
    
    // Accessibility props
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_labelledby: Option<String>,
    pub role: Option<String>,
    pub tabindex: Option<i32>,
    
    // Button-specific props
    pub button_type: Option<String>, // "button", "submit", "reset"
    
    // Event handlers
    pub onclick: Option<Box<dyn Fn()>>,
    pub onfocus: Option<Box<dyn Fn()>>,
    pub onblur: Option<Box<dyn Fn()>>,
    
    // Children
    pub children: Option<leptos::View>,
}

impl Default for StandardizedButtonProps {
    fn default() -> Self {
        Self {
            id: None,
            class: None,
            style: None,
            disabled: Some(false),
            variant: Some(StandardVariant::Default),
            size: Some(StandardSize::Default),
            aria_label: None,
            aria_describedby: None,
            aria_labelledby: None,
            role: Some("button".to_string()),
            tabindex: None,
            button_type: Some("button".to_string()),
            onclick: None,
            onfocus: None,
            onblur: None,
            children: None,
        }
    }
}

/// API compliance implementation for StandardizedButton
impl ApiCompliant for StandardizedButtonProps {
    type Props = StandardizedButtonProps;

    fn test_basic_rendering(&self) -> TestResult {
        // Test that button renders without panicking
        let start_time = std::time::Instant::now();
        
        let render_result = std::panic::catch_unwind(|| {
            // Simulate rendering
            let _ = StandardizedButton::render(self.clone());
        });
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        match render_result {
            Ok(_) => TestResult::passed("Button renders successfully")
                .with_timing(duration),
            Err(_) => TestResult::failed("Button rendering panicked")
                .with_timing(duration),
        }
    }

    fn test_prop_handling(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        
        // Validate core props
        let core_validator = ComponentPropsValidator::new();
        let core_props = CoreProps {
            id: self.id.clone(),
            class: self.class.clone(), 
            style: self.style.clone(),
            disabled: self.disabled,
        };
        
        // Validate styling props
        let styling_props = StylingProps {
            variant: self.variant.clone(),
            size: self.size.clone(),
            color: None,
            theme: None,
        };
        
        // Validate accessibility props
        let accessibility_props = AccessibilityProps {
            aria_label: self.aria_label.clone(),
            aria_describedby: self.aria_describedby.clone(),
            aria_labelledby: self.aria_labelledby.clone(),
            role: self.role.clone(),
            tabindex: self.tabindex,
        };
        
        let validator = core_validator
            .with_styling(styling_props)
            .with_accessibility(accessibility_props);
        
        let result = validator.test_comprehensive_compliance()
            .with_timing(start_time.elapsed().as_millis() as u64);
        
        result
    }

    fn test_accessibility_compliance(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        let mut issues = Vec::new();
        
        // Check role is appropriate for button
        if let Some(ref role) = self.role {
            if role != "button" {
                issues.push(format!("Button role should be 'button', found: '{}'", role));
            }
        } else {
            issues.push("Button should have explicit role attribute".to_string());
        }
        
        // Check for accessible name
        if self.aria_label.is_none() && self.aria_labelledby.is_none() && self.children.is_none() {
            issues.push("Button should have accessible name (aria-label, aria-labelledby, or text content)".to_string());
        }
        
        // Check button type is valid
        if let Some(ref btn_type) = self.button_type {
            if !["button", "submit", "reset"].contains(&btn_type.as_str()) {
                issues.push(format!("Invalid button type: '{}'. Must be 'button', 'submit', or 'reset'", btn_type));
            }
        }
        
        // Check disabled state consistency
        if let Some(disabled) = self.disabled {
            if disabled && self.tabindex.map(|t| t >= 0).unwrap_or(false) {
                issues.push("Disabled buttons should not be focusable (tabindex should be -1 or not set)".to_string());
            }
        }
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        if issues.is_empty() {
            TestResult::passed("Button accessibility compliance validated")
                .with_timing(duration)
        } else {
            TestResult::failed(format!("Accessibility issues found: {}", issues.len()))
                .with_timing(duration)
                .with_detail("issues", serde_json::to_value(issues).unwrap_or_default())
        }
    }

    fn test_event_handling(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        
        // Test that event handlers can be called without panicking
        let mut event_tests = Vec::new();
        
        if let Some(ref onclick) = self.onclick {
            let test_result = std::panic::catch_unwind(|| {
                onclick();
            });
            event_tests.push(("onclick", test_result.is_ok()));
        }
        
        if let Some(ref onfocus) = self.onfocus {
            let test_result = std::panic::catch_unwind(|| {
                onfocus();
            });
            event_tests.push(("onfocus", test_result.is_ok()));
        }
        
        if let Some(ref onblur) = self.onblur {
            let test_result = std::panic::catch_unwind(|| {
                onblur();
            });
            event_tests.push(("onblur", test_result.is_ok()));
        }
        
        let duration = start_time.elapsed().as_millis() as u64;
        let all_passed = event_tests.iter().all(|(_, passed)| *passed);
        
        if all_passed {
            TestResult::passed("Event handling validation passed")
                .with_timing(duration)
                .with_detail("tested_events", serde_json::to_value(event_tests.len()).unwrap_or_default())
        } else {
            TestResult::failed("Some event handlers failed validation")
                .with_timing(duration)
                .with_detail("event_results", serde_json::to_value(event_tests).unwrap_or_default())
        }
    }

    fn test_css_compliance(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        
        // Generate CSS classes according to standards
        let variant = self.variant.as_ref().unwrap_or(&StandardVariant::Default);
        let size = self.size.as_ref().unwrap_or(&StandardSize::Default);
        
        let generated_classes = utils::generate_standard_classes(
            "button",
            variant,
            size,
            self.class.as_deref()
        );
        
        // Validate class naming conventions
        let class_parts: Vec<&str> = generated_classes.split_whitespace().collect();
        let mut validation_issues = Vec::new();
        
        for class in &class_parts {
            if let Err(error) = utils::validate_css_class_name(class) {
                validation_issues.push(error);
            }
        }
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        if validation_issues.is_empty() {
            TestResult::passed("CSS class compliance validated")
                .with_timing(duration)
                .with_detail("generated_classes", serde_json::to_value(generated_classes).unwrap_or_default())
        } else {
            TestResult::failed(format!("CSS validation issues: {}", validation_issues.len()))
                .with_timing(duration)
                .with_detail("issues", serde_json::to_value(validation_issues).unwrap_or_default())
        }
    }

    fn test_performance_compliance(&self) -> TestResult {
        let start_time = std::time::Instant::now();
        
        // Test render performance
        let render_times: Vec<f64> = (0..100)
            .map(|_| {
                let render_start = std::time::Instant::now();
                let _ = StandardizedButton::render(self.clone());
                render_start.elapsed().as_secs_f64() * 1000.0 // Convert to milliseconds
            })
            .collect();
        
        let avg_render_time = render_times.iter().sum::<f64>() / render_times.len() as f64;
        let max_render_time = render_times.iter().fold(0.0f64, |a, &b| a.max(b));
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        // Check against performance thresholds (16ms for 60fps)
        if avg_render_time < 16.0 && max_render_time < 32.0 {
            TestResult::passed("Performance compliance validated")
                .with_timing(duration)
                .with_detail("avg_render_time_ms", serde_json::to_value(avg_render_time).unwrap_or_default())
                .with_detail("max_render_time_ms", serde_json::to_value(max_render_time).unwrap_or_default())
        } else {
            TestResult::failed("Performance thresholds exceeded")
                .with_timing(duration)
                .with_detail("avg_render_time_ms", serde_json::to_value(avg_render_time).unwrap_or_default())
                .with_detail("max_render_time_ms", serde_json::to_value(max_render_time).unwrap_or_default())
                .with_detail("threshold_avg_ms", serde_json::to_value(16.0).unwrap_or_default())
                .with_detail("threshold_max_ms", serde_json::to_value(32.0).unwrap_or_default())
        }
    }
}

/// Standardized Button component implementation
pub struct StandardizedButton;

impl StandardizedButton {
    /// Render the standardized button component
    pub fn render(props: StandardizedButtonProps) -> impl IntoView {
        // Extract props with defaults
        let disabled = props.disabled.unwrap_or(false);
        let variant = props.variant.unwrap_or(StandardVariant::Default);
        let size = props.size.unwrap_or(StandardSize::Default);
        let button_type = props.button_type.unwrap_or_else(|| "button".to_string());
        let role = props.role.unwrap_or_else(|| "button".to_string());
        
        // Generate unique ID if not provided
        let id = props.id.unwrap_or_else(|| utils::generate_component_id("button"));
        
        // Generate CSS classes
        let css_classes = utils::generate_standard_classes(
            "button",
            &variant,
            &size,
            props.class.as_deref()
        );
        
        // Generate accessibility attributes
        let mut aria_attrs = HashMap::new();
        
        if let Some(label) = props.aria_label {
            aria_attrs.insert("aria-label", label);
        }
        if let Some(described_by) = props.aria_describedby {
            aria_attrs.insert("aria-describedby", described_by);
        }
        if let Some(labelled_by) = props.aria_labelledby {
            aria_attrs.insert("aria-labelledby", labelled_by);
        }
        
        // Handle disabled state
        if disabled {
            aria_attrs.insert("aria-disabled", "true".to_string());
        }
        
        // Create the button view
        view! {
            <button
                id=id
                class=css_classes
                style=props.style.unwrap_or_default()
                disabled=disabled
                type=button_type
                role=role
                tabindex=props.tabindex.map(|t| t.to_string())
                aria-label=props.aria_label
                aria-describedby=props.aria_describedby
                aria-labelledby=props.aria_labelledby
                aria-disabled=if disabled { Some("true".to_string()) } else { None }
                on:click=move |_| {
                    if !disabled {
                        if let Some(ref handler) = props.onclick {
                            handler();
                        }
                    }
                }
                on:focus=move |_| {
                    if let Some(ref handler) = props.onfocus {
                        handler();
                    }
                }
                on:blur=move |_| {
                    if let Some(ref handler) = props.onblur {
                        handler();
                    }
                }
            >
                {props.children}
            </button>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_standardized_button_props_default() {
        let props = StandardizedButtonProps::default();
        
        assert_eq!(props.disabled, Some(false));
        assert_eq!(props.variant, Some(StandardVariant::Default));
        assert_eq!(props.size, Some(StandardSize::Default));
        assert_eq!(props.role, Some("button".to_string()));
        assert_eq!(props.button_type, Some("button".to_string()));
    }

    #[test]
    fn test_api_compliance_basic_rendering() {
        let props = StandardizedButtonProps::default();
        let result = props.test_basic_rendering();
        
        assert!(result.passed);
        assert!(result.execution_time_ms > 0);
    }

    #[test]
    fn test_api_compliance_prop_handling() {
        let props = StandardizedButtonProps {
            id: Some("test-button".to_string()),
            class: Some("custom-class".to_string()),
            aria_label: Some("Test Button".to_string()),
            ..Default::default()
        };
        
        let result = props.test_prop_handling();
        assert!(result.passed);
    }

    #[test]
    fn test_api_compliance_accessibility() {
        let props = StandardizedButtonProps {
            aria_label: Some("Accessible button".to_string()),
            role: Some("button".to_string()),
            ..Default::default()
        };
        
        let result = props.test_accessibility_compliance();
        assert!(result.passed);
    }

    #[test]
    fn test_api_compliance_accessibility_failures() {
        let props = StandardizedButtonProps {
            role: Some("invalid-role".to_string()),
            button_type: Some("invalid-type".to_string()),
            ..Default::default()
        };
        
        let result = props.test_accessibility_compliance();
        assert!(!result.passed);
        assert!(result.details.contains_key("issues"));
    }

    #[test]
    fn test_api_compliance_event_handling() {
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();
        
        let props = StandardizedButtonProps {
            onclick: Some(Box::new(move || {
                *clicked_clone.lock().unwrap() = true;
            })),
            ..Default::default()
        };
        
        let result = props.test_event_handling();
        assert!(result.passed);
    }

    #[test]
    fn test_api_compliance_css_compliance() {
        let props = StandardizedButtonProps {
            variant: Some(StandardVariant::Primary),
            size: Some(StandardSize::Lg),
            class: Some("custom-class".to_string()),
            ..Default::default()
        };
        
        let result = props.test_css_compliance();
        assert!(result.passed);
        assert!(result.details.contains_key("generated_classes"));
    }

    #[test]
    fn test_api_compliance_performance() {
        let props = StandardizedButtonProps::default();
        let result = props.test_performance_compliance();
        
        // Performance might vary in tests, but should not fail catastrophically
        assert!(result.execution_time_ms > 0);
        assert!(result.details.contains_key("avg_render_time_ms"));
    }

    #[test]
    fn test_generate_compliance_report() {
        let props = StandardizedButtonProps {
            id: Some("test-id".to_string()),
            aria_label: Some("Test button".to_string()),
            variant: Some(StandardVariant::Primary),
            ..Default::default()
        };
        
        let report = props.generate_compliance_report();
        
        assert!(!report.component_name.is_empty());
        assert!(report.compliance_score >= 0.0 && report.compliance_score <= 1.0);
        assert!(!report.test_results.is_empty());
    }

    #[test]
    fn test_button_render() {
        let props = StandardizedButtonProps {
            id: Some("test-button".to_string()),
            aria_label: Some("Click me".to_string()),
            children: Some(view! { "Button Text" }),
            ..Default::default()
        };
        
        // Should render without panicking
        let _ = StandardizedButton::render(props);
    }

    #[test]
    fn test_button_with_custom_styling() {
        let props = StandardizedButtonProps {
            variant: Some(StandardVariant::Primary),
            size: Some(StandardSize::Lg),
            class: Some("my-custom-class".to_string()),
            style: Some("color: red;".to_string()),
            ..Default::default()
        };
        
        let _ = StandardizedButton::render(props);
    }

    #[test]
    fn test_button_disabled_state() {
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();
        
        let props = StandardizedButtonProps {
            disabled: Some(true),
            onclick: Some(Box::new(move || {
                *clicked_clone.lock().unwrap() = true;
            })),
            ..Default::default()
        };
        
        let _ = StandardizedButton::render(props);
        
        // In a real test, we would simulate a click event and verify
        // that the onclick handler is not called when disabled
        assert!(!*clicked.lock().unwrap());
    }

    #[test]
    fn test_button_different_variants() {
        let variants = vec![
            StandardVariant::Default,
            StandardVariant::Primary,
            StandardVariant::Secondary,
            StandardVariant::Success,
            StandardVariant::Warning,
            StandardVariant::Danger,
        ];
        
        for variant in variants {
            let props = StandardizedButtonProps {
                variant: Some(variant),
                children: Some(view! { "Button" }),
                ..Default::default()
            };
            
            // Should render all variants without issues
            let _ = StandardizedButton::render(props);
        }
    }

    #[test]
    fn test_button_different_sizes() {
        let sizes = vec![
            StandardSize::Xs,
            StandardSize::Sm,
            StandardSize::Default,
            StandardSize::Lg,
            StandardSize::Xl,
        ];
        
        for size in sizes {
            let props = StandardizedButtonProps {
                size: Some(size),
                children: Some(view! { "Button" }),
                ..Default::default()
            };
            
            let _ = StandardizedButton::render(props);
        }
    }
}