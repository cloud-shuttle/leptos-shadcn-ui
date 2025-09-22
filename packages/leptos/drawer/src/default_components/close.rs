//! Drawer close component
//! 
//! This module contains the DrawerClose component for closing the drawer.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn DrawerClose(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = move |_e: MouseEvent| {
        open_state.set(false);
        if let Some(callback) = &on_open_change {
            callback.run(false);
        }
    };

    view! {
        <button
            class=move || format!("drawer-close {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            aria-label="Close drawer"
        >
            {children.map(|c| c())}
        </button>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawerCloseChildProps {
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}
