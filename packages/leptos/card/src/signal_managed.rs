//! Signal-managed version of the Card component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

pub const CARD_CLASS: &str = "rounded-lg border bg-card text-card-foreground shadow-sm";
pub const CARD_HEADER_CLASS: &str = "flex flex-col space-y-1.5 p-6";
pub const CARD_TITLE_CLASS: &str = "text-2xl font-semibold leading-none tracking-tight";
pub const CARD_DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";
pub const CARD_CONTENT_CLASS: &str = "p-6 pt-0";
pub const CARD_FOOTER_CLASS: &str = "flex items-center p-6 pt-0";

/// Signal-managed card state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedCardState {
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_selected: bool,
    pub click_count: u32,
    pub hover_duration: u64,
}

impl Default for SignalManagedCardState {
    fn default() -> Self {
        Self {
            is_hovered: false,
            is_focused: false,
            is_selected: false,
            click_count: 0,
            hover_duration: 0,
        }
    }
}

/// Signal-managed Card component
#[component]
pub fn SignalManagedCard(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let card_state = ArcRwSignal::new(SignalManagedCardState::default());

    // Create computed class using ArcMemo
    let card_state_for_class = card_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = card_state_for_class.get();
        let base_class = CARD_CLASS;
        let hover_class = if state.is_hovered { "hover:shadow-md" } else { "" };
        let focus_class = if state.is_focused { "focus:ring-2 focus:ring-ring" } else { "" };
        let selected_class = if state.is_selected { "ring-2 ring-primary" } else { "" };
        
        format!("{} {} {} {} {}", 
            base_class, 
            hover_class, 
            focus_class, 
            selected_class,
            class.get().unwrap_or_default()
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(card_state.clone());
    theme_manager.track_memo(computed_class.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create event handlers
    let handle_click = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.click_count += 1;
                state.is_selected = !state.is_selected;
            });
        }
    };

    let handle_mouse_enter = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let _card_state_for_disabled = card_state.clone();
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

/// Enhanced Card component with advanced signal management
#[component]
pub fn EnhancedCard(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let card_state = ArcRwSignal::new(SignalManagedCardState::default());

    // Create computed class using ArcMemo
    let card_state_for_class = card_state.clone();
    let computed_class = ArcMemo::new(move |_| {
        let state = card_state_for_class.get();
        let base_class = CARD_CLASS;
        let hover_class = if state.is_hovered { "hover:shadow-md transition-shadow" } else { "" };
        let focus_class = if state.is_focused { "focus:ring-2 focus:ring-ring" } else { "" };
        let selected_class = if state.is_selected { "ring-2 ring-primary" } else { "" };
        
        format!("{} {} {} {} {}", 
            base_class, 
            hover_class, 
            focus_class, 
            selected_class,
            class.get().unwrap_or_default()
        )
    });

    // Create performance metrics
    let card_state_for_metrics = card_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {
        let state = card_state_for_metrics.get();
        format!("Clicks: {}, Hovered: {}, Selected: {}", 
            state.click_count, 
            state.is_hovered, 
            state.is_selected
        )
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(card_state.clone());
    theme_manager.track_memo(computed_class.clone());
    theme_manager.track_memo(performance_metrics.clone());

    // Create memory manager for monitoring
    let _memory_manager = SignalMemoryManager::new();

    // Create batched updater for performance
    let _batched_updater = BatchedSignalUpdater::new();

    // Create event handlers with performance monitoring
    let handle_click = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.click_count += 1;
                state.is_selected = !state.is_selected;
            });
        }
    };

    let handle_mouse_enter = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.is_hovered = true;
            });
        }
    };

    let handle_mouse_leave = {
        let card_state = card_state.clone();
        move |_event: leptos::ev::MouseEvent| {
            card_state.update(|state| {
                state.is_hovered = false;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    view! {
        <div class="enhanced-card-container">
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

/// Signal-managed CardHeader component
#[component]
pub fn SignalManagedCardHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = ArcMemo::new(move |_| {
        format!("{} {}", CARD_HEADER_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Signal-managed CardTitle component
#[component]
pub fn SignalManagedCardTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = ArcMemo::new(move |_| {
        format!("{} {}", CARD_TITLE_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Signal-managed CardDescription component
#[component]
pub fn SignalManagedCardDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = ArcMemo::new(move |_| {
        format!("{} {}", CARD_DESCRIPTION_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Signal-managed CardContent component
#[component]
pub fn SignalManagedCardContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = ArcMemo::new(move |_| {
        format!("{} {}", CARD_CONTENT_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Signal-managed CardFooter component
#[component]
pub fn SignalManagedCardFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = ArcMemo::new(move |_| {
        format!("{} {}", CARD_FOOTER_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}
