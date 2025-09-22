//! Data table component for the Table package
//! 
//! This module contains a comprehensive data table component with sorting,
//! filtering, pagination, and selection capabilities.

pub mod types;
pub mod component;

// Re-export main types and components
pub use types::*;
pub use component::DataTable;
