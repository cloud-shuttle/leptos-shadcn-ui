use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Remove loading screen
    if let Some(loading_div) = document.get_element_by_id("loading") {
        loading_div.remove();
    }
    
    // Add simple content
    let body = document.body().unwrap();
    body.set_inner_html("<h1>Minimal WASM Test Works!</h1><p>This proves WASM initialization works.</p>");
}
