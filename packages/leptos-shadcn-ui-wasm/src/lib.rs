//! # Leptos ShadCN UI - WASM Optimized
//!
//! A WASM-optimized version of ShadCN UI components for Leptos 0.8+ with minimal dependencies.
//! This package is specifically designed for WebAssembly environments and excludes
//! dependencies that are not WASM-compatible.
//!
//! ## Features
//!
//! - 🚀 **WASM-Optimized**: Minimal dependencies, fast compilation
//! - 📦 **Small Bundle Size**: Optimized for web deployment
//! - 🎯 **Core Components**: Essential UI components for web apps
//! - 🔧 **Easy Integration**: Simple API, works with existing Leptos apps
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! leptos-shadcn-ui-wasm = "0.1"
//! ```
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_shadcn_ui_wasm::prelude::*;
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     view! {
//!         <div class="p-4">
//!             <Button>"Click me"</Button>
//!             <Input placeholder="Enter text..." />
//!         </div>
//!     }
//! }
//! ```

// Re-export core components for easy access
#[cfg(feature = "button")]
pub use leptos_shadcn_button::Button;

#[cfg(feature = "input")]
pub use leptos_shadcn_input::Input;

#[cfg(feature = "card")]
pub use leptos_shadcn_card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[cfg(feature = "label")]
pub use leptos_shadcn_label::Label;

#[cfg(feature = "badge")]
pub use leptos_shadcn_badge::Badge;

#[cfg(feature = "avatar")]
pub use leptos_shadcn_avatar::{Avatar, AvatarFallback, AvatarImage};

#[cfg(feature = "separator")]
pub use leptos_shadcn_separator::Separator;

#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::Skeleton;

#[cfg(feature = "alert")]
pub use leptos_shadcn_alert::{Alert, AlertDescription, AlertTitle};

#[cfg(feature = "alert-dialog")]
pub use leptos_shadcn_alert_dialog::{AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger};

/// Convenience module for easy imports
pub mod prelude {
    //! Re-exports commonly used components and utilities
    
    #[cfg(feature = "button")]
    pub use super::Button;
    
    #[cfg(feature = "input")]
    pub use super::Input;
    
    #[cfg(feature = "card")]
    pub use super::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
    
    #[cfg(feature = "label")]
    pub use super::Label;
    
    #[cfg(feature = "badge")]
    pub use super::Badge;
    
    #[cfg(feature = "avatar")]
    pub use super::{Avatar, AvatarFallback, AvatarImage};
    
    #[cfg(feature = "separator")]
    pub use super::Separator;
    
    #[cfg(feature = "skeleton")]
    pub use super::Skeleton;
    
    #[cfg(feature = "alert")]
    pub use super::{Alert, AlertDescription, AlertTitle};
    
    #[cfg(feature = "alert-dialog")]
    pub use super::{AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger};
}

/// WASM-specific utilities and helpers
pub mod wasm_utils {
    //! Utilities specifically designed for WASM environments
    
    use wasm_bindgen::prelude::*;
    use web_sys::*;
    
    /// Initialize WASM-specific features
    pub fn init_wasm() {
        // Set up panic hook for better error reporting in browser
        console_error_panic_hook::set_once();
        
        // Log that WASM version is being used
        web_sys::console::log_1(&"Leptos ShadCN UI WASM version initialized".into());
    }
    
    /// Get current timestamp for WASM environments
    pub fn now() -> f64 {
        let window = window().unwrap();
        let performance = window.performance().unwrap();
        performance.now()
    }
    
    /// Log to browser console
    pub fn log(message: &str) {
        web_sys::console::log_1(&message.into());
    }
    
    /// Log error to browser console
    pub fn log_error(message: &str) {
        web_sys::console::error_1(&message.into());
    }
}

/// Bundle size optimization utilities
pub mod bundle_utils {
    //! Utilities for optimizing bundle size in WASM
    
    use super::BundleInfo;
    
    /// Get bundle size information
    pub fn get_bundle_info() -> BundleInfo {
        BundleInfo {
            version: env!("CARGO_PKG_VERSION"),
            features: get_enabled_features(),
            wasm_optimized: cfg!(feature = "wasm-optimized"),
        }
    }
    
    /// Get list of enabled features
    pub fn get_enabled_features() -> Vec<&'static str> {
        let mut features = Vec::new();
        
        #[cfg(feature = "button")]
        features.push("button");
        #[cfg(feature = "input")]
        features.push("input");
        #[cfg(feature = "card")]
        features.push("card");
        #[cfg(feature = "label")]
        features.push("label");
        #[cfg(feature = "badge")]
        features.push("badge");
        #[cfg(feature = "avatar")]
        features.push("avatar");
        #[cfg(feature = "separator")]
        features.push("separator");
        #[cfg(feature = "skeleton")]
        features.push("skeleton");
        #[cfg(feature = "alert")]
        features.push("alert");
        #[cfg(feature = "alert-dialog")]
        features.push("alert-dialog");
        
        features
    }
}

/// Bundle information structure
#[derive(Debug, Clone)]
pub struct BundleInfo {
    pub version: &'static str,
    pub features: Vec<&'static str>,
    pub wasm_optimized: bool,
}

impl std::fmt::Display for BundleInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leptos ShadCN UI WASM v{} (features: {})", 
               self.version, 
               self.features.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_bundle_info() {
        let info = bundle_utils::get_bundle_info();
        assert!(!info.version.is_empty());
        assert!(info.wasm_optimized || !cfg!(feature = "wasm-optimized"));
    }
    
    #[wasm_bindgen_test]
    fn test_wasm_utils() {
        wasm_utils::init_wasm();
        let timestamp = wasm_utils::now();
        assert!(timestamp >= 0.0);
    }
}
