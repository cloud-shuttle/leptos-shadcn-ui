//! AlertDialog action and cancel components
//! 
//! This module contains the AlertDialogAction and AlertDialogCancel
//! components for dialog actions.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn AlertDialogAction(
    #[prop(into, optional)] class: MaybeProp<String>,
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
        <button
            class=move || format!("inline-flex h-10 items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-semibold text-primary-foreground ring-offset-background transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}", class.get().unwrap_or_default())
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] class: MaybeProp<String>,
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
        <button
            class=move || format!("inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-semibold ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}", class.get().unwrap_or_default())
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}
