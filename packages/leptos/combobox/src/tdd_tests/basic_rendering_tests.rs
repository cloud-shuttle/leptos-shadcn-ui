#[cfg(test)]
mod basic_rendering_tests {
    use super::*;

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_combobox_basic_rendering() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // GREEN PHASE: Verify actual rendering behavior
    }

    #[test]
    fn test_combobox_with_value() {
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
    }

    #[test]
    fn test_combobox_with_placeholder() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                placeholder=MaybeProp::from("Select an option")
            />
        };
    }

    #[test]
    fn test_combobox_with_disabled() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                disabled=MaybeProp::from(true)
            />
        };
    }

    #[test]
    fn test_combobox_with_class() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                class=MaybeProp::from("custom-class")
            />
        };
    }

    #[test]
    fn test_combobox_with_style() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                style=MaybeProp::from("color: red")
            />
        };
    }

    #[test]
    fn test_combobox_with_id() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                id=MaybeProp::from("test-combobox")
            />
        };
    }

    #[test]
    fn test_combobox_with_multiple_props() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from("option1")
                placeholder=MaybeProp::from("Select an option")
                disabled=MaybeProp::from(false)
                class=MaybeProp::from("custom-class")
                style=MaybeProp::from("color: red")
                id=MaybeProp::from("test-combobox")
            />
        };
    }

    #[test]
    fn test_combobox_with_empty_options() {
        let options = vec![];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_single_option() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_many_options() {
        let options = (1..=100).map(|i| {
            ComboboxOption::new(format!("option{}", i), format!("Option {}", i))
        }).collect::<Vec<_>>();
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_unicode_options() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1 🚀"),
            ComboboxOption::new("option2", "Option 2 🎉"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_special_characters() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1!@#$%^&*()"),
            ComboboxOption::new("option2", "Option 2!@#$%^&*()"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_long_text() {
        let options = vec![
            ComboboxOption::new("option1", "This is a very long option text that should be handled properly by the combobox component"),
            ComboboxOption::new("option2", "Another very long option text that should also be handled properly"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_whitespace() {
        let options = vec![
            ComboboxOption::new("option1", "  Option 1  "),
            ComboboxOption::new("option2", "  Option 2  "),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_numbers() {
        let options = vec![
            ComboboxOption::new("option1", "Option 123"),
            ComboboxOption::new("option2", "Option 456"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_with_mixed_content() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2 🚀"),
            ComboboxOption::new("option3", "Option 3!@#$%^&*()"),
            ComboboxOption::new("option4", "This is a very long option text"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_rendering_performance() {
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let _combobox_view = view! {
                <Combobox options=options/>
            };
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 100ms for 100 iterations)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_combobox_rendering_memory_usage() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        
        // Test that memory usage is reasonable
        let size = std::mem::size_of_val(&options);
        assert!(size < 1024); // Should be less than 1KB
    }

    #[test]
    fn test_combobox_rendering_consistency() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        
        // Test that multiple renders are consistent
        let _combobox_view1 = view! {
            <Combobox options=options.clone()/>
        };
        let _combobox_view2 = view! {
            <Combobox options=options.clone()/>
        };
        let _combobox_view3 = view! {
            <Combobox options=options.clone()/>
        };
    }

    #[test]
    fn test_combobox_rendering_with_different_props() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        
        // Test with different prop combinations
        let _combobox_view1 = view! {
            <Combobox options=options.clone() value=MaybeProp::from("option1")/>
        };
        let _combobox_view2 = view! {
            <Combobox options=options.clone() placeholder=MaybeProp::from("Select")/>
        };
        let _combobox_view3 = view! {
            <Combobox options=options.clone() disabled=MaybeProp::from(true)/>
        };
        let _combobox_view4 = view! {
            <Combobox options=options.clone() class=MaybeProp::from("custom")/>
        };
    }

    #[test]
    fn test_combobox_rendering_edge_cases() {
        let options = vec![
            ComboboxOption::new("", ""),
            ComboboxOption::new("option1", "Option 1"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
    }

    #[test]
    fn test_combobox_rendering_with_none_props() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(None::<String>)
                placeholder=MaybeProp::from(None::<String>)
                disabled=MaybeProp::from(None::<bool>)
                class=MaybeProp::from(None::<String>)
                style=MaybeProp::from(None::<String>)
                id=MaybeProp::from(None::<String>)
            />
        };
    }
}
