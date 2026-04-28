//! Main Drawer component

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
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);
    provide_context(direction);
    provide_context(should_scale_background);

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div class="drawer-root">
                {children()}
            </div>
        </Show>
    }
}
