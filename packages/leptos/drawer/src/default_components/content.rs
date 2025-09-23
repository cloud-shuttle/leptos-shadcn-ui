//! Drawer content components
//! 
//! This module contains the DrawerContent component for the main drawer content area.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;
use super::types::DrawerDirection;

#[component]
pub fn DrawerContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<RwSignal<bool>>();
    let direction = expect_context::<Signal<DrawerDirection>>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    let content_class = move || {
        let base_class = "fixed z-50 bg-background";
        let direction_class = match direction.get() {
            DrawerDirection::Top => " top-0 inset-x-0 border-b",
            DrawerDirection::Bottom => " bottom-0 inset-x-0 border-t",
            DrawerDirection::Left => " left-0 top-0 h-full w-3/4 border-r sm:max-w-sm",
            DrawerDirection::Right => " right-0 top-0 h-full w-3/4 border-l sm:max-w-sm",
        };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, direction_class, custom_class)
    };

    view! {
        <Show
            when=move || open_state.get()
            fallback=|| view! { <div></div> }
        >
            <div
                class=content_class
                id=move || id.get().unwrap_or_default()
                style=move || style.get().unwrap_or_default()
                on:click=handle_click
                role="dialog"
                aria-modal="true"
            >
                {children.map(|c| c())}
            </div>
        </Show>
    }
}
