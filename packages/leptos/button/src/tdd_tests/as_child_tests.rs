//! as_child Functionality Tests for Button Component
//! 
//! This module tests the as_child functionality and ButtonChildProps behavior.

#[cfg(test)]
mod as_child_tests {
    use crate::default::{ButtonChildProps, BUTTON_CLASS};
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_as_child_props_structure() {
        // TDD: Test ButtonChildProps structure and behavior
        let props = ButtonChildProps {
            class: "test-class bg-primary h-10".to_string(),
            id: "test-button-id".to_string(),
            style: "color: red; margin: 10px;".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };
        
        // Test property access
        assert_eq!(props.class, "test-class bg-primary h-10");
        assert_eq!(props.id, "test-button-id");
        assert_eq!(props.style, "color: red; margin: 10px;");
        assert!(!props.disabled);
        assert_eq!(props.r#type, "button");
        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_as_child_callback_execution() {
        // TDD: Test as_child callback behavior
        let callback_executed = Arc::new(Mutex::new(false));
        let callback_executed_clone = Arc::clone(&callback_executed);
        
        let as_child_callback = Callback::new(move |props: ButtonChildProps| {
            *callback_executed_clone.lock().unwrap() = true;
            
            // Verify props are properly passed
            assert!(props.class.contains("inline-flex"));
            assert_eq!(props.r#type, "button");
            
            // Return a mock view (in real usage this would be a proper view)
            view! { <div class=props.class>Custom Element</div> }.into_any()
        });
        
        // Simulate as_child execution with proper props
        let test_props = ButtonChildProps {
            class: format!("{} bg-primary h-10", BUTTON_CLASS),
            id: "test-id".to_string(),
            style: "".to_string(),
            disabled: false,
            r#type: "button".to_string(),
            onclick: None,
        };
        
        as_child_callback.run(test_props);
        assert!(*callback_executed.lock().unwrap());
    }
}
