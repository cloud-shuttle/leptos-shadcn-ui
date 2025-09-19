#[cfg(test)]
mod error_tests {
    use crate::*;

    #[test]
    fn test_signal_management_error_types() {
        // Test SignalManagementError types
        let signal_error = SignalManagementError::SignalError("Test signal error".to_string());
        let memo_error = SignalManagementError::MemoError("Test memo error".to_string());
        let cleanup_error = SignalManagementError::CleanupError("Test cleanup error".to_string());
        let memory_error = SignalManagementError::MemoryError("Test memory error".to_string());
        let batch_error = SignalManagementError::BatchError("Test batch error".to_string());
        
        // Test error type matching
        match signal_error {
            SignalManagementError::SignalError(_) => assert!(true),
            _ => assert!(false, "Expected SignalError"),
        }
        
        match memo_error {
            SignalManagementError::MemoError(_) => assert!(true),
            _ => assert!(false, "Expected MemoError"),
        }
        
        match cleanup_error {
            SignalManagementError::CleanupError(_) => assert!(true),
            _ => assert!(false, "Expected CleanupError"),
        }
        
        match memory_error {
            SignalManagementError::MemoryError(_) => assert!(true),
            _ => assert!(false, "Expected MemoryError"),
        }
        
        match batch_error {
            SignalManagementError::BatchError(_) => assert!(true),
            _ => assert!(false, "Expected BatchError"),
        }
    }

    #[test]
    fn test_signal_management_error_messages() {
        // Test error messages
        let error_message = "Test error message";
        let signal_error = SignalManagementError::SignalError(error_message.to_string());
        let memo_error = SignalManagementError::MemoError(error_message.to_string());
        let cleanup_error = SignalManagementError::CleanupError(error_message.to_string());
        let memory_error = SignalManagementError::MemoryError(error_message.to_string());
        let batch_error = SignalManagementError::BatchError(error_message.to_string());
        
        // Test error message extraction
        match signal_error {
            SignalManagementError::SignalError(msg) => assert_eq!(msg, error_message),
            _ => assert!(false, "Expected SignalError"),
        }
        
        match memo_error {
            SignalManagementError::MemoError(msg) => assert_eq!(msg, error_message),
            _ => assert!(false, "Expected MemoError"),
        }
        
        match cleanup_error {
            SignalManagementError::CleanupError(msg) => assert_eq!(msg, error_message),
            _ => assert!(false, "Expected CleanupError"),
        }
        
        match memory_error {
            SignalManagementError::MemoryError(msg) => assert_eq!(msg, error_message),
            _ => assert!(false, "Expected MemoryError"),
        }
        
        match batch_error {
            SignalManagementError::BatchError(msg) => assert_eq!(msg, error_message),
            _ => assert!(false, "Expected BatchError"),
        }
    }

    #[test]
    fn test_signal_management_error_clone() {
        // Test error cloning
        let original_error = SignalManagementError::SignalError("Test error".to_string());
        let cloned_error = original_error.clone();
        
        // Test cloning
        assert_eq!(original_error, cloned_error);
        
        // Test that cloned error has same message
        match (original_error, cloned_error) {
            (SignalManagementError::SignalError(msg1), SignalManagementError::SignalError(msg2)) => {
                assert_eq!(msg1, msg2);
            }
            _ => assert!(false, "Expected SignalError for both"),
        }
    }

    #[test]
    fn test_signal_management_error_debug() {
        // Test error debug formatting
        let error = SignalManagementError::SignalError("Test error".to_string());
        let debug_str = format!("{:?}", error);
        
        // Test debug string contains error type and message
        assert!(debug_str.contains("SignalError"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_signal_management_error_partial_eq() {
        // Test error equality
        let error1 = SignalManagementError::SignalError("Test error".to_string());
        let error2 = SignalManagementError::SignalError("Test error".to_string());
        let error3 = SignalManagementError::SignalError("Different error".to_string());
        let error4 = SignalManagementError::MemoError("Test error".to_string());
        
        // Test equal errors
        assert_eq!(error1, error2);
        
        // Test different messages
        assert_ne!(error1, error3);
        
        // Test different types
        assert_ne!(error1, error4);
    }

    #[test]
    fn test_signal_management_error_display() {
        // Test error display formatting
        let error = SignalManagementError::SignalError("Test error".to_string());
        let display_str = format!("{}", error);
        
        // Test display string contains error message
        assert!(display_str.contains("Test error"));
    }

    #[test]
    fn test_signal_management_error_all_types() {
        // Test all error types
        let error_types = vec![
            SignalManagementError::SignalError("signal error".to_string()),
            SignalManagementError::MemoError("memo error".to_string()),
            SignalManagementError::CleanupError("cleanup error".to_string()),
            SignalManagementError::MemoryError("memory error".to_string()),
            SignalManagementError::BatchError("batch error".to_string()),
        ];
        
        // Test that all error types are unique
        for (i, error1) in error_types.iter().enumerate() {
            for (j, error2) in error_types.iter().enumerate() {
                if i == j {
                    assert_eq!(error1, error2);
                } else {
                    assert_ne!(error1, error2);
                }
            }
        }
    }

    #[test]
    fn test_signal_management_error_long_messages() {
        // Test error with long message
        let long_message = "This is a very long error message that contains multiple words and should be handled properly by the error system without any issues or truncation";
        let error = SignalManagementError::SignalError(long_message.to_string());
        
        // Test error message is preserved
        match error {
            SignalManagementError::SignalError(msg) => assert_eq!(msg, long_message),
            _ => assert!(false, "Expected SignalError"),
        }
        
        // Test debug formatting with long message
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains(long_message));
        
        // Test display formatting with long message
        let display_str = format!("{}", error);
        assert!(display_str.contains(long_message));
    }

    #[test]
    fn test_signal_management_error_special_characters() {
        // Test error with special characters
        let special_message = "Error with special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
        let error = SignalManagementError::SignalError(special_message.to_string());
        
        // Test error message is preserved
        match error {
            SignalManagementError::SignalError(msg) => assert_eq!(msg, special_message),
            _ => assert!(false, "Expected SignalError"),
        }
        
        // Test debug formatting with special characters
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains(special_message));
        
        // Test display formatting with special characters
        let display_str = format!("{}", error);
        assert!(display_str.contains(special_message));
    }

    #[test]
    fn test_signal_management_error_unicode() {
        // Test error with unicode characters
        let unicode_message = "Error with unicode: Hello 世界 🌍";
        let error = SignalManagementError::SignalError(unicode_message.to_string());
        
        // Test error message is preserved
        match error {
            SignalManagementError::SignalError(msg) => assert_eq!(msg, unicode_message),
            _ => assert!(false, "Expected SignalError"),
        }
        
        // Test debug formatting with unicode
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains(unicode_message));
        
        // Test display formatting with unicode
        let display_str = format!("{}", error);
        assert!(display_str.contains(unicode_message));
    }

    #[test]
    fn test_signal_management_error_empty_message() {
        // Test error with empty message
        let empty_message = "";
        let error = SignalManagementError::SignalError(empty_message.to_string());
        
        // Test error message is preserved
        match error {
            SignalManagementError::SignalError(msg) => assert_eq!(msg, empty_message),
            _ => assert!(false, "Expected SignalError"),
        }
        
        // Test debug formatting with empty message
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("SignalError"));
        
        // Test display formatting with empty message
        let display_str = format!("{}", error);
        assert!(display_str.contains(""));
    }
}
