//! AlertDialog header and footer components
//! 
//! This module contains the AlertDialogHeader and AlertDialogFooter
//! components for organizing alert dialog content.

use leptos::prelude::*;
use leptos_style::Style;

#[component]
pub fn AlertDialogHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("flex flex-col space-y-2 text-center sm:text-left {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            {children.map(|c| c())}
        </div>
    }
}
