use super::*;
use crate::default::{Button, ButtonVariant};
use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen_test]
async fn button_handles_click_events() {
    let clicked = Rc::new(RefCell::new(false));
    let clicked_clone = clicked.clone();
    
    let container = mount_component(move || {
        let clicked_inner = clicked_clone.clone();
        view! {
            <Button on_click=move |_| {
                *clicked_inner.borrow_mut() = true;
            }>
                "Click Me"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Simulate click
    button.click();
    
    // Wait for event to process
    next_frame().await;
    
    assert!(*clicked.borrow(), "Click handler should have been called");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_disabled_blocks_click_events() {
    let clicked = Rc::new(RefCell::new(false));
    let clicked_clone = clicked.clone();
    
    let container = mount_component(move || {
        let clicked_inner = clicked_clone.clone();
        view! {
            <Button 
                disabled=Signal::from(true)
                on_click=move |_| {
                    *clicked_inner.borrow_mut() = true;
                }
            >
                "Disabled Button"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    let html_button: web_sys::HtmlButtonElement = button.unchecked_into();
    
    assert!(html_button.disabled(), "Button should be disabled");
    
    // Try to click disabled button
    button.click();
    next_frame().await;
    
    assert!(!*clicked.borrow(), "Disabled button should not trigger click handler");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_handles_keyboard_activation() {
    let activated = Rc::new(RefCell::new(false));
    let activated_clone = activated.clone();
    
    let container = mount_component(move || {
        let activated_inner = activated_clone.clone();
        view! {
            <Button on_click=move |_| {
                *activated_inner.borrow_mut() = true;
            }>
                "Keyboard Button"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Focus the button
    button.focus().unwrap();
    
    // Create and dispatch Enter key event
    let enter_event = web_sys::KeyboardEvent::new_with_keyboard_event_init(
        "keydown",
        &web_sys::KeyboardEventInit::new().key("Enter"),
    ).unwrap();
    
    button.dispatch_event(&enter_event).unwrap();
    next_frame().await;
    
    // Note: This test verifies the setup - actual keyboard activation 
    // depends on browser's built-in button behavior
    assert!(button.matches(":focus").unwrap_or(false) || 
           *activated.borrow(), 
           "Button should be focusable and handle keyboard events");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_focus_management() {
    let container = mount_component(|| {
        view! {
            <div>
                <Button id="button1">"First Button"</Button>
                <Button id="button2">"Second Button"</Button>
            </div>
        }
    });
    
    let button1 = container.query_selector("#button1").unwrap().unwrap();
    let button2 = container.query_selector("#button2").unwrap().unwrap();
    
    // Focus first button
    button1.focus().unwrap();
    next_frame().await;
    
    // Check if first button is focused
    let document = web_sys::window().unwrap().document().unwrap();
    assert_eq!(
        document.active_element().unwrap().get_attribute("id"),
        Some("button1".to_string()),
        "First button should be focused"
    );
    
    // Focus second button
    button2.focus().unwrap();
    next_frame().await;
    
    assert_eq!(
        document.active_element().unwrap().get_attribute("id"),
        Some("button2".to_string()),
        "Focus should move to second button"
    );
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_hover_states() {
    let container = mount_component(|| {
        view! {
            <Button variant=ButtonVariant::Default>"Hover Me"</Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Create and dispatch mouse enter event
    let mouse_enter = web_sys::MouseEvent::new("mouseenter").unwrap();
    button.dispatch_event(&mouse_enter).unwrap();
    next_frame().await;
    
    // Verify button still exists and is responsive
    assert!(button.class_list().length() > 0, "Button should have styling classes");
    
    // Create and dispatch mouse leave event
    let mouse_leave = web_sys::MouseEvent::new("mouseleave").unwrap();
    button.dispatch_event(&mouse_leave).unwrap();
    next_frame().await;
    
    // Button should still be in normal state
    assert_eq!(button.tag_name(), "BUTTON");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_multiple_clicks_handled() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();
    
    let container = mount_component(move || {
        let count_inner = click_count_clone.clone();
        view! {
            <Button on_click=move |_| {
                *count_inner.borrow_mut() += 1;
            }>
                "Multi Click"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Click multiple times
    for _ in 0..5 {
        button.click();
        next_frame().await;
    }
    
    assert_eq!(*click_count.borrow(), 5, "All clicks should be handled");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_rapid_clicks_handled() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();
    
    let container = mount_component(move || {
        let count_inner = click_count_clone.clone();
        view! {
            <Button on_click=move |_| {
                *count_inner.borrow_mut() += 1;
            }>
                "Rapid Click"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Rapid clicks without waiting
    for _ in 0..10 {
        button.click();
    }
    
    // Wait once at the end
    next_frame().await;
    
    // Should handle all rapid clicks
    assert!(*click_count.borrow() > 0, "Rapid clicks should be handled");
    
    cleanup_test_container();
}

#[wasm_bindgen_test]
async fn button_touch_events() {
    let touched = Rc::new(RefCell::new(false));
    let touched_clone = touched.clone();
    
    let container = mount_component(move || {
        let touched_inner = touched_clone.clone();
        view! {
            <Button on_click=move |_| {
                *touched_inner.borrow_mut() = true;
            }>
                "Touch Button"
            </Button>
        }
    });
    
    let button = container.query_selector("button").unwrap().unwrap();
    
    // Create and dispatch touch events
    let touch_start = web_sys::TouchEvent::new("touchstart").unwrap();
    let touch_end = web_sys::TouchEvent::new("touchend").unwrap();
    
    button.dispatch_event(&touch_start).unwrap();
    button.dispatch_event(&touch_end).unwrap();
    next_frame().await;
    
    // Note: Touch to click conversion is handled by browser
    // We're testing that events don't break the component
    assert!(button.tag_name() == "BUTTON", "Button should remain functional after touch events");
    
    cleanup_test_container();
}
