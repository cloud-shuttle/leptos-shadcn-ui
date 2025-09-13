//! Signal-managed version of the collapsible component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

/// Signal-managed collapsible state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedCollapsibleState {
    pub is_active: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub click_count: u32,
}

impl Default for SignalManagedCollapsibleState {
    fn default() -> Self {
        Self {
            is_active: false,
            is_hovered: false,
            is_focused: false,
            click_count: 0,
        }
    }
}

/// Signal-managed collapsible component
#[component]
pub fn SignalManagedCollapsible(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let collapsible_state = ArcRwSignal::new(SignalManagedCollapsibleState::default());

    // Create computed class using ArcMemo
    let collapsible_state_for_class = collapsible_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = collapsible_state_for_class.get();
        let base_class = "component-base-class"; // TODO: Replace with actual base class
        let active_class = if state.is_active { "active" } else { "" };
        let hover_class = if state.is_hovered { "hover" } else { "" };
        let focus_class = if state.is_focused { "focus" } else { "" };
        
        format!("{} {} {} {} {}", 
            base_class, 
            active_class, 
            hover_class, 
            focus_class,
            class.get().unwrap_or_default()
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(collapsible_state.clone());
    theme_manager.track_memo(computed_class.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers
    let handle_click = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.click_count += 1;
                state.is_active = !state.is_active;
            });
        }
    };

    let handle_mouse_enter = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let collapsible_state_for_disabled = collapsible_state.clone();
    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:click=handle_click
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Enhanced collapsible component with advanced signal management
#[component]
pub fn EnhancedCollapsible(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let collapsible_state = ArcRwSignal::new(SignalManagedCollapsibleState::default());

    // Create computed class using ArcMemo
    let collapsible_state_for_class = collapsible_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = collapsible_state_for_class.get();
        let base_class = "component-base-class"; // TODO: Replace with actual base class
        let active_class = if state.is_active { "active transition-all" } else { "" };
        let hover_class = if state.is_hovered { "hover:shadow-md" } else { "" };
        let focus_class = if state.is_focused { "focus:ring-2 focus:ring-ring" } else { "" };
        
        format!("{} {} {} {} {}", 
            base_class, 
            active_class, 
            hover_class, 
            focus_class,
            class.get().unwrap_or_default()
        )
    });

    // Create performance metrics
    let collapsible_state_for_metrics = collapsible_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {
        let state = collapsible_state_for_metrics.get();
        format!("Clicks: {}, Active: {}, Hovered: {}", 
            state.click_count, 
            state.is_active, 
            state.is_hovered
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(collapsible_state.clone());
    theme_manager.track_memo(computed_class.clone());
    theme_manager.track_memo(performance_metrics.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers with performance monitoring
    let handle_click = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.click_count += 1;
                state.is_active = !state.is_active;
            });
        }
    };

    let handle_mouse_enter = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let collapsible_state = collapsible_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            collapsible_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    view! {
        <div class="enhanced-collapsible-container">
            <div
                class=move || computed_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                on:click=handle_click
                on:mouseenter=handle_mouse_enter
                on:mouseleave=handle_mouse_leave
            >
                {children.map(|c| c())}
            </div>
            
            // Performance monitoring (only in development)
            #[cfg(debug_assertions)]
            <div class="performance-monitor text-xs text-muted-foreground mt-1">
                {move || performance_metrics.get()}
            </div>
        </div>
    }
}
