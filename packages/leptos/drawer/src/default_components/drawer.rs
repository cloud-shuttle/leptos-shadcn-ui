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

    // Handle escape key
    Effect::new(move |_| {
        if open.get() {
            let handle_keydown = move |e: KeyboardEvent| {
                if e.key() == "Escape" {
                    open.set(false);
                    if let Some(callback) = &on_open_change {
                        callback.run(false);
                    }
                }
            };

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_keydown) as Box<dyn Fn(KeyboardEvent)>);
                    let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                    closure.forget();
                }
            }
        }
    });

    view! {
        <div class="drawer-root">
            {children.map(|c| c())}
        </div>
    }
}
