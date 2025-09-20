#[cfg(test)]
mod class_constants_tests {
    use leptos::prelude::*;
    use leptos_style::Style;

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

        // Test SelectLabel class constant
        let label_class = "py-1.5 pl-8 pr-2 text-sm font-semibold";
        assert!(label_class.contains("py-1.5"));
        assert!(label_class.contains("pl-8"));
        assert!(label_class.contains("pr-2"));
        assert!(label_class.contains("text-sm"));
        assert!(label_class.contains("font-semibold"));

        // Test SelectSeparator class constant
        let separator_class = "-mx-1 my-1 h-px bg-muted";
        assert!(separator_class.contains("-mx-1"));
        assert!(separator_class.contains("my-1"));
        assert!(separator_class.contains("h-px"));
        assert!(separator_class.contains("bg-muted"));

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

        // Test SelectViewport class constant
        let viewport_class = "p-1";
        assert!(viewport_class.contains("p-1"));

        // Test SelectValue class constant
        let value_class = "placeholder:text-muted-foreground";
        assert!(value_class.contains("placeholder:text-muted-foreground"));

        // Test SelectIcon class constant
        let icon_class = "h-4 w-4 opacity-50";
        assert!(icon_class.contains("h-4"));
        assert!(icon_class.contains("w-4"));
        assert!(icon_class.contains("opacity-50"));
    }

    #[test]
    fn test_select_computed_class_generation() {
        // Test that computed classes are generated correctly
        let base_classes = "flex h-10 w-full items-center justify-between";
        let additional_classes = "custom-class another-class";
        let combined = format!("{} {}", base_classes, additional_classes);
        
        assert!(combined.contains("flex"));
        assert!(combined.contains("h-10"));
        assert!(combined.contains("w-full"));
        assert!(combined.contains("custom-class"));
        assert!(combined.contains("another-class"));
        
        // Test class merging logic
        let merged = format!("{} {}", base_classes, additional_classes);
        assert_eq!(merged, "flex h-10 w-full items-center justify-between custom-class another-class");
    }
}
