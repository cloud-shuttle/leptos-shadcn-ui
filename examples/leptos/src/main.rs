mod comprehensive_demo;

use leptos::prelude::*;
use leptos_meta::*;
use console_error_panic_hook::set_once as set_panic_hook;

fn main() {
    // Set up panic hook for better error messages in the browser
    set_panic_hook();

    // Mount the app to the body
    mount_to_body(|| view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Leptos ShadCN UI Demo"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Meta name="description" content="Interactive demo of Leptos ShadCN UI components"/>
        
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" rel="stylesheet"/>
        
        <comprehensive_demo::ComprehensiveDemo />
    }
}