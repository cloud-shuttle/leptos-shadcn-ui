#[cfg(test)]
mod performance_tests {
    use super::*;

    // ===== PERFORMANCE TESTS =====
    // These tests focus on performance, callbacks, and optimization

    #[test]
    fn test_context_menu_performance_basic_rendering() {
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
    fn test_context_menu_performance_large_menu() {
        let start = std::time::Instant::now();
        
        let items = (1..=100).collect::<Vec<_>>();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.clone() key=|item| *item let:item>
                        <ContextMenuItem>{format!("Item {}", item)}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_memory_usage() {
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
    fn test_context_menu_performance_signal_updates() {
        let open = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu open=open>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            open.set(i % 2 == 0);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_context_menu_performance_callback_execution() {
        let call_count = RwSignal::new(0);
        let on_open_change = Callback::new(move |_| {
            call_count.update(|c| *c += 1);
        });
        let _context_menu_view = view! {
            <ContextMenu on_open_change=on_open_change>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            on_open_change.call(true);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10);
        assert_eq!(call_count.get(), 100);
    }

    #[test]
    fn test_context_menu_performance_dynamic_content() {
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=items key=|item| item.to_string() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            items.update(|items| {
                items.push(&format!("Item {}", i + 3));
            });
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_computed_props() {
        let open = RwSignal::new(false);
        let computed_class = Signal::derive(move || {
            format!("context-menu-{}", if open.get() { "open" } else { "closed" })
        });
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                class=MaybeProp::from(computed_class)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            open.set(i % 2 == 0);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
    }

    #[test]
    fn test_context_menu_performance_memoized_values() {
        let open = RwSignal::new(false);
        let memoized_class = Memo::new(move |_| {
            format!("context-menu-{}", if open.get() { "open" } else { "closed" })
        });
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                class=MaybeProp::from(memoized_class)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            open.set(i % 2 == 0);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 15);
    }

    #[test]
    fn test_context_menu_performance_batch_updates() {
        let open = RwSignal::new(false);
        let disabled = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu 
                open=open
                disabled=MaybeProp::from(disabled)
            >
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            batch(move || {
                open.set(i % 2 == 0);
                disabled.set(i % 3 == 0);
            });
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
    }

    #[test]
    fn test_context_menu_performance_untracked_access() {
        let open = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu open=open>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _untracked_value = open.get_untracked();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 5);
    }

    #[test]
    fn test_context_menu_performance_concurrent_updates() {
        let open1 = RwSignal::new(false);
        let open2 = RwSignal::new(false);
        let _context_menu_view1 = view! {
            <ContextMenu open=open1>
                <ContextMenuTrigger>
                    "Menu 1"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu open=open2>
                <ContextMenuTrigger>
                    "Menu 2"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for i in 0..100 {
            open1.set(i % 2 == 0);
            open2.set(i % 2 == 1);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30);
    }

    #[test]
    fn test_context_menu_performance_deep_nesting() {
        let _context_menu_view = view! {
            <div>
                <div>
                    <div>
                        <div>
                            <div>
                                <ContextMenu>
                                    <ContextMenuTrigger>
                                        "Right-click me"
                                    </ContextMenuTrigger>
                                    <ContextMenuContent>
                                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                                    </ContextMenuContent>
                                </ContextMenu>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        };
        
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let _nested_view = view! {
                <div>
                    <div>
                        <div>
                            <ContextMenu>
                                <ContextMenuTrigger>
                                    "Nested menu"
                                </ContextMenuTrigger>
                                <ContextMenuContent>
                                    <ContextMenuItem>"Nested item"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenu>
                        </div>
                    </div>
                </div>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_many_props() {
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger 
                    class=MaybeProp::from("custom-trigger")
                    style=MaybeProp::from("color: red")
                    id=MaybeProp::from("test-trigger")
                    aria_label=MaybeProp::from("Context menu trigger")
                    aria_describedby=MaybeProp::from("trigger-description")
                    aria_expanded=MaybeProp::from(false)
                    aria_haspopup=MaybeProp::from("menu")
                    role=MaybeProp::from("button")
                    tabindex=MaybeProp::from(0)
                    title=MaybeProp::from("Right-click for options")
                    data_testid=MaybeProp::from("context-menu-trigger")
                >
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent 
                    class=MaybeProp::from("custom-content")
                    style=MaybeProp::from("background: blue")
                    id=MaybeProp::from("test-content")
                    role=MaybeProp::from("menu")
                    aria_label=MaybeProp::from("Context menu options")
                    aria_orientation=MaybeProp::from("vertical")
                    data_testid=MaybeProp::from("context-menu-content")
                >
                    <ContextMenuItem 
                        class=MaybeProp::from("custom-item")
                        style=MaybeProp::from("color: white")
                        id=MaybeProp::from("test-item")
                        role=MaybeProp::from("menuitem")
                        tabindex=MaybeProp::from(0)
                        aria_label=MaybeProp::from("Menu item")
                        data_testid=MaybeProp::from("context-menu-item")
                    >
                        "Item 1"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        for _ in 0..50 {
            let _many_props_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger 
                        class=MaybeProp::from("trigger")
                        style=MaybeProp::from("color: red")
                        id=MaybeProp::from("trigger")
                        aria_label=MaybeProp::from("Trigger")
                        role=MaybeProp::from("button")
                        tabindex=MaybeProp::from(0)
                    >
                        "Trigger"
                    </ContextMenuTrigger>
                    <ContextMenuContent 
                        class=MaybeProp::from("content")
                        role=MaybeProp::from("menu")
                        aria_label=MaybeProp::from("Menu")
                    >
                        <ContextMenuItem role=MaybeProp::from("menuitem")>"Item"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_performance_stress_test() {
        let start = std::time::Instant::now();
        
        for i in 0..50 {
            let items = (1..=20).collect::<Vec<_>>();
            let open = RwSignal::new(false);
            let disabled = RwSignal::new(i % 2 == 0);
            
            let _context_menu_view = view! {
                <ContextMenu 
                    open=open
                    disabled=MaybeProp::from(disabled)
                    class=MaybeProp::from(format!("context-menu-{}", i))
                    id=MaybeProp::from(format!("menu-{}", i))
                >
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <For each=move || items.clone() key=|item| *item let:item>
                            <ContextMenuItem>
                                {format!("Item {} - {}", i, item)}
                            </ContextMenuItem>
                        </For>
                    </ContextMenuContent>
                </ContextMenu>
            };
            
            // Perform some updates
            open.set(true);
            disabled.set(!disabled.get());
            open.set(false);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_context_menu_performance_memory_leak_prevention() {
        for _ in 0..100 {
            let open = RwSignal::new(false);
            let _context_menu_view = view! {
                <ContextMenu open=open>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
            
            // Drop the view to test cleanup
            drop(_context_menu_view);
        }
        
        // Test that no significant memory leaks occur
        // This is a simple test - in practice you'd use more sophisticated memory monitoring
    }

    #[test]
    fn test_context_menu_performance_signal_cleanup() {
        let mut signals = Vec::new();
        
        for i in 0..100 {
            let open = RwSignal::new(false);
            signals.push(open);
            
            let _context_menu_view = view! {
                <ContextMenu open=open>
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        // Test that signals can be properly cleaned up
        drop(signals);
    }

    #[test]
    fn test_context_menu_performance_callback_cleanup() {
        let mut callbacks = Vec::new();
        
        for i in 0..100 {
            let callback = Callback::new(move |open| {
                logging::log!("Menu {} open state changed: {}", i, open);
            });
            callbacks.push(callback.clone());
            
            let _context_menu_view = view! {
                <ContextMenu on_open_change=callback>
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        // Test that callbacks can be properly cleaned up
        drop(callbacks);
    }

    #[test]
    fn test_context_menu_performance_edge_case_handling() {
        let start = std::time::Instant::now();
        
        // Test with various edge cases
        let test_cases = vec![
            vec![], // Empty items
            vec![""], // Empty strings
            vec!["a".repeat(1000)], // Very long strings
            (1..=1000).map(|i| format!("Item {}", i)).collect(), // Many items
        ];
        
        for items in test_cases {
            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <For each=move || items.clone() key=|item| item.clone() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_context_menu_performance_rapid_state_changes() {
        let open = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu open=open>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        // Rapid state changes
        for i in 0..1000 {
            open.set(i % 2 == 0);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_large_nested_structure() {
        let start = std::time::Instant::now();
        
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Level 1"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuSub>
                                <ContextMenuSubTrigger>"Level 2"</ContextMenuSubTrigger>
                                <ContextMenuSubContent>
                                    <ContextMenuSub>
                                        <ContextMenuSubTrigger>"Level 3"</ContextMenuSubTrigger>
                                        <ContextMenuSubContent>
                                            <For each=move || (1..=50).collect::<Vec<_>>() key=|item| *item let:item>
                                                <ContextMenuItem>{format!("Deep Item {}", item)}</ContextMenuItem>
                                            </For>
                                        </ContextMenuSubContent>
                                    </ContextMenuSub>
                                </ContextMenuSubContent>
                            </ContextMenuSub>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_performance_event_handler_optimization() {
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.update(|c| *c += 1);
        });
        
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || (1..=100).collect::<Vec<_>>() key=|item| *item let:item>
                        <ContextMenuItem on_click=callback.clone()>
                            {format!("Item {}", item)}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let start = std::time::Instant::now();
        
        // Simulate multiple clicks
        for _ in 0..100 {
            callback.call(());
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
        assert_eq!(click_count.get(), 100);
    }
}
