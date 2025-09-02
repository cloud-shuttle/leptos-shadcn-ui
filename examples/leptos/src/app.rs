use leptos::*;
use leptos::prelude::*;
use crate::default::components_demo::ComponentsDemo;

#[component]
pub fn App() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("default".to_string());

    let toggle_theme = move |_| {
        let new_theme = if current_theme.get_untracked() == "default" { "new_york" } else { "default" };
        set_current_theme.set(new_theme.to_string());
    };

    view! {
        <div class="app" data-theme={current_theme}>
            <header class="app-header">
                <div class="flex items-center justify-between p-4 border-b">
                    <h1 class="text-2xl font-bold">"Leptos ShadCN UI Demo"</h1>
                    <button on:click={toggle_theme} class="px-4 py-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90">
                        "Toggle Theme"
                    </button>
                </div>
            </header>

            <main class="app-main">
                <ComponentsDemo />
            </main>
        </div>
    }
}
