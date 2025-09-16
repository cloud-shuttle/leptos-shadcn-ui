use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Remove the loading screen
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(loading_div) = document.get_element_by_id("loading") {
        loading_div.remove();
    }
    
    // Mount a simple Leptos app
    mount_to_body(|| {
        view! {
            <div>
                <h1>"Simple Leptos WASM Test Works!"</h1>
                <p>"This proves Leptos WASM initialization works."</p>
            </div>
        }
    });
}