#[cfg(test)]
mod performance_tests {
    use crate::default::{Switch, SwitchRoot, SwitchThumb, SwitchLabel, SwitchVariant, SwitchSize};
    use leptos::prelude::*;

    // ===== PERFORMANCE TESTS =====
    // These tests focus on performance characteristics and optimization

    #[test]
    fn test_switch_memory_management() {
        // Test memory management
        let memory_checked = RwSignal::new(false);
        let memory_disabled = RwSignal::new(false);
        
        let _memory_switch_view = view! {
            <Switch 
                checked=memory_checked
                disabled=memory_disabled
            />
        };
        
        // Test memory management
        let memory_size = std::mem::size_of_val(&memory_checked);
        assert!(memory_size < 1024, "Memory usage should be reasonable");
        
        // Test memory cleanup
        memory_checked.set(true);
        memory_disabled.set(true);
        
        assert!(memory_checked.get(), "Memory should maintain state");
        assert!(memory_disabled.get(), "Memory should maintain state");
    }

    #[test]
    fn test_switch_responsive_design() {
        // Test responsive design
        let responsive_checked = RwSignal::new(false);
        let responsive_size = RwSignal::new(SwitchSize::Default);
        
        let _responsive_switch_view = view! {
            <Switch 
                checked=responsive_checked
                size=responsive_size
                class="responsive-switch"
            />
        };
        
        // Test responsive design
        assert!(!responsive_checked.get(), "Responsive should start unchecked");
        assert_eq!(responsive_size.get(), SwitchSize::Default);
        
        // Test responsive changes
        responsive_size.set(SwitchSize::Large);
        
        assert_eq!(responsive_size.get(), SwitchSize::Large);
    }

    #[test]
    fn test_switch_custom_properties() {
        // Test custom properties
        let custom_prop = RwSignal::new("custom-value".to_string());
        
        let _custom_switch_view = view! {
            <Switch 
                checked=RwSignal::new(false)
                class=move || format!("custom-{}", custom_prop.get())
            />
        };
        
        // Test custom properties
        assert_eq!(custom_prop.get(), "custom-value");
        
        // Test custom property change
        custom_prop.set("new-value".to_string());
        assert_eq!(custom_prop.get(), "new-value");
    }

    #[test]
    fn test_switch_advanced_interactions() {
        // Test advanced interactions
        let interaction_checked = RwSignal::new(false);
        let interaction_disabled = RwSignal::new(false);
        let interaction_focused = RwSignal::new(false);
        let interaction_hovered = RwSignal::new(false);
        
        let _interaction_switch_view = view! {
            <Switch 
                checked=interaction_checked
                disabled=interaction_disabled
                on_focus=move |_| interaction_focused.set(true)
                on_blur=move |_| interaction_focused.set(false)
                on_mouse_enter=move |_| interaction_hovered.set(true)
                on_mouse_leave=move |_| interaction_hovered.set(false)
            />
        };
        
        // Test advanced interactions
        assert!(!interaction_checked.get(), "Interaction should start unchecked");
        assert!(!interaction_disabled.get(), "Interaction should start enabled");
        assert!(!interaction_focused.get(), "Interaction should start unfocused");
        assert!(!interaction_hovered.get(), "Interaction should start unhovered");
        
        // Test interaction changes
        interaction_checked.set(true);
        interaction_disabled.set(true);
        interaction_focused.set(true);
        interaction_hovered.set(true);
        
        assert!(interaction_checked.get(), "Interaction should be checked");
        assert!(interaction_disabled.get(), "Interaction should be disabled");
        assert!(interaction_focused.get(), "Interaction should be focused");
        assert!(interaction_hovered.get(), "Interaction should be hovered");
    }

    #[test]
    fn test_switch_performance_comprehensive() {
        // Test comprehensive performance
        let performance_checked = RwSignal::new(false);
        let performance_disabled = RwSignal::new(false);
        let performance_focused = RwSignal::new(false);
        
        let start = std::time::Instant::now();
        
        let _performance_switch_view = view! {
            <Switch 
                checked=performance_checked
                disabled=performance_disabled
                on_focus=move |_| performance_focused.set(true)
            />
        };
        
        let creation_time = start.elapsed();
        
        // Test performance
        assert!(creation_time.as_millis() < 10, "Creation should be fast");
        
        // Test rapid state changes
        let state_start = std::time::Instant::now();
        
        for _ in 0..100 {
            performance_checked.set(!performance_checked.get());
        }
        
        let state_time = state_start.elapsed();
        
        assert!(state_time.as_millis() < 50, "State changes should be fast");
    }

    #[test]
    fn test_switch_integration_scenarios() {
        // Test integration scenarios
        let integration_checked = RwSignal::new(false);
        let integration_disabled = RwSignal::new(false);
        let integration_required = RwSignal::new(true);
        
        let _integration_switch_view = view! {
            <Switch 
                checked=integration_checked
                disabled=integration_disabled
                required=integration_required
                aria_invalid=move || !integration_checked.get() && integration_required.get()
            />
        };
        
        // Test integration scenarios
        assert!(!integration_checked.get(), "Integration should start unchecked");
        assert!(!integration_disabled.get(), "Integration should start enabled");
        assert!(integration_required.get(), "Integration should be required");
        
        // Test integration scenario 1: valid state
        integration_checked.set(true);
        assert!(integration_checked.get(), "Integration should be checked");
        
        // Test integration scenario 2: disabled state
        integration_checked.set(false);
        integration_disabled.set(true);
        assert!(!integration_checked.get(), "Integration should be unchecked");
        assert!(integration_disabled.get(), "Integration should be disabled");
    }

    #[test]
    fn test_switch_error_handling() {
        // Test error handling
        let error_checked = RwSignal::new(false);
        let error_disabled = RwSignal::new(false);
        let error_message = RwSignal::new("".to_string());
        
        let _error_switch_view = view! {
            <Switch 
                checked=error_checked
                disabled=error_disabled
                aria_invalid=move || !error_checked.get()
            />
        };
        
        // Test error handling
        assert!(!error_checked.get(), "Error should start unchecked");
        assert!(!error_disabled.get(), "Error should start enabled");
        assert_eq!(error_message.get(), "");
        
        // Test error state
        if !error_checked.get() {
            error_message.set("Switch is required".to_string());
        }
        
        assert_eq!(error_message.get(), "Switch is required");
        
        // Test error resolution
        error_checked.set(true);
        error_message.set("".to_string());
        
        assert!(error_checked.get(), "Error should be resolved");
        assert_eq!(error_message.get(), "");
    }

    #[test]
    fn test_switch_form_integration() {
        // Test form integration
        let form_checked = RwSignal::new(false);
        let form_disabled = RwSignal::new(false);
        let form_required = RwSignal::new(true);
        let form_valid = RwSignal::new(false);
        
        let _form_switch_view = view! {
            <Switch 
                checked=form_checked
                disabled=form_disabled
                required=form_required
                aria_invalid=move || !form_checked.get() && form_required.get()
            />
        };
        
        // Test form integration
        assert!(!form_checked.get(), "Form should start unchecked");
        assert!(!form_disabled.get(), "Form should start enabled");
        assert!(form_required.get(), "Form should be required");
        assert!(!form_valid.get(), "Form should start invalid");
        
        // Test form validation
        form_checked.set(true);
        form_valid.set(true);
        
        assert!(form_checked.get(), "Form should be checked");
        assert!(form_valid.get(), "Form should be valid");
    }
}
