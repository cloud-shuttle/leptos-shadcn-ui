//! Lazy loading system for shadcn/ui Leptos components

use leptos::prelude::*;
use leptos::html::ElementChild;
use leptos::task::spawn_local;
use leptos::web_sys;
use leptos::ev::SubmitEvent;
use leptos_shadcn_error_boundary::{ErrorContext, ErrorSeverity};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// Lazy component loader that manages dynamic imports
#[derive(Clone)]
pub struct LazyComponentLoader {
    components: Arc<Mutex<HashMap<String, ComponentLoader>>>,
}

/// Component loader function type
pub type ComponentLoader = Box<dyn Fn() -> Result<View<()>, String> + Send + Sync>;

impl LazyComponentLoader {
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_component<F>(&self, name: &str, loader: F)
    where
        F: Fn() -> Result<View<()>, String> + Send + Sync + 'static,
    {
        let mut components = self.components.lock().unwrap();
        components.insert(name.to_string(), Box::new(loader));
    }

    pub fn load_component(&self, name: &str) -> Result<View<()>, String> {
        let components = self.components.lock().unwrap();
        if let Some(loader) = components.get(name) {
            loader()
        } else {
            Err(format!("Component '{}' not found", name))
        }
    }

    pub fn has_component(&self, name: &str) -> bool {
        let components = self.components.lock().unwrap();
        components.contains_key(name)
    }

    pub fn registered_components(&self) -> Vec<String> {
        let components = self.components.lock().unwrap();
        components.keys().cloned().collect()
    }
}

impl Default for LazyComponentLoader {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Lazy Component Wrapper
// =============================================================================

#[component]
pub fn LazyComponent(
    #[prop(into)] name: String,
    #[prop(optional)] fallback: Option<Box<dyn Fn() -> AnyView + Send + Sync>>,
    #[prop(optional)] error_fallback: Option<Box<dyn Fn(String) -> AnyView + Send + Sync>>,
) -> impl IntoView {
    let loader = use_context::<LazyComponentLoader>()
        .expect("LazyComponentLoader not found in context");

    let (component, set_component) = signal(None::<Result<View<()>, String>>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (is_mounted, set_is_mounted) = signal(true);

    Effect::new(move |_| {
        let name = name.clone();
        let loader = loader.clone();
        let is_mounted = is_mounted.clone();

        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            let result = loader.load_component(&name);
            if is_mounted.get() {
                set_component.set(Some(result.clone()));
                set_loading.set(false);
                if let Err(err) = result {
                    set_error.set(Some(err));
                }
            }
        });
    });

    on_cleanup(move || {
        set_is_mounted.set(false);
    });

    move || {
        if loading.get() {
            if let Some(fallback_fn) = &fallback {
                fallback_fn()
            } else {
                view! {
                    <div class="lazy-loading-fallback">
                        <div class="loading-spinner"></div>
                        <p>"Loading component..."</p>
                    </div>
                }.into_any()
            }
        } else if let Some(Ok(comp)) = component.get() {
            comp.into_any()
        } else if let Some(err) = error.get() {
            if let Some(error_fn) = &error_fallback {
                error_fn(err)
            } else {
                let error_context = ErrorContext::new(format!("Failed to load component: {}", err))
                    .with_severity(ErrorSeverity::Error);

                let retry_loading = move |_| {
                    set_loading.set(true);
                    set_error.set(None);
                };

                view! {
                    <div class="lazy-loading-error-fallback">
                        <p class="error-message">{error_context.message}</p>
                        <button class="error-retry" on:click=retry_loading>"Retry"</button>
                    </div>
                }.into_any()
            }
        } else {
            view! { <div></div> }.into_any()
        }
    }
}

/// Hook for lazy loading components with proper cleanup
pub fn use_lazy_component(name: &str) -> (ReadSignal<bool>, ReadSignal<Option<Result<View<()>, String>>>, WriteSignal<bool>) {
    let (loading, set_loading) = signal(false);
    let (component, set_component) = signal(None::<Result<View<()>, String>>);
    let (is_mounted, set_is_mounted) = signal(true);

    let loader = use_context::<LazyComponentLoader>()
        .expect("LazyComponentLoader not found in context");

    let name = name.to_string();
    let is_mounted_for_load = is_mounted.clone();
    let load = move || {
        set_loading.set(true);
        spawn_local(async move {
            let result = loader.load_component(&name);
            if is_mounted_for_load.get() {
                set_component.set(Some(result));
                set_loading.set(false);
            }
        });
    };

    on_cleanup(move || {
        set_is_mounted.set(false);
    });

    (loading, component, set_loading)
}

/// Component bundle analyzer for optimization
pub struct BundleAnalyzer;

impl BundleAnalyzer {
    pub fn analyze_usage(components: &[String]) -> BundleAnalysis {
        let mut analysis = BundleAnalysis::new();
        let form_components = ["input", "label", "checkbox", "radio-group", "select", "textarea", "form"];
        let layout_components = ["card", "separator", "skeleton", "tabs"];
        let interactive_components = ["button", "checkbox", "radio-group", "select", "switch", "tabs"];

        let form_count = components.iter().filter(|c| form_components.contains(&c.as_str())).count();
        let layout_count = components.iter().filter(|c| layout_components.contains(&c.as_str())).count();
        let interactive_count = components.iter().filter(|c| interactive_components.contains(&c.as_str())).count();

        analysis.form_components = form_count;
        analysis.layout_components = layout_count;
        analysis.interactive_components = interactive_count;
        analysis.total_components = components.len();

        if form_count > 4 {
            analysis.recommendations.push("Consider lazy loading form components to reduce initial bundle size".to_string());
        }
        if layout_count > 3 {
            analysis.recommendations.push("Layout components can be loaded on demand for better performance".to_string());
        }
        if interactive_count > 5 {
            analysis.recommendations.push("Interactive components benefit from lazy loading for better UX".to_string());
        }
        analysis
    }
}

#[derive(Debug, Clone)]
pub struct BundleAnalysis {
    pub form_components: usize,
    pub layout_components: usize,
    pub interactive_components: usize,
    pub total_components: usize,
    pub recommendations: Vec<String>,
}

impl BundleAnalysis {
    fn new() -> Self {
        Self {
            form_components: 0,
            layout_components: 0,
            interactive_components: 0,
            total_components: 0,
            recommendations: Vec::new(),
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_lazy_component_loader() {
        let loader = LazyComponentLoader::new();
        loader.register_component("test", || { Ok(View::new(())) });
        assert!(loader.has_component("test"));
        assert!(!loader.has_component("nonexistent"));
        let components = loader.registered_components();
        assert!(components.contains(&"test".to_string()));
    }
    #[test]
    fn test_bundle_analyzer() {
        let components = vec!["button".to_string(), "input".to_string(), "card".to_string()];
        let analysis = BundleAnalyzer::analyze_usage(&components);
        assert_eq!(analysis.total_components, 3);
        assert_eq!(analysis.form_components, 1);
        assert_eq!(analysis.layout_components, 1);
        assert_eq!(analysis.interactive_components, 1);
    }
}
