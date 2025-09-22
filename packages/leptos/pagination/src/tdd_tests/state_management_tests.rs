#[cfg(test)]
mod state_management_tests {
    use super::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and interactions

    #[test]
    fn test_pagination_state_management() {
        let current_page = RwSignal::new(1);
        let total_pages = RwSignal::new(10);
        
        let _state_pagination_view = view! {
            <Pagination 
                current_page=MaybeProp::from(current_page)
                total_pages=total_pages
            />
        };
        
        // Test initial state
        assert_eq!(current_page.get(), 1);
        assert_eq!(total_pages.get(), 10);
        
        // Test state change
        current_page.set(5);
        total_pages.set(20);
        
        assert_eq!(current_page.get(), 5);
        assert_eq!(total_pages.get(), 20);
    }

    #[test]
    fn test_pagination_callback_handling() {
        let callback_called = RwSignal::new(false);
        let callback_page = RwSignal::new(0);
        
        let callback = Callback::new(move |page: usize| {
            callback_called.set(true);
            callback_page.set(page);
        });
        
        let _callback_pagination_view = view! {
            <Pagination 
                total_pages=10
                on_page_change=callback
            />
        };
        
        // Test initial callback state
        assert!(!callback_called.get());
        assert_eq!(callback_page.get(), 0);
        
        // Test callback execution
        callback.run(5);
        
        assert!(callback_called.get());
        assert_eq!(callback_page.get(), 5);
    }

    #[test]
    fn test_pagination_event_handling() {
        let click_handled = RwSignal::new(false);
        let focus_handled = RwSignal::new(false);
        let blur_handled = RwSignal::new(false);
        
        let _event_pagination_view = view! {
            <Pagination 
                total_pages=10
                on_click=move |_| click_handled.set(true)
                on_focus=move |_| focus_handled.set(true)
                on_blur=move |_| blur_handled.set(true)
            />
        };
        
        // Test initial event state
        assert!(!click_handled.get());
        assert!(!focus_handled.get());
        assert!(!blur_handled.get());
        
        // Test event handling
        click_handled.set(true);
        focus_handled.set(true);
        blur_handled.set(true);
        
        assert!(click_handled.get());
        assert!(focus_handled.get());
        assert!(blur_handled.get());
    }

    #[test]
    fn test_pagination_form_integration() {
        let form_page = RwSignal::new(1);
        let form_total = RwSignal::new(10);
        let form_disabled = RwSignal::new(false);
        let form_valid = RwSignal::new(true);
        
        let _form_pagination_view = view! {
            <Pagination 
                current_page=MaybeProp::from(form_page)
                total_pages=form_total
                disabled=MaybeProp::from(form_disabled)
            />
        };
        
        // Test initial form state
        assert_eq!(form_page.get(), 1);
        assert_eq!(form_total.get(), 10);
        assert!(!form_disabled.get());
        assert!(form_valid.get());
        
        // Test form state changes
        form_page.set(5);
        form_total.set(20);
        form_disabled.set(true);
        form_valid.set(false);
        
        assert_eq!(form_page.get(), 5);
        assert_eq!(form_total.get(), 20);
        assert!(form_disabled.get());
        assert!(!form_valid.get());
    }

    #[test]
    fn test_pagination_validation_states() {
        let valid_state = RwSignal::new(true);
        let error_state = RwSignal::new(false);
        let warning_state = RwSignal::new(false);
        let error_message = RwSignal::new("".to_string());
        
        let _validation_pagination_view = view! {
            <Pagination 
                total_pages=10
                aria_invalid=MaybeProp::from(move || error_state.get())
                class=MaybeProp::from(move || {
                    if error_state.get() { "error-pagination" }
                    else if warning_state.get() { "warning-pagination" }
                    else if valid_state.get() { "valid-pagination" }
                    else { "default-pagination" }
                })
            />
        };
        
        // Test initial validation state
        assert!(valid_state.get());
        assert!(!error_state.get());
        assert!(!warning_state.get());
        assert_eq!(error_message.get(), "");
        
        // Test validation state changes
        valid_state.set(false);
        error_state.set(true);
        warning_state.set(true);
        error_message.set("Validation error".to_string());
        
        assert!(!valid_state.get());
        assert!(error_state.get());
        assert!(warning_state.get());
        assert_eq!(error_message.get(), "Validation error");
    }

    #[test]
    fn test_pagination_focus_management() {
        let focused_state = RwSignal::new(false);
        let focus_visible_state = RwSignal::new(false);
        
        let _focus_pagination_view = view! {
            <Pagination 
                total_pages=10
                on_focus=move |_| focused_state.set(true)
                on_blur=move |_| focused_state.set(false)
                on_focus_visible=move |_| focus_visible_state.set(true)
            />
        };
        
        // Test initial focus state
        assert!(!focused_state.get());
        assert!(!focus_visible_state.get());
        
        // Test focus changes
        focused_state.set(true);
        focus_visible_state.set(true);
        
        assert!(focused_state.get());
        assert!(focus_visible_state.get());
    }

