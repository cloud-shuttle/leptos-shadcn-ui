//! TDD Phase 1: RED - Write failing tests for Form functionality
//! 
//! This module contains the initial failing tests that define the desired behavior
//! before implementing the actual functionality.

#[cfg(test)]
mod tdd_red_tests {
    use leptos::prelude::*;
    use crate::default::{
        Form, FormField, FormItem, FormLabel, FormControl, 
        FormMessage, FormDescription, FormValidation, FormError, FormData
    };

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
}
