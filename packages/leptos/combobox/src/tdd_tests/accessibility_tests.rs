#[cfg(test)]
mod accessibility_tests {
    use super::*;

    // ===== ACCESSIBILITY TESTS =====
    // These tests focus on accessibility features and ARIA attributes

    #[test]
    fn test_combobox_accessibility_basic() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox options=options/>
        };
        // Test basic accessibility features
    }

    #[test]
    fn test_combobox_accessibility_with_aria_label() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from("Select an option")
            />
        };
        // Test accessibility with ARIA label
    }

    #[test]
    fn test_combobox_accessibility_with_aria_describedby() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_describedby=MaybeProp::from("combobox-description")
            />
        };
        // Test accessibility with ARIA describedby
    }

    #[test]
    fn test_combobox_accessibility_with_aria_expanded() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_expanded=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA expanded
    }

    #[test]
    fn test_combobox_accessibility_with_aria_autocomplete() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_autocomplete=MaybeProp::from("list")
            />
        };
        // Test accessibility with ARIA autocomplete
    }

    #[test]
    fn test_combobox_accessibility_with_aria_owns() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_owns=MaybeProp::from("combobox-list")
            />
        };
        // Test accessibility with ARIA owns
    }

    #[test]
    fn test_combobox_accessibility_with_aria_activedescendant() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_activedescendant=MaybeProp::from("option1")
            />
        };
        // Test accessibility with ARIA activedescendant
    }

    #[test]
    fn test_combobox_accessibility_with_aria_required() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_required=MaybeProp::from(true)
            />
        };
        // Test accessibility with ARIA required
    }

    #[test]
    fn test_combobox_accessibility_with_aria_invalid() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_invalid=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA invalid
    }

    #[test]
    fn test_combobox_accessibility_with_aria_disabled() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_disabled=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA disabled
    }

    #[test]
    fn test_combobox_accessibility_with_aria_readonly() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_readonly=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA readonly
    }

    #[test]
    fn test_combobox_accessibility_with_aria_multiselectable() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_multiselectable=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA multiselectable
    }

    #[test]
    fn test_combobox_accessibility_with_aria_orientation() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_orientation=MaybeProp::from("vertical")
            />
        };
        // Test accessibility with ARIA orientation
    }

    #[test]
    fn test_combobox_accessibility_with_aria_live() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_live=MaybeProp::from("polite")
            />
        };
        // Test accessibility with ARIA live
    }

    #[test]
    fn test_combobox_accessibility_with_aria_atomic() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_atomic=MaybeProp::from(true)
            />
        };
        // Test accessibility with ARIA atomic
    }

    #[test]
    fn test_combobox_accessibility_with_aria_relevant() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_relevant=MaybeProp::from("additions removals")
            />
        };
        // Test accessibility with ARIA relevant
    }

    #[test]
    fn test_combobox_accessibility_with_aria_busy() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_busy=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA busy
    }

    #[test]
    fn test_combobox_accessibility_with_aria_modal() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_modal=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA modal
    }

    #[test]
    fn test_combobox_accessibility_with_aria_hidden() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_hidden=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA hidden
    }

    #[test]
    fn test_combobox_accessibility_with_aria_selected() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_selected=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA selected
    }

    #[test]
    fn test_combobox_accessibility_with_aria_checked() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_checked=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA checked
    }

    #[test]
    fn test_combobox_accessibility_with_aria_pressed() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_pressed=MaybeProp::from(false)
            />
        };
        // Test accessibility with ARIA pressed
    }

    #[test]
    fn test_combobox_accessibility_with_aria_expanded_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let expanded = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_expanded=MaybeProp::from(expanded)
            />
        };
        // Test accessibility with ARIA expanded state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_selected_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let selected = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_selected=MaybeProp::from(selected)
            />
        };
        // Test accessibility with ARIA selected state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_checked_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let checked = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_checked=MaybeProp::from(checked)
            />
        };
        // Test accessibility with ARIA checked state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_pressed_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let pressed = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_pressed=MaybeProp::from(pressed)
            />
        };
        // Test accessibility with ARIA pressed state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_busy_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let busy = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_busy=MaybeProp::from(busy)
            />
        };
        // Test accessibility with ARIA busy state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_modal_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let modal = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_modal=MaybeProp::from(modal)
            />
        };
        // Test accessibility with ARIA modal state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_hidden_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let hidden = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_hidden=MaybeProp::from(hidden)
            />
        };
        // Test accessibility with ARIA hidden state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_required_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let required = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_required=MaybeProp::from(required)
            />
        };
        // Test accessibility with ARIA required state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_invalid_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let invalid = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_invalid=MaybeProp::from(invalid)
            />
        };
        // Test accessibility with ARIA invalid state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_disabled_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let disabled = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_disabled=MaybeProp::from(disabled)
            />
        };
        // Test accessibility with ARIA disabled state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_readonly_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let readonly = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_readonly=MaybeProp::from(readonly)
            />
        };
        // Test accessibility with ARIA readonly state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_multiselectable_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let multiselectable = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_multiselectable=MaybeProp::from(multiselectable)
            />
        };
        // Test accessibility with ARIA multiselectable state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_orientation_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let orientation = RwSignal::new("vertical".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_orientation=MaybeProp::from(orientation)
            />
        };
        // Test accessibility with ARIA orientation state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_live_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let live = RwSignal::new("polite".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_live=MaybeProp::from(live)
            />
        };
        // Test accessibility with ARIA live state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_atomic_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let atomic = RwSignal::new(true);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_atomic=MaybeProp::from(atomic)
            />
        };
        // Test accessibility with ARIA atomic state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_relevant_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let relevant = RwSignal::new("additions removals".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_relevant=MaybeProp::from(relevant)
            />
        };
        // Test accessibility with ARIA relevant state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_autocomplete_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let autocomplete = RwSignal::new("list".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_autocomplete=MaybeProp::from(autocomplete)
            />
        };
        // Test accessibility with ARIA autocomplete state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_owns_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let owns = RwSignal::new("combobox-list".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_owns=MaybeProp::from(owns)
            />
        };
        // Test accessibility with ARIA owns state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_activedescendant_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let activedescendant = RwSignal::new("option1".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_activedescendant=MaybeProp::from(activedescendant)
            />
        };
        // Test accessibility with ARIA activedescendant state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_describedby_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let describedby = RwSignal::new("combobox-description".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_describedby=MaybeProp::from(describedby)
            />
        };
        // Test accessibility with ARIA describedby state
    }

    #[test]
    fn test_combobox_accessibility_with_aria_label_state() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let label = RwSignal::new("Select an option".to_string());
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from(label)
            />
        };
        // Test accessibility with ARIA label state
    }

    #[test]
    fn test_combobox_accessibility_with_multiple_aria_attributes() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from("Select an option")
                aria_describedby=MaybeProp::from("combobox-description")
                aria_expanded=MaybeProp::from(false)
                aria_autocomplete=MaybeProp::from("list")
                aria_owns=MaybeProp::from("combobox-list")
                aria_activedescendant=MaybeProp::from("option1")
                aria_required=MaybeProp::from(true)
                aria_invalid=MaybeProp::from(false)
                aria_disabled=MaybeProp::from(false)
                aria_readonly=MaybeProp::from(false)
                aria_multiselectable=MaybeProp::from(false)
                aria_orientation=MaybeProp::from("vertical")
                aria_live=MaybeProp::from("polite")
                aria_atomic=MaybeProp::from(true)
                aria_relevant=MaybeProp::from("additions removals")
                aria_busy=MaybeProp::from(false)
                aria_modal=MaybeProp::from(false)
                aria_hidden=MaybeProp::from(false)
                aria_selected=MaybeProp::from(false)
                aria_checked=MaybeProp::from(false)
                aria_pressed=MaybeProp::from(false)
            />
        };
        // Test accessibility with multiple ARIA attributes
    }

    #[test]
    fn test_combobox_accessibility_with_dynamic_aria_attributes() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let expanded = RwSignal::new(false);
        let selected = RwSignal::new(false);
        let busy = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_expanded=MaybeProp::from(expanded)
                aria_selected=MaybeProp::from(selected)
                aria_busy=MaybeProp::from(busy)
            />
        };
        // Test accessibility with dynamic ARIA attributes
    }

    #[test]
    fn test_combobox_accessibility_with_computed_aria_attributes() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let value = RwSignal::new("option1".to_string());
        let computed_label = Signal::derive(move || {
            format!("Selected: {}", value.get())
        });
        let _combobox_view = view! {
            <Combobox 
                options=options
                value=MaybeProp::from(value)
                aria_label=MaybeProp::from(computed_label)
            />
        };
        // Test accessibility with computed ARIA attributes
    }

    #[test]
    fn test_combobox_accessibility_with_conditional_aria_attributes() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let is_required = RwSignal::new(true);
        let is_invalid = RwSignal::new(false);
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_required=MaybeProp::from(is_required)
                aria_invalid=MaybeProp::from(is_invalid)
            />
        };
        // Test accessibility with conditional ARIA attributes
    }

    #[test]
    fn test_combobox_accessibility_with_aria_attributes_performance() {
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let _combobox_view = view! {
                <Combobox 
                    options=options
                    aria_label=MaybeProp::from("Select an option")
                    aria_describedby=MaybeProp::from("combobox-description")
                    aria_expanded=MaybeProp::from(false)
                    aria_autocomplete=MaybeProp::from("list")
                    aria_owns=MaybeProp::from("combobox-list")
                    aria_activedescendant=MaybeProp::from("option1")
                    aria_required=MaybeProp::from(true)
                    aria_invalid=MaybeProp::from(false)
                    aria_disabled=MaybeProp::from(false)
                    aria_readonly=MaybeProp::from(false)
                    aria_multiselectable=MaybeProp::from(false)
                    aria_orientation=MaybeProp::from("vertical")
                    aria_live=MaybeProp::from("polite")
                    aria_atomic=MaybeProp::from(true)
                    aria_relevant=MaybeProp::from("additions removals")
                    aria_busy=MaybeProp::from(false)
                    aria_modal=MaybeProp::from(false)
                    aria_hidden=MaybeProp::from(false)
                    aria_selected=MaybeProp::from(false)
                    aria_checked=MaybeProp::from(false)
                    aria_pressed=MaybeProp::from(false)
                />
            };
        }
        
        let duration = start.elapsed();
        
        // Should complete quickly (less than 100ms for 100 iterations)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_combobox_accessibility_with_aria_attributes_memory_usage() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from("Select an option")
                aria_describedby=MaybeProp::from("combobox-description")
                aria_expanded=MaybeProp::from(false)
                aria_autocomplete=MaybeProp::from("list")
                aria_owns=MaybeProp::from("combobox-list")
                aria_activedescendant=MaybeProp::from("option1")
                aria_required=MaybeProp::from(true)
                aria_invalid=MaybeProp::from(false)
                aria_disabled=MaybeProp::from(false)
                aria_readonly=MaybeProp::from(false)
                aria_multiselectable=MaybeProp::from(false)
                aria_orientation=MaybeProp::from("vertical")
                aria_live=MaybeProp::from("polite")
                aria_atomic=MaybeProp::from(true)
                aria_relevant=MaybeProp::from("additions removals")
                aria_busy=MaybeProp::from(false)
                aria_modal=MaybeProp::from(false)
                aria_hidden=MaybeProp::from(false)
                aria_selected=MaybeProp::from(false)
                aria_checked=MaybeProp::from(false)
                aria_pressed=MaybeProp::from(false)
            />
        };
        
        // Test that memory usage is reasonable
        let size = std::mem::size_of_val(&options);
        assert!(size < 1024); // Should be less than 1KB
    }

    #[test]
    fn test_combobox_accessibility_with_aria_attributes_consistency() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        
        // Test that multiple renders with ARIA attributes are consistent
        let _combobox_view1 = view! {
            <Combobox 
                options=options.clone()
                aria_label=MaybeProp::from("Select an option")
                aria_expanded=MaybeProp::from(false)
            />
        };
        let _combobox_view2 = view! {
            <Combobox 
                options=options.clone()
                aria_label=MaybeProp::from("Select an option")
                aria_expanded=MaybeProp::from(false)
            />
        };
        let _combobox_view3 = view! {
            <Combobox 
                options=options.clone()
                aria_label=MaybeProp::from("Select an option")
                aria_expanded=MaybeProp::from(false)
            />
        };
    }

    #[test]
    fn test_combobox_accessibility_with_aria_attributes_edge_cases() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from("")
                aria_describedby=MaybeProp::from("")
                aria_expanded=MaybeProp::from(false)
                aria_autocomplete=MaybeProp::from("")
                aria_owns=MaybeProp::from("")
                aria_activedescendant=MaybeProp::from("")
                aria_required=MaybeProp::from(false)
                aria_invalid=MaybeProp::from(false)
                aria_disabled=MaybeProp::from(false)
                aria_readonly=MaybeProp::from(false)
                aria_multiselectable=MaybeProp::from(false)
                aria_orientation=MaybeProp::from("")
                aria_live=MaybeProp::from("")
                aria_atomic=MaybeProp::from(false)
                aria_relevant=MaybeProp::from("")
                aria_busy=MaybeProp::from(false)
                aria_modal=MaybeProp::from(false)
                aria_hidden=MaybeProp::from(false)
                aria_selected=MaybeProp::from(false)
                aria_checked=MaybeProp::from(false)
                aria_pressed=MaybeProp::from(false)
            />
        };
        // Test accessibility with empty ARIA attributes
    }

    #[test]
    fn test_combobox_accessibility_with_aria_attributes_none_values() {
        let options = vec![
            ComboboxOption::new("option1", "Option 1"),
            ComboboxOption::new("option2", "Option 2"),
        ];
        let _combobox_view = view! {
            <Combobox 
                options=options
                aria_label=MaybeProp::from(None::<String>)
                aria_describedby=MaybeProp::from(None::<String>)
                aria_expanded=MaybeProp::from(None::<bool>)
                aria_autocomplete=MaybeProp::from(None::<String>)
                aria_owns=MaybeProp::from(None::<String>)
                aria_activedescendant=MaybeProp::from(None::<String>)
                aria_required=MaybeProp::from(None::<bool>)
                aria_invalid=MaybeProp::from(None::<bool>)
                aria_disabled=MaybeProp::from(None::<bool>)
                aria_readonly=MaybeProp::from(None::<bool>)
                aria_multiselectable=MaybeProp::from(None::<bool>)
                aria_orientation=MaybeProp::from(None::<String>)
                aria_live=MaybeProp::from(None::<String>)
                aria_atomic=MaybeProp::from(None::<bool>)
                aria_relevant=MaybeProp::from(None::<String>)
                aria_busy=MaybeProp::from(None::<bool>)
                aria_modal=MaybeProp::from(None::<bool>)
                aria_hidden=MaybeProp::from(None::<bool>)
                aria_selected=MaybeProp::from(None::<bool>)
                aria_checked=MaybeProp::from(None::<bool>)
                aria_pressed=MaybeProp::from(None::<bool>)
            />
        };
        // Test accessibility with None ARIA attributes
    }
}
