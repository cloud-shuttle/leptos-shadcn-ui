//! ContextMenu content component
//! 
//! This module contains the ContextMenuContent component for the main
//! context menu content area.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn ContextMenuContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let position = expect_context::<RwSignal<(i32, i32)>>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    let content_style = move || {
        let (x, y) = position.get();
        format!("position: fixed; left: {}px; top: {}px; z-index: 50; {}", x, y, style.get().unwrap_or_default())
    };

    view! {
        <div
            class=move || format!("fixed z-50 min-w-[10rem] overflow-hidden rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 p-1 text-gray-900 dark:text-gray-100 shadow-xl backdrop-blur-sm {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=content_style
            on:click=handle_click
            role="menu"
            aria-orientation="vertical"
        >
            {children.map(|c| c())}
        </div>
    }
}
