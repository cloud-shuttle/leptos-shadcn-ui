//! AlertDialog overlay component
//! 
//! This module contains the AlertDialogOverlay component for the
//! background overlay of the alert dialog.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn AlertDialogOverlay(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = move |_e: MouseEvent| {
        open.set(false);
        if let Some(callback) = &on_open_change {
            callback.run(false);
        }
    };

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div
                class=move || format!("fixed inset-0 z-50 bg-background/80 backdrop-blur-sm {}", class.get().unwrap_or_default())
                id=move || id.get().unwrap_or_default()
                style=move || style.get().unwrap_or_default()
                on:click=handle_click
            >
                {children.map(|c| c())}
            </div>
        </Show>
    }
}
