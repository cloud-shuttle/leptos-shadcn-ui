//! Leptos port of shadcn/ui table

pub mod default;
pub mod new_york;
pub mod data_table;

pub use default::{Table};
pub use new_york::{Table as TableNewYork};
pub use data_table::{
    DataTable, DataRow, DataColumn, DataTableState,
    SortDirection, FilterType, FilterOperator, SelectionMode, ExportFormat,
    ColumnFilter, RowAction
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod data_table_tests;
