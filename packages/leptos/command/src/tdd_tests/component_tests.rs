#[cfg(test)]
mod component_tests {
    use leptos::prelude::*;
    use crate::default::*;

    #[test]
    fn test_command_input_basic() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_input_with_placeholder() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Enter search term...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_list_basic() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_list_with_items() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_empty() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_empty_custom_message() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"Custom empty message"</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_group_basic() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_group_with_heading() {
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
    fn test_command_group_multiple() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                    <CommandGroup heading=MaybeProp::from("Recent")>
                        <CommandItem>"Recent Item 1"</CommandItem>
                        <CommandItem>"Recent Item 2"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_item_basic() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_item_with_shortcut() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>
                            "Calendar"
                            <CommandShortcut>"⌘K"</CommandShortcut>
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_item_disabled() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem >"Disabled Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_shortcut() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>
                            "Calendar"
                            <CommandShortcut>"⌘K"</CommandShortcut>
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_separator() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                    <CommandSeparator />
                    <CommandGroup heading=MaybeProp::from("Recent")>
                        <CommandItem>"Recent Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_complex_structure() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup heading=MaybeProp::from("Suggestions")>
                        <CommandItem>
                            "Calendar"
                            <CommandShortcut>"⌘K"</CommandShortcut>
                        </CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem >"Disabled Item"</CommandItem>
                    </CommandGroup>
                    <CommandSeparator />
                    <CommandGroup heading=MaybeProp::from("Recent")>
                        <CommandItem>"Recent Item 1"</CommandItem>
                        <CommandItem>"Recent Item 2"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_empty_list() {
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }
}
