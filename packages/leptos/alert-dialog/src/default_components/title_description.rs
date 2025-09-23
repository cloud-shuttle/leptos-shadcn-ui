//! AlertDialog title and description components
//! 
//! This module contains the AlertDialogTitle and AlertDialogDescription
//! components for providing accessible labels and descriptions for the alert dialog.

use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn AlertDialogTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <h2
            class=move || format!("text-lg font-semibold {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn AlertDialogDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <p
            class=move || format!("text-sm text-muted-foreground {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </p>
    }
}
