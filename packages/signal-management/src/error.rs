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
}
