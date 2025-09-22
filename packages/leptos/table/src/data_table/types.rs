use leptos::prelude::*;
use leptos_style::Style;
use std::collections::HashMap;

/// Sort direction for columns
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::None
    }
}

/// Filter type for columns
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    Text,
    Number,
    Date,
    Select,
    Boolean,
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::Text
    }
}

/// Filter operator for column filters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Default for FilterOperator {
    fn default() -> Self {
        FilterOperator::Contains
    }
}

/// Selection mode for rows
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::None
    }
}

/// Export format for data
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Csv,
    Json,
    Excel,
}

/// Data row structure
#[derive(Debug, Clone)]
pub struct DataRow {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
}

/// Data column configuration
#[derive(Debug, Clone)]
pub struct DataColumn {
    pub key: String,
    pub title: String,
    pub sortable: bool,
    pub filterable: bool,
    pub filter_type: Option<FilterType>,
    pub resizable: Option<bool>,
    pub width: Option<u32>,
    pub draggable: Option<bool>,
    pub order: Option<u32>,
}

impl DataColumn {
    pub fn new(key: String, title: String) -> Self {
        Self {
            key,
            title,
            sortable: true,
            filterable: true,
            filter_type: Some(FilterType::Text),
            resizable: Some(true),
            width: None,
            draggable: Some(true),
            order: None,
        }
    }
}

/// Filter configuration
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub column_key: String,
    pub filter_type: FilterType,
    pub operator: FilterOperator,
    pub value: String,
    pub active: bool,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            column_key: String::new(),
            filter_type: FilterType::Text,
            operator: FilterOperator::Contains,
            value: String::new(),
            active: false,
        }
    }
}

/// Sort configuration
#[derive(Debug, Clone)]
pub struct SortConfig {
    pub column_key: String,
    pub direction: SortDirection,
    pub active: bool,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            column_key: String::new(),
            direction: SortDirection::None,
            active: false,
        }
    }
}

/// Pagination configuration
#[derive(Debug, Clone)]
pub struct PaginationConfig {
    pub page_size: u32,
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub show_size_selector: bool,
    pub show_page_info: bool,
}

impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            page_size: 10,
            current_page: 1,
            total_pages: 1,
            total_items: 0,
            show_size_selector: true,
            show_page_info: true,
        }
    }
}

/// Selection configuration
#[derive(Debug, Clone)]
pub struct SelectionConfig {
    pub mode: SelectionMode,
    pub selected_rows: Vec<i32>,
    pub select_all: bool,
    pub indeterminate: bool,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            mode: SelectionMode::None,
            selected_rows: Vec::new(),
            select_all: false,
            indeterminate: false,
        }
    }
}

/// Data table state
#[derive(Debug, Clone)]
pub struct DataTableState {
    pub data: Vec<DataRow>,
    pub columns: Vec<DataColumn>,
    pub filters: Vec<FilterConfig>,
    pub sort_config: SortConfig,
    pub pagination: PaginationConfig,
    pub selection: SelectionConfig,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for DataTableState {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            columns: Vec::new(),
            filters: Vec::new(),
            sort_config: SortConfig::default(),
            pagination: PaginationConfig::default(),
            selection: SelectionConfig::default(),
            loading: false,
            error: None,
        }
    }
}
