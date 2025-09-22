//! Component Creation & Props Tests for Button Component
//! 
//! This module tests the basic creation and configuration of Button components.

#[cfg(test)]
mod component_creation_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize};
    use leptos::prelude::*;

    #[test]
    fn test_button_component_creation_with_default_props() {
        // TDD: Test that Button component can be created with default properties
        let button_view = view! {
            <Button>"Default Button"</Button>
        };
        
        // Component creation should not panic
        let _view = button_view.into_view();
    }

    #[test] 
    fn test_button_component_with_all_variants() {
        // TDD: Test that Button can be created with each variant
        let variants = vec![
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ];
        
        for variant in variants {
            let button_view = view! {
                <Button variant=variant.clone()>"Test Button"</Button>
            };
            
            // Each variant should create a valid component
            let _view = button_view.into_view();
        }
    }

    #[test]
    fn test_button_component_with_all_sizes() {
        // TDD: Test that Button can be created with each size
        let sizes = vec![
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ];
        
        for size in sizes {
            let button_view = view! {
                <Button size=size.clone()>"Test Button"</Button>
            };
            
            // Each size should create a valid component
            let _view = button_view.into_view();
        }
    }
}
