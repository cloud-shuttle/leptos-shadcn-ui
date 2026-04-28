use leptos::prelude::*;
use leptos_style::Style;
use std::collections::HashMap;
use super::types::*;

#[component]
pub fn DataTable(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] data: MaybeProp<Vec<DataRow>>,
    #[prop(into, optional)] columns: MaybeProp<Vec<DataColumn>>,
    #[prop(into, optional)] loading: MaybeProp<bool>,
    #[prop(into, optional)] error: MaybeProp<Option<String>>,
    #[prop(into, optional)] on_row_click: Option<Callback<DataRow>>,
    #[prop(into, optional)] on_row_select: Option<Callback<Vec<i32>>>,
    #[prop(into, optional)] on_sort: Option<Callback<SortConfig>>,
    #[prop(into, optional)] on_filter: Option<Callback<Vec<FilterConfig>>>,
    #[prop(into, optional)] on_export: Option<Callback<ExportFormat>>,
    children: Children,
) -> impl IntoView {
    let state = RwSignal::new(DataTableState::default());

    if let Some(data) = data.get() { state.update(|s| s.data = data.clone()); }
    if let Some(cols) = columns.get() { state.update(|s| s.columns = cols.clone()); }
    if let Some(ld) = loading.get() { state.update(|s| s.loading = ld); }
    if let Some(err) = error.get() { state.update(|s| s.error = err.clone()); }

    let filtered_data = Signal::derive(move || {
        let state_snapshot = state.get();
        let mut filtered = state_snapshot.data.clone();
        for filter in &state_snapshot.filters {
            if filter.active {
                filtered = filtered.into_iter().filter(|row| {
                    match filter.column_key.as_str() {
                        "name" => match filter.operator {
                            FilterOperator::Contains => row.name.contains(&filter.value),
                            FilterOperator::Equals => row.name == filter.value,
                            FilterOperator::StartsWith => row.name.starts_with(&filter.value),
                            FilterOperator::EndsWith => row.name.ends_with(&filter.value),
                            _ => true,
                        },
                        "email" => match filter.operator {
                            FilterOperator::Contains => row.email.contains(&filter.value),
                            FilterOperator::Equals => row.email == filter.value,
                            _ => true,
                        },
                        "age" => {
                            if let Ok(age) = filter.value.parse::<i32>() {
                                match filter.operator {
                                    FilterOperator::Equals => row.age == age,
                                    FilterOperator::GreaterThan => row.age > age,
                                    FilterOperator::LessThan => row.age < age,
                                    _ => true,
                                }
                            } else { true }
                        },
                        _ => true,
                    }
                }).collect();
            }
        }
        if state_snapshot.sort_config.active {
            filtered.sort_by(|a, b| {
                match state_snapshot.sort_config.column_key.as_str() {
                    "name" => match state_snapshot.sort_config.direction {
                        SortDirection::Ascending => a.name.cmp(&b.name),
                        SortDirection::Descending => b.name.cmp(&a.name),
                        _ => std::cmp::Ordering::Equal,
                    },
                    "email" => match state_snapshot.sort_config.direction {
                        SortDirection::Ascending => a.email.cmp(&b.email),
                        SortDirection::Descending => b.email.cmp(&a.email),
                        _ => std::cmp::Ordering::Equal,
                    },
                    "age" => match state_snapshot.sort_config.direction {
                        SortDirection::Ascending => a.age.cmp(&b.age),
                        SortDirection::Descending => b.age.cmp(&a.age),
                        _ => std::cmp::Ordering::Equal,
                    },
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        filtered
    });

    let paginated_data = Signal::derive(move || {
        let state_snapshot = state.get();
        let filtered = filtered_data.get();
        let start = (state_snapshot.pagination.current_page - 1) * state_snapshot.pagination.page_size;
        let end = start + state_snapshot.pagination.page_size;
        filtered.into_iter().skip(start as usize).take(state_snapshot.pagination.page_size as usize).collect::<Vec<_>>()
    });

    let handle_sort = move |column_key: String| {
        let current_sort = state.get().sort_config.clone();
        let new_direction = if current_sort.column_key == column_key {
            match current_sort.direction {
                SortDirection::None => SortDirection::Ascending,
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::None,
            }
        } else { SortDirection::Ascending };
        let new_sort = SortConfig { column_key: column_key.clone(), direction: new_direction, active: new_direction != SortDirection::None };
        state.update(|s| s.sort_config = new_sort.clone());
        if let Some(cb) = &on_sort { cb.run(new_sort); }
    };

    let handle_row_click = move |row: DataRow| {
        if let Some(cb) = &on_row_click { cb.run(row); }
    };

    let handle_row_select = move |row_id: i32| {
        let mut selected = state.get().selection.selected_rows.clone();
        if selected.contains(&row_id) { selected.retain(|&id| id != row_id); } else { selected.push(row_id); }
        state.update(|s| s.selection.selected_rows = selected.clone());
        if let Some(cb) = &on_row_select { cb.run(selected); }
    };

    let style_signal = move || style.get().unwrap_or_default();

    view! {
        <div class=move || format!("data-table {}", class.get().unwrap_or_default()) id=id style=style_signal>
            <div class="data-table-header">
                <div class="data-table-title">"Data Table"</div>
                <div class="data-table-actions">
                    <button class="data-table-export-btn"
                        on:click=move |_| { if let Some(cb) = &on_export { cb.run(ExportFormat::Csv); } }>
                        "Export CSV"
                    </button>
                </div>
            </div>
            <div class="data-table-content">
                {move || {
                    let ss = state.get();
                    if ss.loading {
                        view! { <div class="data-table-loading">"Loading..."</div> }.into_any()
                    } else if let Some(err) = &ss.error {
                        view! { <div class="data-table-error">{err.clone()}</div> }.into_any()
                    } else {
                        let cols = ss.columns.clone(); // owned copy
                        view! {
                            <table class="data-table-table">
                                <thead>
                                    <tr>
                                        {cols.into_iter().map(|col| {
                                            let key = col.key.clone();
                                            view! {
                                                <th class=move || format!("data-table-header-cell {}", if col.sortable { "sortable" } else { "" })
                                                    on:click=move |_| if col.sortable { handle_sort(key.clone()) }>
                                                    {col.title.clone()}
                                                    {move || {
                                                        let ss = state.get();
                                                        if ss.sort_config.column_key == col.key {
                                                            match ss.sort_config.direction {
                                                                SortDirection::Ascending => " ↑",
                                                                SortDirection::Descending => " ↓",
                                                                _ => "",
                                                            }
                                                        } else { "" }
                                                    }}
                                                </th>
                                            }.into_view()
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {paginated_data.get().iter().map(|row| {
                                        let rc = row.clone();
                                        view! {
                                            <tr class="data-table-row" on:click=move |_| handle_row_click(rc.clone())>
                                                <td class="data-table-cell">{row.name.clone()}</td>
                                                <td class="data-table-cell">{row.email.clone()}</td>
                                                <td class="data-table-cell">{row.age}</td>
                                            </tr>
                                        }.into_view()
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        }.into_any()
                    }
                }}
            </div>
            <div class="data-table-footer">
                <div class="data-table-pagination">
                    <button class="data-table-pagination-btn"
                        disabled=move || state.get().pagination.current_page <= 1
                        on:click=move |_| state.update(|s| { if s.pagination.current_page > 1 { s.pagination.current_page -= 1; } })>
                        "Previous"
                    </button>
                    <span class="data-table-pagination-info">
                        {move || {
                            let ss = state.get();
                            format!("Page {} of {}", ss.pagination.current_page, ss.pagination.total_pages)
                        }}
                    </span>
                    <button class="data-table-pagination-btn"
                        disabled=move || state.get().pagination.current_page >= state.get().pagination.total_pages
                        on:click=move |_| state.update(|s| { if s.pagination.current_page < s.pagination.total_pages { s.pagination.current_page += 1; } })>
                        "Next"
                    </button>
                </div>
            </div>
            {children()}
        </div>
    }
}
