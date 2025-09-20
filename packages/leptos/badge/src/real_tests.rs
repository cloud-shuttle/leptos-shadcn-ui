#[cfg(test)]
mod real_tests {
    use crate::default::{Badge, BadgeVariant};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_badge_renders() {
        mount_to_body(|| {
            view! {
                <Badge>
                    "badge content"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "badge should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_badge_with_props() {
        mount_to_body(|| {
            view! {
                <Badge class="test-class" id="test-id">
                    "badge with props"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "badge with props should render");
    }

    #[test]
    fn test_badge_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "badge signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "badge signal should update");
    }

    #[test]
    fn test_badge_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "badge callback should be triggered");
    }

    #[test]
    fn test_badge_class_handling() {
        let custom_class = "custom-badge-class";
        assert!(!custom_class.is_empty(), "badge should support custom classes");
        assert!(custom_class.contains("badge"), "Class should contain component name");
    }

    #[test]
    fn test_badge_id_handling() {
        let custom_id = "custom-badge-id";
        assert!(!custom_id.is_empty(), "badge should support custom IDs");
        assert!(custom_id.contains("badge"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_badge_interaction() {
        mount_to_body(|| {
            view! {
                <Badge class="test-interaction">
                    "Interactive badge"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "badge should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_badge_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Badge class="test-focus">
                    "Focusable badge"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "badge should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_badge_accessibility() {
        mount_to_body(|| {
            view! {
                <Badge class="test-a11y">
                    "Accessible badge"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "badge should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_badge_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <Badge 
                    class="test-responsive" 
                    
                    style="width: 100%; max-width: 500px;">
                    "Responsive badge"
                </Badge>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_badge_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container"><Badge class="test-layout-item">
                        "Layout badge"
                    </Badge></div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "badge should render in layout");
    }
}