//! AlertDialog content component
//! 
//! This module contains the AlertDialogContent component for the main
//! alert dialog content area.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <Show
            when=move || open.get()
            fallback=|| view! { <div></div> }
        >
            <div
                class=move || format!("fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm {}", class.get().unwrap_or_default())
                id=move || id.get().unwrap_or_default()
                style=move || style.get().unwrap_or_default()
                on:click=handle_click
                role="alertdialog"
                aria-modal="true"
            >
                <div class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm" on:click=move |_| open.set(false)>
                    <div class="relative z-50 grid w-full max-w-lg gap-4 border bg-background p-6 shadow-lg sm:rounded-lg" on:click=handle_click>
                        {children.map(|c| c())}
                    </div>
                </div>
            </div>
        </Show>
    }
}
