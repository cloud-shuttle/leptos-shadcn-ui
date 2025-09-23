//! Main Drawer component
//! 
//! This module contains the main Drawer component that provides context and handles
//! keyboard events for the drawer system.

use leptos::prelude::*;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;
use super::types::DrawerDirection;

#[component]
pub fn Drawer(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] direction: Signal<DrawerDirection>,
    #[prop(into, optional)] should_scale_background: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);
    provide_context(direction);
    provide_context(should_scale_background);

    // Handle escape key - use a simpler approach without global listeners
    // The escape key handling will be managed by the content components

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div class="drawer-root">
                {children.map(|c| c())}
            </div>
        </Show>
    }
}
