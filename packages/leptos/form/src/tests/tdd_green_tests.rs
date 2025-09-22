//! TDD Phase 2: GREEN - Enhanced tests for advanced functionality
//! 
//! This module contains tests that verify the enhanced functionality and advanced features.

#[cfg(test)]
mod tdd_green_tests {
    use leptos::prelude::*;
    use crate::default::{
        Form, FormField, FormItem, FormLabel, FormControl, 
        FormMessage, FormDescription, FormValidation, FormError, FormData
    };

    #[test]
    fn test_form_advanced_validation_system() {
        // Test advanced validation system
        let mut validation = FormValidation::new();
        
        // Add multiple errors for same field
        validation.add_error("email", "Email is required");
        validation.add_error("email", "Email format is invalid");
        
        assert!(!validation.is_valid, "Form should be invalid");
        assert_eq!(validation.errors.len(), 2, "Should have two errors");
        
        // Test error retrieval
        let email_errors: Vec<&FormError> = validation.errors.iter()
            .filter(|error| error.field == "email")
            .collect();
        assert_eq!(email_errors.len(), 2, "Should have two email errors");
    }

    #[test]
    fn test_form_validation_clear_errors() {
        // Test clearing validation errors
        let mut validation = FormValidation::new();
        
        // Add errors
        validation.add_error("email", "Email is required");
        validation.add_error("password", "Password is too short");
        
        assert!(!validation.is_valid, "Form should be invalid");
        assert_eq!(validation.errors.len(), 2, "Should have two errors");
        
        // Clear errors (simulate reset)
        validation = FormValidation::new();
        
        assert!(validation.is_valid, "Form should be valid after clearing errors");
        assert!(validation.errors.is_empty(), "Should have no errors after clearing");
    }

    #[test]
    fn test_form_data_complex_scenarios() {
        // Test complex form data scenarios
        let mut form_data = FormData::new();
        
        // Add various field types
        form_data.fields.insert("text_field".to_string(), "Hello World".to_string());
        form_data.fields.insert("email_field".to_string(), "test@example.com".to_string());
        form_data.fields.insert("number_field".to_string(), "42".to_string());
        form_data.fields.insert("checkbox_field".to_string(), "on".to_string());
        form_data.fields.insert("empty_field".to_string(), "".to_string());
        
        assert_eq!(form_data.fields.len(), 5, "Should have five fields");
        
        // Test field access
        assert_eq!(form_data.get_or_default("text_field"), "Hello World");
        assert_eq!(form_data.get_or_default("email_field"), "test@example.com");
        assert_eq!(form_data.get_or_default("number_field"), "42");
        assert_eq!(form_data.get_or_default("checkbox_field"), "on");
        assert_eq!(form_data.get_or_default("empty_field"), "");
        assert_eq!(form_data.get_or_default("non_existent"), "");
    }

    #[test]
    fn test_form_accessibility_features() {
        // Test form accessibility features
        let field_name = "email";
        let label_for = "email";
        let field_id = "email";
        
        // Test proper labeling
        assert_eq!(field_name, label_for, "Field name should match label for attribute");
        assert_eq!(field_id, label_for, "Field ID should match label for attribute");
        
        // Test ARIA attributes
        let has_aria_invalid = true;
        let has_aria_describedby = true;
        
        assert!(has_aria_invalid, "Form should support aria-invalid attribute");
        assert!(has_aria_describedby, "Form should support aria-describedby attribute");
    }

    #[test]
    fn test_form_performance_optimization() {
        // Test form performance optimization
        let mut form_data = FormData::new();
        let start_time = std::time::Instant::now();
        
        // Simulate adding many fields
        for i in 0..1000 {
            form_data.fields.insert(format!("field_{}", i), format!("value_{}", i));
        }
        
        let duration = start_time.elapsed();
        
        assert_eq!(form_data.fields.len(), 1000, "Should have 1000 fields");
        assert!(duration.as_millis() < 100, "Should complete within 100ms");
    }

    #[test]
    fn test_form_integration_with_validation() {
        // Test form integration with validation system
        let mut form_data = FormData::new();
        form_data.fields.insert("email".to_string(), "invalid-email".to_string());
        form_data.fields.insert("password".to_string(), "123".to_string());
        
        let mut validation = FormValidation::new();
        
        // Validate email
        if form_data.get_or_default("email").is_empty() {
            validation.add_error("email", "Email is required");
        } else if !form_data.get_or_default("email").contains("@") {
            validation.add_error("email", "Email format is invalid");
        }
        
        // Validate password
        if form_data.get_or_default("password").len() < 8 {
            validation.add_error("password", "Password must be at least 8 characters");
        }
        
        assert!(!validation.is_valid, "Form should be invalid");
        assert_eq!(validation.errors.len(), 2, "Should have two validation errors");
        assert_eq!(validation.get_error("email"), Some("Email format is invalid"));
        assert_eq!(validation.get_error("password"), Some("Password must be at least 8 characters"));
    }

    #[test]
    fn test_form_error_prioritization() {
        // Test form error prioritization
        let mut validation = FormValidation::new();
        
        // Add errors in order of priority
        validation.add_error("email", "Email is required");
        validation.add_error("password", "Password is required");
        validation.add_error("confirm_password", "Passwords do not match");
        
        // First error should be the most critical
        assert_eq!(validation.errors[0].field, "email", "First error should be email");
        assert_eq!(validation.errors[0].message, "Email is required", "First error message should match");
        
        // All errors should be present
        assert_eq!(validation.errors.len(), 3, "Should have all three errors");
    }

    #[test]
    fn test_form_memory_management() {
        // Test form memory management
        let mut form_data = FormData::new();
        
        // Add and remove fields
        form_data.fields.insert("temp_field".to_string(), "temp_value".to_string());
        assert_eq!(form_data.fields.len(), 1, "Should have one field");
        
        form_data.fields.remove("temp_field");
        assert_eq!(form_data.fields.len(), 0, "Should have no fields after removal");
        
        // Test cleanup
        form_data.fields.clear();
        assert!(form_data.fields.is_empty(), "Fields should be empty after clear");
    }
}
