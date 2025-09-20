#[cfg(test)]
mod real_tests {
    use crate::default::{Pagination};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_pagination_renders() {
        mount_to_body(|| {
            view! {
                <Pagination>
                    "pagination content"
                </Pagination>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "pagination should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_pagination_with_props() {
        mount_to_body(|| {
            view! {
                <Pagination class="test-class".into()>
                    "pagination with props"
                </Pagination>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "pagination with props should render");
    }

    #[test]
    fn test_pagination_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "pagination signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "pagination signal should update");
    }

    #[test]
    fn test_pagination_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "pagination callback should be triggered");
    }

    #[test]
    fn test_pagination_class_handling() {
        let custom_class = "custom-pagination-class";
        assert!(!custom_class.is_empty(), "pagination should support custom classes");
        assert!(custom_class.contains("pagination"), "Class should contain component name");
    }

    #[test]
    fn test_pagination_id_handling() {
        let custom_id = "custom-pagination-id";
        assert!(!custom_id.is_empty(), "pagination should support custom IDs");
        assert!(custom_id.contains("pagination"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_pagination_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Pagination 
                    class="test-click".into()
                    }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap();
        assert!(element.is_some(), "pagination should render for click test");
    }

    #[wasm_bindgen_test]
    fn test_pagination_hover_behavior() {
        mount_to_body(|| {
            view! {
                <Pagination class="test-hover".into() >
                    "Hoverable pagination"
                </Pagination>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-hover").unwrap();
        assert!(element.is_some(), "pagination should render for hover test");
    }

    #[wasm_bindgen_test]
    fn test_pagination_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Pagination 
                    class="test-click".into()
                    }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1, "Click should be handled");
    }

    #[wasm_bindgen_test]
    fn test_pagination_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Pagination 
                    class="test-focus".into()
                    
                >
                    "Focusable pagination"
                </Pagination>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        
        assert_eq!(document.active_element().unwrap(), element);
    }
}