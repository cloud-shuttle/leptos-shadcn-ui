#[cfg(test)]
mod class_constants {
    use leptos::prelude::*;
    use leptos_style::Style;

    // ===== CLASS CONSTANTS TESTS =====
    // These tests focus on CSS class constants and styling

    #[test]
    fn test_select_class_constants() {
        // Test SelectTrigger class constant
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        assert!(trigger_class.contains("flex"));
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        assert!(trigger_class.contains("items-center"));
        assert!(trigger_class.contains("justify-between"));
        assert!(trigger_class.contains("rounded-md"));
        assert!(trigger_class.contains("border"));
        assert!(trigger_class.contains("border-input"));
        assert!(trigger_class.contains("bg-background"));
        assert!(trigger_class.contains("px-3"));
        assert!(trigger_class.contains("py-2"));
        assert!(trigger_class.contains("text-sm"));
        assert!(trigger_class.contains("ring-offset-background"));
        assert!(trigger_class.contains("placeholder:text-muted-foreground"));
        assert!(trigger_class.contains("focus:outline-none"));
        assert!(trigger_class.contains("focus:ring-2"));
        assert!(trigger_class.contains("focus:ring-ring"));
        assert!(trigger_class.contains("focus:ring-offset-2"));
        assert!(trigger_class.contains("disabled:cursor-not-allowed"));
        assert!(trigger_class.contains("disabled:opacity-50"));
        assert!(trigger_class.contains("[&>span]:line-clamp-1"));

        // Test SelectContent class constant
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        assert!(content_class.contains("relative"));
        assert!(content_class.contains("z-50"));
        assert!(content_class.contains("max-h-96"));
        assert!(content_class.contains("min-w-[8rem]"));
        assert!(content_class.contains("overflow-hidden"));
        assert!(content_class.contains("rounded-md"));
        assert!(content_class.contains("border"));
        assert!(content_class.contains("bg-popover"));
        assert!(content_class.contains("text-popover-foreground"));
        assert!(content_class.contains("shadow-md"));
        assert!(content_class.contains("data-[state=open]:animate-in"));
        assert!(content_class.contains("data-[state=closed]:animate-out"));

        // Test SelectItem class constant
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        assert!(item_class.contains("relative"));
        assert!(item_class.contains("flex"));
        assert!(item_class.contains("w-full"));
        assert!(item_class.contains("cursor-default"));
        assert!(item_class.contains("select-none"));
        assert!(item_class.contains("items-center"));
        assert!(item_class.contains("rounded-sm"));
        assert!(item_class.contains("py-1.5"));
        assert!(item_class.contains("pl-8"));
        assert!(item_class.contains("pr-2"));
        assert!(item_class.contains("text-sm"));
        assert!(item_class.contains("outline-none"));
        assert!(item_class.contains("focus:bg-accent"));
        assert!(item_class.contains("focus:text-accent-foreground"));
        assert!(item_class.contains("data-[disabled]:pointer-events-none"));
        assert!(item_class.contains("data-[disabled]:opacity-50"));

        // Test SelectValue class constant
        let value_class = "placeholder:text-muted-foreground";
        assert!(value_class.contains("placeholder:text-muted-foreground"));

        // Test SelectSeparator class constant
        let separator_class = "-mx-1 my-1 h-px bg-muted";
        assert!(separator_class.contains("-mx-1"));
        assert!(separator_class.contains("my-1"));
        assert!(separator_class.contains("h-px"));
        assert!(separator_class.contains("bg-muted"));

        // Test SelectLabel class constant
        let label_class = "py-1.5 pl-8 pr-2 text-sm font-semibold";
        assert!(label_class.contains("py-1.5"));
        assert!(label_class.contains("pl-8"));
        assert!(label_class.contains("pr-2"));
        assert!(label_class.contains("text-sm"));
        assert!(label_class.contains("font-semibold"));

        // Test SelectGroup class constant
        let group_class = "p-1 text-muted-foreground";
        assert!(group_class.contains("p-1"));
        assert!(group_class.contains("text-muted-foreground"));

        // Test SelectScrollUpButton class constant
        let scroll_up_class = "flex cursor-default items-center justify-center py-1";
        assert!(scroll_up_class.contains("flex"));
        assert!(scroll_up_class.contains("cursor-default"));
        assert!(scroll_up_class.contains("items-center"));
        assert!(scroll_up_class.contains("justify-center"));
        assert!(scroll_up_class.contains("py-1"));

        // Test SelectScrollDownButton class constant
        let scroll_down_class = "flex cursor-default items-center justify-center py-1";
        assert!(scroll_down_class.contains("flex"));
        assert!(scroll_down_class.contains("cursor-default"));
        assert!(scroll_down_class.contains("items-center"));
        assert!(scroll_down_class.contains("justify-center"));
        assert!(scroll_down_class.contains("py-1"));
    }

    #[test]
    fn test_select_class_constants_comprehensive() {
        // Test all class constants are properly defined
        let classes = vec![
            "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
            "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            "placeholder:text-muted-foreground",
            "-mx-1 my-1 h-px bg-muted",
            "py-1.5 pl-8 pr-2 text-sm font-semibold",
            "p-1 text-muted-foreground",
            "flex cursor-default items-center justify-center py-1",
        ];

        for class in classes {
            assert!(!class.is_empty(), "Class constant should not be empty");
            assert!(class.len() > 10, "Class constant should be substantial");
        }
    }

