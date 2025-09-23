//! Main AlertDialog component
//! 
//! This module contains the main AlertDialog component that provides context
//! and handles keyboard events for the alert dialog system.

use leptos::prelude::*;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;

#[component]
pub fn AlertDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);

    // Handle escape key - use a simpler approach without global listeners
    // The escape key handling will be managed by the content components

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div>
                {children.map(|c| c())}
            </div>
        </Show>
    }
}
