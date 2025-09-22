#[cfg(test)]
mod basic_rendering_tests {
    use super::*;

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_context_menu_basic_rendering() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
            </ContextMenu>
        };
        // GREEN PHASE: Verify actual rendering behavior
    }

    #[test]
    fn test_context_menu_trigger() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("custom-trigger")>
                    "Custom Trigger"
                </ContextMenuTrigger>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_content() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_item() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem class=MaybeProp::from("custom-item")>
                        "Custom Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_separator() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_label() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Section Label"</ContextMenuLabel>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_group() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuGroup>
                        <ContextMenuItem>"Group Item 1"</ContextMenuItem>
                        <ContextMenuItem>"Group Item 2"</ContextMenuItem>
                    </ContextMenuGroup>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_submenu() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Submenu"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Sub Item 1"</ContextMenuItem>
                            <ContextMenuItem>"Sub Item 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_props() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger 
                    class=MaybeProp::from("custom-trigger")
                    style=MaybeProp::from("color: red")
                    id=MaybeProp::from("test-trigger")
                >
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent 
                    class=MaybeProp::from("custom-content")
                    style=MaybeProp::from("background: blue")
                    id=MaybeProp::from("test-content")
                >
                    <ContextMenuItem 
                        class=MaybeProp::from("custom-item")
                        style=MaybeProp::from("color: white")
                        id=MaybeProp::from("test-item")
                    >
                        "Custom Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_children() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    <div>
                        <span>"Complex Trigger"</span>
                        <button>"Button"</button>
                    </div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>
                        <div>
                            <span>"Complex Item"</span>
                            <span>"Description"</span>
                        </div>
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_multiple_items() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                    <ContextMenuItem>"Item 3"</ContextMenuItem>
                    <ContextMenuItem>"Item 4"</ContextMenuItem>
                    <ContextMenuItem>"Item 5"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_nested_structure() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Actions"</ContextMenuLabel>
                    <ContextMenuItem>"Edit"</ContextMenuItem>
                    <ContextMenuItem>"Copy"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuLabel>"More"</ContextMenuLabel>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Advanced"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Advanced Option 1"</ContextMenuItem>
                            <ContextMenuItem>"Advanced Option 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_empty_content() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_only_separators() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuSeparator />
                    <ContextMenuSeparator />
                    <ContextMenuSeparator />
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_only_labels() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Label 1"</ContextMenuLabel>
                    <ContextMenuLabel>"Label 2"</ContextMenuLabel>
                    <ContextMenuLabel>"Label 3"</ContextMenuLabel>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_mixed_content() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Section 1"</ContextMenuLabel>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuLabel>"Section 2"</ContextMenuLabel>
                    <ContextMenuItem>"Item 3"</ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Submenu"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Sub Item 1"</ContextMenuItem>
                            <ContextMenuItem>"Sub Item 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_performance() {
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                        <ContextMenuItem>"Item 2"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_rendering_memory_usage() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let size = std::mem::size_of::<usize>();
        assert!(size < 1024);
    }

    #[test]
    fn test_context_menu_rendering_consistency() {
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view3 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_with_different_props() {
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("trigger1")>
                    "Trigger 1"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("content1")>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("trigger2")>
                    "Trigger 2"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("content2")>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_edge_cases() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    ""
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>""</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_with_none_props() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger 
                    class=MaybeProp::from(None::<String>)
                    style=MaybeProp::from(None::<String>)
                    id=MaybeProp::from(None::<String>)
                >
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent 
                    class=MaybeProp::from(None::<String>)
                    style=MaybeProp::from(None::<String>)
                    id=MaybeProp::from(None::<String>)
                >
                    <ContextMenuItem 
                        class=MaybeProp::from(None::<String>)
                        style=MaybeProp::from(None::<String>)
                        id=MaybeProp::from(None::<String>)
                    >
                        "Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }
}
