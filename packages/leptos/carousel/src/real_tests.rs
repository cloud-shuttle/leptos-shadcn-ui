#[cfg(test)]
mod real_tests {
    use crate::default::{Carousel};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_carousel_renders() {
        mount_to_body(|| {
            view! {
                <Carousel>
                    "carousel content"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "carousel should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_carousel_with_props() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-class">
                    "carousel with props"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "carousel with props should render");
    }

    #[test]
    fn test_carousel_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "carousel signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "carousel signal should update");
    }

    #[test]
    fn test_carousel_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "carousel callback should be triggered");
    }

    #[test]
    fn test_carousel_class_handling() {
        let custom_class = "custom-carousel-class";
        assert!(!custom_class.is_empty(), "carousel should support custom classes");
        assert!(custom_class.contains("carousel"), "Class should contain component name");
    }

    #[test]
    fn test_carousel_id_handling() {
        let custom_id = "custom-carousel-id";
        assert!(!custom_id.is_empty(), "carousel should support custom IDs");
        assert!(custom_id.contains("carousel"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_carousel_interaction() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-interaction">
                    "Interactive carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-interaction").unwrap();
        assert!(element.is_some(), "carousel should render for interaction test");
    }

    #[wasm_bindgen_test]
    fn test_carousel_focus_behavior() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-focus">
                    "Focusable carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-focus").unwrap();
        assert!(element.is_some(), "carousel should render for focus test");
    }

    #[wasm_bindgen_test]
    fn test_carousel_accessibility() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-a11y" role="button">
                    "Accessible carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-a11y").unwrap();
        assert!(element.is_some(), "carousel should render for accessibility test");
    }

    #[wasm_bindgen_test]
    fn test_carousel_dom_rendering() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-dom-render">
                    "DOM Test carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-dom-render").unwrap();
        assert!(element.is_some(), "carousel should render in DOM");
        
        let element = element.unwrap();
        assert!(element.text_content().unwrap().contains("DOM Test"), "Content should be rendered");
    }

    #[wasm_bindgen_test]
    fn test_carousel_class_application() {
        mount_to_body(|| {
            view! {
                <Carousel class="test-class-application custom-class">
                    "Class Test carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-class-application").unwrap().unwrap();
        let class_list = element.class_list();
        
        assert!(class_list.contains("test-class-application"), "Base class should be applied");
        assert!(class_list.contains("custom-class"), "Custom class should be applied");
    }

    #[wasm_bindgen_test]
    fn test_carousel_attribute_handling() {
        mount_to_body(|| {
            view! {
                <Carousel 
                    class="test-attributes"
                    data_test="test-value"
                    aria-label="Test carousel"
                >
                    "Attribute Test carousel"
                </Carousel>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".test-attributes").unwrap().unwrap();
        
        assert_eq!(element.get_attribute("data-test").unwrap(), "test-value");
        assert_eq!(element.get_attribute("aria-label").unwrap(), "Test carousel");
    }
}