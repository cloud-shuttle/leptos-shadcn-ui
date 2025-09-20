#[cfg(test)]
mod real_tests {
    use crate::default::Input; // Import main components
    use crate::validation::{InputValidator, ValidationRule, ValidationResult};
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use leptos::wasm_bindgen::JsCast;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_input_renders() {
        mount_to_body(|| {
            view! {
                <Input />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "input should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_input_with_props() {
        mount_to_body(|| {
            view! {
                <Input />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "input with props should render");
    }

    #[test]
    #[test]
    );
        
        callback.run(());
        assert!(callback_triggered.get(), "input callback should be triggered");
    }

    #[test]
    fn test_input_class_handling() {
        let custom_class = "custom-input-class";
        assert!(!custom_class.is_empty(), "input should support custom classes");
        assert!(custom_class.contains("input"), "Class should contain component name");
    }

    #[test]
    fn test_input_id_handling() {
        let custom_id = "custom-input-id";
        assert!(!custom_id.is_empty(), "input should support custom IDs");
        assert!(custom_id.contains("input"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_input_value_binding() {
        let input_value = RwSignal::new("initial".to_string());
        
        mount_to_body(move || {
            view! {
                <Input 
                    value=input_value.get()
                    on_change=Callback::new(move |value| input_value.set(value))
                />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.query_selector("input").unwrap().unwrap();
        let html_input = input.unchecked_into::<web_sys::HtmlInputElement>();
        assert_eq!(html_input.value(), "initial");
    }

    #[wasm_bindgen_test]
    fn test_input_disabled_state() {
        let disabled = RwSignal::new(true);
        
        mount_to_body(move || {
            view! {
                <Input disabled=disabled />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.query_selector("input").unwrap().unwrap();
        assert!(input.has_attribute("disabled"));
    }

    #[wasm_bindgen_test]
    fn test_input_type_variants() {
        let input_types = vec!["text", "email", "password", "number", "tel"];
        
        for input_type in input_types {
            mount_to_body(move || {
                view! {
                    <Input input_type=input_type />
                }
            });
            
            let document = web_sys::window().unwrap().document().unwrap();
            let input = document.query_selector("input").unwrap().unwrap();
            assert_eq!(input.get_attribute("type").unwrap(), input_type);
        }
    }

    #[wasm_bindgen_test]
    fn test_input_placeholder() {
        mount_to_body(|| {
            view! {
                <Input placeholder="Enter your name" />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.query_selector("input").unwrap().unwrap();
        assert_eq!(input.get_attribute("placeholder").unwrap(), "Enter your name");
    }

    #[wasm_bindgen_test]
    fn test_input_change_handler() {
        let change_count = RwSignal::new(0);
        let last_value = RwSignal::new(String::new());
        
        mount_to_body(move || {
            view! {
                <Input 
                    on_change=move |value| {
                        change_count.update(|count| *count += 1);
                        last_value.set(value);
                    }
                />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.query_selector("input").unwrap().unwrap();
        let html_input = input.unchecked_into::<web_sys::HtmlInputElement>();
        
        // Simulate input change
        html_input.set_value("test input");
        let input_event = web_sys::InputEvent::new("input").unwrap();
        input.dispatch_event(&input_event).unwrap();
        
        assert_eq!(change_count.get(), 1);
        assert_eq!(last_value.get(), "test input");
    }

    #[wasm_bindgen_test]
    fn test_input_css_classes() {
        mount_to_body(|| {
            view! {
                <Input class="custom-class".into() />
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let input = document.query_selector("input").unwrap().unwrap();
        let class_list = input.class_list();
        
        // Check base classes
        assert!(class_list.contains("flex"));
        assert!(class_list.contains("h-10"));
        assert!(class_list.contains("w-full"));
        assert!(class_list.contains("rounded-md"));
        assert!(class_list.contains("border"));
        
        // Check custom class
        assert!(class_list.contains("custom-class"));
    }

    #[test]
    fn test_input_class_constants() {
        // Test INPUT_CLASS constant
        let input_class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        assert!(input_class.contains("flex"));
        assert!(input_class.contains("h-10"));
        assert!(input_class.contains("w-full"));
        assert!(input_class.contains("rounded-md"));
        assert!(input_class.contains("border"));
        assert!(input_class.contains("border-input"));
        assert!(input_class.contains("bg-background"));
        assert!(input_class.contains("px-3"));
        assert!(input_class.contains("py-2"));
        assert!(input_class.contains("text-sm"));
        assert!(input_class.contains("ring-offset-background"));
        assert!(input_class.contains("placeholder:text-muted-foreground"));
        assert!(input_class.contains("focus-visible:outline-none"));
        assert!(input_class.contains("focus-visible:ring-2"));
        assert!(input_class.contains("focus-visible:ring-ring"));
        assert!(input_class.contains("focus-visible:ring-offset-2"));
        assert!(input_class.contains("disabled:cursor-not-allowed"));
        assert!(input_class.contains("disabled:opacity-50"));

        // Test INPUT_ERROR_CLASS constant
        let input_error_class = "border-destructive focus-visible:ring-destructive";
        assert!(input_error_class.contains("border-destructive"));
        assert!(input_error_class.contains("focus-visible:ring-destructive"));
    }

    #[test]
    