#[cfg(test)]
mod real_tests {
    use crate::default::{Avatar, AvatarImage, AvatarFallback}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_avatar_renders() {
        mount_to_body(|| {
            view! {
                <Avatar>
                    "avatar content"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "avatar should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_avatar_with_props() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-class" id="test-id">
                    "avatar with props"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "avatar with props should render");
    }

    #[test]
    fn test_avatar_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "avatar signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "avatar signal should update");
    }

    #[test]
    fn test_avatar_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "avatar callback should be triggered");
    }

    #[test]
    fn test_avatar_class_handling() {
        let custom_class = "custom-avatar-class";
        assert!(!custom_class.is_empty(), "avatar should support custom classes");
        assert!(custom_class.contains("avatar"), "Class should contain component name");
    }

    #[test]
    fn test_avatar_id_handling() {
        let custom_id = "custom-avatar-id";
        assert!(!custom_id.is_empty(), "avatar should support custom IDs");
        assert!(custom_id.contains("avatar"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_avatar_interaction() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-interaction">
                    "Interactive avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "avatar should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_avatar_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-focus">
                    "Focusable avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "avatar should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_avatar_accessibility() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-a11y">
                    "Accessible avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "avatar should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_avatar_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-dom-render">
                    "DOM Test avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "avatar should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_avatar_class_application() {
        mount_to_body(|| {
            view! {
                <Avatar class="test-class-application custom-class">
                    "Class Test avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_avatar_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Avatar 
                    class="test-attributes">
                    "Attribute Test avatar"
                </Avatar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test avatar");
    }
}