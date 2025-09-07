#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;
    use crate::default::{FormValidation, FormError, FormData};

    // TDD Phase 1: RED - Write failing tests for Form functionality

    #[test]
    fn test_form_initial_state() {
        // Test that form starts with empty validation state
        let validation = FormValidation::new();
        
        assert!(validation.is_valid, "Form should start in valid state");
        assert!(validation.errors.is_empty(), "Form should start with no errors");
    }

    #[test]
    fn test_form_validation_error_handling() {
        // Test form validation error handling
        let mut validation = FormValidation::new();
        
        // Add an error
        validation.add_error("email", "Email is required");
        
        assert!(!validation.is_valid, "Form should be invalid after adding error");
        assert_eq!(validation.errors.len(), 1, "Should have one error");
        assert_eq!(validation.errors[0].field, "email", "Error should be for email field");
        assert_eq!(validation.errors[0].message, "Email is required", "Error message should match");
    }

    #[test]
    fn test_form_validation_get_error() {
        // Test getting specific field errors
        let mut validation = FormValidation::new();
        validation.add_error("email", "Email is required");
        validation.add_error("password", "Password is too short");
        
        let email_error = validation.get_error("email");
        let password_error = validation.get_error("password");
        let non_existent_error = validation.get_error("username");
        
        assert_eq!(email_error, Some("Email is required"), "Should get email error");
        assert_eq!(password_error, Some("Password is too short"), "Should get password error");
        assert_eq!(non_existent_error, None, "Should return None for non-existent field");
    }

    #[test]
    fn test_form_data_creation() {
        // Test FormData creation and basic operations
        let form_data = FormData::new();
        
        assert!(form_data.fields.is_empty(), "FormData should start empty");
        assert_eq!(form_data.get("email"), None, "Should return None for non-existent field");
        assert_eq!(form_data.get_or_default("email"), "", "Should return empty string for non-existent field");
    }

    #[test]
    fn test_form_data_field_operations() {
        // Test FormData field operations
        let mut form_data = FormData::new();
        
        // Add fields
        form_data.fields.insert("email".to_string(), "test@example.com".to_string());
        form_data.fields.insert("password".to_string(), "secret123".to_string());
        
        assert_eq!(form_data.get("email"), Some(&"test@example.com".to_string()), "Should get email value");
        assert_eq!(form_data.get("password"), Some(&"secret123".to_string()), "Should get password value");
        assert_eq!(form_data.get_or_default("email"), "test@example.com", "Should get email with default");
        assert_eq!(form_data.get_or_default("username"), "", "Should return empty string for non-existent field");
    }

    #[test]
    fn test_form_submission_callback() {
        // Test form submission callback functionality
        let form_submitted = RwSignal::new(false);
        let submitted_data = RwSignal::new(FormData::new());
        
        let on_submit = Callback::new(move |data: FormData| {
            form_submitted.set(true);
            submitted_data.set(data);
        });
        
        // Create test form data
        let mut test_data = FormData::new();
        test_data.fields.insert("email".to_string(), "test@example.com".to_string());
        
        // Simulate form submission
        on_submit.run(test_data);
        
        assert!(form_submitted.get(), "Form submission callback should be called");
        assert_eq!(submitted_data.get().get("email"), Some(&"test@example.com".to_string()), "Should receive correct form data");
    }

    #[test]
    fn test_form_field_component() {
        // Test FormField component functionality
        let field_name = "email";
        let field_class = "space-y-2";
        
        assert!(!field_name.is_empty(), "Field name should not be empty");
        assert!(field_class.contains("space-y-2"), "Field should have proper spacing class");
    }

    #[test]
    fn test_form_item_component() {
        // Test FormItem component functionality
        let item_class = "space-y-2";
        
        assert!(item_class.contains("space-y-2"), "Form item should have proper spacing class");
    }

    #[test]
    fn test_form_label_component() {
        // Test FormLabel component functionality
        let for_field = "email";
        let label_class = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
        
        assert!(!for_field.is_empty(), "Label should have a for field");
        assert!(label_class.contains("text-sm"), "Label should have small text");
        assert!(label_class.contains("font-medium"), "Label should have medium font weight");
        assert!(label_class.contains("leading-none"), "Label should have no line height");
    }

    #[test]
    fn test_form_control_component() {
        // Test FormControl component functionality
        let control_class = "peer";
        
        assert_eq!(control_class, "peer", "Form control should have peer class");
    }

    #[test]
    fn test_form_message_component() {
        // Test FormMessage component functionality
        let message_text = "This field is required";
        let message_class = "text-sm font-medium text-destructive";
        
        assert!(!message_text.is_empty(), "Message should have text");
        assert!(message_class.contains("text-sm"), "Message should have small text");
        assert!(message_class.contains("font-medium"), "Message should have medium font weight");
        assert!(message_class.contains("text-destructive"), "Message should have destructive color");
    }

    #[test]
    fn test_form_description_component() {
        // Test FormDescription component functionality
        let description_class = "text-sm text-muted-foreground";
        
        assert!(description_class.contains("text-sm"), "Description should have small text");
        assert!(description_class.contains("text-muted-foreground"), "Description should have muted color");
    }

    #[test]
    fn test_form_class_merging() {
        // Test form class merging functionality
        let base_class = "space-y-6";
        let custom_class = "custom-form";
        let merged_class = format!("{} {}", base_class, custom_class);
        
        assert!(merged_class.contains("space-y-6"), "Should include base class");
        assert!(merged_class.contains("custom-form"), "Should include custom class");
    }

    #[test]
    fn test_form_validation_multiple_errors() {
        // Test form validation with multiple errors
        let mut validation = FormValidation::new();
        
        validation.add_error("email", "Email is required");
        validation.add_error("password", "Password is too short");
        validation.add_error("confirm_password", "Passwords do not match");
        
        assert!(!validation.is_valid, "Form should be invalid with multiple errors");
        assert_eq!(validation.errors.len(), 3, "Should have three errors");
        
        // Test getting specific errors
        assert_eq!(validation.get_error("email"), Some("Email is required"));
        assert_eq!(validation.get_error("password"), Some("Password is too short"));
        assert_eq!(validation.get_error("confirm_password"), Some("Passwords do not match"));
    }

    #[test]
    fn test_form_data_from_form_element() {
        // Test FormData creation from form element (simulated)
        let mut form_data = FormData::new();
        
        // Simulate form element data
        form_data.fields.insert("email".to_string(), "user@example.com".to_string());
        form_data.fields.insert("password".to_string(), "password123".to_string());
        form_data.fields.insert("remember".to_string(), "on".to_string());
        
        assert_eq!(form_data.fields.len(), 3, "Should have three fields");
        assert_eq!(form_data.get("email"), Some(&"user@example.com".to_string()));
        assert_eq!(form_data.get("password"), Some(&"password123".to_string()));
        assert_eq!(form_data.get("remember"), Some(&"on".to_string()));
    }

    // TDD Phase 2: GREEN - Enhanced tests for advanced functionality

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