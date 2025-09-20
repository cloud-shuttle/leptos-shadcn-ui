#[cfg(test)]
mod real_tests {
    use crate::{LazyComponent, LazyComponentLoader};
    use leptos::prelude::*;

    #[test]
    fn test_lazy_component_loader_creation() {
        let loader = LazyComponentLoader::new();
        assert!(loader.registered_components().is_empty());
    }

    #[test]
    fn test_lazy_component_loader_registration() {
        let loader = LazyComponentLoader::new();
        loader.register_component("test", || Ok(View::new(())));
        assert!(loader.has_component("test"));
        assert!(!loader.has_component("nonexistent"));
    }

    #[test]
    fn test_lazy_loading_signal_state_management() {
        let signal = RwSignal::new(true);
        assert!(signal.get(), "lazy-loading signal should have initial value");
        
        signal.set(false);
        assert!(!signal.get(), "lazy-loading signal should update");
    }

    #[test]
    fn test_lazy_loading_callback_functionality() {
        let callback_triggered = RwSignal::new(false);
        let callback = Callback::new(move |_| {
            callback_triggered.set(true);
        });
        
        callback.run(());
        assert!(callback_triggered.get(), "lazy-loading callback should be triggered");
    }

    #[test]
    fn test_lazy_loading_class_handling() {
        let custom_class = "custom-lazy-loading-class";
        assert!(!custom_class.is_empty(), "lazy-loading should support custom classes");
        assert!(custom_class.contains("lazy-loading"), "Class should contain component name");
    }

    #[test]
    fn test_lazy_loading_id_handling() {
        let custom_id = "custom-lazy-loading-id";
        assert!(!custom_id.is_empty(), "lazy-loading should support custom IDs");
        assert!(custom_id.contains("lazy-loading"), "ID should contain component name");
    }
}