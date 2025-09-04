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
            sortable: false,
            filterable: false,
            filter_type: None,
            resizable: None,
            width: None,
            draggable: None,
            order: None,
        }
    }
}

impl Default for DataColumn {
    fn default() -> Self {
        Self {
            key: String::new(),
            title: String::new(),
            sortable: false,
            filterable: false,
            filter_type: None,
            resizable: None,
            width: None,
            draggable: None,
            order: None,
        }
    }
}

/// Column filter definition
#[derive(Debug, Clone)]
pub struct ColumnFilter {
    pub column: String,
    pub value: String,
    pub operator: FilterOperator,
}

/// Row action definition
#[derive(Debug, Clone)]
pub struct RowAction {
    pub label: String,
    pub icon: String,
    pub action: Callback<i32>,
}

/// Data table state
#[derive(Debug, Clone)]
pub struct DataTableState {
    pub sort_column: Option<String>,
    pub sort_direction: SortDirection,
    pub filters: Vec<ColumnFilter>,
    pub search_query: String,
    pub current_page: usize,
    pub page_size: usize,
    pub selected_rows: Vec<i32>,
    pub column_widths: HashMap<String, u32>,
    pub column_order: Vec<String>,
}

impl Default for DataTableState {
    fn default() -> Self {
        Self {
            sort_column: None,
            sort_direction: SortDirection::None,
            filters: Vec::new(),
            search_query: String::new(),
            current_page: 1,
            page_size: 10,
            selected_rows: Vec::new(),
            column_widths: HashMap::new(),
            column_order: Vec::new(),
        }
    }
}

