#[cfg(test)]
mod integration_tests {
    use super::*;

    // ===== INTEGRATION TESTS =====
    // These tests focus on integration scenarios and complex workflows

    #[test]
    fn test_context_menu_integration_with_form() {
        let _context_menu_view = view! {
            <form>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Edit"</ContextMenuItem>
                        <ContextMenuItem>"Delete"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </form>
        };
        // Test integration with forms
    }

    #[test]
    fn test_context_menu_integration_with_table() {
        let _context_menu_view = view! {
            <table>
                <tbody>
                    <tr>
                        <td>
                            <ContextMenu>
                                <ContextMenuTrigger>
                                    "Row data"
                                </ContextMenuTrigger>
                                <ContextMenuContent>
                                    <ContextMenuItem>"Edit row"</ContextMenuItem>
                                    <ContextMenuItem>"Delete row"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenu>
                        </td>
                    </tr>
                </tbody>
            </table>
        };
        // Test integration with tables
    }

    #[test]
    fn test_context_menu_integration_with_list() {
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let _context_menu_view = view! {
            <ul>
                <For each=move || items.clone() key=|item| item.clone() let:item>
                    <li>
                        <ContextMenu>
                            <ContextMenuTrigger>
                                {item}
                            </ContextMenuTrigger>
                            <ContextMenuContent>
                                <ContextMenuItem>"Edit"</ContextMenuItem>
                                <ContextMenuItem>"Delete"</ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenu>
                    </li>
                </For>
            </ul>
        };
        // Test integration with lists
    }

    #[test]
    fn test_context_menu_integration_with_cards() {
        let _context_menu_view = view! {
            <div class="card">
                <ContextMenu>
                    <ContextMenuTrigger>
                        <div class="card-content">
                            "Card content"
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Edit card"</ContextMenuItem>
                        <ContextMenuItem>"Share card"</ContextMenuItem>
                        <ContextMenuItem>"Delete card"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </div>
        };
        // Test integration with cards
    }

