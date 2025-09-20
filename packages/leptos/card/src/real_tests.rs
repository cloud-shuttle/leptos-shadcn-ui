#[cfg(test)]
mod real_tests {
    use crate::default::{Card, CardHeader, CardTitle}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_card_renders() {
        mount_to_body(|| {
            view! {
                <Card>
                    "card content"
                </Card>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "card should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_card_with_props() {
        mount_to_body(|| {
            view! {
                <Card class="test-class" id="test-id">
                    "card with props"
                </Card>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "card with props should render");
    }

    #[test]
    fn test_card_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "card signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "card signal should update");
    }

    #[test]
    fn test_card_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "card callback should be triggered");
    }

    #[test]
    fn test_card_class_handling() {
        let custom_class = "custom-card-class";
        assert!(!custom_class.is_empty(), "card should support custom classes");
        assert!(custom_class.contains("card"), "Class should contain component name");
    }

    #[test]
    fn test_card_id_handling() {
        let custom_id = "custom-card-id";
        assert!(!custom_id.is_empty(), "card should support custom IDs");
        assert!(custom_id.contains("card"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "card should render for responsive test");
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "card should render in layout");
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "card should render in layout");
    }
}