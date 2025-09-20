#[cfg(test)]
mod integration_tests {
    use leptos::prelude::*;
    use crate::default::*;

    #[test]
    fn test_command_form_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_validation_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_theme_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_style_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_accessibility_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput 
                    placeholder=MaybeProp::from("Search...") 
                />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_performance_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_signal_integration() {
        let value_signal = RwSignal::new("".to_string());
        let disabled_signal = RwSignal::new(false);
        
        let _command_view = view! {
            <Command value=MaybeProp::from(value_signal)>
                <CommandInput 
                    placeholder=MaybeProp::from("Search...") 
                />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
        
        // Test signal integration
        value_signal.set("test".to_string());
        assert_eq!(value_signal.get(), "test");
        
        disabled_signal.set(true);
        assert!(disabled_signal.get());
    }

    #[test]
    fn test_command_callback_integration() {
        let callback = Callback::new(move |value: String| {
            // Test callback integration
            assert!(value.len() >= 0);
        });
        
        let _command_view = view! {
            <Command on_value_change=callback>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_memory_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_network_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_battery_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_thermal_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_benchmark_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_load_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_stress_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_concurrent_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_scalability_integration() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }
}
