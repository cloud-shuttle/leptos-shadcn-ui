use leptos::*;
use leptos::prelude::*;
use crate::enhanced_demo::EnhancedDemo;

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());

    view! {
        <div class="app" data-theme={current_theme}>
            <EnhancedDemo />
        </div>
    }
}
