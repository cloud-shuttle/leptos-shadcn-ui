//! AlertDialog trigger component
//! 
//! This module contains the AlertDialogTrigger component for triggering
//! the alert dialog.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn AlertDialogTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = move |_e: MouseEvent| {
        open.set(true);
        if let Some(callback) = &on_open_change {
            callback.run(true);
        }
    };

    view! {
        <button
            class=move || format!("alert-dialog-trigger {}", class.get().unwrap_or_default())
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}
