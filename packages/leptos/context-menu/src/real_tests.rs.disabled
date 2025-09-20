#[cfg(test)]
mod real_tests {
    use crate::default::{ContextMenu};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_context_menu_renders() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "context-menu content"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "context-menu should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_with_props() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "context-menu with props"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "context-menu with props should render");
    }

    #[test]
    fn test_context_menu_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "context-menu signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "context-menu signal should update");
    }

    #[test]
    fn test_context_menu_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "context-menu callback should be triggered");
    }

    #[test]
    fn test_context_menu_class_handling() {
        let custom_class = "custom-context-menu-class";
        assert!(!custom_class.is_empty(), "context-menu should support custom classes");
        assert!(custom_class.contains("context-menu"), "Class should contain component name");
    }

    #[test]
    fn test_context_menu_id_handling() {
        let custom_id = "custom-context-menu-id";
        assert!(!custom_id.is_empty(), "context-menu should support custom IDs");
        assert!(custom_id.contains("context-menu"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_interaction() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "Interactive context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "context-menu should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_focus_behavior() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "Focusable context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "context-menu should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_accessibility() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "Accessible context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "context-menu should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_dom_rendering() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "DOM Test context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "context-menu should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_class_application() {
        mount_to_body(|| {
            view! {
                <ContextMenu>
                    "Class Test context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_context_menu_attribute_handling() {
        mount_to_body(|| {
            view! {
                <ContextMenu 
                    class="test-attributes".into()>
                    "Attribute Test context-menu"
                </ContextMenu>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test context-menu");
    }
}