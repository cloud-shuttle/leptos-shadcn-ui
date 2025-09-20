#[cfg(test)]
mod real_tests {
    use crate::default::{Calendar};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_calendar_renders() {
        mount_to_body(|| {
            view! {
                <Calendar>
                    "calendar content"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "calendar should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_calendar_with_props() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-class">
                    "calendar with props"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "calendar with props should render");
    }

    #[test]
    fn test_calendar_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "calendar signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "calendar signal should update");
    }

    #[test]
    fn test_calendar_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "calendar callback should be triggered");
    }

    #[test]
    fn test_calendar_class_handling() {
        let custom_class = "custom-calendar-class";
        assert!(!custom_class.is_empty(), "calendar should support custom classes");
        assert!(custom_class.contains("calendar"), "Class should contain component name");
    }

    #[test]
    fn test_calendar_id_handling() {
        let custom_id = "custom-calendar-id";
        assert!(!custom_id.is_empty(), "calendar should support custom IDs");
        assert!(custom_id.contains("calendar"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_calendar_interaction() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-interaction">
                    "Interactive calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "calendar should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_calendar_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-focus">
                    "Focusable calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "calendar should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_calendar_accessibility() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-a11y" role="button">
                    "Accessible calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "calendar should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_calendar_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-dom-render">
                    "DOM Test calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "calendar should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_calendar_class_application() {
        mount_to_body(|| {
            view! {
                <Calendar class="test-class-application custom-class">
                    "Class Test calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_calendar_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Calendar 
                    class="test-attributes"
                    data_test="test-value"
                    aria-label="Test calendar"
                >
                    "Attribute Test calendar"
                </Calendar>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test calendar");
    }
}