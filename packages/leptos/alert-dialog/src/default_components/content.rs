//! AlertDialog content component
//! 
//! This module contains the AlertDialogContent component for the main
//! alert dialog content area.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <div
            class=move || format!("alert-dialog-content {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            role="alertdialog"
            aria-modal="true"
        >
            {children.map(|c| c())}
        </div>
    }
}
