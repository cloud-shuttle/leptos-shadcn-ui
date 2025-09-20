// Real test modules for Button component
pub mod rendering;
pub mod interactions; 
pub mod accessibility;
pub mod variants;
pub mod integration;
pub mod wasm_tests;

// Test utilities
use leptos::prelude::*;
use wasm_bindgen_test::*;
use web_sys;
use wasm_bindgen_futures::JsFuture;

wasm_bindgen_test_configure!(run_in_browser);

// Test helper functions
pub fn create_test_container() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    container.set_attribute("id", "test-container").unwrap();
    document.body().unwrap().append_child(&container).unwrap();
    container
}

pub fn cleanup_test_container() {
    if let Some(container) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("test-container"))
    {
        container.remove();
    }
}

pub fn mount_component<F>(component_fn: F) -> web_sys::Element
where
    F: Fn() -> View + 'static,
{
    let container = create_test_container();
    
    mount_to(
        container.clone().unchecked_into::<web_sys::Element>(),
        component_fn,
    );
    
    container
}

// Wait for next animation frame (useful for testing DOM updates)
pub async fn next_frame() {
    use wasm_bindgen_futures::JsFuture;
    let window = web_sys::window().unwrap();
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        window
            .request_animation_frame(&resolve)
            .expect("Failed to request animation frame");
    });
    JsFuture::from(promise).await.unwrap();
}
