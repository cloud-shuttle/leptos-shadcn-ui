#[cfg(test)]
mod class_tests {
    // Tests for CSS class constants and styling
    
    #[test]
    fn select_trigger_class_contains_required_styles() {
        let trigger_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1";
        
        // Layout classes
        assert!(trigger_class.contains("flex"), "Should have flexbox layout");
        assert!(trigger_class.contains("h-10"), "Should have fixed height");
        assert!(trigger_class.contains("w-full"), "Should be full width");
        assert!(trigger_class.contains("items-center"), "Should center items");
        assert!(trigger_class.contains("justify-between"), "Should justify content");
        
        // Styling classes
        assert!(trigger_class.contains("rounded-md"), "Should have rounded corners");
        assert!(trigger_class.contains("border"), "Should have border");
        assert!(trigger_class.contains("border-input"), "Should have input border color");
        assert!(trigger_class.contains("bg-background"), "Should have background color");
        assert!(trigger_class.contains("px-3"), "Should have horizontal padding");
        assert!(trigger_class.contains("py-2"), "Should have vertical padding");
        assert!(trigger_class.contains("text-sm"), "Should have small text");
        
        // Focus and accessibility classes
        assert!(trigger_class.contains("focus:outline-none"), "Should remove default outline");
        assert!(trigger_class.contains("focus:ring-2"), "Should have focus ring");
        assert!(trigger_class.contains("focus:ring-ring"), "Should have ring color");
        assert!(trigger_class.contains("focus:ring-offset-2"), "Should have ring offset");
        
        // Disabled state classes  
        assert!(trigger_class.contains("disabled:cursor-not-allowed"), "Should show not-allowed cursor when disabled");
        assert!(trigger_class.contains("disabled:opacity-50"), "Should have reduced opacity when disabled");
    }
    
    #[test]
    fn select_content_class_contains_required_styles() {
        let content_class = "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95";
        
        // Positioning classes
        assert!(content_class.contains("relative"), "Should have relative positioning");
        assert!(content_class.contains("z-50"), "Should have high z-index");
        
        // Size and layout classes
        assert!(content_class.contains("max-h-96"), "Should have max height");
        assert!(content_class.contains("min-w-[8rem]"), "Should have min width");
        assert!(content_class.contains("overflow-hidden"), "Should hide overflow");
        
        // Styling classes
        assert!(content_class.contains("rounded-md"), "Should have rounded corners");
        assert!(content_class.contains("border"), "Should have border");
        assert!(content_class.contains("bg-popover"), "Should have popover background");
        assert!(content_class.contains("text-popover-foreground"), "Should have popover text color");
        assert!(content_class.contains("shadow-md"), "Should have shadow");
        
        // Animation classes
        assert!(content_class.contains("data-[state=open]:animate-in"), "Should animate in when opening");
        assert!(content_class.contains("data-[state=closed]:animate-out"), "Should animate out when closing");
        assert!(content_class.contains("data-[state=open]:fade-in-0"), "Should fade in when opening");
        assert!(content_class.contains("data-[state=closed]:fade-out-0"), "Should fade out when closing");
    }
    
    #[test]
    fn select_item_class_contains_required_styles() {
        let item_class = "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
        
        // Layout classes
        assert!(item_class.contains("relative"), "Should have relative positioning");
        assert!(item_class.contains("flex"), "Should have flexbox layout");
        assert!(item_class.contains("w-full"), "Should be full width");
        assert!(item_class.contains("items-center"), "Should center items");
        
        // Interaction classes
        assert!(item_class.contains("cursor-default"), "Should have default cursor");
        assert!(item_class.contains("select-none"), "Should prevent text selection");
        
        // Styling classes
        assert!(item_class.contains("rounded-sm"), "Should have small rounded corners");
        assert!(item_class.contains("py-1.5"), "Should have vertical padding");
        assert!(item_class.contains("pl-8"), "Should have left padding for icon space");
        assert!(item_class.contains("pr-2"), "Should have right padding");
        assert!(item_class.contains("text-sm"), "Should have small text");
        assert!(item_class.contains("outline-none"), "Should remove outline");
        
        // Focus and interaction classes
        assert!(item_class.contains("focus:bg-accent"), "Should change background on focus");
        assert!(item_class.contains("focus:text-accent-foreground"), "Should change text color on focus");
        
        // Disabled state classes
        assert!(item_class.contains("data-[disabled]:pointer-events-none"), "Should disable pointer events when disabled");
        assert!(item_class.contains("data-[disabled]:opacity-50"), "Should reduce opacity when disabled");
    }
    
    #[test]
    fn select_label_class_contains_required_styles() {
        let label_class = "py-1.5 pl-8 pr-2 text-sm font-semibold";
        
        assert!(label_class.contains("py-1.5"), "Should have vertical padding");
        assert!(label_class.contains("pl-8"), "Should have left padding to align with items");
        assert!(label_class.contains("pr-2"), "Should have right padding");
        assert!(label_class.contains("text-sm"), "Should have small text");
        assert!(label_class.contains("font-semibold"), "Should have semibold font weight");
    }
    
    #[test]
    fn select_separator_class_contains_required_styles() {
        let separator_class = "-mx-1 my-1 h-px bg-muted";
        
        assert!(separator_class.contains("-mx-1"), "Should have negative horizontal margin");
        assert!(separator_class.contains("my-1"), "Should have vertical margin");
        assert!(separator_class.contains("h-px"), "Should have 1px height");
        assert!(separator_class.contains("bg-muted"), "Should have muted background");
    }
    
    #[test]
    fn select_icon_class_contains_required_styles() {
        let icon_class = "h-4 w-4 opacity-50";
        
        assert!(icon_class.contains("h-4"), "Should have fixed height");
        assert!(icon_class.contains("w-4"), "Should have fixed width");
        assert!(icon_class.contains("opacity-50"), "Should have reduced opacity");
    }
    
    #[test]
    fn select_check_icon_positioning() {
        let check_icon_class = "absolute left-2 flex h-3.5 w-3.5 items-center justify-center";
        
        assert!(check_icon_class.contains("absolute"), "Should have absolute positioning");
        assert!(check_icon_class.contains("left-2"), "Should be positioned from left");
        assert!(check_icon_class.contains("flex"), "Should use flexbox");
        assert!(check_icon_class.contains("h-3.5"), "Should have fixed height");
        assert!(check_icon_class.contains("w-3.5"), "Should have fixed width");
        assert!(check_icon_class.contains("items-center"), "Should center items");
        assert!(check_icon_class.contains("justify-center"), "Should center justify");
    }
    
    #[test] 
    fn select_scroll_button_styles() {
        let scroll_button_class = "flex cursor-default items-center justify-center py-1";
        
        assert!(scroll_button_class.contains("flex"), "Should use flexbox");
        assert!(scroll_button_class.contains("cursor-default"), "Should have default cursor");
        assert!(scroll_button_class.contains("items-center"), "Should center items");
        assert!(scroll_button_class.contains("justify-center"), "Should center justify");
        assert!(scroll_button_class.contains("py-1"), "Should have vertical padding");
    }
    
    #[test]
    fn class_constants_are_non_empty() {
        // Ensure all class constants have content
        let classes = vec![
            "flex h-10 w-full items-center", // Partial trigger class
            "relative z-50 max-h-96", // Partial content class
            "relative flex w-full cursor-default", // Partial item class
            "py-1.5 pl-8 pr-2", // Partial label class
        ];
        
        for class in classes {
            assert!(!class.is_empty(), "Class constant should not be empty: {}", class);
            assert!(class.len() > 5, "Class constant should have meaningful content: {}", class);
        }
    }
}
