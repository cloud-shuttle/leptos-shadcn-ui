//! Drawer portal and overlay components
//! 
//! This module contains the DrawerPortal and DrawerOverlay components for
//! rendering the drawer outside the normal DOM hierarchy.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn DrawerPortal(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-50">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerOverlay(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();
    let should_scale_background = expect_context::<Signal<bool>>();

    let handle_click = move |_e: MouseEvent| {
        open_state.set(false);
        if let Some(callback) = &on_open_change {
            callback.run(false);
        }
    };

    let overlay_class = move || {
        let base_class = "fixed inset-0 z-50 bg-background/80 backdrop-blur-sm";
        let scale_class = if should_scale_background.get() { " data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, scale_class, custom_class)
    };

    view! {
        <Show
            when=move || open_state.get()
            fallback=|| view! { <div></div> }
        >
            <div
                class=overlay_class
                id=move || id.get().unwrap_or_default()
                style=move || style.get().unwrap_or_default()
                on:click=handle_click
            >
                {children.map(|c| c())}
            </div>
        </Show>
    }
}
