mod app;
mod default;
mod new_york;
mod lazy_loading;
mod bundle_analyzer;
mod dynamic_loader;
mod enhanced_demo;

use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to_body;
use crate::app::App;

fn main() {
    // Set the page title
    document().set_title("leptos-shadcn-ui Demo - Performance Champion");
    
    mount_to_body(|| view! { <App /> })
}
