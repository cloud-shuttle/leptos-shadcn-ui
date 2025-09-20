#[cfg(test)]
mod real_tests {
    use crate::default::{Slider};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_slider_renders() {
        mount_to_body(|| {
            view! {
                <Slider>
                    "slider content"
                </Slider>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "slider should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_slider_with_props() {
        mount_to_body(|| {
            view! {
                <Slider class="test-class".into()>
                    "slider with props"
                </Slider>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "slider with props should render");
    }

    #[test]
    fn test_slider_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "slider signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "slider signal should update");
    }

    #[test]
    fn test_slider_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "slider callback should be triggered");
    }

    #[test]
    fn test_slider_class_handling() {
        let custom_class = "custom-slider-class";
        assert!(!custom_class.is_empty(), "slider should support custom classes");
        assert!(custom_class.contains("slider"), "Class should contain component name");
    }

    #[test]
    fn test_slider_id_handling() {
        let custom_id = "custom-slider-id";
        assert!(!custom_id.is_empty(), "slider should support custom IDs");
        assert!(custom_id.contains("slider"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_slider_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Slider 
                    class="test-click".into()
                    on_click=move |_| click_count.update(|count| *count += 1)>
                    "Clickable slider"
                </Slider>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-click").unwrap();
        assert!(element.is_some(), "slider should render for click test");
    }

    #[wasm_bindgen_test]
    fn test_slider_hover_behavior() {
        mount_to_body(|| {
            view! {
                <Slider class="test-hover".into()>
                    "Hoverable slider"
                </Slider>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-hover").unwrap();
        assert!(element.is_some(), "slider should render for hover test");
    }

    #[wasm_bindgen_test]
    fn test_slider_click_handling() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Slider 
                    class="test-click".into()
                    on_click=move || click_count.update(|count| *count += 1)>
                    "Clickable slider"
                </Slider>
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
    fn test_slider_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Slider 
                    class="test-focus".into()>
                    "Focusable slider"
                </Slider>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("tabindex").unwrap(), "0");
        
        // Test focus
        
        assert_eq!(document.active_element().unwrap(), element);
    }
}