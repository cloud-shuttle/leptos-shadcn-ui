//! Main ContextMenu component
//! 
//! This module contains the main ContextMenu component that provides context
//! for the context menu system.

use leptos::prelude::*;

#[component]
pub fn ContextMenu(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let position = RwSignal::new((0, 0));

    provide_context(open);
    provide_context(position);

    view! {
        <div class="relative">
            {children.map(|c| c())}
        </div>
    }
}
