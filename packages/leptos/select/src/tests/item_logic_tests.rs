#[cfg(test)]
mod item_logic_tests {
    use leptos::prelude::*;
    use leptos_style::Style;

    #[test]
    fn test_select_item_selection_logic() {
        // Test item selection state
        let selected_item = RwSignal::new(None::<String>);
        let available_items = vec!["item1", "item2", "item3"];
        
        // Test initial state
        assert!(selected_item.get().is_none(), "No item should be selected initially");
        
        // Test selecting an item
        selected_item.set(Some("item2".to_string()));
        assert_eq!(selected_item.get(), Some("item2".to_string()));
        
        // Test item availability
        assert!(available_items.contains(&"item1"));
        assert!(available_items.contains(&"item2"));
        assert!(available_items.contains(&"item3"));
        
        // Test selection validation
        let selected = selected_item.get().unwrap();
        assert!(available_items.contains(&selected.as_str()), "Selected item should be available");
    }

    #[test]
    fn test_select_item_disabled_logic() {
        // Test disabled item handling
        let disabled_items = RwSignal::new(vec!["disabled_item1".to_string(), "disabled_item2".to_string()]);
        let all_items = vec!["item1", "item2", "disabled_item1", "disabled_item2", "item3"];
        
        // Test disabled item identification
        let disabled = disabled_items.get();
        assert!(disabled.contains(&"disabled_item1".to_string()));
        assert!(disabled.contains(&"disabled_item2".to_string()));
        
        // Test enabled items
        let enabled_items: Vec<&str> = all_items.iter()
            .filter(|item| !disabled.contains(&item.to_string()))
            .copied()
            .collect();
        
        assert_eq!(enabled_items, vec!["item1", "item2", "item3"]);
        
        // Test item state checking
        assert!(disabled.contains(&"disabled_item1".to_string()), "Item should be disabled");
        assert!(!disabled.contains(&"item1".to_string()), "Item should be enabled");
    }

    #[test]
    fn test_select_event_handling_logic() {
        // Test click event handling
        let click_count = RwSignal::new(0);
        let last_clicked_item = RwSignal::new(None::<String>);
        
        let handle_click = move |item: String| {
            click_count.update(|count| *count += 1);
            last_clicked_item.set(Some(item));
        };
        
        // Simulate clicks
        handle_click("item1".to_string());
        assert_eq!(click_count.get(), 1);
        assert_eq!(last_clicked_item.get(), Some("item1".to_string()));
        
        handle_click("item2".to_string());
        assert_eq!(click_count.get(), 2);
        assert_eq!(last_clicked_item.get(), Some("item2".to_string()));
        
        // Test keyboard event handling
        let keyboard_events = RwSignal::new(Vec::<String>::new());
        
        let handle_keyboard = move |key: String| {
            keyboard_events.update(|events| events.push(key));
        };
        
        handle_keyboard("ArrowDown".to_string());
        handle_keyboard("Enter".to_string());
        
        let events = keyboard_events.get();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], "ArrowDown");
        assert_eq!(events[1], "Enter");
    }
}
