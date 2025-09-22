//! Validation rules implementation for the Input component
//! 
//! This module contains the core validation logic for different rule types.

use super::types::{ValidationRule, ValidationResult, ValidationError};

/// Validates a value against a specific validation rule
pub fn validate_rule(value: &str, rule: &ValidationRule) -> Result<(), ValidationError> {
    match rule {
        ValidationRule::Required => {
            if value.trim().is_empty() {
                Err(ValidationError {
                    field: String::new(),
                    message: "This field is required".to_string(),
                    rule: rule.clone(),
                })
            } else {
                Ok(())
            }
        }
        ValidationRule::MinLength(min_len) => {
            if value.len() < *min_len {
                Err(ValidationError {
                    field: String::new(),
                    message: format!("Must be at least {} characters long", min_len),
                    rule: rule.clone(),
                })
            } else {
                Ok(())
            }
        }
        ValidationRule::MaxLength(max_len) => {
            if value.len() > *max_len {
                Err(ValidationError {
                    field: String::new(),
                    message: format!("Must be no more than {} characters long", max_len),
                    rule: rule.clone(),
                })
            } else {
                Ok(())
            }
        }
        ValidationRule::Email => {
            if is_valid_email(value) {
                Ok(())
            } else {
                Err(ValidationError {
                    field: String::new(),
                    message: "Please enter a valid email address".to_string(),
                    rule: rule.clone(),
                })
            }
        }
        ValidationRule::Pattern(pattern) => {
            if matches_pattern(value, pattern) {
                Ok(())
            } else {
                Err(ValidationError {
                    field: String::new(),
                    message: "Please enter a valid format".to_string(),
                    rule: rule.clone(),
                })
            }
        }
        ValidationRule::Custom(_description) => {
            // For custom rules, we assume they're valid by default
            // In a real implementation, you'd call the custom validation function
            Ok(())
        }
    }
}

/// Validates a value against multiple rules
pub fn validate_rules(value: &str, rules: &[ValidationRule], field_name: &str) -> ValidationResult {
    let mut result = ValidationResult::new();
    
    for rule in rules {
        if let Err(mut error) = validate_rule(value, rule) {
            error.field = field_name.to_string();
            result.add_error(field_name, error.message.clone(), error.rule.clone());
        }
    }
    
    result
}

/// Simple email validation
fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() {
        return false;
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let (local, domain) = (parts[0], parts[1]);
    
    // Basic validation - local part and domain should not be empty
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    
    // Domain should contain at least one dot
    if !domain.contains('.') {
        return false;
    }
    
    // Domain should not start or end with a dot
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }
    
    true
}

/// Simple pattern matching (basic regex-like functionality)
fn matches_pattern(value: &str, pattern: &str) -> bool {
    // This is a simplified pattern matcher
    // In a real implementation, you'd use a proper regex library
    
    match pattern {
        "numeric" => value.chars().all(|c| c.is_ascii_digit()),
        "alpha" => value.chars().all(|c| c.is_ascii_alphabetic()),
        "alphanumeric" => value.chars().all(|c| c.is_ascii_alphanumeric()),
        "phone" => {
            // Simple phone number validation
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            digits.len() >= 10 && digits.len() <= 15
        }
        pattern if pattern.starts_with("^\\d{3}-\\d{2}-\\d{4}$") => {
            // SSN pattern: XXX-XX-XXXX
            if value.len() == 11 {
                let chars: Vec<char> = value.chars().collect();
                chars[3] == '-' && chars[6] == '-' &&
                chars[0..3].iter().all(|c| c.is_ascii_digit()) &&
                chars[4..6].iter().all(|c| c.is_ascii_digit()) &&
                chars[7..11].iter().all(|c| c.is_ascii_digit())
            } else {
                false
            }
        }
        pattern if pattern.starts_with("^(?=.*[A-Z])(?=.*[a-z])(?=.*\\d)") => {
            // Password pattern: at least one uppercase, one lowercase, one digit
            value.chars().any(|c| c.is_ascii_uppercase()) &&
            value.chars().any(|c| c.is_ascii_lowercase()) &&
            value.chars().any(|c| c.is_ascii_digit())
        }
        _ => {
            // For other patterns, do basic string matching
            value.contains(pattern)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_validation() {
        assert!(validate_rule("", &ValidationRule::Required).is_err());
        assert!(validate_rule("   ", &ValidationRule::Required).is_err());
        assert!(validate_rule("valid", &ValidationRule::Required).is_ok());
    }

    #[test]
    fn test_min_length_validation() {
        assert!(validate_rule("hi", &ValidationRule::MinLength(3)).is_err());
        assert!(validate_rule("hello", &ValidationRule::MinLength(3)).is_ok());
    }

    #[test]
    fn test_max_length_validation() {
        assert!(validate_rule("hello", &ValidationRule::MaxLength(3)).is_err());
        assert!(validate_rule("hi", &ValidationRule::MaxLength(3)).is_ok());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_rule("invalid", &ValidationRule::Email).is_err());
        assert!(validate_rule("test@example.com", &ValidationRule::Email).is_ok());
        assert!(validate_rule("user@domain.co.uk", &ValidationRule::Email).is_ok());
    }

    #[test]
    fn test_pattern_validation() {
        assert!(validate_rule("abc", &ValidationRule::Pattern("numeric".to_string())).is_err());
        assert!(validate_rule("123", &ValidationRule::Pattern("numeric".to_string())).is_ok());
    }

    #[test]
    fn test_multiple_rules_validation() {
        let rules = vec![
            ValidationRule::Required,
            ValidationRule::MinLength(3),
            ValidationRule::MaxLength(10),
        ];
        
        let result = validate_rules("", &rules, "test_field");
        assert!(!result.is_valid);
        assert!(result.has_field_error("test_field"));
        
        let result = validate_rules("valid", &rules, "test_field");
        assert!(result.is_valid);
    }
}
