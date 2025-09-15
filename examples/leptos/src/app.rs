use leptos::*;
use leptos::prelude::*;
use crate::comprehensive_demo::ComprehensiveDemo;

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());

    view! {
        <div class="min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100" data-theme={current_theme}>
            <ComprehensiveDemo />
        </div>
    }
}
