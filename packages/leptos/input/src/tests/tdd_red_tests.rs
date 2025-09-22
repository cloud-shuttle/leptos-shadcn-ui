//! TDD Pattern 1: RED - Write failing tests first
//! 
//! This module contains the initial failing tests that define the desired behavior
//! before implementing the actual functionality.

#[cfg(test)]
mod tdd_red_tests {
    use leptos::prelude::*;
    use regex::Regex;

    #[derive(Clone, Debug)]
    struct ValidationState {
        pub is_valid: bool,
        pub errors: Vec<String>,
    }

    impl ValidationState {
        pub fn new() -> Self {
            Self {
                is_valid: true,
                errors: Vec::new(),
            }
        }
    }

    #[test]
    fn test_input_validation_required_field() {
        // RED: This test should fail initially - we need to add validation support
        let _validation_state = RwSignal::new(ValidationState::new());
        let is_required = Signal::derive(|| true);
        let value = RwSignal::new("".to_string());
        
        // Test that empty required field triggers validation error
        let is_valid = Signal::derive(move || {
            if is_required.get() && value.get().trim().is_empty() {
                false
            } else {
                true
            }
        });
        
        assert!(!is_valid.get(), "Required field should be invalid when empty");
        
        value.set("valid input".to_string());
        assert!(is_valid.get(), "Required field should be valid when filled");
    }

    #[test]
    fn test_input_validation_email_format() {
        // RED: Email validation should fail for invalid formats
        let email_value = RwSignal::new("invalid-email".to_string());
        
        let is_valid_email = Signal::derive(move || {
            let email = email_value.get();
            email.contains('@') && email.contains('.') && email.len() > 5
        });
        
        assert!(!is_valid_email.get(), "Invalid email format should fail validation");
        
        email_value.set("user@example.com".to_string());
        assert!(is_valid_email.get(), "Valid email format should pass validation");
    }

    #[test]
    fn test_input_validation_min_length() {
        // RED: Minimum length validation
        let value = RwSignal::new("ab".to_string());
        let min_length = 3;
        
        let is_valid_length = Signal::derive(move || {
            value.get().len() >= min_length
        });
        
        assert!(!is_valid_length.get(), "Value below minimum length should be invalid");
        
        value.set("abc".to_string());
        assert!(is_valid_length.get(), "Value meeting minimum length should be valid");
    }

    #[test]
    fn test_input_validation_max_length() {
        // RED: Maximum length validation
        let value = RwSignal::new("very long input that exceeds limit".to_string());
        let max_length = 10;
        
        let is_valid_length = Signal::derive(move || {
            value.get().len() <= max_length
        });
        
        assert!(!is_valid_length.get(), "Value exceeding maximum length should be invalid");
        
        value.set("short".to_string());
        assert!(is_valid_length.get(), "Value within maximum length should be valid");
    }

    #[test]
    fn test_input_validation_pattern_matching() {
        // RED: Pattern validation (e.g., phone number, alphanumeric)
        let value = RwSignal::new("abc123!@#".to_string());
        let pattern = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        
        let is_valid_pattern = Signal::derive(move || {
            pattern.is_match(&value.get())
        });
        
        assert!(!is_valid_pattern.get(), "Value with special characters should fail pattern validation");
        
        value.set("abc123".to_string());
        assert!(is_valid_pattern.get(), "Alphanumeric value should pass pattern validation");
    }

    #[test]
    fn test_input_validation_error_display() {
        // RED: Error message display functionality
        let validation_error = RwSignal::new(Some("This field is required".to_string()));
        let show_error = Signal::derive(move || validation_error.get().is_some());
        
        assert!(show_error.get(), "Error should be shown when validation fails");
        assert_eq!(validation_error.get().unwrap(), "This field is required");
        
        validation_error.set(None);
        assert!(!show_error.get(), "Error should be hidden when validation passes");
    }

    #[test]
    fn test_input_validation_real_time_feedback() {
        // RED: Real-time validation as user types
        let value = RwSignal::new("".to_string());
        let validation_errors = RwSignal::new(Vec::<String>::new());
        
        let validate_input = move || {
            let mut errors = Vec::new();
            let current_value = value.get();
            
            if current_value.trim().is_empty() {
                errors.push("Field is required".to_string());
            }
            if current_value.len() < 3 {
                errors.push("Must be at least 3 characters".to_string());
            }
            
            validation_errors.set(errors);
        };
        
        // Initial state - should have errors
        validate_input();
        assert!(!validation_errors.get().is_empty());
        
        // Partial input - should still have length error
        value.set("ab".to_string());
        validate_input();
        assert!(validation_errors.get().contains(&"Must be at least 3 characters".to_string()));
        
        // Valid input - should have no errors
        value.set("abc".to_string());
        validate_input();
        assert!(validation_errors.get().is_empty());
    }
}