/// Advanced data table component
#[component]
pub fn DataTable(
    #[prop(into)] data: Vec<DataRow>,
    #[prop(into)] columns: Vec<DataColumn>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] sortable: MaybeProp<bool>,
    #[prop(into, optional)] filterable: MaybeProp<bool>,
    #[prop(into, optional)] pagination: MaybeProp<bool>,
    #[prop(into, optional)] selectable: MaybeProp<bool>,
    #[prop(into, optional)] searchable: MaybeProp<bool>,
    #[prop(into, optional)] resizable: MaybeProp<bool>,
    #[prop(into, optional)] reorderable: MaybeProp<bool>,
    #[prop(into, optional)] exportable: MaybeProp<bool>,
    #[prop(into, optional)] virtual_scrolling: MaybeProp<bool>,
    #[prop(into, optional)] sort_column: MaybeProp<String>,
    #[prop(into, optional)] sort_direction: MaybeProp<SortDirection>,
    #[prop(into, optional)] filters: MaybeProp<Vec<ColumnFilter>>,
    #[prop(into, optional)] search_query: MaybeProp<String>,
    #[prop(into, optional)] page_size: MaybeProp<usize>,
    #[prop(into, optional)] current_page: MaybeProp<usize>,
    #[prop(into, optional)] total_pages: MaybeProp<usize>,
    #[prop(into, optional)] selection_mode: MaybeProp<SelectionMode>,
    #[prop(into, optional)] selected_rows: MaybeProp<Vec<i32>>,
    #[prop(into, optional)] search_columns: MaybeProp<Vec<String>>,
    #[prop(into, optional)] export_formats: MaybeProp<Vec<ExportFormat>>,
    #[prop(into, optional)] row_height: MaybeProp<u32>,
    #[prop(into, optional)] visible_rows: MaybeProp<usize>,
    #[prop(into, optional)] row_actions: MaybeProp<Vec<RowAction>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (state, set_state) = signal(DataTableState::default());
    
    // Initialize state with props
    if let Some(sort_col) = sort_column.get() {
        set_state.update(|s| s.sort_column = Some(sort_col));
    }
    if let Some(sort_dir) = sort_direction.get() {
        set_state.update(|s| s.sort_direction = sort_dir);
    }
    if let Some(filters_vec) = filters.get() {
        set_state.update(|s| s.filters = filters_vec);
    }
    if let Some(search) = search_query.get() {
        set_state.update(|s| s.search_query = search);
    }
    if let Some(page_sz) = page_size.get() {
        set_state.update(|s| s.page_size = page_sz);
    }
    if let Some(page) = current_page.get() {
        set_state.update(|s| s.current_page = page);
    }
    if let Some(selected) = selected_rows.get() {
        set_state.update(|s| s.selected_rows = selected);
    }

    // Computed filtered and sorted data
    let processed_data = Signal::derive(move || {
        let mut result = data.clone();
        
        // Apply search filter
        if let Some(search_cols) = search_columns.get() {
            let query = state.get().search_query.clone();
            if !query.is_empty() {
                result.retain(|row| {
                    search_cols.iter().any(|col| {
                        match col.as_str() {
                            "name" => row.name.to_lowercase().contains(&query.to_lowercase()),
                            "email" => row.email.to_lowercase().contains(&query.to_lowercase()),
                            _ => false,
                        }
                    })
                });
            }
        }
        
        // Apply column filters
        for filter in &state.get().filters {
            result.retain(|row| {
                match filter.column.as_str() {
                    "name" => match filter.operator {
                        FilterOperator::Contains => row.name.to_lowercase().contains(&filter.value.to_lowercase()),
                        FilterOperator::Equals => row.name == filter.value,
                        _ => true,
                    },
                    "age" => match filter.operator {
                        FilterOperator::Equals => row.age.to_string() == filter.value,
                        FilterOperator::GreaterThan => row.age > filter.value.parse::<i32>().unwrap_or(0),
                        FilterOperator::LessThan => row.age < filter.value.parse::<i32>().unwrap_or(0),
                        _ => true,
                    },
                    "email" => match filter.operator {
                        FilterOperator::Contains => row.email.to_lowercase().contains(&filter.value.to_lowercase()),
                        FilterOperator::Equals => row.email == filter.value,
                        _ => true,
                    },
                    _ => true,
                }
            });
        }
        
        // Apply sorting
        if let Some(sort_col) = &state.get().sort_column {
            match sort_col.as_str() {
                "name" => {
                    result.sort_by(|a, b| {
                        match state.get().sort_direction {
                            SortDirection::Ascending => a.name.cmp(&b.name),
                            SortDirection::Descending => b.name.cmp(&a.name),
                            SortDirection::None => std::cmp::Ordering::Equal,
                        }
                    });
                },
                "age" => {
                    result.sort_by(|a, b| {
                        match state.get().sort_direction {
                            SortDirection::Ascending => a.age.cmp(&b.age),
                            SortDirection::Descending => b.age.cmp(&a.age),
                            SortDirection::None => std::cmp::Ordering::Equal,
                        }
                    });
                },
                "email" => {
                    result.sort_by(|a, b| {
                        match state.get().sort_direction {
                            SortDirection::Ascending => a.email.cmp(&b.email),
                            SortDirection::Descending => b.email.cmp(&a.email),
                            SortDirection::None => std::cmp::Ordering::Equal,
                        }
                    });
                },
                _ => {}
            }
        }
        
        result
    });

    // Computed pagination
    let paginated_data = Signal::derive(move || {
        let data = processed_data.get();
        let page_sz = state.get().page_size;
        let current_page = state.get().current_page;
        
        if pagination.get().unwrap_or(false) {
            let start = (current_page - 1) * page_sz;
            let end = (start + page_sz).min(data.len());
            data[start..end].to_vec()
        } else {
            data
        }
    });

    let computed_class = Signal::derive(move || {
        let mut classes = vec!["data-table".to_string()];
        
        if sortable.get().unwrap_or(false) {
            classes.push("sortable".to_string());
        }
        if filterable.get().unwrap_or(false) {
            classes.push("filterable".to_string());
        }
        if pagination.get().unwrap_or(false) {
            classes.push("pagination".to_string());
        }
        if selectable.get().unwrap_or(false) {
            classes.push("selectable".to_string());
        }
        if searchable.get().unwrap_or(false) {
            classes.push("searchable".to_string());
        }
        if resizable.get().unwrap_or(false) {
            classes.push("resizable".to_string());
        }
        if reorderable.get().unwrap_or(false) {
            classes.push("reorderable".to_string());
        }
        if exportable.get().unwrap_or(false) {
            classes.push("exportable".to_string());
        }
        if virtual_scrolling.get().unwrap_or(false) {
            classes.push("virtual-scrolling".to_string());
        }
        
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            // Search bar
            {if searchable.get().unwrap_or(false) {
                view! {
                    <div class="data-table-search mb-4">
                        <input
                            type="text"
                            placeholder="Search..."
                            class="w-full px-3 py-2 border rounded-md"
                            value=move || state.get().search_query.clone()
                            on:input=move |evt| {
                                let value = event_target_value(&evt);
                                set_state.update(|s| s.search_query = value);
                            }
                        />
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            // Filters
            {if filterable.get().unwrap_or(false) {
                view! {
                    <div class="data-table-filters mb-4">
                        <div class="flex gap-2 flex-wrap">
                            {columns.clone().into_iter().filter(|col| col.filterable).map(|col| {
                                view! {
                                    <div class="filter-group">
                                        <label class="block text-sm font-medium mb-1">{col.title.clone()}</label>
                                        <input
                                            type="text"
                                            placeholder=format!("Filter {}", col.title)
                                            class="px-2 py-1 border rounded text-sm"
                                            on:input=move |evt| {
                                                let value = event_target_value(&evt);
                                                if !value.is_empty() {
                                                    set_state.update(|s| {
                                                        s.filters.retain(|f| f.column != col.key);
                                                        s.filters.push(ColumnFilter {
                                                            column: col.key.clone(),
                                                            value,
                                                            operator: FilterOperator::Contains,
                                                        });
                                                    });
                                                } else {
                                                    set_state.update(|s| {
                                                        s.filters.retain(|f| f.column != col.key);
                                                    });
                                                }
                                            }
                                        />
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            // Export buttons
            {if exportable.get().unwrap_or(false) {
                view! {
                    <div class="data-table-export mb-4">
                        <div class="flex gap-2">
                            {if let Some(formats) = export_formats.get() {
                                formats.into_iter().map(|format| {
                                    view! {
                                        <button
                                            class="px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600"
                                            on:click=move |_| {
                                                match format {
                                                    ExportFormat::Csv => println!("Exporting to CSV"),
                                                    ExportFormat::Json => println!("Exporting to JSON"),
                                                    ExportFormat::Excel => println!("Exporting to Excel"),
                                                }
                                            }
                                        >
                                            {match format {
                                                ExportFormat::Csv => "Export CSV",
                                                ExportFormat::Json => "Export JSON",
                                                ExportFormat::Excel => "Export Excel",
                                            }}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()
                            } else {
                                vec![]
                            }}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            // Table
            <div class="data-table-container overflow-x-auto">
                <table class="w-full border-collapse">
                    <thead>
                        <tr class="border-b">
                            // Selection column
                            {if selectable.get().unwrap_or(false) {
                                view! {
                                    <th class="p-2 text-left">
                                        {if selection_mode.get().unwrap_or(SelectionMode::Single) == SelectionMode::Multiple {
                                            view! {
                                                <input
                                                    type="checkbox"
                                                    class="select-all"
                                                    on:change=move |_| {
                                                        // Toggle all selection logic
                                                    }
                                                />
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </th>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                            
                            // Data columns
                            {columns.clone().into_iter().map(|col| {
                                let col_key = col.key.clone();
                                let col_key_for_click = col_key.clone();
                                let col_key_for_display = col_key.clone();
                                view! {
                                    <th class="p-2 text-left">
                                        <div class="flex items-center gap-2">
                                            <span>{col.title.clone()}</span>
                                            {if col.sortable && sortable.get().unwrap_or(false) {
                                                view! {
                                                    <button
                                                        class="sort-button"
                                                        on:click={
                                                            let col_key = col_key_for_click.clone();
                                                            move |_| {
                                                                set_state.update(|s| {
                                                                    if s.sort_column == Some(col_key.clone()) {
                                                                        s.sort_direction = match s.sort_direction {
                                                                            SortDirection::None => SortDirection::Ascending,
                                                                            SortDirection::Ascending => SortDirection::Descending,
                                                                            SortDirection::Descending => SortDirection::None,
                                                                        };
                                                                    } else {
                                                                        s.sort_column = Some(col_key.clone());
                                                                        s.sort_direction = SortDirection::Ascending;
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    >
                                                        {move || {
                                                            if state.get().sort_column == Some(col_key_for_display.clone()) {
                                                                match state.get().sort_direction {
                                                                    SortDirection::Ascending => "↑",
                                                                    SortDirection::Descending => "↓",
                                                                    SortDirection::None => "↕",
                                                                }
                                                            } else {
                                                                "↕"
                                                            }
                                                        }}
                                                    </button>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}
                                        </div>
                                    </th>
                                }
                            }).collect::<Vec<_>>()}
                            
                            // Actions column
                            {if row_actions.get().is_some() {
                                view! {
                                    <th class="p-2 text-left">"Actions"</th>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            paginated_data.get().into_iter().map(|row| {
                                view! {
                                    <tr class="border-b hover:bg-gray-50">
                                        // Selection cell
                                        {if selectable.get().unwrap_or(false) {
                                            view! {
                                                <td class="p-2">
                                                    <input
                                                        type="checkbox"
                                                        class="row-select"
                                                        checked=move || state.get().selected_rows.contains(&row.id)
                                                        on:change=move |_| {
                                                            set_state.update(|s| {
                                                                if s.selected_rows.contains(&row.id) {
                                                                    s.selected_rows.retain(|&id| id != row.id);
                                                                } else {
                                                                    s.selected_rows.push(row.id);
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                        
                                        // Data cells
                                        {columns.iter().map(|col| {
                                            view! {
                                                <td class="p-2">
                                                    {match col.key.as_str() {
                                                        "name" => row.name.clone(),
                                                        "age" => row.age.to_string(),
                                                        "email" => row.email.clone(),
                                                        _ => "".to_string(),
                                                    }}
                                                </td>
                                            }
                                        }).collect::<Vec<_>>()}
                                        
                                        // Actions cell
                                        {if let Some(actions) = row_actions.get() {
                                            view! {
                                                <td class="p-2">
                                                    <div class="flex gap-1">
                                                        {actions.into_iter().map(|action| {
                                                            view! {
                                                                <button
                                                                    class="px-2 py-1 text-xs bg-gray-100 hover:bg-gray-200 rounded"
                                                                    on:click={
                                                                        let action = action.clone();
                                                                        move |_| action.action.run(row.id)
                                                                    }
                                                                >
                                                                    {action.label.clone()}
                                                                </button>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </td>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>
            
            // Pagination
            {if pagination.get().unwrap_or(false) {
                view! {
                    <div class="data-table-pagination mt-4 flex justify-between items-center">
                        <div class="text-sm text-gray-600">
                            "Showing " {move || {
                                let start = (state.get().current_page - 1) * state.get().page_size + 1;
                                let end = (start + state.get().page_size - 1).min(processed_data.get().len());
                                format!("{} to {} of {}", start, end, processed_data.get().len())
                            }}
                        </div>
                        <div class="flex gap-2">
                            <button
                                class="px-3 py-1 border rounded disabled:opacity-50"
                                disabled=move || state.get().current_page <= 1
                                on:click=move |_| {
                                    set_state.update(|s| {
                                        if s.current_page > 1 {
                                            s.current_page -= 1;
                                        }
                                    });
                                }
                            >
                                "Previous"
                            </button>
                            <span class="px-3 py-1">
                                {move || format!("Page {} of {}", state.get().current_page, (processed_data.get().len() + state.get().page_size - 1) / state.get().page_size)}
                            </span>
                            <button
                                class="px-3 py-1 border rounded disabled:opacity-50"
                                disabled=move || state.get().current_page >= (processed_data.get().len() + state.get().page_size - 1) / state.get().page_size
                                on:click=move |_| {
                                    set_state.update(|s| {
                                        let total_pages = (processed_data.get().len() + s.page_size - 1) / s.page_size;
                                        if s.current_page < total_pages {
                                            s.current_page += 1;
                                        }
                                    });
                                }
                            >
                                "Next"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {children.map(|c| c())}
        </div>
    }
}
