use super::*;
use crate::default::{Button, ButtonVariant, ButtonSize, BUTTON_CLASS};

#[wasm_bindgen_test]
fn button_renders_as_button_element() {
    let container = mount_component(|| {
        view! {
            <Button>"Test Button"</Button>
        }
    });
    
    let button = container
        .query_selector("button")
        .unwrap()
        .expect("Button should render as <button> element");
    
    assert_eq!(button.tag_name(), "BUTTON");
    assert_eq!(button.get_attribute("type"), Some("button".to_string()));
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_displays_children_text() {
    let container = mount_component(|| {
        view! {
            <Button>"Click Me"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    assert_eq!(button.text_content(), Some("Click Me".to_string()));
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_applies_base_classes() {
    let container = mount_component(|| {
        view! {
            <Button>"Test"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    // Check for key base classes
    assert!(class_list.contains("inline-flex"), "Should have inline-flex class");
    assert!(class_list.contains("items-center"), "Should have items-center class");
    assert!(class_list.contains("justify-center"), "Should have justify-center class");
    assert!(class_list.contains("rounded-md"), "Should have rounded-md class");
    assert!(class_list.contains("text-sm"), "Should have text-sm class");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_applies_variant_classes() {
    let container = mount_component(|| {
        view! {
            <Button variant=ButtonVariant::Destructive>"Delete"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    assert!(class_list.contains("bg-destructive"), "Should have destructive background");
    assert!(class_list.contains("text-destructive-foreground"), "Should have destructive text color");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_applies_size_classes() {
    let container = mount_component(|| {
        view! {
            <Button size=ButtonSize::Lg>"Large Button"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    // Large button should have specific height and padding
    assert!(class_list.contains("h-11") || class_list.contains("px-8"), 
           "Large button should have appropriate size classes");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_applies_custom_class() {
    let container = mount_component(|| {
        view! {
            <Button class="my-custom-class">"Custom"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    assert!(class_list.contains("my-custom-class"), "Should apply custom CSS classes");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_applies_id_attribute() {
    let container = mount_component(|| {
        view! {
            <Button id="test-button">"With ID"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    assert_eq!(button.get_attribute("id"), Some("test-button".to_string()));
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_disabled_state_renders_correctly() {
    let container = mount_component(|| {
        view! {
            <Button disabled=Signal::from(true)>"Disabled"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let html_button: web_sys::HtmlButtonElement = button.unchecked_into();
    
    assert!(html_button.disabled(), "Button should be disabled");
    assert!(button.class_list().contains("opacity-50"), "Should have disabled opacity");
    assert!(button.class_list().contains("pointer-events-none"), "Should have pointer events disabled");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_all_variants_render() {
    let variants = vec![
        ButtonVariant::Default,
        ButtonVariant::Destructive,
        ButtonVariant::Outline,
        ButtonVariant::Secondary,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
    ];
    
    for variant in variants {
        let container = mount_component(move || {
            view! {
                <Button variant=variant.clone()>{format!("{:?} Button", variant)}</Button>
            }
        });
        
        let button = container.query_selector("button").unwrap()
            .expect(&format!("Button with variant {:?} should render", variant));
        
        assert_eq!(button.tag_name(), "BUTTON");
        assert!(button.text_content().unwrap().contains(&format!("{:?}", variant)));
        
        cleanup_test_container();
    }
}

#[wasm_bindgen_test]
fn button_all_sizes_render() {
    let sizes = vec![
        ButtonSize::Default,
        ButtonSize::Sm,
        ButtonSize::Lg,
        ButtonSize::Icon,
    ];
    
    for size in sizes {
        let container = mount_component(move || {
            view! {
                <Button size=size.clone()>{format!("{:?} Size", size)}</Button>
            }
        });
        
        let button = container.query_selector("button").unwrap()
            .expect(&format!("Button with size {:?} should render", size));
        
        assert_eq!(button.tag_name(), "BUTTON");
        
        cleanup_test_container();
    }
}

#[wasm_bindgen_test] 
fn button_with_complex_children_renders() {
    let container = mount_component(|| {
        view! {
            <Button>
                <span class="icon">"🚀"</span>
                " Launch"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let span = button.query_selector("span").unwrap()
        .expect("Should render child span element");
    
    assert_eq!(span.text_content(), Some("🚀".to_string()));
    assert!(button.text_content().unwrap().contains("Launch"));
    
    cleanup_test_container();
}
