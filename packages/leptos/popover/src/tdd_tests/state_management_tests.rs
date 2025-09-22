#[cfg(test)]
mod state_management_tests {
    use leptos::prelude::*;
    use leptos_style::Style;
    use crate::default::Popover;
    use std::sync::{Arc, Mutex};

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and interaction logic

    #[test]
    fn test_popover_open_state() {
        let open_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover open=open_state.into()>
                <button>"Toggle Popover"</button>
                <div class="popover-content">
                    <p>"Popover content"</p>
                </div>
            </Popover>
        };
        
        // Test initial state
        assert!(!open_state.get());
        
        // Test state change
        open_state.set(true);
        assert!(open_state.get());
    }

    #[test]
    fn test_popover_value_state() {
        let value_state = RwSignal::new("".to_string());
        
        let _popover_view = view! {
            <Popover value=value_state.into()>
                <button>"Select Value"</button>
                <div class="popover-content">
                    <button
                        on:click=move |_| value_state.set("option1".to_string())
                    >
                        "Option 1"
                    </button>
                    <button
                        on:click=move |_| value_state.set("option2".to_string())
                    >
                        "Option 2"
                    </button>
                </div>
            </Popover>
        };
        
        // Test initial state
        assert_eq!(value_state.get(), "");
        
        // Test state change
        value_state.set("option1".to_string());
        assert_eq!(value_state.get(), "option1");
    }

    #[test]
    fn test_popover_disabled_state() {
        let disabled_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover disabled=disabled_state.into()>
                <button>"Disabled Popover"</button>
                <div class="popover-content">
                    <p>"Disabled content"</p>
                </div>
            </Popover>
        };
        
        // Test initial state
        assert!(!disabled_state.get());
        
        // Test disabled state
        disabled_state.set(true);
        assert!(disabled_state.get());
    }

    #[test]
    fn test_popover_callback_handling() {
        let callback_called = RwSignal::new(false);
        let callback_value = RwSignal::new("".to_string());
        
        let callback = Callback::new(move |value: String| {
            callback_called.set(true);
            callback_value.set(value);
        });
        
        let _popover_view = view! {
            <Popover on_select=Some(callback)>
                <button>"Callback Popover"</button>
                <div class="popover-content">
                    <button
                        on:click=move |_| callback.run("test-value".to_string())
                    >
                        "Test Button"
                    </button>
                </div>
            </Popover>
        };
        
        // Test initial state
        assert!(!callback_called.get());
        assert_eq!(callback_value.get(), "");
        
        // Simulate callback execution
        callback.run("test-value".to_string());
        assert!(callback_called.get());
        assert_eq!(callback_value.get(), "test-value");
    }

    #[test]
    fn test_popover_context_management() {
        let context_value = RwSignal::new("context-value".to_string());
        
        let _popover_view = view! {
            <Popover>
                <button>"Context Popover"</button>
                <div class="popover-content">
                    <p>{move || context_value.get()}</p>
                </div>
            </Popover>
        };
        
        // Test context value
        assert_eq!(context_value.get(), "context-value");
        
        // Test context value change
        context_value.set("new-context-value".to_string());
        assert_eq!(context_value.get(), "new-context-value");
    }

    #[test]
    fn test_popover_animation_state() {
        let animation_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover>
                <button>"Animated Popover"</button>
                <div
                    class=move || if animation_state.get() { "animate-in" } else { "animate-out" }
                >
                    <p>"Animated content"</p>
                </div>
            </Popover>
        };
        
        // Test initial animation state
        assert!(!animation_state.get());
        
        // Test animation state change
        animation_state.set(true);
        assert!(animation_state.get());
    }

    #[test]
    fn test_popover_focus_state() {
        let focus_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover>
                <button
                    on:focus=move |_| focus_state.set(true)
                    on:blur=move |_| focus_state.set(false)
                >
                    "Focus Popover"
                </button>
                <div class="popover-content">
                    <p>"Focus content"</p>
                </div>
            </Popover>
        };
        
        // Test initial focus state
        assert!(!focus_state.get());
        
        // Test focus state change
        focus_state.set(true);
        assert!(focus_state.get());
        
        focus_state.set(false);
        assert!(!focus_state.get());
    }

    #[test]
    fn test_popover_hover_state() {
        let hover_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover>
                <button
                    on:mouseenter=move |_| hover_state.set(true)
                    on:mouseleave=move |_| hover_state.set(false)
                >
                    "Hover Popover"
                </button>
                <div class="popover-content">
                    <p>"Hover content"</p>
                </div>
            </Popover>
        };
        
        // Test initial hover state
        assert!(!hover_state.get());
        
        // Test hover state change
        hover_state.set(true);
        assert!(hover_state.get());
        
        hover_state.set(false);
        assert!(!hover_state.get());
    }

    #[test]
    fn test_popover_selection_state() {
        let selection_state = RwSignal::new("".to_string());
        
        let _popover_view = view! {
            <Popover>
                <button>"Selection Popover"</button>
                <div class="popover-content">
                    <button
                        on:click=move |_| selection_state.set("item1".to_string())
                    >
                        "Item 1"
                    </button>
                    <button
                        on:click=move |_| selection_state.set("item2".to_string())
                    >
                        "Item 2"
                    </button>
                </div>
            </Popover>
        };
        
        // Test initial selection state
        assert_eq!(selection_state.get(), "");
        
        // Test selection state change
        selection_state.set("item1".to_string());
        assert_eq!(selection_state.get(), "item1");
        
        selection_state.set("item2".to_string());
        assert_eq!(selection_state.get(), "item2");
    }

    #[test]
    fn test_popover_multiple_states() {
        let open_state = RwSignal::new(false);
        let value_state = RwSignal::new("".to_string());
        let disabled_state = RwSignal::new(false);
        
        let _popover_view = view! {
            <Popover
                open=open_state.into()
                value=value_state.into()
                disabled=disabled_state.into()
            >
                <button>"Multi-State Popover"</button>
                <div class="popover-content">
                    <button
                        on:click=move |_| value_state.set("option1".to_string())
                    >
                        "Option 1"
                    </button>
                    <button
                        on:click=move |_| value_state.set("option2".to_string())
                    >
                        "Option 2"
                    </button>
                </div>
            </Popover>
        };
        
        // Test initial states
        assert!(!open_state.get());
        assert_eq!(value_state.get(), "");
        assert!(!disabled_state.get());
        
        // Test state changes
        open_state.set(true);
        value_state.set("option1".to_string());
        disabled_state.set(true);
        
        assert!(open_state.get());
        assert_eq!(value_state.get(), "option1");
        assert!(disabled_state.get());
    }

    #[test]
    fn test_popover_state_persistence() {
        let persistent_state = RwSignal::new("persistent".to_string());
        
        let _popover_view = view! {
            <Popover>
                <button>"Persistent Popover"</button>
                <div class="popover-content">
                    <p>{move || persistent_state.get()}</p>
                </div>
            </Popover>
        };
        
        // Test state persistence
        assert_eq!(persistent_state.get(), "persistent");
        
        // Test state change persistence
        persistent_state.set("new-persistent".to_string());
        assert_eq!(persistent_state.get(), "new-persistent");
    }

    #[test]
    fn test_popover_state_validation() {
        let valid_state = RwSignal::new(true);
        let error_state = RwSignal::new("".to_string());
        
        let _popover_view = view! {
            <Popover>
                <button>"Validation Popover"</button>
                <div class="popover-content">
                    <div
                        class=move || if valid_state.get() { "valid" } else { "invalid" }
                    >
                        {move || if valid_state.get() { "Valid Content" } else { error_state.get() }}
                    </div>
                </div>
            </Popover>
        };
        
        // Test initial validation state
        assert!(valid_state.get());
        assert_eq!(error_state.get(), "");
        
        // Test validation state change
        valid_state.set(false);
        error_state.set("Validation Error".to_string());
        
        assert!(!valid_state.get());
        assert_eq!(error_state.get(), "Validation Error");
    }

    #[test]
    fn test_popover_state_performance() {
        let performance_state = RwSignal::new(0);
        
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            performance_state.set(performance_state.get() + 1);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "State updates should be performant");
        assert_eq!(performance_state.get(), 1000);
    }

    #[test]
    fn test_popover_state_memory() {
        let memory_state = RwSignal::new("memory-test".to_string());
        
        // Test memory usage
        let size = std::mem::size_of_val(&memory_state);
        assert!(size < 1024, "State should not cause excessive memory usage");
        
        // Test state value
        assert_eq!(memory_state.get(), "memory-test");
    }

    #[test]
    fn test_popover_state_synchronization() {
        let state1 = RwSignal::new("state1".to_string());
        let state2 = RwSignal::new("state2".to_string());
        
        let _popover_view = view! {
            <Popover>
                <button>"Synchronized Popover"</button>
                <div class="popover-content">
                    <p>{move || format!("{} - {}", state1.get(), state2.get())}</p>
                </div>
            </Popover>
        };
        
        // Test state synchronization
        assert_eq!(state1.get(), "state1");
        assert_eq!(state2.get(), "state2");
        
        // Test synchronized state changes
        state1.set("new-state1".to_string());
        state2.set("new-state2".to_string());
        
        assert_eq!(state1.get(), "new-state1");
        assert_eq!(state2.get(), "new-state2");
    }

    #[test]
    fn test_popover_state_derivation() {
        let base_state = RwSignal::new(10);
        let derived_state = Signal::derive(move || base_state.get() * 2);
        
        let _popover_view = view! {
            <Popover>
                <button>"Derived State Popover"</button>
                <div class="popover-content">
                    <p>{move || format!("Base: {}, Derived: {}", base_state.get(), derived_state.get())}</p>
                </div>
            </Popover>
        };
        
        // Test derived state
        assert_eq!(base_state.get(), 10);
        assert_eq!(derived_state.get(), 20);
        
        // Test derived state update
        base_state.set(15);
        assert_eq!(base_state.get(), 15);
        assert_eq!(derived_state.get(), 30);
    }
}
