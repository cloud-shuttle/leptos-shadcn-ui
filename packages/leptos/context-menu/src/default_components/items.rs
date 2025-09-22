//! ContextMenu item components
//! 
//! This module contains the ContextMenuItem component for individual
//! context menu items.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn ContextMenuItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |e: MouseEvent| {
        if !disabled.get().unwrap_or(false) {
            e.stop_propagation();
            open.set(false);
        }
    };

    let item_class = move || {
        let base_class = "context-menu-item";
        let disabled_class = if disabled.get().unwrap_or(false) { " disabled" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, disabled_class, custom_class)
    };

    view! {
        <div
            class=item_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            role="menuitem"
            aria-disabled=disabled.get().unwrap_or(false)
        >
            {children.map(|c| c())}
        </div>
    }
}
