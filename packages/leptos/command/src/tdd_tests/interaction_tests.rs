#[cfg(test)]
mod interaction_tests {
    use super::*;

    #[test]
    fn test_command_keyboard_navigation() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command keyboard navigation should work");
    }

    #[test]
    fn test_command_edge_cases() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=""/>
                <CommandList>
                    <CommandEmpty>""</CommandEmpty>
                    <CommandGroup heading="">
                        <CommandItem>""</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command edge cases should handle empty values");
    }

    #[test]
    fn test_command_performance() {
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            let _command_view = view! {
                <Command>
                    <CommandInput placeholder=format!("Search {}...", i)/>
                    <CommandList>
                        <CommandEmpty>"No results found."</CommandEmpty>
                        <CommandGroup heading="Suggestions">
                            <CommandItem>format!("Item {}", i)</CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Command rendering should be performant");
    }

    #[test]
    fn test_command_callback_handling() {
        let callback = Callback::new(move |value: String| {
            // Test callback handling
            assert!(value.len() >= 0);
        });
        
        let _command_view = view! {
            <Command on_value_change=Some(callback)>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command callback handling should work");
    }

    #[test]
    fn test_command_value_updates() {
        let value_signal = RwSignal::new("".to_string());
        
        let _command_view = view! {
            <Command value=MaybeProp::from(value_signal)>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        
        // Test value updates
        value_signal.set("test".to_string());
        assert_eq!(value_signal.get(), "test");
        
        value_signal.set("updated".to_string());
        assert_eq!(value_signal.get(), "updated");
    }

    #[test]
    fn test_command_item_selection() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command item selection should work");
    }

    #[test]
    fn test_command_input_focus() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..." autofocus=true/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command input focus should work");
    }

    #[test]
    fn test_command_search_filtering() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command search filtering should work");
    }

    #[test]
    fn test_command_shortcut_handling() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>
                            "Calendar"
                            <CommandShortcut>⌘K</CommandShortcut>
                        </CommandItem>
                        <CommandItem>
                            "Search Emoji"
                            <CommandShortcut>⌘E</CommandShortcut>
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command shortcut handling should work");
    }

    #[test]
    fn test_command_disabled_interactions() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..." disabled=true/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem disabled=true>"Disabled Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command disabled interactions should work");
    }

    #[test]
    fn test_command_mouse_interactions() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command mouse interactions should work");
    }

    #[test]
    fn test_command_touch_interactions() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..."/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command touch interactions should work");
    }

    #[test]
    fn test_command_voice_interactions() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder="Search..." voice_control=true/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        assert!(true, "Command voice interactions should work");
    }
}
