//! DOM testing utilities for Leptos ShadCN UI components
//! 
//! This module provides basic DOM rendering test capabilities to complement
//! the unit tests. It uses wasm-bindgen-test for browser-based testing.

use leptos::prelude::*;
use leptos::mount::mount_to;
use wasm_bindgen_test::*;
use web_sys::wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);

/// A test harness for rendering Leptos components in a test environment
pub struct ComponentTestHarness {
    mount_point: String,
}

impl ComponentTestHarness {
    /// Create a new test harness with a unique mount point
    pub fn new() -> Self {
        let mount_id = format!("test-mount-{}", uuid::Uuid::new_v4().to_string());
        Self {
            mount_point: mount_id,
        }
    }
    
    /// Render a component and return the DOM element for testing
    pub fn render<F>(&self, component: F) -> web_sys::HtmlElement 
    where
        F: Fn() -> AnyView + 'static,
    {
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Create mount point
        let mount_element = document.create_element("div").unwrap();
        let mount_element = mount_element.dyn_into::<web_sys::HtmlElement>().unwrap();
        mount_element.set_id(&self.mount_point);
        document.body().unwrap().append_child(&mount_element).unwrap();
        
        // Mount the component
        let _dispose = mount_to(
            mount_element.clone(),
            component
        );
        
        // Store dispose function for cleanup (in real implementation)
        // For now, return the mount element
        mount_element
    }
    
    /// Helper to query for elements by CSS selector
    pub fn query_selector(&self, selector: &str) -> Option<web_sys::Element> {
        let document = web_sys::window().unwrap().document().unwrap();
        let mount_element = document.get_element_by_id(&self.mount_point)?;
        mount_element.query_selector(selector).unwrap_or(None)
    }
    
    /// Helper to get element text content
    pub fn get_text_content(&self, selector: &str) -> Option<String> {
        self.query_selector(selector)?.text_content()
    }
    
    /// Helper to check if element has specific class
    pub fn has_class(&self, selector: &str, class_name: &str) -> bool {
        if let Some(element) = self.query_selector(selector) {
            element.class_list().contains(class_name)
        } else {
            false
        }
    }
    
    /// Helper to get computed style
    pub fn get_computed_style(&self, selector: &str, property: &str) -> Option<String> {
        let element = self.query_selector(selector)?;
        let window = web_sys::window().unwrap();
        let computed_style = window.get_computed_style(&element).unwrap()?;
        computed_style.get_property_value(property).unwrap_or_default().into()
    }
    
    /// Cleanup the test harness
    pub fn cleanup(&self) {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(element) = document.get_element_by_id(&self.mount_point) {
                element.remove();
            }
        }
    }
}

impl Drop for ComponentTestHarness {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Test utilities for component accessibility
pub struct AccessibilityTester;

impl AccessibilityTester {
    /// Check if element has proper ARIA attributes
    pub fn check_aria_attributes(element: &web_sys::Element) -> Vec<String> {
        let mut missing_attributes = Vec::new();
        
        // Check for common ARIA attributes based on element type
        let tag_name = element.tag_name().to_lowercase();
        
        match tag_name.as_str() {
            "button" => {
                if !element.has_attribute("aria-label") && element.text_content().unwrap_or_default().is_empty() {
                    missing_attributes.push("aria-label or text content".to_string());
                }
            },
            "input" => {
                if !element.has_attribute("aria-label") && !element.has_attribute("aria-labelledby") {
                    missing_attributes.push("aria-label or aria-labelledby".to_string());
                }
            },
            _ => {}
        }
        
        missing_attributes
    }
    
    /// Check color contrast (simplified)
    pub fn check_color_contrast(element: &web_sys::Element) -> bool {
        // Simplified contrast check - in real implementation would use proper algorithms
        let window = web_sys::window().unwrap();
        if let Ok(Some(computed_style)) = window.get_computed_style(element) {
            let color = computed_style.get_property_value("color").unwrap_or_default();
            let background = computed_style.get_property_value("background-color").unwrap_or_default();
            
            // Basic check - ensure we have both color and background
            !color.is_empty() && !background.is_empty()
        } else {
            false
        }
    }
    
