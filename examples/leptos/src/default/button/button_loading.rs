use leptos::prelude::*;

use crate::default::components::ui::button::Button;

#[component]
pub fn ButtonLoading() -> impl IntoView {
    view! {
        <Button disabled=true>
            <svg class="mr-2 h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 12a9 9 0 11-6.219-8.56"/>
            </svg>
            "Please wait"
        </Button>
    }
}
