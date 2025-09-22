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
        let base_class = "drawer-content";
        let direction_class = match direction.get() {
            DrawerDirection::Top => " drawer-content-top",
            DrawerDirection::Bottom => " drawer-content-bottom",
            DrawerDirection::Left => " drawer-content-left",
            DrawerDirection::Right => " drawer-content-right",
        };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, direction_class, custom_class)
    };

    view! {
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
    }
}
