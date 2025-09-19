//! Error types for signal management operations

use thiserror::Error;

/// Errors that can occur during signal management operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SignalManagementError {
    /// Signal has been disposed and is no longer valid
    #[error("Signal has been disposed and is no longer valid")]
    SignalDisposed,
    
    /// Signal update failed due to invalid state
    #[error("Signal update failed: {reason}")]
    UpdateFailed { reason: String },
    
    /// Memory management operation failed
    #[error("Memory management operation failed: {reason}")]
    MemoryManagementFailed { reason: String },
    
    /// Batched update operation failed
    #[error("Batched update operation failed: {reason}")]
    BatchedUpdateFailed { reason: String },
    
    /// Group not found error
    #[error("Group not found: {group_name}")]
    GroupNotFound { group_name: String },
    
    /// Signal operation error
    #[error("Signal error: {0}")]
    SignalError(String),
    
    /// Memo operation error
    #[error("Memo error: {0}")]
    MemoError(String),
    
    /// Cleanup operation error
    #[error("Cleanup error: {0}")]
    CleanupError(String),
    
    /// Memory operation error
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    /// Batch operation error
    #[error("Batch error: {0}")]
    BatchError(String),
}

impl SignalManagementError {
    /// Create a new update failed error
    pub fn update_failed(reason: impl Into<String>) -> Self {
        Self::UpdateFailed {
            reason: reason.into(),
        }
    }
    
    /// Create a new memory management failed error
    pub fn memory_management_failed(reason: impl Into<String>) -> Self {
        Self::MemoryManagementFailed {
            reason: reason.into(),
        }
    }
    
    /// Create a new batched update failed error
    pub fn batched_update_failed(reason: impl Into<String>) -> Self {
        Self::BatchedUpdateFailed {
            reason: reason.into(),
        }
    }
    
    /// Create a new group not found error
    pub fn group_not_found(group_name: impl Into<String>) -> Self {
        Self::GroupNotFound {
            group_name: group_name.into(),
        }
    }
    
    /// Create a new signal error
    pub fn signal_error(reason: impl Into<String>) -> Self {
        Self::SignalError(reason.into())
    }
    
    /// Create a new memo error
    pub fn memo_error(reason: impl Into<String>) -> Self {
        Self::MemoError(reason.into())
    }
    
    /// Create a new cleanup error
    pub fn cleanup_error(reason: impl Into<String>) -> Self {
        Self::CleanupError(reason.into())
    }
    
    /// Create a new memory error
    pub fn memory_error(reason: impl Into<String>) -> Self {
        Self::MemoryError(reason.into())
    }
    
    /// Create a new batch error
    pub fn batch_error(reason: impl Into<String>) -> Self {
        Self::BatchError(reason.into())
    }
}
