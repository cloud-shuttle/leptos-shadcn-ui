#[cfg(test)]
mod performance_tests {
    use crate::validation::{ValidationResult, ValidationRule, InputValidator, validation_builders};
    use leptos::prelude::*;

    #[test]
    fn test_validation_performance_characteristics() {
        // Test validation performance characteristics
        
        // Test rapid signal updates
        let value_signal = RwSignal::new("initial".to_string());
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            value_signal.set(format!("value_{}", i));
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test validation performance
        let validator = InputValidator::new("test")
            .required()
            .min_length(5)
            .max_length(100)
            .email();
        
        let start_validation = std::time::Instant::now();
        
        for i in 0..100 {
            let test_value = format!("test{}@example.com", i);
            let _result = validator.validate(&test_value);
        }
        
        let validation_duration = start_validation.elapsed();
        assert!(validation_duration.as_millis() < 50); // Should be very fast
    }

    #[test]
    fn test_validation_memory_management() {
        // Test memory management
        
        // Test signal cleanup
        let value_signal = RwSignal::new("test".to_string());
        let initial_value = value_signal.get();
        assert_eq!(initial_value, "test");
        
        // Test large string handling
        let large_string = "x".repeat(100000);
        value_signal.set(large_string.clone());
        assert_eq!(value_signal.get(), large_string);
        
        // Test memory cleanup by setting smaller value
        value_signal.set("small".to_string());
        assert_eq!(value_signal.get(), "small");
        
        // Test validation result memory
        let mut result = ValidationResult::new();
        for i in 0..100 {
            result.add_error(&format!("field_{}", i), &format!("error_{}", i), ValidationRule::Required);
        }
        
        assert_eq!(result.errors.len(), 100);
        result.clear_errors();
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_validation_batch_performance() {
        // Test batch validation performance
        let validators = vec![
            validation_builders::email_validator("email"),
            validation_builders::password_validator("password"),
            validation_builders::username_validator("username"),
        ];
        
        let test_data = vec![
            ("email", "user@example.com"),
            ("password", "Password123"),
            ("username", "user123"),
        ];
        
        let start = std::time::Instant::now();
        
        // Test batch validation
        for _ in 0..1000 {
            for (field, value) in &test_data {
                if let Some(validator) = validators.iter().find(|v| v.field_name == *field) {
                    let _result = validator.validate(value);
                }
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200); // Should be fast for batch operations
    }

    #[test]
    fn test_validation_memoization_performance() {
        // Test memoization performance
        let value_signal = RwSignal::new("".to_string());
        
        // Test memoized validation
        let memoized_validation = Memo::new(move |_| {
            let value = value_signal.get();
            let mut result = ValidationResult::new();
            
            if value.is_empty() {
                result.add_error("field", "Required", ValidationRule::Required);
            } else if value.len() < 3 {
                result.add_error("field", "Too short", ValidationRule::MinLength(3));
            }
            
            result
        });
        
        let start = std::time::Instant::now();
        
        // Test memoization performance
        for i in 0..1000 {
            value_signal.set(format!("test_{}", i));
            let _result = memoized_validation.get();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast with memoization
    }

    #[test]
    fn test_validation_signal_batching_performance() {
        // Test signal batching performance
        let value_signal = RwSignal::new("".to_string());
        let validation_signal = RwSignal::new(ValidationResult::new());
        let is_validating_signal = RwSignal::new(false);
        
        let start = std::time::Instant::now();
        
        // Test sequential updates
        for i in 0..1000 {
            value_signal.set(format!("test_{}", i));
            is_validating_signal.set(i % 2 == 0);
            
            let mut result = ValidationResult::new();
            if i % 3 == 0 {
                result.add_error("field", "Test error", ValidationRule::Required);
            }
            validation_signal.set(result);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 150); // Should be fast with batching
    }

    #[test]
    fn test_validation_large_dataset_performance() {
        // Test performance with large datasets
        let mut validators = std::collections::HashMap::new();
        
        // Create many validators
        for i in 0..1000 {
            let field_name = format!("field_{}", i);
            let validator = InputValidator::new(&field_name)
                .required()
                .min_length(3)
                .max_length(50);
            validators.insert(field_name, validator);
        }
        
        let start = std::time::Instant::now();
        
        // Test validation with many fields
        for i in 0..1000 {
            let field_name = format!("field_{}", i);
            let value = format!("value_{}", i);
            
            if let Some(validator) = validators.get(&field_name) {
                let _result = validator.validate(&value);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500); // Should be reasonable for large dataset
    }

    #[test]
    fn test_validation_error_accumulation_performance() {
        // Test error accumulation performance
        let validator = InputValidator::new("test")
            .required()
            .min_length(5)
            .max_length(20)
            .pattern(r"^[a-zA-Z0-9]+$".to_string());
        
        let start = std::time::Instant::now();
        
        // Test error accumulation
        for i in 0..1000 {
            let test_value = if i % 2 == 0 { "" } else { "ab" };
            let result = validator.validate(test_value);
            
            // Simulate error accumulation
            let mut accumulated = ValidationResult::new();
            if !result.is_valid {
                accumulated.errors.extend(result.errors);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
    }

    #[test]
    fn test_validation_concurrent_performance() {
        // Test concurrent validation performance
        let validators = vec![
            validation_builders::email_validator("email"),
            validation_builders::password_validator("password"),
            validation_builders::username_validator("username"),
        ];
        
        let test_data = vec![
            ("email", "user@example.com"),
            ("password", "Password123"),
            ("username", "user123"),
        ];
        
        let start = std::time::Instant::now();
        
        // Test concurrent-like validation
        for _ in 0..100 {
            for (field, value) in &test_data {
                if let Some(validator) = validators.iter().find(|v| v.field_name == *field) {
                    let _result = validator.validate(value);
                }
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be fast
    }

    #[test]
    fn test_validation_memory_cleanup_performance() {
        // Test memory cleanup performance
        let start = std::time::Instant::now();
        
        // Test memory cleanup
        for _ in 0..100 {
            let mut result = ValidationResult::new();
            
            // Add many errors
            for i in 0..100 {
                result.add_error(&format!("field_{}", i), &format!("error_{}", i), ValidationRule::Required);
            }
            
            // Clear errors
            result.clear_errors();
            
            // Drop result
            drop(result);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
    }

    #[test]
    fn test_validation_string_handling_performance() {
        // Test string handling performance
        let validator = InputValidator::new("test").required();
        
        let start = std::time::Instant::now();
        
        // Test string handling
        for i in 0..1000 {
            let test_string = format!("test_string_{}", i);
            let _result = validator.validate(&test_string);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should be very fast
    }
}
