//! Main AlertDialog component

use leptos::prelude::*;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;

#[component]
pub fn AlertDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div>
                {children()}
            </div>
        </Show>
    }
}