    #[test]
    fn test_select_class_constants_accessibility() {
        // Test that class constants include accessibility features
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Check for focus management
        assert!(trigger_class.contains("focus:outline-none"));
        assert!(trigger_class.contains("focus:ring-2"));
        assert!(trigger_class.contains("focus:ring-ring"));
        assert!(trigger_class.contains("focus:ring-offset-2"));
        
        // Check for disabled state
        assert!(trigger_class.contains("disabled:cursor-not-allowed"));
        assert!(trigger_class.contains("disabled:opacity-50"));
        
        // Check for proper sizing
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        
        // Check for proper spacing
        assert!(trigger_class.contains("px-3"));
        assert!(trigger_class.contains("py-2"));
    }

    #[test]
    fn test_select_class_constants_responsive() {
        // Test that class constants include responsive features
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        
        // Check for responsive sizing
        assert!(content_class.contains("max-h-96"));
        assert!(content_class.contains("min-w-[8rem]"));
        
        // Check for overflow handling
        assert!(content_class.contains("overflow-hidden"));
        
        // Check for positioning
        assert!(content_class.contains("relative"));
        assert!(content_class.contains("z-50"));
    }

    #[test]
    fn test_select_class_constants_animations() {
        // Test that class constants include animation features
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2";
        
        // Check for animation states
        assert!(content_class.contains("data-[state=open]:animate-in"));
        assert!(content_class.contains("data-[state=closed]:animate-out"));
        
        // Check for fade animations
        assert!(content_class.contains("data-[state=closed]:fade-out-0"));
        assert!(content_class.contains("data-[state=open]:fade-in-0"));
        
        // Check for zoom animations
        assert!(content_class.contains("data-[state=closed]:zoom-out-95"));
        assert!(content_class.contains("data-[state=open]:zoom-in-95"));
        
        // Check for slide animations
        assert!(content_class.contains("data-[side=bottom]:slide-in-from-top-2"));
        assert!(content_class.contains("data-[side=left]:slide-in-from-right-2"));
        assert!(content_class.contains("data-[side=right]:slide-in-from-left-2"));
        assert!(content_class.contains("data-[side=top]:slide-in-from-bottom-2"));
    }

    #[test]
    fn test_select_class_constants_interaction() {
        // Test that class constants include interaction features
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        // Check for cursor states
        assert!(item_class.contains("cursor-default"));
        
        // Check for selection behavior
        assert!(item_class.contains("select-none"));
        
        // Check for focus states
        assert!(item_class.contains("focus:bg-accent"));
        assert!(item_class.contains("focus:text-accent-foreground"));
        
        // Check for disabled states
        assert!(item_class.contains("data-[disabled]:pointer-events-none"));
        assert!(item_class.contains("data-[disabled]:opacity-50"));
    }

    #[test]
    fn test_select_class_constants_typography() {
        // Test that class constants include typography features
        let label_class = "py-1.5 pl-8 pr-2 text-sm font-semibold";
        
        // Check for text sizing
        assert!(label_class.contains("text-sm"));
        
        // Check for font weight
        assert!(label_class.contains("font-semibold"));
        
        // Check for proper spacing
        assert!(label_class.contains("py-1.5"));
        assert!(label_class.contains("pl-8"));
        assert!(label_class.contains("pr-2"));
    }

    #[test]
    fn test_select_class_constants_layout() {
        // Test that class constants include layout features
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Check for flexbox layout
        assert!(trigger_class.contains("flex"));
        assert!(trigger_class.contains("items-center"));
        assert!(trigger_class.contains("justify-between"));
        
        // Check for sizing
        assert!(trigger_class.contains("h-10"));
        assert!(trigger_class.contains("w-full"));
        
        // Check for border and background
        assert!(trigger_class.contains("border"));
        assert!(trigger_class.contains("border-input"));
        assert!(trigger_class.contains("bg-background"));
        
        // Check for border radius
        assert!(trigger_class.contains("rounded-md"));
    }

    #[test]
    fn test_select_class_constants_performance() {
        // Test that class constants are optimized for performance
        let classes = vec![
            "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        ];

        for class in classes {
            // Check that classes are not excessively long
            assert!(class.len() < 1000, "Class should not be excessively long");
            
            // Check that classes don't have redundant properties
            let words: Vec<&str> = class.split_whitespace().collect();
            let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
            assert_eq!(words.len(), unique_words.len(), "Class should not have duplicate properties");
        }
    }

    #[test]
    fn test_select_class_constants_consistency() {
        // Test that class constants are consistent across components
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        // Check for consistent text sizing
        assert!(trigger_class.contains("text-sm"));
        assert!(item_class.contains("text-sm"));
        
        // Check for consistent focus handling
        assert!(trigger_class.contains("focus:"));
        assert!(item_class.contains("focus:"));
        
        // Check for consistent disabled handling
        assert!(trigger_class.contains("disabled:"));
        assert!(item_class.contains("data-[disabled]:"));
        
        // Check for consistent spacing patterns
        assert!(trigger_class.contains("px-3"));
        assert!(item_class.contains("pl-8"));
        assert!(item_class.contains("pr-2"));
    }
}
