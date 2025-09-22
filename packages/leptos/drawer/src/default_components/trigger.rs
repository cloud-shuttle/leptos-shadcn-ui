//! Drawer trigger components
//! 
//! This module contains the DrawerTrigger component and related trigger functionality.

use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn DrawerTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = move |_| {
        open_state.set(true);
        if let Some(callback) = &on_open_change {
            callback.run(true);
        }
    };

    view! {
        <button
            class=move || format!("drawer-trigger {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawerTriggerChildProps {
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}
