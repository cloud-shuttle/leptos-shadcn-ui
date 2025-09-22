//! ContextMenu label and separator components
//! 
//! This module contains the ContextMenuLabel and ContextMenuSeparator
//! components for organizing context menu content.

use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn ContextMenuLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("context-menu-label {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("context-menu-separator {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            role="separator"
        />
    }
}
