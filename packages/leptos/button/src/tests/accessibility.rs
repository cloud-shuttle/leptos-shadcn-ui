use super::*;
use crate::default::{Button, ButtonVariant};

#[wasm_bindgen_test]
fn button_has_proper_semantics() {
    let container = mount_component(|| {
        view! {
            <Button>"Accessible Button"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Should be a button element (implicit role="button")
    assert_eq!(button.tag_name(), "BUTTON");
    
    // Should have type="button" by default
    assert_eq!(button.get_attribute("type"), Some("button".to_string()));
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_supports_aria_label() {
    let container = mount_component(|| {
        view! {
            <Button 
                aria-label="Save document"
                class="icon-only"
            >
                "💾"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Should have accessible name via aria-label
    assert_eq!(
        button.get_attribute("aria-label"),
        Some("Save document".to_string()),
        "Button should support aria-label for accessibility"
    );
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn disabled_button_aria_state() {
    let container = mount_component(|| {
        view! {
            <Button disabled=Signal::from(true)>
                "Disabled Button"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let html_button: web_sys::HtmlButtonElement = button.unchecked_into();
    
    // Should be disabled via HTML attribute (preferred over aria-disabled)
    assert!(html_button.disabled(), "Button should be disabled via HTML disabled attribute");
    
    cleanup_test_container();
}

#[wasm_bindgen_test] 
fn button_focus_visible_indicators() {
    let container = mount_component(|| {
        view! {
            <Button>"Focusable Button"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    // Should have focus-visible styles
    assert!(
        class_list.contains("focus-visible:outline-none") &&
        class_list.contains("focus-visible:ring-2"),
        "Button should have proper focus-visible styling for accessibility"
    );
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_color_contrast_classes() {
    let container = mount_component(|| {
        view! {
            <Button variant=ButtonVariant::Default>
                "Default Button"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();
    
    // Should have proper text/background contrast classes
    assert!(
        class_list.contains("bg-primary") || 
        class_list.contains("text-primary-foreground"),
        "Button should have proper contrast classes for accessibility"
    );
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_keyboard_navigation() {
    let container = mount_component(|| {
        view! {
            <div>
                <Button id="btn1">"Button 1"</Button>
                <Button id="btn2">"Button 2"</Button>
                <Button id="btn3" disabled=Signal::from(true)>"Button 3 (Disabled)"</Button>
            </div>
        }
    });
    
    let button1 = container.query_selector("#btn1").unwrap().unwrap();
    let button2 = container.query_selector("#btn2").unwrap().unwrap();
    let button3 = container.query_selector("#btn3").unwrap().unwrap();
    
    // First button should be focusable
    button1.focus().unwrap();
    next_frame().await;
    
    let document = web_sys::window().unwrap().document().unwrap();
    assert_eq!(
        document.active_element().unwrap().get_attribute("id"),
        Some("btn1".to_string()),
        "First button should be focusable"
    );
    
    // Second button should be focusable
    button2.focus().unwrap();
    next_frame().await;
    
    assert_eq!(
        document.active_element().unwrap().get_attribute("id"),
        Some("btn2".to_string()),
        "Second button should be focusable"
    );
    
    // Disabled button should not be focusable via normal means
    // (though focus() method will still work - browser behavior varies)
    let html_button3: web_sys::HtmlButtonElement = button3.unchecked_into();
    assert!(html_button3.disabled(), "Disabled button should have disabled attribute");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_screen_reader_content() {
    let container = mount_component(|| {
        view! {
            <Button>
                <span class="sr-only">"Save"</span>
                <span aria-hidden="true">"💾"</span>
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let sr_only = button.query_selector(".sr-only").unwrap().unwrap();
    let icon = button.query_selector("[aria-hidden='true']").unwrap().unwrap();
    
    assert_eq!(sr_only.text_content(), Some("Save".to_string()));
    assert_eq!(icon.text_content(), Some("💾".to_string()));
    assert_eq!(icon.get_attribute("aria-hidden"), Some("true".to_string()));
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_role_and_type_attributes() {
    let container = mount_component(|| {
        view! {
            <div>
                <Button>"Default Button"</Button>
                <Button 
                    type="submit"
                    form="test-form"
                >
                    "Submit Button"
                </Button>
            </div>
        }
    });
    
    let default_button = container.query_selector("button").unwrap().unwrap();
    let buttons = container.query_selector_all("button").unwrap();
    
    // Default button should have type="button"
    assert_eq!(
        default_button.get_attribute("type"),
        Some("button".to_string()),
        "Default button should have type='button'"
    );
    
    // Should be able to have multiple buttons
    assert_eq!(buttons.length(), 2, "Should support multiple buttons in container");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_with_aria_describedby() {
    let container = mount_component(|| {
        view! {
            <div>
                <Button 
                    aria-describedby="btn-help"
                    id="help-button"
                >
                    "Help"
                </Button>
                <div id="btn-help">
                    "This button opens the help dialog"
                </div>
            </div>
        }
    });
    
    let button = container.query_selector("#help-button").unwrap().unwrap();
    let help_text = container.query_selector("#btn-help").unwrap().unwrap();
    
    assert_eq!(
        button.get_attribute("aria-describedby"),
        Some("btn-help".to_string()),
        "Button should support aria-describedby"
    );
    
    assert_eq!(
        help_text.text_content(),
        Some("This button opens the help dialog".to_string()),
        "Help text should be available"
    );
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
fn button_variants_maintain_accessibility() {
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
                <Button variant=variant.clone()>
                    {format!("{:?} Action", variant)}
                </Button>
            }
        });
        
        let button = container.query_selector("button").unwrap()
            .expect(&format!("Button variant {:?} should render", variant));
        
        // All variants should maintain button semantics
        assert_eq!(button.tag_name(), "BUTTON", 
                  "All button variants should render as button elements");
        
        // Should have focus styling
        let class_list = button.class_list();
        assert!(
            class_list.contains("focus-visible:outline-none") ||
            class_list.contains("focus-visible:ring-2"),
            "All button variants should have focus indicators"
        );
        
        cleanup_test_container();
    }
}
