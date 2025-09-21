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
//! - 🎯 **50+ Components**: Complete ShadCN UI component library
//! - 🔧 **Easy Integration**: Simple API, works with existing Leptos apps
//! - ⚡ **Feature Flags**: Include only the components you need
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! leptos-shadcn-ui-wasm = "0.2"
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

// Re-export all available components for easy access
// Note: We re-export the main component from each package
// Users can access sub-components directly from the individual packages if needed

#[cfg(feature = "accordion")]
pub use leptos_shadcn_accordion::*;

#[cfg(feature = "alert")]
pub use leptos_shadcn_alert::*;

#[cfg(feature = "alert-dialog")]
pub use leptos_shadcn_alert_dialog::*;

#[cfg(feature = "aspect-ratio")]
pub use leptos_shadcn_aspect_ratio::*;

#[cfg(feature = "avatar")]
pub use leptos_shadcn_avatar::*;

#[cfg(feature = "badge")]
pub use leptos_shadcn_badge::*;

#[cfg(feature = "breadcrumb")]
pub use leptos_shadcn_breadcrumb::*;

#[cfg(feature = "button")]
pub use leptos_shadcn_button::*;

#[cfg(feature = "calendar")]
pub use leptos_shadcn_calendar::*;

#[cfg(feature = "card")]
pub use leptos_shadcn_card::*;

#[cfg(feature = "carousel")]
pub use leptos_shadcn_carousel::*;

#[cfg(feature = "checkbox")]
pub use leptos_shadcn_checkbox::*;

#[cfg(feature = "collapsible")]
pub use leptos_shadcn_collapsible::*;

#[cfg(feature = "combobox")]
pub use leptos_shadcn_combobox::*;

#[cfg(feature = "command")]
pub use leptos_shadcn_command::*;

#[cfg(feature = "context-menu")]
pub use leptos_shadcn_context_menu::*;

#[cfg(feature = "date-picker")]
pub use leptos_shadcn_date_picker::*;

#[cfg(feature = "dialog")]
pub use leptos_shadcn_dialog::*;

#[cfg(feature = "drawer")]
pub use leptos_shadcn_drawer::*;

#[cfg(feature = "dropdown-menu")]
pub use leptos_shadcn_dropdown_menu::*;

#[cfg(feature = "error-boundary")]
pub use leptos_shadcn_error_boundary::*;

#[cfg(feature = "form")]
pub use leptos_shadcn_form::*;

#[cfg(feature = "hover-card")]
pub use leptos_shadcn_hover_card::*;

#[cfg(feature = "input")]
pub use leptos_shadcn_input::*;

#[cfg(feature = "input-otp")]
pub use leptos_shadcn_input_otp::*;

#[cfg(feature = "label")]
pub use leptos_shadcn_label::*;

#[cfg(feature = "menubar")]
pub use leptos_shadcn_menubar::*;

#[cfg(feature = "navigation-menu")]
pub use leptos_shadcn_navigation_menu::*;

#[cfg(feature = "pagination")]
pub use leptos_shadcn_pagination::*;

#[cfg(feature = "popover")]
pub use leptos_shadcn_popover::*;

#[cfg(feature = "progress")]
pub use leptos_shadcn_progress::*;

#[cfg(feature = "radio-group")]
pub use leptos_shadcn_radio_group::*;

#[cfg(feature = "resizable")]
pub use leptos_shadcn_resizable::*;

#[cfg(feature = "scroll-area")]
pub use leptos_shadcn_scroll_area::*;

#[cfg(feature = "select")]
pub use leptos_shadcn_select::*;

#[cfg(feature = "separator")]
pub use leptos_shadcn_separator::*;

#[cfg(feature = "sheet")]
pub use leptos_shadcn_sheet::*;

#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::*;

#[cfg(feature = "slider")]
pub use leptos_shadcn_slider::*;

#[cfg(feature = "switch")]
pub use leptos_shadcn_switch::*;