    /// Check keyboard navigation
    pub fn is_keyboard_accessible(element: &web_sys::Element) -> bool {
        // Check if element is focusable
        element.has_attribute("tabindex") || 
        matches!(element.tag_name().to_lowercase().as_str(), "button" | "input" | "select" | "textarea" | "a")
    }
}

/// Performance testing utilities
pub struct PerformanceTester;

impl PerformanceTester {
    /// Measure component render time
    pub fn measure_render_time<F>(render_fn: F) -> f64 
    where
        F: FnOnce(),
    {
        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();
        render_fn();
        let end = performance.now();
        end - start
    }
    
    /// Check bundle size impact (simplified)
    pub fn estimate_bundle_impact(component_name: &str) -> usize {
        // Simplified estimation - in real implementation would measure actual bundle sizes
        match component_name {
            "button" | "input" | "label" => 1024, // ~1KB
            "card" | "dialog" => 2048, // ~2KB
            "table" | "calendar" => 4096, // ~4KB
            _ => 1500, // Default estimation
        }
    }
}

/// Macro to create DOM tests more easily
#[macro_export]
macro_rules! dom_test {
    ($test_name:ident, $component:expr, $test_body:block) => {
        #[wasm_bindgen_test]
        fn $test_name() {
            let harness = ComponentTestHarness::new();
            let _element = harness.render(|| $component);
            
            $test_body
            
            // Cleanup is handled by Drop trait
        }
    };
}

/// Example usage and integration tests
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;
    
    // Example DOM test for Button component
    // Note: This would require the actual Button component to be imported
    #[wasm_bindgen_test]
    fn test_button_dom_rendering() {
        let harness = ComponentTestHarness::new();
        
        // This is a conceptual test - would need actual Button component
        let _element = harness.render(|| {
            view! {
                <button class="btn-primary">{"Test Button"}</button>
            }
        });
        
        // Test that button rendered correctly
        assert!(harness.query_selector("button").is_some());
        assert_eq!(harness.get_text_content("button"), Some("Test Button".to_string()));
        assert!(harness.has_class("button", "btn-primary"));
    }
    
    #[wasm_bindgen_test]
    fn test_accessibility_checking() {
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.create_element("button").unwrap();
        button.set_text_content(Some("Accessible Button"));
        
        let missing_attrs = AccessibilityTester::check_aria_attributes(&button);
        assert!(missing_attrs.is_empty(), "Button with text content should not need additional ARIA labels");
        
        assert!(AccessibilityTester::is_keyboard_accessible(&button));
    }
    
    #[wasm_bindgen_test]
    fn test_performance_measurement() {
        let render_time = PerformanceTester::measure_render_time(|| {
            // Simulate component rendering work
            for _ in 0..1000 {
                let _ = web_sys::window().unwrap().document().unwrap().create_element("div");
            }
        });
        
        assert!(render_time > 0.0, "Should measure some render time");
        assert!(render_time < 1000.0, "Render time should be reasonable (< 1 second)");
    }
}

/// Integration with existing TDD framework
impl crate::TestResult {
    /// Create a DOM test result
    pub fn dom_test(passed: bool, component: &str, test_type: &str, details: Option<String>) -> Self {
        let message = if passed {
            format!("✅ DOM test passed: {} {}", component, test_type)
        } else {
            format!("❌ DOM test failed: {} {}", component, test_type)
        };
        
        let mut result = if passed {
            Self::success(message)
        } else {
            Self::failure(message)
        };
        
        if let Some(details) = details {
            result = result.with_detail("details", details);
        }
        
        result.with_detail("test_type", "dom")
              .with_detail("component", component)
    }
}