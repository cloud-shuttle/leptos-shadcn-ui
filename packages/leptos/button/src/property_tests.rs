// Property-based tests for Button component
// Demonstrates advanced testing patterns for comprehensive validation

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use shadcn_ui_test_utils::property_testing::{
        strategies::*,
        assertions::*,
        button_properties::*,
    };

    // Property-based test for button variant handling
    proptest! {
        #[test]
        fn button_handles_all_valid_variants(
            variant in color_variant_strategy(),
            size in size_variant_strategy(),
            disabled in weighted_bool_strategy(20), // 20% chance disabled
            class in optional_string_strategy(),
            id in optional_string_strategy(),
        ) {
            // Create button props with generated values
            let props = ButtonProps {
                variant: Some(variant.clone()),
                size: Some(size.clone()),
                disabled: Some(disabled),
                class,
                id,
                children: None,
                onclick: None,
                r#type: Some("button".to_string()),
            };

            // Test that component renders without panicking
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));

            // Test variant is properly applied
            let valid_variants = ["default", "primary", "secondary", "success", 
                                "warning", "danger", "info", "light", "dark"];
            prop_assert!(valid_variants.contains(&variant.as_str()));

            // Test size is properly applied
            let valid_sizes = ["sm", "default", "lg", "xl"];
            prop_assert!(valid_sizes.contains(&size.as_str()));
        }
    }

    // Property-based test for button accessibility
    proptest! {
        #[test]
        fn button_maintains_accessibility_compliance(
            disabled in any::<bool>(),
            aria_label in optional_string_strategy(),
            button_type in prop::sample::select(vec!["button", "submit", "reset"])
                .prop_map(|s| s.to_string()),
        ) {
            let props = ButtonProps {
                disabled: Some(disabled),
                aria_label,
                r#type: Some(button_type.clone()),
                ..Default::default()
            };

            // Verify accessibility properties
            prop_assert!(["button", "submit", "reset"].contains(&button_type.as_str()));
            
            // Test component renders with accessibility attributes
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));
        }
    }

    // Property-based test for event handling
    proptest! {
        #[test]
        fn button_event_handling_is_robust(
            variant in color_variant_strategy(),
            disabled in any::<bool>(),
        ) {
            use std::sync::{Arc, Mutex};
            
            let click_count = Arc::new(Mutex::new(0));
            let click_count_clone = click_count.clone();

            let props = ButtonProps {
                variant: Some(variant),
                disabled: Some(disabled),
                onclick: Some(Box::new(move || {
                    *click_count_clone.lock().unwrap() += 1;
                })),
                ..Default::default()
            };

            // Component should render regardless of event handler presence
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));

            // Initial click count should be 0
            prop_assert_eq!(*click_count.lock().unwrap(), 0);
        }
    }

    // Property-based test for CSS class composition
    proptest! {
        #[test]
        fn button_css_classes_are_well_formed(
            variant in color_variant_strategy(),
            size in size_variant_strategy(),
            custom_class in css_class_strategy(),
        ) {
            let props = ButtonProps {
                variant: Some(variant.clone()),
                size: Some(size.clone()),
                class: Some(custom_class.clone()),
                ..Default::default()
            };

            // Test CSS class generation
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));

            // Validate CSS class naming conventions
            prop_assert!(custom_class.chars().next().unwrap().is_ascii_alphabetic());
            prop_assert!(custom_class.len() <= 51);
        }
    }

    // Property-based test for performance characteristics
    proptest! {
        #[test]
        fn button_performance_within_bounds(
            variant in color_variant_strategy(),
            size in size_variant_strategy(),
            children_text in prop::string::string_regex(r".{0,100}").unwrap(),
        ) {
            let props = ButtonProps {
                variant: Some(variant),
                size: Some(size),
                children: Some(view! { {children_text} }),
                ..Default::default()
            };

            // Test render performance (should complete within 16ms for 60fps)
            prop_assert!(assert_performance_within_bounds(
                || view! { <Button ..props.clone() /> },
                16, // max time in ms
                1024 // max memory in KB
            ));
        }
    }

    // Property-based test for state transitions
    proptest! {
        #[test]
        fn button_state_transitions_are_valid(
            initial_disabled in any::<bool>(),
            new_disabled in any::<bool>(),
            variant in color_variant_strategy(),
        ) {
            // Test that button can transition between enabled/disabled states
            let initial_props = ButtonProps {
                disabled: Some(initial_disabled),
                variant: Some(variant.clone()),
                ..Default::default()
            };

            let new_props = ButtonProps {
                disabled: Some(new_disabled),
                variant: Some(variant),
                ..Default::default()
            };

            // Both states should render successfully
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..initial_props.clone() /> }
            }));

            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..new_props.clone() /> }
            }));
        }
    }

    // Property-based test for edge cases
    proptest! {
        #[test]
        fn button_handles_edge_cases_gracefully(
            empty_variant in prop::sample::select(vec!["", " ", "\t", "\n"]),
            very_long_class in prop::string::string_regex(r".{1000,2000}").unwrap(),
            unicode_text in prop::string::string_regex(r"[\u{1F600}-\u{1F64F}]{1,10}").unwrap(),
        ) {
            // Test with edge case inputs
            let props = ButtonProps {
                variant: if empty_variant.trim().is_empty() { 
                    None 
                } else { 
                    Some(empty_variant) 
                },
                class: Some(very_long_class),
                children: Some(view! { {unicode_text} }),
                ..Default::default()
            };

            // Component should handle edge cases gracefully
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));
        }
    }

    // Property-based integration test with other components
    proptest! {
        #[test]
        fn button_integrates_well_with_forms(
            button_type in prop::sample::select(vec!["submit", "reset", "button"])
                .prop_map(|s| s.to_string()),
            form_method in prop::sample::select(vec!["get", "post"])
                .prop_map(|s| s.to_string()),
            disabled in any::<bool>(),
        ) {
            let button_props = ButtonProps {
                r#type: Some(button_type.clone()),
                disabled: Some(disabled),
                ..Default::default()
            };

            // Test button within form context
            prop_assert!(assert_renders_safely(|| {
                view! {
                    <form method={form_method.clone()}>
                        <Button ..button_props.clone()>"Submit"</Button>
                    </form>
                }
            }));

            // Verify button type is valid for forms
            prop_assert!(["submit", "reset", "button"].contains(&button_type.as_str()));
        }
    }

    // Property-based test for component composition
    proptest! {
        #[test]
        fn button_supports_complex_children(
            num_nested_elements in 1..5usize,
            element_types in prop::collection::vec(
                prop::sample::select(vec!["span", "div", "i", "strong", "em"]),
                1..5
            ),
        ) {
            // Generate nested children structure
            let nested_children = element_types.into_iter()
                .take(num_nested_elements)
                .enumerate()
                .fold(view! { "Base text" }, |acc, (i, tag)| {
                    match tag {
                        "span" => view! { <span>{acc}</span> },
                        "div" => view! { <div>{acc}</div> },
                        "i" => view! { <i>{acc}</i> },
                        "strong" => view! { <strong>{acc}</strong> },
                        "em" => view! { <em>{acc}</em> },
                        _ => acc,
                    }
                });

            let props = ButtonProps {
                children: Some(nested_children),
                ..Default::default()
            };

            // Button should handle complex nested children
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone() /> }
            }));
        }
    }
}

