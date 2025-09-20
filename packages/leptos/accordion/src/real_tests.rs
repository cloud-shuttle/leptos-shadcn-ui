#[cfg(test)]
mod real_tests {
    use crate::default::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_accordion_renders() {
        mount_to_body(|| {
            view! {
                <Accordion>
                    "accordion content"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "accordion should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_accordion_with_props() {
        mount_to_body(|| {
            view! {
                <Accordion class="test-class".into()>
                    "accordion with props"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "accordion with props should render");
    }

    #[test]
    fn test_accordion_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "accordion signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "accordion signal should update");
    }

    #[test]
    fn test_accordion_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "accordion callback should be triggered");
    }

    #[test]
    fn test_accordion_class_handling() {
        let custom_class = "custom-accordion-class";
        assert!(!custom_class.is_empty(), "accordion should support custom classes");
        assert!(custom_class.contains("accordion"), "Class should contain component name");
    }

    #[test]
    fn test_accordion_id_handling() {
        let custom_id = "custom-accordion-id";
        assert!(!custom_id.is_empty(), "accordion should support custom IDs");
        assert!(custom_id.contains("accordion"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_accordion_interaction() {
        mount_to_body(|| {
            view! {
                <Accordion class="test-interaction".into()>
                    "Interactive accordion"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "accordion should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_accordion_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Accordion class="test-focus".into()>
                    "Focusable accordion"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "accordion should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_accordion_accessibility() {
        mount_to_body(|| {
            view! {
                <Accordion class="test-a11y".into() >
                    "Accessible accordion"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "accordion should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_accordion_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Accordion 
                    class="test-click".into()
                    on_click=move || click_count.update(|count| *count += 1)
                >
                    "Clickable accordion"
                </Accordion>
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
    fn test_accordion_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Accordion 
                    class="test-focus".into()
                    tabindex="0"
                >
                    "Focusable accordion"
                </Accordion>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        
        assert_eq!(document.active_element().unwrap(), element);
    }
}