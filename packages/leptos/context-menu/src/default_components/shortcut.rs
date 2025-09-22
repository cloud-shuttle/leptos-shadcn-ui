//! ContextMenu shortcut component
//! 
//! This module contains the ContextMenuShortcut component for displaying
//! keyboard shortcuts in context menu items.

use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn ContextMenuShortcut(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <span
            class=move || format!("context-menu-shortcut {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </span>
    }
}
