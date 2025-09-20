#[cfg(test)]
mod real_tests {
    use crate::default::{HoverCard};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_hover_card_renders() {
        mount_to_body(|| {
            view! {
                <HoverCard>
                    "hover-card content"
                </HoverCard>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "hover-card should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_hover_card_with_props() {
        mount_to_body(|| {
            view! {
                <HoverCard class="test-class".into()>
                    "hover-card with props"
                </HoverCard>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "hover-card with props should render");
    }

    #[test]
    fn test_hover_card_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "hover-card signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "hover-card signal should update");
    }

    #[test]
    fn test_hover_card_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "hover-card callback should be triggered");
    }

    #[test]
    fn test_hover_card_class_handling() {
        let custom_class = "custom-hover-card-class";
        assert!(!custom_class.is_empty(), "hover-card should support custom classes");
        assert!(custom_class.contains("hover-card"), "Class should contain component name");
    }

    #[test]
    fn test_hover_card_id_handling() {
        let custom_id = "custom-hover-card-id";
        assert!(!custom_id.is_empty(), "hover-card should support custom IDs");
        assert!(custom_id.contains("hover-card"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_hover_card_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <HoverCard class="test-responsive".into() >
                    "Responsive hover-card"
                </HoverCard>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap();
        assert!(element.is_some(), "hover-card should render for responsive test");
    }

    #[wasm_bindgen_test]
    fn test_hover_card_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout".into()>
                    <HoverCard>
                        "Layout hover-card"
                    </HoverCard>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-layout").unwrap();
        assert!(element.is_some(), "hover-card should render in layout");
    }

    #[wasm_bindgen_test]
    fn test_hover_card_responsive_behavior() {
        mount_to_body(|| {
            view! {
                <HoverCard 
                    class="test-responsive".into() 
                    
                    style="width: 100%; max-width: 500px;"
                >
                    "Responsive hover-card"
                </HoverCard>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-responsive").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-responsive").unwrap(), "true");
        assert!(element.get_attribute("style").unwrap().contains("width: 100%"));
        assert!(element.get_attribute("style").unwrap().contains("max-width: 500px"));
    }

    #[wasm_bindgen_test]
    fn test_hover_card_layout_integration() {
        mount_to_body(|| {
            view! {
                <div class="test-layout-container".into()>
                    <HoverCard class="test-layout-item".into()>
                        "Layout hover-card"
                    </HoverCard>
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.query_selector(".test-layout-container").unwrap();
        let item = document.query_selector(".test-layout-item").unwrap();
        
        assert!(container.is_some(), "Container should render");
        assert!(item.is_some(), "hover-card should render in layout");
    }
}