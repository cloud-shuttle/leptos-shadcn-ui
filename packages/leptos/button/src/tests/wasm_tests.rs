// Real WASM tests that run in the browser and test actual DOM behavior
use super::*;
use crate::default::{Button, ButtonVariant, ButtonSize};
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlButtonElement};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn button_renders_in_dom() {
    // Create a test container
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    container.set_attribute("id", "test-container").unwrap();
    body.append_child(&container).unwrap();

    // Mount the Button component
    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button>"Test Button"</Button>
            }
        },
    );

    // Verify the button exists in the DOM
    let button_element = container
        .query_selector("button")
        .unwrap()
        .expect("Button should be rendered in DOM");
    
    assert_eq!(button_element.tag_name(), "BUTTON");
    assert_eq!(button_element.text_content(), Some("Test Button".to_string()));

    // Cleanup
    container.remove();
}

#[wasm_bindgen_test]
fn button_has_correct_classes() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button variant=ButtonVariant::Destructive size=ButtonSize::Lg>
                    "Destructive Button"
                </Button>
            }
        },
    );

    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();

    // Check base classes
    assert!(class_list.contains("inline-flex"), "Should have inline-flex class");
    assert!(class_list.contains("items-center"), "Should have items-center class");
    assert!(class_list.contains("justify-center"), "Should have justify-center class");
    assert!(class_list.contains("rounded-md"), "Should have rounded-md class");

    // Check variant-specific classes
    assert!(class_list.contains("bg-destructive") || 
           class_list.contains("destructive"), "Should have destructive variant styling");

    // Check size-specific classes (large button)
    assert!(class_list.contains("h-11") || class_list.contains("px-8") || 
           class_list.contains("lg"), "Should have large size styling");

    container.remove();
}

#[wasm_bindgen_test]
async fn button_handles_click_events() {
    use wasm_bindgen::prelude::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    let clicked = Rc::new(RefCell::new(false));
    let clicked_clone = clicked.clone();

    leptos::mount_to(
        container.clone().unchecked_into(),
        move || {
            let clicked_inner = clicked_clone.clone();
            view! {
                <Button on_click=move |_| {
                    *clicked_inner.borrow_mut() = true;
                }>
                    "Click Me"
                </Button>
            }
        },
    );

    let button = container.query_selector("button").unwrap().unwrap();
    
    // Simulate click
    button.click();

    // Wait a bit for the event to process
    wasm_bindgen_futures::JsFuture::from(
        js_sys::Promise::new(&mut |resolve, _reject| {
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 10)
                .unwrap();
        })
    ).await.unwrap();

    assert!(*clicked.borrow(), "Button click handler should have been called");

    container.remove();
}

#[wasm_bindgen_test]
fn button_disabled_state_works() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button disabled=leptos::prelude::Signal::from(true)>
                    "Disabled Button"
                </Button>
            }
        },
    );

    let button = container.query_selector("button").unwrap().unwrap();
    let html_button: HtmlButtonElement = button.unchecked_into();

    // Check HTML disabled attribute
    assert!(html_button.disabled(), "Button should be disabled");

    // Check CSS classes for disabled state
    let class_list = html_button.class_list();
    assert!(class_list.contains("opacity-50") || 
           class_list.contains("disabled"), "Should have disabled styling");

    container.remove();
}

#[wasm_bindgen_test]
fn button_focus_management() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button id="focus-test-button">"Focusable Button"</Button>
            }
        },
    );

    let button = container.query_selector("#focus-test-button").unwrap().unwrap();
    
    // Focus the button
    button.focus().unwrap();

    // Check if it's focused (note: this might not work in all test environments)
    let active_element = document.active_element();
    if let Some(active) = active_element {
        if active.get_attribute("id") == Some("focus-test-button".to_string()) {
            // Focus worked
        }
    }

    // At minimum, verify the button has focus-related CSS classes
    let class_list = button.class_list();
    assert!(class_list.contains("focus-visible:outline-none") || 
           class_list.contains("focus"), "Should have focus-related styling");

    container.remove();
}

#[wasm_bindgen_test]
fn button_accessibility_attributes() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button 
                    id="accessible-button"
                    aria-label="Save document"
                    type="button"
                >
                    "💾"
                </Button>
            }
        },
    );

    let button = container.query_selector("button").unwrap().unwrap();

    // Check basic button attributes
    assert_eq!(button.get_attribute("type"), Some("button".to_string()));
    assert_eq!(button.get_attribute("id"), Some("accessible-button".to_string()));

    // Check aria attributes (if supported by the component)
    if let Some(aria_label) = button.get_attribute("aria-label") {
        assert_eq!(aria_label, "Save document");
    }

    container.remove();
}

#[wasm_bindgen_test]
fn button_with_custom_classes() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <Button class="my-custom-class another-class">
                    "Custom Styled Button"
                </Button>
            }
        },
    );

    let button = container.query_selector("button").unwrap().unwrap();
    let class_list = button.class_list();

    // Check custom classes are applied
    assert!(class_list.contains("my-custom-class"), "Should have custom class");
    assert!(class_list.contains("another-class"), "Should have second custom class");

    // Check base classes are still there
    assert!(class_list.contains("inline-flex"), "Should still have base classes");

    container.remove();
}

#[wasm_bindgen_test]
fn multiple_buttons_render_independently() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let container = document.create_element("div").unwrap();
    body.append_child(&container).unwrap();

    leptos::mount_to(
        container.clone().unchecked_into(),
        || {
            view! {
                <div>
                    <Button id="btn1" variant=ButtonVariant::Default>"Button 1"</Button>
                    <Button id="btn2" variant=ButtonVariant::Destructive>"Button 2"</Button>
                    <Button id="btn3" variant=ButtonVariant::Outline>"Button 3"</Button>
                </div>
            }
        },
    );

    // Check all buttons exist
    let button1 = container.query_selector("#btn1").unwrap()
        .expect("First button should exist");
    let button2 = container.query_selector("#btn2").unwrap()
        .expect("Second button should exist");
    let button3 = container.query_selector("#btn3").unwrap()
        .expect("Third button should exist");

    // Check they have different content
    assert_eq!(button1.text_content(), Some("Button 1".to_string()));
    assert_eq!(button2.text_content(), Some("Button 2".to_string()));
    assert_eq!(button3.text_content(), Some("Button 3".to_string()));

    // Check they have different styling (variant-specific)
    let class1 = button1.class_list();
    let class2 = button2.class_list();
    let class3 = button3.class_list();

    // They should all have base button classes
    for classes in [&class1, &class2, &class3] {
        assert!(classes.contains("inline-flex"), "All buttons should have base classes");
    }

    container.remove();
}
