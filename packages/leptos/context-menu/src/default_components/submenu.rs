//! ContextMenu submenu components
//! 
//! This module contains the ContextMenuSub, ContextMenuSubTrigger,
//! and ContextMenuSubContent components for nested context menus.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn ContextMenuSub(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(open);

    view! {
        <div class="context-menu-sub">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSubTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_mouse_enter = move |_| {
        if !disabled.get().unwrap_or(false) {
            open.set(true);
        }
    };

    let handle_mouse_leave = move |_| {
        open.set(false);
    };

    let trigger_class = move || {
        let base_class = "context-menu-sub-trigger";
        let disabled_class = if disabled.get().unwrap_or(false) { " disabled" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, disabled_class, custom_class)
    };

    view! {
        <div
            class=trigger_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=open.get()
            aria-disabled=disabled.get().unwrap_or(false)
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSubContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <div
            class=move || format!("context-menu-sub-content {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            role="menu"
            aria-orientation="vertical"
        >
            {children.map(|c| c())}
        </div>
    }
}
