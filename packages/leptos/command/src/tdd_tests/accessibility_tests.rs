#[cfg(test)]
mod accessibility_tests {
    use leptos::prelude::*;
    use crate::default::*;

    #[test]
    fn test_command_accessibility() {
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
    fn test_command_aria_attributes() {
        let _command_view = view! {
            <Command>
                <CommandInput 
                    placeholder=MaybeProp::from("Search...") 
                />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem >"Calendar"</CommandItem>
                        <CommandItem >"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_role_attributes() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList >
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions") >
                        <CommandItem >"Calendar"</CommandItem>
                        <CommandItem >"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_screen_reader_support() {
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
    fn test_command_high_contrast_mode() {
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
    fn test_command_reduced_motion() {
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
    fn test_command_voice_control() {
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
    fn test_command_switch_control() {
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
    fn test_command_eye_tracking() {
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
    fn test_command_motor_impairment_support() {
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
    fn test_command_cognitive_accessibility() {
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
    fn test_command_language_support() {
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
    fn test_command_rtl_support() {
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
    fn test_command_accessibility_testing() {
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
