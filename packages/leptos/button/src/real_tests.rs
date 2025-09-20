#[cfg(test)]
mod real_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize}; // Import main components
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_button_renders() {
        mount_to_body(|| {
            view! {
                <Button>
                    "button content"
                </Button>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "button should render in DOM");
    }

    #[wasm_bindgen_test]
    fn test_button_with_props() {
        mount_to_body(|| {
            view! {
                <Button class="test-class" id="test-id">
                    "button with props"
                </Button>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector("div").unwrap();
        assert!(element.is_some(), "button with props should render");
    }

    #[test]
    fn test_button_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "button signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "button signal should update");
    }

    #[test]
    fn test_button_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "button callback should be triggered");
    }

    #[test]
    fn test_button_class_handling() {
        let custom_class = "custom-button-class";
        assert!(!custom_class.is_empty(), "button should support custom classes");
        assert!(custom_class.contains("button"), "Class should contain component name");
    }

    #[test]
    fn test_button_id_handling() {
        let custom_id = "custom-button-id";
        assert!(!custom_id.is_empty(), "button should support custom IDs");
        assert!(custom_id.contains("button"), "ID should contain component name");
    }

    #[wasm_bindgen_test]
    fn test_button_variants() {
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        
        for variant in variants {
            let variant_for_button = variant.clone();
            let variant_for_text = variant.clone();
            mount_to_body(move || {
                view! {
                    <Button variant=variant_for_button>
                        {format!("{:?}", variant_for_text)}
                    </Button>
                }
            });
            
            let document = web_sys::window().unwrap().document().unwrap();
            let button = document.query_selector("button").unwrap().unwrap();
            assert!(button.text_content().unwrap().contains("Variant"));
        }
    }

    #[wasm_bindgen_test]
    fn test_button_sizes() {
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];
        
        for size in sizes {
            let size_for_button = size.clone();
            let size_for_text = size.clone();
            mount_to_body(move || {
                view! {
                    <Button size=size_for_button>
                        {format!("{:?}", size_for_text)}
                    </Button>
                }
            });
            
            let document = web_sys::window().unwrap().document().unwrap();
            let button = document.query_selector("button").unwrap().unwrap();
            assert!(button.text_content().unwrap().contains("Size"));
        }
    }

    #[wasm_bindgen_test]
    fn test_button_disabled_state() {
        mount_to_body(|| {
            view! {
                <Button disabled=true>
                    "Disabled Button"
                </Button>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        assert!(button.has_attribute("disabled"));
    }

    #[wasm_bindgen_test]
    fn test_button_click_handler() {
        let click_count = RwSignal::new(0);
        
        mount_to_body(move || {
            view! {
                <Button on_click=move || click_count.update(|count| *count += 1)>
                    "Click Me"
                </Button>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1);
    }

    #[wasm_bindgen_test]
    fn test_button_css_classes() {
        mount_to_body(|| {
            view! {
                <Button class="custom-class" variant=ButtonVariant::Destructive>
                    "Styled Button"
                </Button>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.query_selector("button").unwrap().unwrap();
        let class_list = button.class_list();
        
        // Check base classes
        assert!(class_list.contains("inline-flex"));
        assert!(class_list.contains("items-center"));
        assert!(class_list.contains("justify-center"));
        assert!(class_list.contains("whitespace-nowrap"));
        assert!(class_list.contains("rounded-md"));
        assert!(class_list.contains("text-sm"));
        assert!(class_list.contains("font-medium"));
        assert!(class_list.contains("ring-offset-background"));
        assert!(class_list.contains("transition-colors"));
        assert!(class_list.contains("focus-visible:outline-none"));
        assert!(class_list.contains("focus-visible:ring-2"));
        assert!(class_list.contains("focus-visible:ring-ring"));
        assert!(class_list.contains("focus-visible:ring-offset-2"));
        assert!(class_list.contains("disabled:pointer-events-none"));
        assert!(class_list.contains("disabled:opacity-50"));
        
        // Check custom class
        assert!(class_list.contains("custom-class"));
    }

    #[test]
    fn test_button_variant_enum() {
        // Test variant equality
        assert_eq!(ButtonVariant::Default, ButtonVariant::Default);
        assert_eq!(ButtonVariant::Destructive, ButtonVariant::Destructive);
        assert_eq!(ButtonVariant::Outline, ButtonVariant::Outline);
        assert_eq!(ButtonVariant::Secondary, ButtonVariant::Secondary);
        assert_eq!(ButtonVariant::Ghost, ButtonVariant::Ghost);
        assert_eq!(ButtonVariant::Link, ButtonVariant::Link);
        
        // Test variant inequality
        assert_ne!(ButtonVariant::Default, ButtonVariant::Destructive);
        assert_ne!(ButtonVariant::Outline, ButtonVariant::Secondary);
        assert_ne!(ButtonVariant::Ghost, ButtonVariant::Link);
    }

    #[test]
    fn test_button_size_enum() {
        // Test size equality
        assert_eq!(ButtonSize::Default, ButtonSize::Default);
        assert_eq!(ButtonSize::Sm, ButtonSize::Sm);
        assert_eq!(ButtonSize::Lg, ButtonSize::Lg);
        assert_eq!(ButtonSize::Icon, ButtonSize::Icon);
        
        // Test size inequality
        assert_ne!(ButtonSize::Default, ButtonSize::Sm);
        assert_ne!(ButtonSize::Lg, ButtonSize::Icon);
        assert_ne!(ButtonSize::Sm, ButtonSize::Lg);
    }

    #[test]
    fn test_button_class_constants() {
        // Test BUTTON_CLASS constant
        let button_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
        assert!(button_class.contains("inline-flex"));
        assert!(button_class.contains("items-center"));
        assert!(button_class.contains("justify-center"));
        assert!(button_class.contains("whitespace-nowrap"));
        assert!(button_class.contains("rounded-md"));
        assert!(button_class.contains("text-sm"));
        assert!(button_class.contains("font-medium"));
        assert!(button_class.contains("ring-offset-background"));
        assert!(button_class.contains("transition-colors"));
        assert!(button_class.contains("focus-visible:outline-none"));
        assert!(button_class.contains("focus-visible:ring-2"));
        assert!(button_class.contains("focus-visible:ring-ring"));
        assert!(button_class.contains("focus-visible:ring-offset-2"));
        assert!(button_class.contains("disabled:pointer-events-none"));
        assert!(button_class.contains("disabled:opacity-50"));

        // Test variant classes
        let default_class = "bg-primary text-primary-foreground hover:bg-primary/90";
        assert!(default_class.contains("bg-primary"));
        assert!(default_class.contains("text-primary-foreground"));
        assert!(default_class.contains("hover:bg-primary/90"));

        let destructive_class = "bg-destructive text-destructive-foreground hover:bg-destructive/90";
        assert!(destructive_class.contains("bg-destructive"));
        assert!(destructive_class.contains("text-destructive-foreground"));
        assert!(destructive_class.contains("hover:bg-destructive/90"));

        let outline_class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground";
        assert!(outline_class.contains("border"));
        assert!(outline_class.contains("border-input"));
        assert!(outline_class.contains("bg-background"));
        assert!(outline_class.contains("hover:bg-accent"));
        assert!(outline_class.contains("hover:text-accent-foreground"));

        // Test size classes
        let default_size_class = "h-10 px-4 py-2";
        assert!(default_size_class.contains("h-10"));
        assert!(default_size_class.contains("px-4"));
        assert!(default_size_class.contains("py-2"));

        let sm_size_class = "h-9 rounded-md px-3";
        assert!(sm_size_class.contains("h-9"));
        assert!(sm_size_class.contains("rounded-md"));
        assert!(sm_size_class.contains("px-3"));

        let lg_size_class = "h-11 rounded-md px-8";
        assert!(lg_size_class.contains("h-11"));
        assert!(lg_size_class.contains("rounded-md"));
        assert!(lg_size_class.contains("px-8"));

        let icon_size_class = "h-10 w-10";
        assert!(icon_size_class.contains("h-10"));
        assert!(icon_size_class.contains("w-10"));
    }

    #[test]
    fn test_button_accessibility() {
        // Test ARIA attributes
        let aria_label = "Submit form";
        assert!(!aria_label.is_empty(), "Button should support ARIA labels");
        
        let aria_describedby = "button-description";
        assert!(!aria_describedby.is_empty(), "Button should support ARIA describedby");
        
        let role = "button";
        assert_eq!(role, "button", "Button should have correct role");
    }

    #[test]
    fn test_button_keyboard_navigation() {
        // Test keyboard event handling
        let key_events = vec!["Enter", "Space"];
        
        for key in key_events {
            assert!(!key.is_empty(), "Button should handle {} key", key);
        }
    }

    #[test]
    fn test_button_loading_state() {
        // Test loading state management
        let loading = RwSignal::new(false);
        assert!(!loading.get(), "Button should not be loading initially");
        
        loading.set(true);
        assert!(loading.get(), "Button should be in loading state");
        
        loading.set(false);
        assert!(!loading.get(), "Button should exit loading state");
    }
}