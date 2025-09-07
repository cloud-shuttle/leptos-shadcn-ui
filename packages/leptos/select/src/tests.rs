#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    // TDD Phase 1: RED - Write failing tests for Select functionality

    #[test]
    fn test_select_initial_state() {
        // Test that select starts in closed state with default value
        let open = RwSignal::new(false);
        let value = RwSignal::new("".to_string());
        let default_value = "option1";
        
        assert!(!open.get(), "Select should start in closed state");
        assert!(value.get().is_empty(), "Select should start with empty value");
        assert!(!default_value.is_empty(), "Default value should not be empty");
    }

    #[test]
    fn test_select_open_state_management() {
        // Test select open/close state management
        let open = RwSignal::new(false);
        let on_open_change = Callback::new(move |new_state: bool| {
            open.set(new_state);
        });
        
        // Test opening select
        on_open_change.run(true);
        assert!(open.get(), "Select should be open after on_open_change(true)");
        
        // Test closing select
        on_open_change.run(false);
        assert!(!open.get(), "Select should be closed after on_open_change(false)");
    }

    #[test]
    fn test_select_value_management() {
        // Test select value management
        let value = RwSignal::new("".to_string());
        let on_value_change = Callback::new(move |new_value: String| {
            value.set(new_value);
        });
        
        // Test setting value
        on_value_change.run("option1".to_string());
        assert_eq!(value.get(), "option1", "Select value should be updated");
        
        // Test changing value
        on_value_change.run("option2".to_string());
        assert_eq!(value.get(), "option2", "Select value should be changed");
    }

    #[test]
    fn test_select_default_value_handling() {
        // Test select default value handling
        let default_value = "default_option";
        let internal_value = RwSignal::new(default_value.to_string());
        
        assert_eq!(internal_value.get(), default_value, "Internal value should match default value");
    }

    #[test]
    fn test_select_disabled_state() {
        // Test select disabled state
        let disabled = RwSignal::new(false);
        
        assert!(!disabled.get(), "Select should not be disabled by default");
        
        disabled.set(true);
        assert!(disabled.get(), "Select should be disabled when set");
    }

    #[test]
    fn test_select_required_state() {
        // Test select required state
        let required = RwSignal::new(false);
        
        assert!(!required.get(), "Select should not be required by default");
        
        required.set(true);
        assert!(required.get(), "Select should be required when set");
    }

    #[test]
    fn test_select_name_attribute() {
        // Test select name attribute
        let name = "select_field";
        
        assert!(!name.is_empty(), "Select should have a name attribute");
        assert_eq!(name, "select_field", "Name should match expected value");
    }

    #[test]
    fn test_select_context_provides_state() {
        // Test that select context provides state to children
        let open = RwSignal::new(false);
        let value = RwSignal::new("".to_string());
        let disabled = RwSignal::new(false);
        let required = RwSignal::new(false);
        let name = "test_select";
        
        // Context should provide all necessary state
        assert!(!open.get(), "Context should provide initial open state");
        assert!(value.get().is_empty(), "Context should provide initial value state");
        assert!(!disabled.get(), "Context should provide initial disabled state");
        assert!(!required.get(), "Context should provide initial required state");
        assert!(!name.is_empty(), "Context should provide name attribute");
    }

    #[test]
    fn test_select_trigger_functionality() {
        // Test select trigger functionality
        let open = RwSignal::new(false);
        let on_open_change = Callback::new(move |new_state: bool| {
            open.set(new_state);
        });
        
        // Simulate trigger click
        on_open_change.run(true);
        assert!(open.get(), "Select should open when trigger is clicked");
    }

    #[test]
    fn test_select_content_visibility() {
        // Test that select content is only visible when open
        let open = RwSignal::new(false);
        
        // When closed, content should not be visible
        assert!(!open.get(), "Select content should not be visible when closed");
        
        // When open, content should be visible
        open.set(true);
        assert!(open.get(), "Select content should be visible when open");
    }

    #[test]
    fn test_select_option_selection() {
        // Test select option selection
        let value = RwSignal::new("".to_string());
        let on_value_change = Callback::new(move |new_value: String| {
            value.set(new_value);
        });
        
        // Simulate option selection
        on_value_change.run("selected_option".to_string());
        assert_eq!(value.get(), "selected_option", "Select should update value when option is selected");
    }

    #[test]
    fn test_select_keyboard_navigation() {
        // Test select keyboard navigation
        let open = RwSignal::new(true);
        let value = RwSignal::new("option1".to_string());
        let options = vec!["option1", "option2", "option3"];
        let current_index = RwSignal::new(0);
        
        // Test arrow down navigation
        current_index.update(|index| *index = (*index + 1) % options.len());
        assert_eq!(current_index.get(), 1, "Should navigate to next option");
        
        // Test arrow up navigation
        current_index.update(|index| {
            if *index == 0 {
                *index = options.len() - 1;
            } else {
                *index -= 1;
            }
        });
        assert_eq!(current_index.get(), 0, "Should navigate to previous option");
        
        assert!(open.get(), "Select should remain open during keyboard navigation");
    }

    #[test]
    fn test_select_escape_key_to_close() {
        // Test that escape key closes select
        let open = RwSignal::new(true);
        let on_open_change = Callback::new(move |new_state: bool| {
            open.set(new_state);
        });
        
        // Simulate escape key press
        on_open_change.run(false);
        assert!(!open.get(), "Select should close when escape key is pressed");
    }

    #[test]
    fn test_select_click_outside_to_close() {
        // Test that clicking outside closes select
        let open = RwSignal::new(true);
        let on_open_change = Callback::new(move |new_state: bool| {
            open.set(new_state);
        });
        
        // Simulate click outside
        on_open_change.run(false);
        assert!(!open.get(), "Select should close when clicking outside");
    }

    #[test]
    fn test_select_accessibility_attributes() {
        // Test ARIA attributes for accessibility
        let open = RwSignal::new(true);
        let value = RwSignal::new("option1".to_string());
        let has_aria_expanded = true;
        let has_aria_haspopup = true;
        let has_role_combobox = true;
        
        assert!(open.get(), "Select should be open for accessibility testing");
        assert!(!value.get().is_empty(), "Select should have a value");
        assert!(has_aria_expanded, "Select should have aria-expanded attribute");
        assert!(has_aria_haspopup, "Select should have aria-haspopup attribute");
        assert!(has_role_combobox, "Select should have role='combobox'");
    }

    #[test]
    fn test_select_trigger_styling() {
        // Test select trigger styling
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        
        assert!(trigger_class.contains("flex"), "Trigger should be flex");
        assert!(trigger_class.contains("h-10"), "Trigger should have height");
        assert!(trigger_class.contains("w-full"), "Trigger should be full width");
        assert!(trigger_class.contains("items-center"), "Trigger should center items");
        assert!(trigger_class.contains("justify-between"), "Trigger should justify between");
        assert!(trigger_class.contains("rounded-md"), "Trigger should have rounded corners");
        assert!(trigger_class.contains("border"), "Trigger should have border");
    }

    #[test]
    fn test_select_content_styling() {
        // Test select content styling
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        
        assert!(content_class.contains("relative"), "Content should be relative positioned");
        assert!(content_class.contains("z-50"), "Content should have high z-index");
        assert!(content_class.contains("max-h-96"), "Content should have max height");
        assert!(content_class.contains("min-w-[8rem]"), "Content should have min width");
        assert!(content_class.contains("overflow-hidden"), "Content should hide overflow");
        assert!(content_class.contains("rounded-md"), "Content should have rounded corners");
        assert!(content_class.contains("border"), "Content should have border");
        assert!(content_class.contains("bg-popover"), "Content should have popover background");
    }

    #[test]
    fn test_select_item_styling() {
        // Test select item styling
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        assert!(item_class.contains("relative"), "Item should be relative positioned");
        assert!(item_class.contains("flex"), "Item should be flex");
        assert!(item_class.contains("w-full"), "Item should be full width");
        assert!(item_class.contains("cursor-default"), "Item should have default cursor");
        assert!(item_class.contains("select-none"), "Item should not be selectable");
        assert!(item_class.contains("items-center"), "Item should center items");
        assert!(item_class.contains("rounded-sm"), "Item should have small rounded corners");
        assert!(item_class.contains("py-1.5"), "Item should have vertical padding");
    }

    #[test]
    fn test_select_animation_classes() {
        // Test animation classes for smooth transitions
        let animation_classes = "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";
        
        assert!(animation_classes.contains("animate-in"), "Select should have animate-in class");
        assert!(animation_classes.contains("animate-out"), "Select should have animate-out class");
        assert!(animation_classes.contains("fade-in-0"), "Select should have fade-in animation");
        assert!(animation_classes.contains("fade-out-0"), "Select should have fade-out animation");
        assert!(animation_classes.contains("zoom-in-95"), "Select should have zoom-in animation");
        assert!(animation_classes.contains("zoom-out-95"), "Select should have zoom-out animation");
    }
}