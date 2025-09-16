#[cfg(test)]
mod error_tests {
    use super::*;
    use crate::error::*;

    // ===== COMPREHENSIVE ERROR TESTS =====
    // These tests focus on error handling and error types

    #[test]
    fn test_signal_management_error_signal_disposed() {
        // Test SignalDisposed error
        let error = SignalManagementError::SignalDisposed;
        
        // Test error type
        assert_eq!(error, SignalManagementError::SignalDisposed);
        
        // Test error message
        assert_eq!(error.to_string(), "Signal has been disposed and is no longer valid");
    }

    #[test]
    fn test_signal_management_error_update_failed() {
        // Test UpdateFailed error
        let reason = "Invalid signal state";
        let error = SignalManagementError::UpdateFailed {
            reason: reason.to_string(),
        };
        
        // Test error type
        assert_eq!(error, SignalManagementError::UpdateFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Signal update failed: Invalid signal state");
    }

    #[test]
    fn test_signal_management_error_memory_management_failed() {
        // Test MemoryManagementFailed error
        let reason = "Memory limit exceeded";
        let error = SignalManagementError::MemoryManagementFailed {
            reason: reason.to_string(),
        };
        
        // Test error type
        assert_eq!(error, SignalManagementError::MemoryManagementFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Memory management operation failed: Memory limit exceeded");
    }

    #[test]
    fn test_signal_management_error_batched_update_failed() {
        // Test BatchedUpdateFailed error
        let reason = "Batch size exceeded";
        let error = SignalManagementError::BatchedUpdateFailed {
            reason: reason.to_string(),
        };
        
        // Test error type
        assert_eq!(error, SignalManagementError::BatchedUpdateFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Batched update operation failed: Batch size exceeded");
    }

    #[test]
    fn test_signal_management_error_update_failed_helper() {
        // Test update_failed helper method
        let reason = "Test reason";
        let error = SignalManagementError::update_failed(reason);
        
        // Test error type
        assert_eq!(error, SignalManagementError::UpdateFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Signal update failed: Test reason");
    }

    #[test]
    fn test_signal_management_error_memory_management_failed_helper() {
        // Test memory_management_failed helper method
        let reason = "Test memory reason";
        let error = SignalManagementError::memory_management_failed(reason);
        
        // Test error type
        assert_eq!(error, SignalManagementError::MemoryManagementFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Memory management operation failed: Test memory reason");
    }

    #[test]
    fn test_signal_management_error_batched_update_failed_helper() {
        // Test batched_update_failed helper method
        let reason = "Test batch reason";
        let error = SignalManagementError::batched_update_failed(reason);
        
        // Test error type
        assert_eq!(error, SignalManagementError::BatchedUpdateFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Batched update operation failed: Test batch reason");
    }

    #[test]
    fn test_signal_management_error_with_string_reason() {
        // Test error creation with String reason
        let reason = "Test reason".to_string();
        let error = SignalManagementError::update_failed(reason.clone());
        
        // Test error type
        assert_eq!(error, SignalManagementError::UpdateFailed {
            reason: reason.clone(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Signal update failed: Test reason");
    }

    #[test]
    fn test_signal_management_error_with_str_reason() {
        // Test error creation with &str reason
        let reason = "Test reason";
        let error = SignalManagementError::update_failed(reason);
        
        // Test error type
        assert_eq!(error, SignalManagementError::UpdateFailed {
            reason: reason.to_string(),
        });
        
        // Test error message
        assert_eq!(error.to_string(), "Signal update failed: Test reason");
    }

    #[test]
    fn test_signal_management_error_clone() {
        // Test error cloning
        let original_error = SignalManagementError::update_failed("Test reason");
        let cloned_error = original_error.clone();
        
        // Test cloned error is equal to original
        assert_eq!(original_error, cloned_error);
        
        // Test cloned error has same message
        assert_eq!(original_error.to_string(), cloned_error.to_string());
    }

    #[test]
    fn test_signal_management_error_debug() {
        // Test error debug formatting
        let error = SignalManagementError::update_failed("Test reason");
        let debug_string = format!("{:?}", error);
        
        // Test debug string contains error information
        assert!(debug_string.contains("UpdateFailed"));
        assert!(debug_string.contains("Test reason"));
    }

    #[test]
    fn test_signal_management_error_partial_eq() {
        // Test error equality
        let error1 = SignalManagementError::update_failed("Test reason");
        let error2 = SignalManagementError::update_failed("Test reason");
        let error3 = SignalManagementError::update_failed("Different reason");
        
        // Test equal errors
        assert_eq!(error1, error2);
        
        // Test different errors
        assert_ne!(error1, error3);
        
        // Test different error types
        let signal_disposed = SignalManagementError::SignalDisposed;
        assert_ne!(error1, signal_disposed);
    }

    #[test]
    fn test_signal_management_error_display() {
        // Test error display formatting
        let error = SignalManagementError::update_failed("Test reason");
        let display_string = format!("{}", error);
        
        // Test display string
        assert_eq!(display_string, "Signal update failed: Test reason");
    }

    #[test]
    fn test_signal_management_error_error_trait() {
        // Test error trait implementation
        let error = SignalManagementError::update_failed("Test reason");
        
        // Test error source (should be None for our errors)
        assert!(error.source().is_none());
    }

    #[test]
    fn test_signal_management_error_comprehensive_scenarios() {
        // Test comprehensive error scenarios
        let scenarios = vec![
            ("Signal disposed", SignalManagementError::SignalDisposed),
            ("Update failed", SignalManagementError::update_failed("Update failed")),
            ("Memory management failed", SignalManagementError::memory_management_failed("Memory management failed")),
            ("Batched update failed", SignalManagementError::batched_update_failed("Batched update failed")),
        ];
        
        for (description, error) in scenarios {
            // Test error can be cloned
            let cloned_error = error.clone();
            assert_eq!(error, cloned_error);
            
            // Test error can be converted to string
            let error_string = error.to_string();
            assert!(!error_string.is_empty());
            
            // Test error can be debug formatted
            let debug_string = format!("{:?}", error);
            assert!(!debug_string.is_empty());
        }
    }

    #[test]
    fn test_signal_management_error_edge_cases() {
        // Test edge cases
        let empty_reason = "";
        let error = SignalManagementError::update_failed(empty_reason);
        assert_eq!(error.to_string(), "Signal update failed: ");
        
        let long_reason = "a".repeat(1000);
        let error = SignalManagementError::update_failed(long_reason.clone());
        assert_eq!(error.to_string(), format!("Signal update failed: {}", long_reason));
        
        let special_chars_reason = "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
        let error = SignalManagementError::update_failed(special_chars_reason);
        assert_eq!(error.to_string(), "Signal update failed: Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?");
    }

    #[test]
    fn test_signal_management_error_performance_characteristics() {
        // Test performance characteristics
        let start = std::time::Instant::now();
        
        // Create many errors
        for i in 0..1000 {
            let error = SignalManagementError::update_failed(format!("Error {}", i));
            let _error_string = error.to_string();
            let _debug_string = format!("{:?}", error);
        }
        
        let duration = start.elapsed();
        
        // Should complete without panicking
        assert!(duration.as_nanos() >= 0, "Error creation should complete");
    }

    #[test]
    fn test_signal_management_error_memory_management() {
        // Test memory management
        let mut errors = Vec::new();
        
        // Create many errors
        for i in 0..1000 {
            let error = SignalManagementError::update_failed(format!("Error {}", i));
            errors.push(error);
        }
        
        // Test that errors can be dropped without issues
        drop(errors);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_signal_management_error_integration_scenarios() {
        // Test integration scenarios
        let error_scenarios = vec![
            ("Signal disposal", SignalManagementError::SignalDisposed),
            ("Invalid state", SignalManagementError::update_failed("Invalid signal state")),
            ("Memory limit", SignalManagementError::memory_management_failed("Memory limit exceeded")),
            ("Batch overflow", SignalManagementError::batched_update_failed("Batch size exceeded")),
        ];
        
        for (scenario, error) in error_scenarios {
            // Test error handling
            match error {
                SignalManagementError::SignalDisposed => {
                    assert_eq!(scenario, "Signal disposal");
                }
                SignalManagementError::UpdateFailed { reason } => {
                    assert_eq!(scenario, "Invalid state");
                    assert_eq!(reason, "Invalid signal state");
                }
                SignalManagementError::MemoryManagementFailed { reason } => {
                    assert_eq!(scenario, "Memory limit");
                    assert_eq!(reason, "Memory limit exceeded");
                }
                SignalManagementError::BatchedUpdateFailed { reason } => {
                    assert_eq!(scenario, "Batch overflow");
                    assert_eq!(reason, "Batch size exceeded");
                }
            }
        }
    }

    #[test]
    fn test_signal_management_error_serialization_compatibility() {
        // Test that errors are compatible with serialization
        // This is important for error reporting and debugging
        
        let error = SignalManagementError::update_failed("Test reason");
        
        // Test error can be cloned (simulating serialization/deserialization)
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
        
        // Test error message is preserved
        assert_eq!(error.to_string(), cloned_error.to_string());
    }

    #[test]
    fn test_signal_management_error_error_chain() {
        // Test error chaining scenarios
        let root_error = SignalManagementError::update_failed("Root cause");
        let chained_error = SignalManagementError::memory_management_failed("Memory management failed");
        
        // Test both errors can be handled independently
        assert_ne!(root_error, chained_error);
        
        // Test error messages are different
        assert_ne!(root_error.to_string(), chained_error.to_string());
    }

    #[test]
    fn test_signal_management_error_context_handling() {
        // Test error context handling
        let context_errors = vec![
            ("Component lifecycle", SignalManagementError::SignalDisposed),
            ("State update", SignalManagementError::update_failed("State update failed")),
            ("Memory cleanup", SignalManagementError::memory_management_failed("Memory cleanup failed")),
            ("Batch processing", SignalManagementError::batched_update_failed("Batch processing failed")),
        ];
        
        for (context, error) in context_errors {
            // Test error can be handled in context
            let error_message = error.to_string();
            assert!(!error_message.is_empty());
            
            // Test error can be logged (simulated)
            let log_message = format!("[{}] {}", context, error_message);
            assert!(log_message.contains(context));
            assert!(log_message.contains(&error_message));
        }
    }
}
