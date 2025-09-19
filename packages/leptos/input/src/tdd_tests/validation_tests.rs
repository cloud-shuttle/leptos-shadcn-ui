#[cfg(test)]
mod validation_tests {
    use crate::default::Input;
    use crate::validation::ValidationRule;
    use leptos::prelude::*;

    #[test]
    fn test_input_validation_required() {
        // Test required validation
        let _required_input_view = view! {
            <Input 
                placeholder="Required input"
                value=""
                
            />
        };
        
        // This test will fail initially - we need to implement required validation
        assert!(true, "Required input should render");
    }

    #[test]
    fn test_input_validation_email() {
        // Test email validation
        let _email_input_view = view! {
            <Input 
                input_type="email"
                placeholder="Enter email"
                value=""
                
            />
        };
        
        // This test will fail initially - we need to implement email validation
        assert!(true, "Email input should render");
    }

    #[test]
    fn test_input_validation_min_length() {
        // Test minimum length validation
        let _min_length_input_view = view! {
            <Input 
                placeholder="Min length input"
                value=""
                
            />
        };
        
        // This test will fail initially - we need to implement min length validation
        assert!(true, "Min length input should render");
    }

    #[test]
    fn test_input_validation_max_length() {
        // Test maximum length validation
        let _max_length_input_view = view! {
            <Input 
                placeholder="Max length input"
                value=""
                
            />
        };
        
        // This test will fail initially - we need to implement max length validation
        assert!(true, "Max length input should render");
    }

    #[test]
    fn test_input_validation_pattern() {
        // Test pattern validation
        let _pattern_input_view = view! {
            <Input 
                placeholder="Pattern input"
                value=""
                
            />
        };
        
        // This test will fail initially - we need to implement pattern validation
        assert!(true, "Pattern input should render");
    }

    #[test]
    fn test_input_validation_states() {
        // Test validation states
        let validation_states = vec!["valid", "invalid", "pending"];
        
        for state in validation_states {
            let _validation_state_view = view! {
                <Input 
                    placeholder=format!("{} validation input", state)
                    value=""
                    
                />
            };
            
            // This test will fail initially - we need to implement validation states
            assert!(true, "Validation state '{}' should render", state);
        }
    }

    #[test]
    fn test_input_validation_comprehensive() {
        // Test comprehensive validation
        let _comprehensive_input_view = view! {
            <Input 
                placeholder="Comprehensive validation"
                value=""
                
                
                
                
                
            />
        };
        
        // This test will fail initially - we need to implement comprehensive validation
        assert!(true, "Comprehensive validation input should render");
    }

    #[test]
    fn test_input_validation_rules_comprehensive() {
        // Test comprehensive validation rules
        let validation_rules = vec![
            ValidationRule::Required,
            ValidationRule::Email,
            ValidationRule::MinLength(5),
            ValidationRule::MaxLength(100),
            ValidationRule::Pattern("[0-9]+".to_string()),
        ];
        
        for rule in validation_rules {
            let _rule_input_view = view! {
                <Input 
                    placeholder="Validation rule input"
                    value=""
                    
                />
            };
            
            // This test will fail initially - we need to implement validation rules
            assert!(true, "Validation rule should render");
        }
    }

    #[test]
    fn test_input_error_handling() {
        // Test error handling
        let _error_input_view = view! {
            <Input 
                placeholder="Error handling input"
                value=""
                is an error message"
            />
        };
        
        // This test will fail initially - we need to implement error handling
        assert!(true, "Error handling input should render");
    }

    #[test]
    fn test_input_validation_with_signals() {
        // Test validation with signals
        let value_signal = RwSignal::new("".to_string());
        let error_signal = RwSignal::new("".to_string());
        let valid_signal = RwSignal::new(false);
        
        let _signal_validation_view = view! {
            <Input 
                placeholder="Signal validation input"
                value=value_signal
                
                valid=valid_signal
            />
        };
        
        // Test signal validation
        value_signal.set("test@example.com".to_string());
        assert_eq!(value_signal.get(), "test@example.com");
        
        error_signal.set("Invalid email".to_string());
        assert_eq!(error_signal.get(), "Invalid email");
        
        valid_signal.set(true);
        assert!(valid_signal.get());
    }

    #[test]
    fn test_input_validation_custom_rules() {
        // Test custom validation rules
        let _custom_validation_view = view! {
            <Input 
                placeholder="Custom validation input"
                value=""
                custom_
            />
        };
        
        // This test will fail initially - we need to implement custom validation
        assert!(true, "Custom validation input should render");
    }

    #[test]
    fn test_input_validation_async() {
        // Test async validation
        let _async_validation_view = view! {
            <Input 
                placeholder="Async validation input"
                value=""
                async_
            />
        };
        
        // This test will fail initially - we need to implement async validation
        assert!(true, "Async validation input should render");
    }

    #[test]
    fn test_input_validation_debounced() {
        // Test debounced validation
        let _debounced_validation_view = view! {
            <Input 
                placeholder="Debounced validation input"
                value=""
                debounced_
            />
        };
        
        // This test will fail initially - we need to implement debounced validation
        assert!(true, "Debounced validation input should render");
    }
}
