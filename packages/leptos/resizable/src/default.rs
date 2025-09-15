use leptos::prelude::*;
use leptos_style::Style;

/// Resize direction for panels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeDirection {
    Horizontal,
    Vertical,
}

impl Default for ResizeDirection {
    fn default() -> Self {
        ResizeDirection::Horizontal
    }
}

/// Resizable panel group component
#[component]
pub fn ResizablePanelGroup(
    #[prop(into, optional)] direction: MaybeProp<ResizeDirection>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] keyboard_resize: MaybeProp<bool>,
    #[prop(into, optional)] touch_support: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_resize: MaybeProp<Callback<Vec<f64>>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (panel_sizes, set_panel_sizes) = signal(Vec::<f64>::new());
    let (is_resizing, set_is_resizing) = signal(false);
    let (resize_direction, set_resize_direction) = signal(direction.get().unwrap_or_default());

    let computed_class = Signal::derive(move || {
        let mut classes = vec!["resizable-panel-group".to_string()];
        
        match resize_direction.get() {
            ResizeDirection::Horizontal => classes.push("flex-row".to_string()),
            ResizeDirection::Vertical => classes.push("flex-col".to_string()),
        }
        
        if is_resizing.get() {
            classes.push("resizing".to_string());
        }
        
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    let handle_resize = move |sizes: Vec<f64>| {
        set_panel_sizes.set(sizes.clone());
        if let Some(callback) = on_resize.get() {
            callback.run(sizes);
        }
    };

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            aria-label=aria_label.get().unwrap_or_default()
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Individual resizable panel component
#[component]
pub fn ResizablePanel(
    #[prop(into, optional)] default_size: MaybeProp<f64>,
    #[prop(into, optional)] min_size: MaybeProp<f64>,
    #[prop(into, optional)] max_size: MaybeProp<f64>,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,
    #[prop(into, optional)] collapsed_size: MaybeProp<f64>,
    #[prop(into, optional)] collapsed: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_resize: MaybeProp<Callback<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (current_size, set_current_size) = signal(default_size.get().unwrap_or(50.0));
    let (is_collapsed, set_is_collapsed) = signal(collapsed.get().unwrap_or(false));
    let (is_resizing, set_is_resizing) = signal(false);

    let min_size_val = min_size.get().unwrap_or(10.0);
    let max_size_val = max_size.get().unwrap_or(90.0);
    let collapsed_size_val = collapsed_size.get().unwrap_or(0.0);

    let computed_class = Signal::derive(move || {
        let mut classes = vec!["resizable-panel".to_string()];
        
        if is_collapsed.get() {
            classes.push("collapsed".to_string());
        }
        
        if is_resizing.get() {
            classes.push("resizing".to_string());
        }
        
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    let computed_style = Signal::derive(move || {
        let size = if is_collapsed.get() {
            collapsed_size_val
        } else {
            current_size.get().clamp(min_size_val, max_size_val)
        };
        
        let mut style_str = style.get().to_string();
        style_str.push_str(&format!("; flex: 0 0 {}%;", size));
        
        style_str
    });

    let handle_resize = move |size: f64| {
        set_current_size.set(size);
        if let Some(callback) = on_resize.get() {
            callback.run(size);
        }
    };

    let toggle_collapse = move |_| {
        if collapsible.get().unwrap_or(false) {
            set_is_collapsed.set(!is_collapsed.get());
        }
    };

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=computed_style
            aria-label=aria_label.get().unwrap_or_default()
            role="region"
        >
            {if collapsible.get().unwrap_or(false) {
                view! {
                    <button
                        class="collapse-button absolute top-2 right-2 z-10 p-1 rounded hover:bg-gray-200"
                        on:click=toggle_collapse
                        aria-label=if is_collapsed.get() { "Expand panel" } else { "Collapse panel" }
                    >
                        {if is_collapsed.get() {
                            "→"
                        } else {
                            "←"
                        }}
                    </button>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {if !is_collapsed.get() {
                view! {
                    <div class="panel-content">
                        {children.map(|c| c())}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

/// Resizable handle component
#[component]
pub fn ResizableHandle(
    #[prop(into, optional)] with_handle: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] keyboard_resize: MaybeProp<bool>,
    #[prop(into, optional)] touch_support: MaybeProp<bool>,
) -> impl IntoView {
    let (is_resizing, set_is_resizing) = signal(false);
    let (is_hovering, set_is_hovering) = signal(false);

    let computed_class = Signal::derive(move || {
        let mut classes = vec!["resizable-handle".to_string()];
        
        if with_handle.get().unwrap_or(true) {
            classes.push("with-handle".to_string());
        }
        
        if is_resizing.get() {
            classes.push("resizing".to_string());
        }
        
        if is_hovering.get() {
            classes.push("hovering".to_string());
        }
        
        if disabled.get().unwrap_or(false) {
            classes.push("disabled".to_string());
        }
        
        classes.push(class.get().unwrap_or_default());
        classes.join(" ")
    });

    let handle_mouse_down = move |_| {
        if !disabled.get().unwrap_or(false) {
            set_is_resizing.set(true);
        }
    };

    let handle_mouse_up = move |_| {
        set_is_resizing.set(false);
    };

    let handle_mouse_enter = move |_| {
        set_is_hovering.set(true);
    };

    let handle_mouse_leave = move |_| {
        set_is_hovering.set(false);
    };

    view! {
        <div
            class=computed_class
            aria-label=aria_label.get().unwrap_or_default()
            role=role.get().unwrap_or_else(|| "separator".to_string())
            aria-orientation="horizontal"
            tabindex=if keyboard_resize.get().unwrap_or(false) { Some(0) } else { None }
            on:mousedown=handle_mouse_down
            on:mouseup=handle_mouse_up
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            {if with_handle.get().unwrap_or(true) {
                view! {
                    <div class="handle-grip">
                        <div class="grip-dots"></div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
