use leptos::prelude::*;


use crate::default::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <Alert>
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="4,17 10,11 4,5"/>
                <line x1="12" y1="19" x2="20" y2="19"/>
            </svg>
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
