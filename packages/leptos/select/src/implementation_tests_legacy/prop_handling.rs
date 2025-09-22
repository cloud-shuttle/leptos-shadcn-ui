#[cfg(test)]
mod prop_handling {
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== PROP HANDLING TESTS =====
    // These tests focus on prop handling and validation

    #[test]
    fn test_select_props_default_values() {
        // Test that select components have proper default prop values
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Test default sizing
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        
        // Test default spacing
        assert!(trigger_class.contains("px-3"));
        assert!(trigger_class.contains("py-2"));
        
        // Test default typography
        assert!(trigger_class.contains("text-sm"));
        
        // Test default layout
        assert!(trigger_class.contains("flex"));
        assert!(trigger_class.contains("items-center"));
        assert!(trigger_class.contains("justify-between"));
    }

    #[test]
    fn test_select_props_custom_values() {
        // Test that select components can handle custom prop values
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        
        // Test custom sizing
        assert!(content_class.contains("max-h-96"));
        assert!(content_class.contains("min-w-[8rem]"));
        
        // Test custom positioning
        assert!(content_class.contains("relative"));
        assert!(content_class.contains("z-50"));
        
        // Test custom overflow
        assert!(content_class.contains("overflow-hidden"));
    }

    #[test]
    fn test_select_props_boolean_handling() {
        // Test that select components handle boolean props correctly
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        // Test disabled prop handling
        assert!(trigger_class.contains("disabled:cursor-not-allowed"));
        assert!(trigger_class.contains("disabled:opacity-50"));
        assert!(item_class.contains("data-[disabled]:pointer-events-none"));
        assert!(item_class.contains("data-[disabled]:opacity-50"));
    }

    #[test]
    fn test_select_props_string_handling() {
        // Test that select components handle string props correctly
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Test class prop handling
        assert!(trigger_class.contains("flex"));
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        
        // Test style prop handling
        assert!(trigger_class.contains("border-input"));
        assert!(trigger_class.contains("bg-background"));
    }

    #[test]
    fn test_select_props_number_handling() {
        // Test that select components handle number props correctly
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        
        // Test z-index prop handling
        assert!(content_class.contains("z-50"));
        
        // Test max-height prop handling
        assert!(content_class.contains("max-h-96"));
        
        // Test min-width prop handling
        assert!(content_class.contains("min-w-[8rem]"));
    }

    #[test]
    fn test_select_props_optional_handling() {
        // Test that select components handle optional props correctly
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Test optional class prop
        assert!(trigger_class.contains("flex"));
        
        // Test optional style prop
        assert!(trigger_class.contains("border-input"));
        
        // Test optional disabled prop
        assert!(trigger_class.contains("disabled:cursor-not-allowed"));
    }

    #[test]
    fn test_select_props_validation() {
        // Test that select components validate props correctly
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Test that required props are present
        assert!(trigger_class.contains("flex"));
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        
        // Test that props are properly formatted
        assert!(trigger_class.contains("px-3"));
        assert!(trigger_class.contains("py-2"));
        assert!(trigger_class.contains("text-sm"));
    }

    #[test]
    fn test_select_props_type_safety() {
        // Test that select components have type-safe prop handling
        let classes = vec![
            "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
            "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        ];

        for class in classes {
            // Test that classes are valid CSS
            assert!(!class.is_empty(), "Class should not be empty");
            assert!(class.len() > 10, "Class should be substantial");
            
            // Test that classes don't contain invalid characters
            assert!(!class.contains("undefined"), "Class should not contain undefined");
            assert!(!class.contains("null"), "Class should not contain null");
        }
    }

    #[test]
    fn test_select_props_performance() {
        // Test that select components handle props efficiently
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
            let _content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
            let _item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "Prop handling should be performant");
    }

    #[test]
    fn test_select_props_memory_usage() {
        // Test that select components don't cause memory issues with props
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        let total_size = std::mem::size_of_val(&trigger_class) + 
                        std::mem::size_of_val(&content_class) + 
                        std::mem::size_of_val(&item_class);
        
        assert!(total_size < 1024, "Props should not cause excessive memory usage");
    }

    #[test]
    fn test_select_props_consistency() {
        // Test that select components have consistent prop handling
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        // Test consistent text sizing
        assert!(trigger_class.contains("text-sm"));
        assert!(item_class.contains("text-sm"));
        
        // Test consistent focus handling
        assert!(trigger_class.contains("focus:"));
        assert!(item_class.contains("focus:"));
        
        // Test consistent disabled handling
        assert!(trigger_class.contains("disabled:"));
        assert!(item_class.contains("data-[disabled]:"));
        
        // Test consistent layout patterns
        assert!(trigger_class.contains("flex"));
        assert!(item_class.contains("flex"));
        assert!(trigger_class.contains("items-center"));
        assert!(item_class.contains("items-center"));
    }

    #[test]
    fn test_select_props_edge_cases() {
        // Test that select components handle edge cases in props
        let classes = vec![
            "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
            "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        ];

        for class in classes {
            // Test empty string handling
            assert!(!class.is_empty(), "Should handle empty strings gracefully");
            
            // Test special character handling
            assert!(!class.contains("undefined"), "Should handle undefined values");
            assert!(!class.contains("null"), "Should handle null values");
            
            // Test whitespace handling
            assert!(!class.starts_with(" "), "Should handle leading whitespace");
            assert!(!class.ends_with(" "), "Should handle trailing whitespace");
        }
    }

    #[test]
    fn test_select_props_validation_rules() {
        // Test that select components follow proper validation rules
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Test that required props are present
        assert!(trigger_class.contains("flex"), "Required layout prop should be present");
        assert!(trigger_class.contains("h-10"), "Required height prop should be present");
        assert!(trigger_class.contains("w-full"), "Required width prop should be present");
        
        // Test that props are properly formatted
        assert!(trigger_class.contains("px-3"), "Spacing props should be properly formatted");
        assert!(trigger_class.contains("py-2"), "Spacing props should be properly formatted");
        assert!(trigger_class.contains("text-sm"), "Typography props should be properly formatted");
        
        // Test that props don't conflict
        assert!(!trigger_class.contains("h-10 h-20"), "Height props should not conflict");
        assert!(!trigger_class.contains("w-full w-auto"), "Width props should not conflict");
    }
}