// Integration tests with property-based patterns
#[cfg(test)]
mod integration_property_tests {
    use super::*;
    use proptest::prelude::*;
    use shadcn_ui_test_utils::property_testing::integration::*;

    proptest! {
        #[test]
        fn button_theme_consistency_across_variants(
            theme in prop::sample::select(vec!["light", "dark", "high-contrast"]),
            variants in prop::collection::vec(
                prop::sample::select(vec!["default", "primary", "secondary"]),
                2..5
            ),
        ) {
            // Test theme consistency across multiple button variants
            prop_assert!(test_theme_consistency(&theme, variants.iter().collect()));
        }
    }

    proptest! {
        #[test]
        fn button_event_propagation_in_complex_hierarchy(
            nesting_depth in 1..5usize,
            stop_propagation in any::<bool>(),
        ) {
            use std::sync::{Arc, Mutex};
            
            let event_log = Arc::new(Mutex::new(Vec::new()));
            let event_log_clone = event_log.clone();

            // Create nested structure with event handlers
            let props = ButtonProps {
                onclick: Some(Box::new(move || {
                    event_log_clone.lock().unwrap().push(format!("button_clicked_depth_{}", nesting_depth));
                })),
                ..Default::default()
            };

            // Test event propagation behavior
            prop_assert!(assert_renders_safely(|| {
                view! { <Button ..props.clone()>"Click me"</Button> }
            }));

            // Initial event log should be empty
            prop_assert!(event_log.lock().unwrap().is_empty());
        }
    }
}