mod minimal_test;

use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to_body;
use crate::minimal_test::MinimalTest;

fn main() {
    // Set the page title
    document().set_title("Minimal Tailwind-RS-Core Test");
    
    mount_to_body(|| view! { <MinimalTest /> })
}
