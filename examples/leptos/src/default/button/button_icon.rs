use leptos::prelude::*;

use crate::default::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn ButtonIcon() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline} size={ButtonSize::Icon}>
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="m9 18 6-6-6-6"/>
            </svg>
        </Button>
    }
}
