# Signal Management Examples

## Overview

This document provides practical examples of using the signal management utilities in real-world scenarios.

## Basic Usage Examples

### 1. Simple Button Component

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;

#[component]
fn SimpleButton(children: Children) -> impl IntoView {
    let button_state = ArcRwSignal::new(ButtonState {
        loading: false,
        disabled: false,
        click_count: 0,
    });
    
    let button_class = ArcMemo::new(move |_| {
        let state = button_state.get();
        format!("btn {}", if state.loading { "loading" } else { "" })
    });
    
    let handle_click = {
        let button_state = button_state.clone();
        move |_| {
            button_state.update(|state| {
                state.click_count += 1;
                state.loading = true;
            });
            
            // Simulate async operation
            button_state.update(|state| {
                state.loading = false;
            });
        }
    };
    
    view! {
        <button
            class=move || button_class.get()
            disabled=move || button_state.get().disabled
            on:click=handle_click
        >
            {children()}
        </button>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ButtonState {
    loading: bool,
    disabled: bool,
    click_count: u32,
}
```

### 2. Form with Validation

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;
use std::collections::HashMap;

#[component]
fn ContactForm() -> impl IntoView {
    let form_state = ArcRwSignal::new(FormState {
        name: String::new(),
        email: String::new(),
        message: String::new(),
        is_submitting: false,
        errors: HashMap::new(),
    });
    
    let validation_state = ArcMemo::new(move |_| {
        let state = form_state.get();
        FormValidationState {
            is_name_valid: !state.name.is_empty() && state.name.len() >= 2,
            is_email_valid: state.email.contains('@') && state.email.contains('.'),
            is_message_valid: !state.message.is_empty() && state.message.len() >= 10,
            can_submit: !state.name.is_empty() && 
                       state.email.contains('@') && 
                       !state.message.is_empty() && 
                       !state.is_submitting,
        }
    });
    
    let handle_submit = {
        let form_state = form_state.clone();
        let validation_state = validation_state.clone();
        move |_| {
            if validation_state.get().can_submit {
                form_state.update(|state| {
                    state.is_submitting = true;
                });
                
                // Simulate form submission
                form_state.update(|state| {
                    state.is_submitting = false;
                    state.name.clear();
                    state.email.clear();
                    state.message.clear();
                });
            }
        }
    };
    
    view! {
        <form on:submit=handle_submit>
            <div>
                <input
                    type="text"
                    placeholder="Name"
                    value=move || form_state.get().name
                    on:input=move |ev| {
                        form_state.update(|state| {
                            state.name = event_target_value(&ev);
                        });
                    }
                />
                {move || if !validation_state.get().is_name_valid {
                    view! { <span class="error">"Name must be at least 2 characters"</span> }
                } else {
                    view! { <></> }
                }}
            </div>
            
            <div>
                <input
                    type="email"
                    placeholder="Email"
                    value=move || form_state.get().email
                    on:input=move |ev| {
                        form_state.update(|state| {
                            state.email = event_target_value(&ev);
                        });
                    }
                />
                {move || if !validation_state.get().is_email_valid {
                    view! { <span class="error">"Please enter a valid email"</span> }
                } else {
                    view! { <></> }
                }}
            </div>
            
            <div>
                <textarea
                    placeholder="Message"
                    value=move || form_state.get().message
                    on:input=move |ev| {
                        form_state.update(|state| {
                            state.message = event_target_value(&ev);
                        });
                    }
                />
                {move || if !validation_state.get().is_message_valid {
                    view! { <span class="error">"Message must be at least 10 characters"</span> }
                } else {
                    view! { <></> }
                }}
            </div>
            
            <button
                type="submit"
                disabled=move || !validation_state.get().can_submit
            >
                {move || if form_state.get().is_submitting {
                    "Submitting..."
                } else {
                    "Submit"
                }}
            </button>
        </form>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FormState {
    name: String,
    email: String,
    message: String,
    is_submitting: bool,
    errors: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
struct FormValidationState {
    is_name_valid: bool,
    is_email_valid: bool,
    is_message_valid: bool,
    can_submit: bool,
}
```

## Advanced Examples

### 3. Data Table with Sorting and Filtering

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;
use std::collections::HashMap;

#[component]
fn DataTable<F, I>(
    data: F,
    #[prop(optional)] sortable: Option<bool>,
    #[prop(optional)] filterable: Option<bool>,
) -> impl IntoView
where
    F: Fn() -> Vec<I> + 'static,
    I: Clone + 'static,
{
    let table_state = ArcRwSignal::new(TableState {
        sort_column: None,
        sort_direction: SortDirection::Asc,
        filter_text: String::new(),
        page: 1,
        page_size: 10,
    });
    
    let filtered_data = ArcMemo::new(move |_| {
        let state = table_state.get();
        let mut items = data();
        
        // Apply filtering
        if !state.filter_text.is_empty() {
            items.retain(|item| {
                // Custom filtering logic based on item type
                true // Placeholder
            });
        }
        
        // Apply sorting
        if let Some(column) = &state.sort_column {
            // Custom sorting logic based on column
            match state.sort_direction {
                SortDirection::Asc => {
                    // Sort ascending
                }
                SortDirection::Desc => {
                    // Sort descending
                }
            }
        }
        
        items
    });
    
    let paginated_data = ArcMemo::new(move |_| {
        let state = table_state.get();
        let all_data = filtered_data.get();
        let start = (state.page - 1) * state.page_size;
        let end = start + state.page_size;
        
        all_data.into_iter().skip(start).take(state.page_size).collect::<Vec<_>>()
    });
    
    let handle_sort = {
        let table_state = table_state.clone();
        move |column: String| {
            table_state.update(|state| {
                if state.sort_column.as_ref() == Some(&column) {
                    state.sort_direction = match state.sort_direction {
                        SortDirection::Asc => SortDirection::Desc,
                        SortDirection::Desc => SortDirection::Asc,
                    };
                } else {
                    state.sort_column = Some(column);
                    state.sort_direction = SortDirection::Asc;
                }
            });
        }
    };
    
    let handle_filter = {
        let table_state = table_state.clone();
        move |text: String| {
            table_state.update(|state| {
                state.filter_text = text;
                state.page = 1; // Reset to first page
            });
        }
    };
    
    view! {
        <div class="data-table">
            <div class="table-controls">
                <input
                    type="text"
                    placeholder="Filter..."
                    value=move || table_state.get().filter_text
                    on:input=move |ev| {
                        handle_filter(event_target_value(&ev));
                    }
                />
            </div>
            
            <table>
                <thead>
                    <tr>
                        <th on:click=move |_| handle_sort("name".to_string())>
                            "Name"
                        </th>
                        <th on:click=move |_| handle_sort("email".to_string())>
                            "Email"
                        </th>
                        <th on:click=move |_| handle_sort("date".to_string())>
                            "Date"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {move || paginated_data.get().into_iter().map(|item| {
                        view! {
                            <tr>
                                <td>{format!("{:?}", item)}</td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
            
            <div class="pagination">
                <button
                    disabled=move || table_state.get().page <= 1
                    on:click=move |_| {
                        table_state.update(|state| {
                            if state.page > 1 {
                                state.page -= 1;
                            }
                        });
                    }
                >
                    "Previous"
                </button>
                
                <span>
                    {move || format!("Page {} of {}", 
                        table_state.get().page,
                        (filtered_data.get().len() + table_state.get().page_size - 1) / table_state.get().page_size
                    )}
                </span>
                
                <button
                    disabled=move || {
                        let state = table_state.get();
                        let total_pages = (filtered_data.get().len() + state.page_size - 1) / state.page_size;
                        state.page >= total_pages
                    }
                    on:click=move |_| {
                        table_state.update(|state| {
                            let total_pages = (filtered_data.get().len() + state.page_size - 1) / state.page_size;
                            if state.page < total_pages {
                                state.page += 1;
                            }
                        });
                    }
                >
                    "Next"
                </button>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TableState {
    sort_column: Option<String>,
    sort_direction: SortDirection,
    filter_text: String,
    page: usize,
    page_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum SortDirection {
    Asc,
    Desc,
}
```

### 4. Theme Switcher with Persistence

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;

#[component]
fn ThemeSwitcher() -> impl IntoView {
    let theme_manager = TailwindSignalManager::new();
    let current_theme = theme_manager.theme();
    
    let toggle_theme = {
        let current_theme = current_theme.clone();
        move |_| {
            current_theme.update(|theme| {
                *theme = match *theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                };
            });
        }
    };
    
    let theme_icon = ArcMemo::new(move |_| {
        match current_theme.get() {
            Theme::Light => "🌙",
            Theme::Dark => "☀️",
        }
    });
    
    let theme_class = ArcMemo::new(move |_| {
        match current_theme.get() {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    });
    
    view! {
        <div class=move || theme_class.get()>
            <button
                class="theme-switcher"
                on:click=toggle_theme
                title=move || format!("Switch to {} theme", 
                    match current_theme.get() {
                        Theme::Light => "dark",
                        Theme::Dark => "light",
                    }
                )
            >
                {move || theme_icon.get()}
            </button>
        </div>
    }
}
```

## Memory Management Examples

### 5. Memory-Aware Component

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;

#[component]
fn MemoryAwareComponent() -> impl IntoView {
    let memory_manager = SignalMemoryManager::new();
    let memory_stats = memory_manager.get_stats();
    let memory_pressure = memory_manager.detect_memory_pressure();
    
    let memory_status = ArcMemo::new(move |_| {
        let stats = memory_stats.get();
        let pressure = memory_pressure;
        
        MemoryStatus {
            total_signals: stats.total_signals,
            total_memos: stats.total_memos,
            memory_usage: stats.memory_usage,
            pressure_level: pressure,
            should_cleanup: pressure.map_or(false, |p| p > MemoryPressureLevel::High),
        }
    });
    
    let handle_cleanup = {
        let memory_manager = memory_manager.clone();
        move |_| {
            memory_manager.perform_automatic_cleanup();
        }
    };
    
    view! {
        <div class="memory-status">
            <h3>"Memory Status"</h3>
            <div class="memory-info">
                <p>
                    "Signals: " {move || memory_status.get().total_signals}
                </p>
                <p>
                    "Memos: " {move || memory_status.get().total_memos}
                </p>
                <p>
                    "Memory Usage: " {move || format!("{:.2} MB", memory_status.get().memory_usage as f64 / 1024.0 / 1024.0)}
                </p>
                <p>
                    "Pressure: " {move || {
                        match memory_status.get().pressure_level {
                            Some(MemoryPressureLevel::Low) => "Low",
                            Some(MemoryPressureLevel::Medium) => "Medium",
                            Some(MemoryPressureLevel::High) => "High",
                            Some(MemoryPressureLevel::Critical) => "Critical",
                            None => "Unknown",
                        }
                    }}
                </p>
            </div>
            
            {move || if memory_status.get().should_cleanup {
                view! {
                    <div class="memory-warning">
                        <p>"High memory pressure detected!"</p>
                        <button on:click=handle_cleanup>
                            "Cleanup Memory"
                        </button>
                    </div>
                }
            } else {
                view! { <></> }
            }}
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MemoryStatus {
    total_signals: usize,
    total_memos: usize,
    memory_usage: usize,
    pressure_level: Option<MemoryPressureLevel>,
    should_cleanup: bool,
}
```

## Performance Optimization Examples

### 6. Optimized List Component

```rust
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;

#[component]
fn OptimizedList<F, I>(
    items: F,
    #[prop(optional)] page_size: Option<usize>,
) -> impl IntoView
where
    F: Fn() -> Vec<I> + 'static,
    I: Clone + 'static,
{
    let page_size = page_size.unwrap_or(50);
    let list_state = ArcRwSignal::new(ListState {
        page: 1,
        search_term: String::new(),
    });
    
    // Use ArcMemo for expensive filtering operations
    let filtered_items = ArcMemo::new(move |_| {
        let state = list_state.get();
        let all_items = items();
        
        if state.search_term.is_empty() {
            all_items
        } else {
            all_items.into_iter()
                .filter(|item| {
                    // Custom filtering logic
                    true // Placeholder
                })
                .collect()
        }
    });
    
    // Use ArcMemo for pagination
    let paginated_items = ArcMemo::new(move |_| {
        let state = list_state.get();
        let filtered = filtered_items.get();
        let start = (state.page - 1) * page_size;
        let end = start + page_size;
        
        filtered.into_iter().skip(start).take(page_size).collect::<Vec<_>>()
    });
    
    // Use batched updates for better performance
    let updater = BatchedSignalUpdater::new();
    updater.auto_tune_batch_size();
    
    let handle_search = {
        let list_state = list_state.clone();
        let updater = updater.clone();
        move |term: String| {
            updater.add_update(Box::new({
                let list_state = list_state.clone();
                move || {
                    list_state.update(|state| {
                        state.search_term = term.clone();
                        state.page = 1; // Reset to first page
                    });
                }
            }));
            updater.flush();
        }
    };
    
    view! {
        <div class="optimized-list">
            <input
                type="text"
                placeholder="Search..."
                value=move || list_state.get().search_term
                on:input=move |ev| {
                    handle_search(event_target_value(&ev));
                }
            />
            
            <div class="list-items">
                {move || paginated_items.get().into_iter().map(|item| {
                    view! {
                        <div class="list-item">
                            {format!("{:?}", item)}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            <div class="pagination">
                <button
                    disabled=move || list_state.get().page <= 1
                    on:click=move |_| {
                        list_state.update(|state| {
                            if state.page > 1 {
                                state.page -= 1;
                            }
                        });
                    }
                >
                    "Previous"
                </button>
                
                <span>
                    {move || format!("Page {}", list_state.get().page)}
                </span>
                
                <button
                    on:click=move |_| {
                        list_state.update(|state| {
                            state.page += 1;
                        });
                    }
                >
                    "Next"
                </button>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ListState {
    page: usize,
    search_term: String,
}
```

## Testing Examples

### 7. Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;
    
    #[test]
    fn test_button_component() {
        let button_state = ArcRwSignal::new(ButtonState {
            loading: false,
            disabled: false,
            click_count: 0,
        });
        
        // Test initial state
        assert_eq!(button_state.get().click_count, 0);
        assert!(!button_state.get().loading);
        
        // Test state update
        button_state.update(|state| {
            state.click_count = 1;
            state.loading = true;
        });
        
        assert_eq!(button_state.get().click_count, 1);
        assert!(button_state.get().loading);
    }
    
    #[test]
    fn test_form_validation() {
        let form_state = ArcRwSignal::new(FormState {
            name: String::new(),
            email: String::new(),
            message: String::new(),
            is_submitting: false,
            errors: HashMap::new(),
        });
        
        let validation_state = ArcMemo::new(move |_| {
            let state = form_state.get();
            FormValidationState {
                is_name_valid: !state.name.is_empty(),
                is_email_valid: state.email.contains('@'),
                is_message_valid: !state.message.is_empty(),
                can_submit: !state.name.is_empty() && 
                           state.email.contains('@') && 
                           !state.message.is_empty(),
            }
        });
        
        // Test initial validation
        assert!(!validation_state.get().can_submit);
        
        // Test with valid data
        form_state.update(|state| {
            state.name = "John Doe".to_string();
            state.email = "john@example.com".to_string();
            state.message = "Hello, world!".to_string();
        });
        
        assert!(validation_state.get().can_submit);
    }
}
```

## Best Practices

### 1. Signal Lifecycle Management
```rust
// Always track signals for lifecycle management
let manager = TailwindSignalManager::new();
manager.track_signal(my_signal);
manager.track_memo(my_memo);
manager.apply_lifecycle_optimization();
```

### 2. Memory Management
```rust
// Monitor memory pressure
let memory_manager = SignalMemoryManager::new();
if let Some(pressure) = memory_manager.detect_memory_pressure() {
    if pressure > MemoryPressureLevel::High {
        memory_manager.perform_automatic_cleanup();
    }
}
```

### 3. Performance Optimization
```rust
// Use batched updates for multiple changes
let updater = BatchedSignalUpdater::new();
updater.auto_tune_batch_size();
```

### 4. Error Handling
```rust
// Handle signal management errors
match result {
    Ok(value) => {
        // Handle success
    }
    Err(SignalManagementError::MemoryLimitExceeded) => {
        // Handle memory limit
    }
    Err(SignalManagementError::InvalidSignal) => {
        // Handle invalid signal
    }
    Err(_) => {
        // Handle other errors
    }
}
```

These examples demonstrate the power and flexibility of the signal management utilities. Use them as starting points for your own components and adapt them to your specific needs.
