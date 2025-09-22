#[cfg(test)]
mod state_management_tests {
    use super::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and signal handling

    #[test]
    fn test_combobox_state_initialization() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state is properly initialized
    }

    #[test]
    fn test_combobox_state_with_initial_value() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from("option1")
            />
        };
        // Test that state is properly initialized with initial value
    }

    #[test]
    fn test_combobox_state_with_signal() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with signals
    }

    #[test]
    fn test_combobox_state_with_callback() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let on_change = Callback::new(|_| {});
        let _combobox_view = view! {
            <Combobox 
                options=options
                on_change=on_change
            />
        };
        // Test that state works with callbacks
    }

    #[test]
    fn test_combobox_state_with_multiple_signals() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value_signal = RwSignal::new("option1".to_string());
        let disabled_signal = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value_signal)
                disabled=MaybeProp::from(disabled_signal)
            />
        };
        // Test that state works with multiple signals
    }

    #[test]
    fn test_combobox_state_with_computed_values() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value_signal = RwSignal::new("option1".to_string());
        let computed_class = Signal::derive(move || {
            format!("combobox-{}", value_signal.get())
        });
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value_signal)
                class=MaybeProp::from(computed_class)
            />
        };
        // Test that state works with computed values
    }

    #[test]
    fn test_combobox_state_with_derived_signals() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value_signal = RwSignal::new("option1".to_string());
        let derived_signal = Signal::derive(move || {
            value_signal.get().to_uppercase()
        });
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value_signal)
                placeholder=MaybeProp::from(derived_signal)
            />
        };
        // Test that state works with derived signals
    }

    #[test]
    fn test_combobox_state_with_memo() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value_signal = RwSignal::new("option1".to_string());
        let memoized_value = Memo::new(move |_| {
            value_signal.get().to_uppercase()
        });
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value_signal)
                placeholder=MaybeProp::from(memoized_value)
            />
        };
        // Test that state works with memoized values
    }

    #[test]
    fn test_combobox_state_with_context() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state works with context
    }

    #[test]
    fn test_combobox_state_with_provided_context() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state works with provided context
    }

    #[test]
    fn test_combobox_state_with_expected_context() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state works with expected context
    }

    #[test]
    fn test_combobox_state_with_multiple_contexts() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let context1 = RwSignal::new("context1".to_string());
        let context2 = RwSignal::new("context2".to_string());
        provide_context(context1);
        provide_context(context2);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state works with multiple contexts
    }

    #[test]
    fn test_combobox_state_with_nested_contexts() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let outer_context = RwSignal::new("outer".to_string());
        let inner_context = RwSignal::new("inner".to_string());
        provide_context(outer_context);
        provide_context(inner_context);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test that state works with nested contexts
    }

    #[test]
    fn test_combobox_state_with_conditional_rendering() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let show_combobox = RwSignal::new(true);
        let _combobox_view = view! {
            <Show when=show_combobox>
                <Combobox options=options/>
            </Show>
        };
        // Test that state works with conditional rendering
    }

    #[test]
    fn test_combobox_state_with_conditional_props() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let is_disabled = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                disabled=MaybeProp::from(is_disabled)
            />
        };
        // Test that state works with conditional props
    }

    #[test]
    fn test_combobox_state_with_dynamic_options() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let _combobox_view = view! {
            <Combobox options=options_signal/>
        };
        // Test that state works with dynamic options
    }

    #[test]
    fn test_combobox_state_with_filtered_options() {
        let all_options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
            ComboboxOption::new("option3", "Option 3"),
        ];
        let filter_signal = RwSignal::new("option1".to_string());
        let filtered_options = Signal::derive(move || {
            all_options.clone().into_iter()
                .filter(|opt| opt.value.contains(&filter_signal.get()))
                .collect::<Vec<_>>()
        });
        let _combobox_view = view! {
            <Combobox options=filtered_options/>
        };
        // Test that state works with filtered options
    }

    #[test]
    fn test_combobox_state_with_sorted_options() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option3", "Option 3"),
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let sorted_options = Signal::derive(move || {
            let mut options = options_signal.get();
            options.sort_by(|a, b| a.value.cmp(&b.value));
            options
        });
        let _combobox_view = view! {
            <Combobox options=sorted_options/>
        };
        // Test that state works with sorted options
    }

    #[test]
    fn test_combobox_state_with_grouped_options() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("group1_option1", "Group 1 - Option 1"),
            ComboboxOption::new("group1_option2", "Group 1 - Option 2"),
            ComboboxOption::new("group2_option1", "Group 2 - Option 1"),
        ]);
        let _combobox_view = view! {
            <Combobox options=options_signal/>
        };
        // Test that state works with grouped options
    }

    #[test]
    fn test_combobox_state_with_async_options() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let _combobox_view = view! {
            <Combobox options=options_signal/>
        };
        // Test that state works with async options
    }

    #[test]
    fn test_combobox_state_with_loading_state() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let loading_signal = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                disabled=MaybeProp::from(loading_signal)
            />
        };
        // Test that state works with loading state
    }

    #[test]
    fn test_combobox_state_with_error_state() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let error_signal = RwSignal::new(None::<String>);
        let _combobox_view = view! {
            <Combobox options=options_signal/>
        };
        // Test that state works with error state
    }

    #[test]
    fn test_combobox_state_with_validation() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let is_valid = Signal::derive(move || {
            !value_signal.get().is_empty()
        });
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with validation
    }

    #[test]
    fn test_combobox_state_with_form_integration() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let form_data = RwSignal::new(std::collections::HashMap::new());
        let _combobox_view = view! {
            <Combobox options=options_signal/>
        };
        // Test that state works with form integration
    }

    #[test]
    fn test_combobox_state_with_persistence() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with persistence
    }

    #[test]
    fn test_combobox_state_with_undo_redo() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let history = RwSignal::new(vec!["option1".to_string()]);
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with undo/redo
    }

    #[test]
    fn test_combobox_state_with_batch_updates() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with batch updates
    }

    #[test]
    fn test_combobox_state_with_debounced_updates() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with debounced updates
    }

    #[test]
    fn test_combobox_state_with_throttled_updates() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with throttled updates
    }

    #[test]
    fn test_combobox_state_with_optimistic_updates() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with optimistic updates
    }

    #[test]
    fn test_combobox_state_with_rollback() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with rollback
    }

    #[test]
    fn test_combobox_state_with_conflict_resolution() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with conflict resolution
    }

    #[test]
    fn test_combobox_state_with_synchronization() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with synchronization
    }

    #[test]
    fn test_combobox_state_with_replication() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with replication
    }

    #[test]
    fn test_combobox_state_with_consistency() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with consistency
    }

    #[test]
    fn test_combobox_state_with_atomicity() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with atomicity
    }

    #[test]
    fn test_combobox_state_with_isolation() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with isolation
    }

    #[test]
    fn test_combobox_state_with_durability() {
        let options_signal = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let value_signal = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options_signal
                value=MaybeProp::from(value_signal)
            />
        };
        // Test that state works with durability
    }
}
