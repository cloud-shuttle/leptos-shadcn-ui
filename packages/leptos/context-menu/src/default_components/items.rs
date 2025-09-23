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
        let base_class = "relative flex cursor-pointer select-none items-center gap-2 rounded-md px-3 py-2 text-sm font-medium outline-none transition-colors duration-150 hover:bg-gray-100 dark:hover:bg-gray-700 focus:bg-gray-100 dark:focus:bg-gray-700 active:bg-gray-200 dark:active:bg-gray-600 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
        let disabled_class = if disabled.get().unwrap_or(false) { " data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50" } else { "" };
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
