//! Signal-managed version of the select component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

/// Signal-managed select state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedSelectState {
    pub is_active: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub click_count: u32,
}

impl Default for SignalManagedSelectState {
    fn default() -> Self {
        Self {
            is_active: false,
            is_hovered: false,
            is_focused: false,
            click_count: 0,
        }
    }
}

/// Signal-managed select component
#[component]
pub fn SignalManagedSelect(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let select_state = ArcRwSignal::new(SignalManagedSelectState::default());

    // Create computed class using ArcMemo
    let select_state_for_class = select_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = select_state_for_class.get();
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
    theme_manager.track_signal(select_state.clone());
    theme_manager.track_memo(computed_class.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers
    let handle_click = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.click_count += 1;
                state.is_active = !state.is_active;
            });
        }
    };

    let handle_mouse_enter = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let select_state_for_disabled = select_state.clone();
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

/// Enhanced select component with advanced signal management
#[component]
pub fn EnhancedSelect(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let select_state = ArcRwSignal::new(SignalManagedSelectState::default());

    // Create computed class using ArcMemo
    let select_state_for_class = select_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = select_state_for_class.get();
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
    let select_state_for_metrics = select_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {
        let state = select_state_for_metrics.get();
        format!("Clicks: {}, Active: {}, Hovered: {}", 
            state.click_count, 
            state.is_active, 
            state.is_hovered
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(select_state.clone());
    theme_manager.track_memo(computed_class.clone());
    theme_manager.track_memo(performance_metrics.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers with performance monitoring
    let handle_click = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.click_count += 1;
                state.is_active = !state.is_active;
            });
        }
    };

    let handle_mouse_enter = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let select_state = select_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            select_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    view! {
        <div class="enhanced-select-container">
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
