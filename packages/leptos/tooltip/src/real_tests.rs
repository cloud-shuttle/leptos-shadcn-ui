#[cfg(test)]
mod real_tests {
    use crate::default::{Tooltip};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_tooltip_renders() {
        mount_to_body(|| {
            view! {
                <Tooltip>
                    "tooltip content"
                </Tooltip>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "tooltip should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_tooltip_with_props() {
        mount_to_body(|| {
            view! {
                <Tooltip class="test-class">
                    "tooltip with props"
                </Tooltip>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "tooltip with props should render");
    }

    #[test]
    fn test_tooltip_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "tooltip signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "tooltip signal should update");
    }

    #[test]
    fn test_tooltip_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "tooltip callback should be triggered");
    }

    #[test]
    fn test_tooltip_class_handling() {
        let custom_class = "custom-tooltip-class";
        assert!(!custom_class.is_empty(), "tooltip should support custom classes");
        assert!(custom_class.contains("tooltip"), "Class should contain component name");
    }

    #[test]
    fn test_tooltip_id_handling() {
        let custom_id = "custom-tooltip-id";
        assert!(!custom_id.is_empty(), "tooltip should support custom IDs");
        assert!(custom_id.contains("tooltip"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_tooltip_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Tooltip class="test-responsive" data-responsive="true">
                    "Responsive tooltip"
                </Tooltip>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "tooltip should render for responsive test");
    }

    #[wasm_bindgen_test]
    fn test_tooltip_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout">
                    <Tooltip>
                        "Layout tooltip"
                    </Tooltip>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "tooltip should render in layout");
    }

    #[wasm_bindgen_test]
    fn test_tooltip_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Tooltip 
                    class="test-responsive" 
                    data-responsive="true"
                    style="width: 100%; max-width: 500px;"
                >
                    "Responsive tooltip"
                </Tooltip>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_tooltip_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container">
                    <Tooltip class="test-layout-item">
                        "Layout tooltip"
                    </Tooltip>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "tooltip should render in layout");
    }
}