#[cfg(feature = "table")]
pub use leptos_shadcn_table::*;

#[cfg(feature = "tabs")]
pub use leptos_shadcn_tabs::*;

#[cfg(feature = "textarea")]
pub use leptos_shadcn_textarea::*;

#[cfg(feature = "toast")]
pub use leptos_shadcn_toast::*;

#[cfg(feature = "toggle")]
pub use leptos_shadcn_toggle::*;

#[cfg(feature = "tooltip")]
pub use leptos_shadcn_tooltip::*;

/// Convenience module for easy imports
pub mod prelude {
    //! Re-exports all available components and utilities
    
    // Re-export all components from the main module
    pub use super::*;
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
        
        #[cfg(feature = "accordion")]
        features.push("accordion");
        #[cfg(feature = "alert")]
        features.push("alert");
        #[cfg(feature = "alert-dialog")]
        features.push("alert-dialog");
        #[cfg(feature = "aspect-ratio")]
        features.push("aspect-ratio");
        #[cfg(feature = "avatar")]
        features.push("avatar");
        #[cfg(feature = "badge")]
        features.push("badge");
        #[cfg(feature = "breadcrumb")]
        features.push("breadcrumb");
        #[cfg(feature = "button")]
        features.push("button");
        #[cfg(feature = "calendar")]
        features.push("calendar");
        #[cfg(feature = "card")]
        features.push("card");
        #[cfg(feature = "carousel")]
        features.push("carousel");
        #[cfg(feature = "checkbox")]
        features.push("checkbox");
        #[cfg(feature = "collapsible")]
        features.push("collapsible");
        #[cfg(feature = "combobox")]
        features.push("combobox");
        #[cfg(feature = "command")]
        features.push("command");
        #[cfg(feature = "context-menu")]
        features.push("context-menu");
        #[cfg(feature = "date-picker")]
        features.push("date-picker");
        #[cfg(feature = "dialog")]
        features.push("dialog");
        #[cfg(feature = "drawer")]
        features.push("drawer");
        #[cfg(feature = "dropdown-menu")]
        features.push("dropdown-menu");
        #[cfg(feature = "error-boundary")]
        features.push("error-boundary");
        #[cfg(feature = "form")]
        features.push("form");
        #[cfg(feature = "hover-card")]
        features.push("hover-card");
        #[cfg(feature = "input")]
        features.push("input");
        #[cfg(feature = "input-otp")]
        features.push("input-otp");
        #[cfg(feature = "label")]
        features.push("label");
        #[cfg(feature = "menubar")]
        features.push("menubar");
        #[cfg(feature = "navigation-menu")]
        features.push("navigation-menu");
        #[cfg(feature = "pagination")]
        features.push("pagination");
        #[cfg(feature = "popover")]
        features.push("popover");
        #[cfg(feature = "progress")]
        features.push("progress");
        #[cfg(feature = "radio-group")]
        features.push("radio-group");
        #[cfg(feature = "resizable")]
        features.push("resizable");
        #[cfg(feature = "scroll-area")]
        features.push("scroll-area");
        #[cfg(feature = "select")]
        features.push("select");
        #[cfg(feature = "separator")]
        features.push("separator");
        #[cfg(feature = "sheet")]
        features.push("sheet");
        #[cfg(feature = "skeleton")]
        features.push("skeleton");
        #[cfg(feature = "slider")]
        features.push("slider");
        #[cfg(feature = "switch")]
        features.push("switch");
        #[cfg(feature = "table")]
        features.push("table");
        #[cfg(feature = "tabs")]
        features.push("tabs");
        #[cfg(feature = "textarea")]
        features.push("textarea");
        #[cfg(feature = "toast")]
        features.push("toast");
        #[cfg(feature = "toggle")]
        features.push("toggle");
        #[cfg(feature = "tooltip")]
        features.push("tooltip");
        
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
