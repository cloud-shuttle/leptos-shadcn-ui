//! Reusable test templates for component testing
//! These templates can be adapted for different component types

#[cfg(test)]
mod test_templates {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Template for basic component rendering tests
    pub fn test_component_renders<T, F>(component_name: &str, create_component: F)
    where
        F: Fn() -> T + 'static,
        T: IntoView,
    {
        mount_to_body(|| {
            view! {
                {create_component()}
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(&format!("[data-testid='{}']", component_name)).unwrap();
        assert!(element.is_some(), "{} should render in DOM", component_name);
    }

    /// Template for variant testing
    pub fn test_component_variants<T, F, V>(component_name: &str, variants: Vec<V>, create_component: F)
    where
        F: Fn(V) -> T + 'static,
        T: IntoView,
        V: Clone + std::fmt::Debug,
    {
        for variant in variants {
            mount_to_body(move || {
                view! {
                    <div data-testid=format!("{}-{:?}", component_name, variant)>
                        {create_component(variant.clone())}
                    </div>
                }
            });
            
            let document = web_sys::window().unwrap().document().unwrap();
            let element = document.query_selector(&format!("[data-testid='{}-{:?}']", component_name, variant)).unwrap();
            assert!(element.is_some(), "{} variant {:?} should render", component_name, variant);
        }
    }

    /// Template for size testing
    pub fn test_component_sizes<T, F, S>(component_name: &str, sizes: Vec<S>, create_component: F)
    where
        F: Fn(S) -> T + 'static,
        T: IntoView,
        S: Clone + std::fmt::Debug,
    {
        for size in sizes {
            mount_to_body(move || {
                view! {
                    <div data-testid=format!("{}-size-{:?}", component_name, size)>
                        {create_component(size.clone())}
                    </div>
                }
            });
            
            let document = web_sys::window().unwrap().document().unwrap();
            let element = document.query_selector(&format!("[data-testid='{}-size-{:?}']", component_name, size)).unwrap();
            assert!(element.is_some(), "{} size {:?} should render", component_name, size);
        }
    }

    /// Template for disabled state testing
    pub fn test_component_disabled_state<T, F>(component_name: &str, create_component: F)
    where
        F: Fn(bool) -> T + 'static,
        T: IntoView,
    {
        // Test enabled state
        mount_to_body(|| {
            view! {
                <div data-testid=format!("{}-enabled", component_name)>
                    {create_component(false)}
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let enabled_element = document.query_selector(&format!("[data-testid='{}-enabled']", component_name)).unwrap();
        assert!(enabled_element.is_some(), "{} should render when enabled", component_name);

        // Test disabled state
        mount_to_body(|| {
            view! {
                <div data-testid=format!("{}-disabled", component_name)>
                    {create_component(true)}
                </div>
            }
        });
        
        let disabled_element = document.query_selector(&format!("[data-testid='{}-disabled']", component_name)).unwrap();
        assert!(disabled_element.is_some(), "{} should render when disabled", component_name);
    }

    /// Template for click handler testing
    pub fn test_component_click_handler<T, F>(component_name: &str, create_component: F)
    where
        F: Fn(Option<Callback<()>>) -> T + 'static,
        T: IntoView,
    {
        let click_count = RwSignal::new(0);
        let click_callback = Callback::new(move |_| {
            click_count.update(|count| *count += 1);
        });
        
        mount_to_body(move || {
            view! {
                <div data-testid=format!("{}-clickable", component_name)>
                    {create_component(Some(click_callback))}
                </div>
            }
        });
        
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(&format!("[data-testid='{}-clickable']", component_name)).unwrap().unwrap();
        
        // Simulate click
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
        
        assert_eq!(click_count.get(), 1, "{} click handler should be triggered", component_name);
    }

    /// Template for class constant testing
    pub fn test_component_class_constants(class_constants: Vec<(&str, &str)>) {
        for (constant_name, class_value) in class_constants {
            assert!(!class_value.is_empty(), "{} class constant should not be empty", constant_name);
            assert!(class_value.contains(" "), "{} class constant should contain multiple classes", constant_name);
        }
    }

    /// Template for signal state management testing
    pub fn test_signal_state_management<T, F>(initial_value: T, test_function: F)
    where
        T: Clone + PartialEq + std::fmt::Debug,
        F: Fn(RwSignal<T>),
    {
        let signal = RwSignal::new(initial_value.clone());
        assert_eq!(signal.get(), initial_value, "Signal should have initial value");
        
        test_function(signal);
    }

    /// Template for callback testing
    pub fn test_callback_functionality<T, F>(callback_test: F)
    where
        F: Fn(Callback<T>),
        T: Clone,
    {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback_test(callback);
        assert!(callback_triggered.get(), "Callback should have been triggered");
    }
}