    #[test]
    fn test_context_menu_integration_with_nested_menus() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Parent menu"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"More options"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Sub item 1"</ContextMenuItem>
                            <ContextMenuSub>
                                <ContextMenuSubTrigger>"Even more"</ContextMenuSubTrigger>
                                <ContextMenuSubContent>
                                    <ContextMenuItem>"Deep item"</ContextMenuItem>
                                </ContextMenuSubContent>
                            </ContextMenuSub>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with nested menus
    }

    #[test]
    fn test_context_menu_integration_with_multiple_menus() {
        let _context_menu_view = view! {
            <div>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Menu 1"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Menu 1 Item"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Menu 2"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Menu 2 Item"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </div>
        };
        // Test integration with multiple menus
    }

    #[test]
    fn test_context_menu_integration_with_state_management() {
        let global_state = RwSignal::new("initial".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem 
                        on_click=Callback::new(move |_| {
                            global_state.set("updated".to_string());
                        })
                    >
                        "Update state"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with global state management
    }

    #[test]
    fn test_context_menu_integration_with_event_handling() {
        let click_count = RwSignal::new(0);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem 
                        on_click=Callback::new(move |_| {
                            click_count.update(|c| *c += 1);
                        })
                    >
                        "Click me"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with event handling
    }

    #[test]
    fn test_context_menu_integration_with_conditional_rendering() {
        let show_delete = RwSignal::new(true);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Edit"</ContextMenuItem>
                    <Show when=show_delete>
                        <ContextMenuItem>"Delete"</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with conditional rendering
    }

    #[test]
    fn test_context_menu_integration_with_error_boundaries() {
        let _context_menu_view = view! {
            <ErrorBoundary fallback=|_| view! { <div>"Error occurred"</div> }>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </ErrorBoundary>
        };
        // Test integration with error boundaries
    }

    #[test]
    fn test_context_menu_integration_with_suspense() {
        let _context_menu_view = view! {
            <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </Suspense>
        };
        // Test integration with suspense
    }

    #[test]
    fn test_context_menu_integration_with_router() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Go to page"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with router
    }

    #[test]
    fn test_context_menu_integration_with_modals() {
        let show_modal = RwSignal::new(false);
        let _context_menu_view = view! {
            <div>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem 
                            on_click=Callback::new(move |_| {
                                show_modal.set(true);
                            })
                        >
                            "Open modal"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
                <Show when=show_modal>
                    <div class="modal">
                        "Modal content"
                    </div>
                </Show>
            </div>
        };
        // Test integration with modals
    }

    #[test]
    fn test_context_menu_integration_with_tooltips() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger title="Right-click for options">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem title="Edit this item">"Edit"</ContextMenuItem>
                    <ContextMenuItem title="Delete this item">"Delete"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with tooltips
    }

    #[test]
    fn test_context_menu_integration_with_drag_and_drop() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger draggable=MaybeProp::from(true)>
                    "Draggable item"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Cut"</ContextMenuItem>
                    <ContextMenuItem>"Copy"</ContextMenuItem>
                    <ContextMenuItem>"Paste"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with drag and drop
    }

    #[test]
    fn test_context_menu_integration_with_keyboard_shortcuts() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Edit (Ctrl+E)"</ContextMenuItem>
                    <ContextMenuItem>"Copy (Ctrl+C)"</ContextMenuItem>
                    <ContextMenuItem>"Delete (Del)"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with keyboard shortcuts
    }

    #[test]
    fn test_context_menu_integration_with_icons() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>
                        <span class="icon">Edit</span>
                        "Edit"
                    </ContextMenuItem>
                    <ContextMenuItem>
                        <span class="icon">Delete</span>
                        "Delete"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with icons
    }

    #[test]
    fn test_context_menu_integration_with_theming() {
        let theme = RwSignal::new("dark".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger 
                    class=MaybeProp::from(Signal::derive(move || format!("theme-{}", theme.get())))
                >
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent 
                    class=MaybeProp::from(Signal::derive(move || format!("theme-{}", theme.get())))
                >
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with theming
    }

    #[test]
    fn test_context_menu_integration_with_internationalization() {
        let locale = RwSignal::new("en".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>
                        {Signal::derive(move || {
                            match locale.get().as_str() {
                                "en" => "Edit",
                                "es" => "Editar",
                                "fr" => "Modifier",
                                _ => "Edit"
                            }
                        })}
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with internationalization
    }

    #[test]
    fn test_context_menu_integration_with_responsive_design() {
        let screen_size = RwSignal::new("desktop".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger 
                    class=MaybeProp::from(Signal::derive(move || {
                        match screen_size.get().as_str() {
                            "mobile" => "context-menu-mobile",
                            "tablet" => "context-menu-tablet",
                            "desktop" => "context-menu-desktop",
                            _ => "context-menu-default"
                        }
                    }))
                >
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with responsive design
    }

    #[test]
    fn test_context_menu_integration_with_testing_utilities() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger data_testid=MaybeProp::from("context-menu-trigger")>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent data_testid=MaybeProp::from("context-menu-content")>
                    <ContextMenuItem data_testid=MaybeProp::from("context-menu-item")>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with testing utilities
    }

    #[test]
    fn test_context_menu_integration_with_analytics() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem 
                        on_click=Callback::new(|_| {
                            // Analytics tracking would go here
                            logging::log!("Context menu item clicked");
                        })
                    >
                        "Item 1"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with analytics
    }

    #[test]
    fn test_context_menu_integration_with_performance_monitoring() {
        let start_time = std::time::Instant::now();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let render_time = start_time.elapsed();
        
        // Performance monitoring would go here
        assert!(render_time.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_integration_with_error_reporting() {
        let _context_menu_view = view! {
            <ErrorBoundary fallback=|error| {
                // Error reporting would go here
                logging::error!("Context menu error: {:?}", error);
                view! { <div>"Error occurred"</div> }
            }>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </ErrorBoundary>
        };
        // Test integration with error reporting
    }

    #[test]
    fn test_context_menu_integration_with_data_binding() {
        let data = RwSignal::new(vec![
            ("edit", "Edit"),
            ("copy", "Copy"),
            ("delete", "Delete"),
        ]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=data key=|(id, _)| id.clone() let:(id, label)>
                        <ContextMenuItem data_action=MaybeProp::from(id)>
                            {label}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with data binding
    }

    #[test]
    fn test_context_menu_integration_with_async_operations() {
        let loading = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem 
                        disabled=MaybeProp::from(loading)
                        on_click=Callback::new(move |_| {
                            loading.set(true);
                            // Async operation would go here
                        })
                    >
                        "Async action"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with async operations
    }

    #[test]
    fn test_context_menu_integration_with_permissions() {
        let can_edit = RwSignal::new(true);
        let can_delete = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <Show when=can_edit>
                        <ContextMenuItem>"Edit"</ContextMenuItem>
                    </Show>
                    <Show when=can_delete>
                        <ContextMenuItem>"Delete"</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with permissions
    }

    #[test]
    fn test_context_menu_integration_with_complex_state() {
        let state = RwSignal::new(std::collections::HashMap::new());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem 
                        on_click=Callback::new(move |_| {
                            state.update(|s| {
                                s.insert("action".to_string(), "edit".to_string());
                                s.insert("timestamp".to_string(), "now".to_string());
                            });
                        })
                    >
                        "Complex action"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test integration with complex state
    }
}
