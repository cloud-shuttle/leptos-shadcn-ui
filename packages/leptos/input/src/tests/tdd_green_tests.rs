//! TDD Pattern 2: GREEN - Make tests pass with minimal implementation
//! 
//! This module contains tests that verify the basic validation state management
//! and minimal implementation functionality.

#[cfg(test)]
mod tdd_green_tests {

    #[derive(Clone, Debug)]
    struct ValidationState {
        pub is_valid: bool,
        pub errors: Vec<String>,
    }
    
    impl ValidationState {
        fn new() -> Self {
            Self {
                is_valid: true,
                errors: Vec::new(),
            }
        }
        
        fn add_error(&mut self, error: String) {
            self.is_valid = false;
            self.errors.push(error);
        }
        
        fn clear_errors(&mut self) {
            self.is_valid = true;
            self.errors.clear();
        }
    }

    #[test]
    fn test_validation_state_creation() {
        let state = ValidationState::new();
        assert!(state.is_valid);
        assert!(state.errors.is_empty());
    }

    #[test]
    fn test_validation_state_add_error() {
        let mut state = ValidationState::new();
        state.add_error("Test error".to_string());
        
        assert!(!state.is_valid);
        assert_eq!(state.errors.len(), 1);
        assert_eq!(state.errors[0], "Test error");
    }

    #[test]
    fn test_validation_state_clear_errors() {
        let mut state = ValidationState::new();
        state.add_error("Test error".to_string());
        state.clear_errors();
        
        assert!(state.is_valid);
        assert!(state.errors.is_empty());
    }

    #[test]
    fn test_validation_state_multiple_errors() {
        let mut state = ValidationState::new();
        state.add_error("First error".to_string());
        state.add_error("Second error".to_string());
        
        assert!(!state.is_valid);
        assert_eq!(state.errors.len(), 2);
        assert!(state.errors.contains(&"First error".to_string()));
        assert!(state.errors.contains(&"Second error".to_string()));
    }
}
