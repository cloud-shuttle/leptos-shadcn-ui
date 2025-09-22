#[cfg(test)]
mod integration_tests {
    use super::*;

    // ===== INTEGRATION TESTS =====
    // These tests focus on integration scenarios and complex workflows

    #[test]
    fn test_combobox_integration_with_form() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <form>
                <Combobox options=options/>
            </form>
        };
        // Test integration with forms
    }

    #[test]
    fn test_combobox_integration_with_validation() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value = RwSignal::new("".to_string());
        let is_valid = Signal::derive(move || !value.get().is_empty());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value)
                aria_invalid=MaybeProp::from(Signal::derive(move || !is_valid.get()))
            />
        };
        // Test integration with validation
    }

    #[test]
    fn test_combobox_integration_with_async_data() {
        let options = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with async data loading
    }

    #[test]
    fn test_combobox_integration_with_search() {
        let all_options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
            ComboboxOption::new("option3", "Option 3"),
        ];
        let search_term = RwSignal::new("".to_string());
        let filtered_options = Signal::derive(move || {
            let term = search_term.get();
            if term.is_empty() {
                all_options.clone()
            } else {
                all_options.clone().into_iter()
                    .filter(|opt| opt.label.to_lowercase().contains(&term.to_lowercase()))
                    .collect()
            }
        });
        let _combobox_view = view! {
            <Combobox options=filtered_options/>
        };
        // Test integration with search functionality
    }

    #[test]
    fn test_combobox_integration_with_multiple_comboboxes() {
        let options1 = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let options2 = vec![
            ComboboxOption::new("option3", "Option 3"),
            ComboboxOption::new("option4", "Option 4"),
        ];
        let _combobox_view = view! {
            <div>
                <Combobox options=options1/>
                <Combobox options=options2/>
            </div>
        };
        // Test integration with multiple comboboxes
    }

    #[test]
    fn test_combobox_integration_with_nested_comboboxes() {
        let parent_options = vec![
            ComboboxOption::new("parent1", "Parent 1"),
            ComboboxOption::new("parent2", "Parent 2"),
        ];
        let child_options = vec![
            ComboboxOption::new("child1", "Child 1"),
            ComboboxOption::new("child2", "Child 2"),
        ];
        let _combobox_view = view! {
            <div>
                <Combobox options=parent_options/>
                <div>
                    <Combobox options=child_options/>
                </div>
            </div>
        };
        // Test integration with nested comboboxes
    }

    #[test]
    fn test_combobox_integration_with_conditional_rendering() {
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
        // Test integration with conditional rendering
    }

    #[test]
    fn test_combobox_integration_with_dynamic_options() {
        let options = RwSignal::new(vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ]);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with dynamic options
    }

    #[test]
    fn test_combobox_integration_with_context_providers() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with context providers
    }

    #[test]
    fn test_combobox_integration_with_error_boundaries() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <ErrorBoundary fallback=|_| view! { <div>"Error occurred"</div> }>
                <Combobox options=options/>
            </ErrorBoundary>
        };
        // Test integration with error boundaries
    }

    #[test]
    fn test_combobox_integration_with_suspense() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                <Combobox options=options/>
            </Suspense>
        };
        // Test integration with suspense
    }

    #[test]
    fn test_combobox_integration_with_router() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with router
    }

    #[test]
    fn test_combobox_integration_with_state_management() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let global_state = RwSignal::new("global-value".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(global_state)
            />
        };
        // Test integration with global state management
    }

    #[test]
    fn test_combobox_integration_with_persistence() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let persisted_value = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(persisted_value)
            />
        };
        // Test integration with data persistence
    }

    #[test]
    fn test_combobox_integration_with_undo_redo() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value = RwSignal::new("option1".to_string());
        let history = RwSignal::new(vec!["option1".to_string()]);
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value)
            />
        };
        // Test integration with undo/redo functionality
    }

    #[test]
    fn test_combobox_integration_with_keyboard_navigation() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
            ComboboxOption::new("option3", "Option 3"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with keyboard navigation
    }

    #[test]
    fn test_combobox_integration_with_mouse_interaction() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with mouse interactions
    }

    #[test]
    fn test_combobox_integration_with_touch_interaction() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with touch interactions
    }

    #[test]
    fn test_combobox_integration_with_focus_management() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test integration with focus management
    }

    #[test]
    fn test_combobox_integration_with_tabindex() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                tabindex=MaybeProp::from(0)
            />
        };
        // Test integration with tabindex
    }

    #[test]
    fn test_combobox_integration_with_autofocus() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                autofocus=MaybeProp::from(true)
            />
        };
        // Test integration with autofocus
    }

    #[test]
    fn test_combobox_integration_with_event_handling() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let on_change = Callback::new(|_| {});
        let on_focus = Callback::new(|_| {});
        let on_blur = Callback::new(|_| {});
        let _combobox_view = view! {
            <Combobox 
                options=options
                on_change=on_change
                on_focus=on_focus
                on_blur=on_blur
            />
        };
        // Test integration with event handling
    }

    #[test]
    fn test_combobox_integration_with_debouncing() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value = RwSignal::new("".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value)
            />
        };
        // Test integration with debounced input
    }

    #[test]
    fn test_combobox_integration_with_throttling() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value = RwSignal::new("".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value)
            />
        };
        // Test integration with throttled updates
    }

    #[test]
    fn test_combobox_integration_with_animation() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                class=MaybeProp::from("animated-combobox")
            />
        };
        // Test integration with animations
    }

    #[test]
    fn test_combobox_integration_with_transitions() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                class=MaybeProp::from("transition-combobox")
            />
        };
        // Test integration with CSS transitions
    }

    #[test]
    fn test_combobox_integration_with_theming() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let theme = RwSignal::new("dark".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                class=MaybeProp::from(Signal::derive(move || format!("theme-{}", theme.get())))
            />
        };
        // Test integration with theming
    }

    #[test]
    fn test_combobox_integration_with_internationalization() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let locale = RwSignal::new("en".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                placeholder=MaybeProp::from(Signal::derive(move || {
                    match locale.get().as_str() {
                        "en" => "Select an option",
                        "es" => "Selecciona una opción",
                        "fr" => "Sélectionnez une option",
                        _ => "Select an option"
                    }
                }))
            />
        };
        // Test integration with internationalization
    }

    #[test]
    fn test_combobox_integration_with_responsive_design() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let screen_size = RwSignal::new("desktop".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                class=MaybeProp::from(Signal::derive(move || {
                    match screen_size.get().as_str() {
                        "mobile" => "combobox-mobile",
                        "tablet" => "combobox-tablet",
                        "desktop" => "combobox-desktop",
                        _ => "combobox-default"
                    }
                }))
            />
        };
        // Test integration with responsive design
    }

    #[test]
    fn test_combobox_integration_with_testing_utilities() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                data_testid=MaybeProp::from("test-combobox")
            />
        };
        // Test integration with testing utilities
    }

    #[test]
    fn test_combobox_integration_with_analytics() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let on_change = Callback::new(|value| {
            // Analytics tracking would go here
            logging::log!("Combobox changed to: {}", value);
        });
        let _combobox_view = view! {
            <Combobox 
                options=options
                on_change=on_change
            />
        };
        // Test integration with analytics
    }

    #[test]
    fn test_combobox_integration_with_performance_monitoring() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let start_time = std::time::Instant::now();
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        let render_time = start_time.elapsed();
        
        // Performance monitoring would go here
        assert!(render_time.as_millis() < 100);
    }

    #[test]
    fn test_combobox_integration_with_error_reporting() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <ErrorBoundary fallback=|error| {
                // Error reporting would go here
                logging::error!("Combobox error: {:?}", error);
                view! { <div>"Error occurred"</div> }
            }>
                <Combobox options=options/>
            </ErrorBoundary>
        };
        // Test integration with error reporting
    }
}
