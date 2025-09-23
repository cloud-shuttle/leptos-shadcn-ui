//! ContextMenu trigger component
//! 
//! This module contains the ContextMenuTrigger component for handling
//! right-click events and positioning.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let position = expect_context::<RwSignal<(i32, i32)>>();

    let handle_context_menu = move |e: MouseEvent| {
        e.prevent_default();
        let x = e.client_x();
        let y = e.client_y();
        position.set((x, y));
        open.set(true);
    };

    let handle_click = move |_| {
        open.set(false);
    };

    // Use a simpler approach without global event listeners
    // The context menu will close when clicking on items or outside

    view! {
        <div
            class=move || format!("cursor-pointer {}", class.get().unwrap_or_default())
            on:contextmenu=handle_context_menu
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}
