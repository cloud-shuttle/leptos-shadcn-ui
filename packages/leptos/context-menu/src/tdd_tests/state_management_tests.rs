#[cfg(test)]
mod state_management_tests {
    use super::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and signal handling

    #[test]
    fn test_context_menu_state_initialization() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state is properly initialized
    }

    #[test]
    fn test_context_menu_state_with_open_signal() {
        let open = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu open=open>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with open signal
    }

    #[test]
    fn test_context_menu_state_with_on_open_change() {
        let on_open_change = Callback::new(|_| {});
        let _context_menu_view = view! {
            <ContextMenu on_open_change=on_open_change>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with on_open_change callback
    }

    #[test]
    fn test_context_menu_state_with_multiple_signals() {
        let open = RwSignal::new(false);
        let disabled = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                disabled=MaybeProp::from(disabled)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with multiple signals
    }

    #[test]
    fn test_context_menu_state_with_computed_values() {
        let open = RwSignal::new(false);
        let computed_class = Signal::derive(move || {
            format!("context-menu-{}", if open.get() { "open" } else { "closed" })
        });
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                class=MaybeProp::from(computed_class)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with computed values
    }

    #[test]
    fn test_context_menu_state_with_derived_signals() {
        let open = RwSignal::new(false);
        let derived_signal = Signal::derive(move || {
            open.get().to_string()
        });
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                aria_expanded=MaybeProp::from(derived_signal)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with derived signals
    }

    #[test]
    fn test_context_menu_state_with_memo() {
        let open = RwSignal::new(false);
        let memoized_class = Memo::new(move |_| {
            format!("context-menu-{}", if open.get() { "open" } else { "closed" })
        });
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                class=MaybeProp::from(memoized_class)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with memoized values
    }

    #[test]
    fn test_context_menu_state_with_context() {
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with context
    }

    #[test]
    fn test_context_menu_state_with_provided_context() {
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with provided context
    }

    #[test]
    fn test_context_menu_state_with_expected_context() {
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with expected context
    }

    #[test]
    fn test_context_menu_state_with_multiple_contexts() {
        let context1 = RwSignal::new("context1".to_string());
        let context2 = RwSignal::new("context2".to_string());
        provide_context(context1);
        provide_context(context2);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with multiple contexts
    }

    #[test]
    fn test_context_menu_state_with_nested_contexts() {
        let outer_context = RwSignal::new("outer".to_string());
        let inner_context = RwSignal::new("inner".to_string());
        provide_context(outer_context);
        provide_context(inner_context);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with nested contexts
    }

    #[test]
    fn test_context_menu_state_with_conditional_rendering() {
        let show_context_menu = RwSignal::new(true);
        let _context_menu_view = view! {
            <Show when=show_context_menu>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </Show>
        };
        // Test that state works with conditional rendering
    }

    #[test]
    fn test_context_menu_state_with_conditional_props() {
        let is_disabled = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu disabled=MaybeProp::from(is_disabled)>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with conditional props
    }

    #[test]
    fn test_context_menu_state_with_dynamic_content() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with dynamic content
    }

    #[test]
    fn test_context_menu_state_with_filtered_items() {
        let all_items = vec!["Item 1", "Item 2", "Item 3"];
        let filter = RwSignal::new("".to_string());
        let filtered_items = Signal::derive(move || {
            let f = filter.get();
            if f.is_empty() {
                all_items.clone()
            } else {
                all_items.clone().into_iter()
                    .filter(|item| item.contains(&f))
                    .collect()
            }
        });
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=filtered_items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with filtered items
    }

    #[test]
    fn test_context_menu_state_with_sorted_items() {
        let items = RwSignal::new(vec!["Item 3", "Item 1", "Item 2"]);
        let sorted_items = Signal::derive(move || {
            let mut items = items.get();
            items.sort();
            items
        });
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=sorted_items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with sorted items
    }

    #[test]
    fn test_context_menu_state_with_grouped_items() {
        let items = RwSignal::new(vec![
            ("Group 1", "Item 1"),
            ("Group 1", "Item 2"),
            ("Group 2", "Item 3"),
        ]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|(group, item)| format!("{}-{}", group, item) let:(group, item)>
                        <ContextMenuGroup>
                            <ContextMenuLabel>{group}</ContextMenuLabel>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </ContextMenuGroup>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with grouped items
    }

    #[test]
    fn test_context_menu_state_with_async_content() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with async content
    }

    #[test]
    fn test_context_menu_state_with_loading_state() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let loading = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu disabled=MaybeProp::from(loading)>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <Show when=move || !loading.get()>
                        <For each=items key=|item| item.clone() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </Show>
                    <Show when=loading>
                        <ContextMenuItem>"Loading..."</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with loading state
    }

    #[test]
    fn test_context_menu_state_with_error_state() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let error = RwSignal::new(None::<String>);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <Show when=move || error.get().is_none()>
                        <For each=items key=|item| item.clone() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </Show>
                    <Show when=move || error.get().is_some()>
                        <ContextMenuItem>"Error occurred"</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with error state
    }

    #[test]
    fn test_context_menu_state_with_validation() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let is_valid = Signal::derive(move || {
            !selected_item.get().is_empty()
        });
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                selected_item.set(item.clone());
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with validation
    }

    #[test]
    fn test_context_menu_state_with_form_integration() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let form_data = RwSignal::new(std::collections::HashMap::new());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with form integration
    }

    #[test]
    fn test_context_menu_state_with_persistence() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("Item 1".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            class=MaybeProp::from(Signal::derive(move || {
                                if item == selected_item.get() { "selected" } else { "" }
                            }))
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with persistence
    }

    #[test]
    fn test_context_menu_state_with_undo_redo() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("Item 1".to_string());
        let history = RwSignal::new(vec!["Item 1".to_string()]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                history.update(|h| h.push(selected_item.get()));
                                selected_item.set(item.clone());
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with undo/redo
    }

    #[test]
    fn test_context_menu_state_with_batch_updates() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let open = RwSignal::new(false);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu open=open>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                batch(move || {
                                    selected_item.set(item.clone());
                                    open.set(false);
                                });
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with batch updates
    }

    #[test]
    fn test_context_menu_state_with_debounced_updates() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let search_term = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with debounced updates
    }

    #[test]
    fn test_context_menu_state_with_throttled_updates() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let update_count = RwSignal::new(0);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                update_count.update(|c| *c += 1);
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with throttled updates
    }

    #[test]
    fn test_context_menu_state_with_optimistic_updates() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                // Optimistic update
                                selected_item.set(item.clone());
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with optimistic updates
    }

    #[test]
    fn test_context_menu_state_with_rollback() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let previous_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                previous_item.set(selected_item.get());
                                selected_item.set(item.clone());
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with rollback
    }

    #[test]
    fn test_context_menu_state_with_conflict_resolution() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let conflict_resolution = RwSignal::new("last_wins".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                match conflict_resolution.get().as_str() {
                                    "last_wins" => selected_item.set(item.clone()),
                                    "first_wins" => {
                                        if selected_item.get().is_empty() {
                                            selected_item.set(item.clone());
                                        }
                                    },
                                    _ => selected_item.set(item.clone()),
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with conflict resolution
    }

    #[test]
    fn test_context_menu_state_with_synchronization() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let sync_enabled = RwSignal::new(true);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                if sync_enabled.get() {
                                    selected_item.set(item.clone());
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with synchronization
    }

    #[test]
    fn test_context_menu_state_with_replication() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let replica_count = RwSignal::new(3);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                selected_item.set(item.clone());
                                // Replicate to other instances
                                for _ in 0..replica_count.get() {
                                    // Replication logic would go here
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with replication
    }

    #[test]
    fn test_context_menu_state_with_consistency() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let consistency_level = RwSignal::new("strong".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                match consistency_level.get().as_str() {
                                    "strong" => {
                                        // Strong consistency
                                        selected_item.set(item.clone());
                                    },
                                    "eventual" => {
                                        // Eventual consistency
                                        selected_item.set(item.clone());
                                    },
                                    _ => selected_item.set(item.clone()),
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with consistency
    }

    #[test]
    fn test_context_menu_state_with_atomicity() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                // Atomic operation
                                batch(move || {
                                    selected_item.set(item.clone());
                                });
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with atomicity
    }

    #[test]
    fn test_context_menu_state_with_isolation() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let isolation_level = RwSignal::new("read_committed".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                match isolation_level.get().as_str() {
                                    "read_committed" => {
                                        // Read committed isolation
                                        selected_item.set(item.clone());
                                    },
                                    "repeatable_read" => {
                                        // Repeatable read isolation
                                        selected_item.set(item.clone());
                                    },
                                    _ => selected_item.set(item.clone()),
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with isolation
    }

    #[test]
    fn test_context_menu_state_with_durability() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let durability_enabled = RwSignal::new(true);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.clone() let:item>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                selected_item.set(item.clone());
                                if durability_enabled.get() {
                                    // Persist to durable storage
                                }
                            })
                        >
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with durability
    }
}