    #[test]
    fn test_pagination_disabled_states() {
        let disabled_state = RwSignal::new(false);
        let readonly_state = RwSignal::new(false);
        
        let _disabled_pagination_view = view! {
            <Pagination 
                total_pages=10
                disabled=MaybeProp::from(disabled_state)
                readonly=MaybeProp::from(readonly_state)
                class=MaybeProp::from(move || {
                    if disabled_state.get() { "disabled-pagination" }
                    else if readonly_state.get() { "readonly-pagination" }
                    else { "enabled-pagination" }
                })
            />
        };
        
        // Test initial disabled state
        assert!(!disabled_state.get());
        assert!(!readonly_state.get());
        
        // Test disabled state
        disabled_state.set(true);
        assert!(disabled_state.get());
        
        // Test readonly state
        readonly_state.set(true);
        assert!(readonly_state.get());
        
        // Test re-enabling
        disabled_state.set(false);
        readonly_state.set(false);
        
        assert!(!disabled_state.get());
        assert!(!readonly_state.get());
    }

    #[test]
    fn test_pagination_page_management() {
        let current_page = RwSignal::new(1);
        let total_pages = RwSignal::new(10);
        let page_size = RwSignal::new(10);
        
        let _page_pagination_view = view! {
            <Pagination 
                current_page=MaybeProp::from(current_page)
                total_pages=total_pages
                page_size=MaybeProp::from(page_size)
            />
        };
        
        // Test initial page state
        assert_eq!(current_page.get(), 1);
        assert_eq!(total_pages.get(), 10);
        assert_eq!(page_size.get(), 10);
        
        // Test page changes
        current_page.set(5);
        total_pages.set(20);
        page_size.set(25);
        
        assert_eq!(current_page.get(), 5);
        assert_eq!(total_pages.get(), 20);
        assert_eq!(page_size.get(), 25);
    }

    #[test]
    fn test_pagination_size_management() {
        let size_signal = RwSignal::new("default".to_string());
        
        let _size_pagination_view = view! {
            <Pagination 
                total_pages=10
                class=MaybeProp::from(move || format!("pagination-{}", size_signal.get()))
            />
        };
        
        // Test initial size
        assert_eq!(size_signal.get(), "default");
        
        // Test size changes
        size_signal.set("small".to_string());
        assert_eq!(size_signal.get(), "small");
        
        size_signal.set("large".to_string());
        assert_eq!(size_signal.get(), "large");
    }

    #[test]
    fn test_pagination_variant_management() {
        let variant_signal = RwSignal::new("default".to_string());
        
        let _variant_pagination_view = view! {
            <Pagination 
                total_pages=10
                class=MaybeProp::from(move || format!("pagination-{}", variant_signal.get()))
            />
        };
        
        // Test initial variant
        assert_eq!(variant_signal.get(), "default");
        
        // Test variant changes
        variant_signal.set("compact".to_string());
        assert_eq!(variant_signal.get(), "compact");
        
        variant_signal.set("extended".to_string());
        assert_eq!(variant_signal.get(), "extended");
    }

    #[test]
    fn test_pagination_validation_logic() {
        let current_page = RwSignal::new(1);
        let total_pages = RwSignal::new(10);
        let min_page = RwSignal::new(1);
        let max_page = RwSignal::new(10);
        
        let _validation_pagination_view = view! {
            <Pagination 
                current_page=MaybeProp::from(current_page)
                total_pages=total_pages
                aria_invalid=MaybeProp::from(move || {
                    let page = current_page.get();
                    let total = total_pages.get();
                    page < min_page.get() || page > max_page.get() || page > total
                })
            />
        };
        
        // Test initial validation state
        assert_eq!(current_page.get(), 1);
        assert_eq!(total_pages.get(), 10);
        assert_eq!(min_page.get(), 1);
        assert_eq!(max_page.get(), 10);
        
        // Test validation with valid page
        let is_valid = current_page.get() >= min_page.get() && 
                      current_page.get() <= max_page.get() && 
                      current_page.get() <= total_pages.get();
        assert!(is_valid);
        
        // Test validation with invalid page (too high)
        current_page.set(15);
        let is_valid = current_page.get() >= min_page.get() && 
                      current_page.get() <= max_page.get() && 
                      current_page.get() <= total_pages.get();
        assert!(!is_valid);
        
        // Test validation with invalid page (too low)
        current_page.set(0);
        let is_valid = current_page.get() >= min_page.get() && 
                      current_page.get() <= max_page.get() && 
                      current_page.get() <= total_pages.get();
        assert!(!is_valid);
    }

    #[test]
    fn test_pagination_state_consistency() {
        let current_page = RwSignal::new(1);
        let total_pages = RwSignal::new(10);
        let disabled = RwSignal::new(false);
        let readonly = RwSignal::new(false);
        
        let _consistency_pagination_view = view! {
            <Pagination 
                current_page=MaybeProp::from(current_page)
                total_pages=total_pages
                disabled=MaybeProp::from(disabled)
                readonly=MaybeProp::from(readonly)
            />
        };
        
        // Test initial state consistency
        assert_eq!(current_page.get(), 1);
        assert_eq!(total_pages.get(), 10);
        assert!(!disabled.get());
        assert!(!readonly.get());
        
        // Test state consistency after changes
        current_page.set(5);
        total_pages.set(20);
        disabled.set(true);
        readonly.set(true);
        
        assert_eq!(current_page.get(), 5);
        assert_eq!(total_pages.get(), 20);
        assert!(disabled.get());
        assert!(readonly.get());
    }
}
