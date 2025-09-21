# WASM-Only Minimal Version Design
## leptos-shadcn-ui-wasm Package Architecture

**Document Version:** 1.0  
**Date:** 2025-01-27  
**Status:** DRAFT - Implementation Ready  

---

## 🎯 Overview

This document outlines the design for a dedicated WASM-only version of leptos-shadcn-ui (`leptos-shadcn-ui-wasm`) that provides a minimal, optimized package specifically for WebAssembly environments. This approach eliminates all non-WASM dependencies and provides the smallest possible bundle size.

## 🏗️ Package Architecture

### Package Structure

```
packages/
├── leptos-shadcn-ui-wasm/              # New WASM-only package
│   ├── Cargo.toml                      # Minimal WASM dependencies
│   ├── src/
│   │   ├── lib.rs                      # Main library entry point
│   │   ├── components/                 # WASM-compatible components
│   │   │   ├── mod.rs                  # Component module declarations
│   │   │   ├── button.rs               # Button component
│   │   │   ├── input.rs                # Input component
│   │   │   ├── card.rs                 # Card component
│   │   │   ├── label.rs                # Label component
│   │   │   ├── dialog.rs               # Dialog component
│   │   │   ├── popover.rs              # Popover component
│   │   │   └── tooltip.rs              # Tooltip component
│   │   ├── utils/                      # WASM-specific utilities
│   │   │   ├── mod.rs                  # Utility module declarations
│   │   │   ├── dom.rs                  # DOM manipulation utilities
│   │   │   ├── performance.rs          # Performance monitoring
│   │   │   ├── storage.rs              # Browser storage utilities
│   │   │   └── events.rs               # Event handling utilities
│   │   ├── styles/                     # Styling utilities
│   │   │   ├── mod.rs                  # Style module declarations
│   │   │   ├── themes.rs               # Theme management
│   │   │   ├── variants.rs             # Component variants
│   │   │   └── animations.rs           # Animation utilities
│   │   └── types/                      # Type definitions
│   │       ├── mod.rs                  # Type module declarations
│   │       ├── common.rs               # Common types
│   │       ├── events.rs               # Event types
│   │       └── props.rs                # Component prop types
│   ├── examples/                       # WASM-specific examples
│   │   ├── basic/                      # Basic component usage
│   │   ├── advanced/                   # Advanced patterns
│   │   └── performance/                # Performance examples
│   ├── tests/                          # WASM-specific tests
│   │   ├── unit/                       # Unit tests
│   │   ├── integration/                # Integration tests
│   │   └── performance/                # Performance tests
│   ├── README.md                       # Package documentation
│   └── CHANGELOG.md                    # Version history
```

### Cargo.toml Configuration

```toml
[package]
name = "leptos-shadcn-ui-wasm"
version = "0.9.0"
edition = "2021"
description = "WASM-only version of leptos-shadcn-ui with minimal dependencies and optimized bundle size"
homepage = "https://github.com/cloud-shuttle/leptos-shadcn-ui"
repository = "https://github.com/cloud-shuttle/leptos-shadcn-ui"
license = "MIT"
authors = ["CloudShuttle <info@cloudshuttle.com>"]
keywords = ["leptos", "ui", "components", "shadcn", "wasm", "webassembly"]
categories = ["wasm", "gui", "web-programming"]
readme = "README.md"
rust-version = "1.70"

# WASM-specific crate type
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# Core Leptos framework (WASM-compatible features only)
leptos = { version = "0.8.9", features = ["csr", "hydrate"] }
leptos_router = { version = "0.8.9", features = ["csr"] }
leptos_meta = "0.8.9"
leptos-node-ref = "0.2.0"
leptos-struct-component = "0.2.0"
leptos-style = "0.2.0"

# WASM-specific dependencies only
wasm-bindgen = "0.2.101"
web-sys = { version = "0.3.77", features = [
    "console",
    "Document",
    "Element", 
    "HtmlElement",
    "HtmlButtonElement",
    "HtmlInputElement",
    "HtmlDivElement",
    "Node",
    "Window",
    "Event",
    "EventTarget",
    "MouseEvent",
    "KeyboardEvent",
    "TouchEvent",
    "Performance",
    "PerformanceTiming",
    "Storage",
    "LocalStorage",
    "SessionStorage",
    "CssStyleDeclaration",
    "Element",
    "HtmlCollection",
    "NodeList"
] }
js-sys = "0.3.77"
console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
log = "0.4.20"

# WASM-compatible random generation
getrandom = { version = "0.2.12", features = ["js"] }

# WASM-compatible UUID generation
uuid = { version = "1.6.1", features = ["v4", "js", "wasm-bindgen"] }

# WASM-compatible serialization
serde = { version = "1.0.193", features = ["derive"] }
serde_json = "1.0.108"

# WASM-compatible async runtime
wasm-bindgen-futures = "0.4.40"
futures = "0.3.30"

# WASM-compatible timers
gloo-timers = { version = "0.3.0", features = ["futures"] }

# Styling and theming
tailwind_fuse = { version = "0.3.2", features = ["variant"] }

# Error handling
thiserror = "1.0.56"
anyhow = "1.0.80"

# Optional: WASM-compatible image handling
# wasm-bindgen-image = { version = "0.1.0", optional = true }

[features]
default = ["essential-components", "performance-monitoring"]
essential-components = ["button", "input", "label", "card"]
extended-components = ["essential-components", "dialog", "popover", "tooltip", "alert", "badge"]
advanced-components = ["extended-components", "table", "calendar", "form", "combobox"]
all-components = ["advanced-components", "navigation-menu", "dropdown-menu", "context-menu"]

# Individual component features
button = []
input = []
label = []
card = []
dialog = []
popover = []
tooltip = []
alert = []
badge = []
table = []
calendar = []
form = []
combobox = []
navigation-menu = []
dropdown-menu = []
context-menu = []

# Utility features
performance-monitoring = []
theme-management = []
animation-support = []
accessibility = []
keyboard-navigation = []

# Optional features
image-support = ["dep:wasm-bindgen-image"]

[dev-dependencies]
# WASM-specific testing
wasm-bindgen-test = "0.3.40"
wasm-bindgen-futures = "0.4.40"

# Testing utilities
proptest = { version = "1.4.0", default-features = false, features = ["std"] }

# Performance testing
criterion = { version = "0.5.1", features = ["html_reports"] }

# Documentation
wasm-pack = "0.12.1"

[[bench]]
name = "component_benchmarks"
harness = false
required-features = ["performance-monitoring"]

[package.metadata.docs.rs]
rustdoc-args = ["--cfg", "docsrs"]
targets = ["wasm32-unknown-unknown"]

[package.metadata.wasm-pack]
# WASM-pack configuration
out-dir = "pkg"
target = "web"
scope = "leptos-shadcn"
```

## 🧩 Component Architecture

### Core Component Structure

```rust
// src/components/button.rs
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use crate::utils::dom::DomUtils;
use crate::utils::performance::PerformanceMonitor;
use crate::types::props::ButtonProps;

/// WASM-optimized Button component
#[component]
pub fn Button(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<ButtonSize>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] loading: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let button_id = id.unwrap_or_else(|| DomUtils::generate_unique_id());
    let is_disabled = disabled.unwrap_or(false);
    let is_loading = loading.unwrap_or(false);
    
    // Performance monitoring
    let perf_monitor = PerformanceMonitor::new("button_render");
    
    // Event handling with WASM optimization
    let handle_click = move |event: MouseEvent| {
        if !is_disabled && !is_loading {
            if let Some(callback) = on_click {
                callback.call(event);
            }
        }
    };
    
    // Styling with WASM-optimized class generation
    let button_classes = generate_button_classes(variant, size, is_disabled, is_loading);
    
    view! {
        <button
            class=button_classes
            id=button_id
            disabled=is_disabled
            on:click=handle_click
        >
            {if is_loading {
                view! { <span class="loading-spinner"></span> }.into_any()
            } else {
                children().into_any()
            }}
        </button>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

fn generate_button_classes(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    disabled: bool,
    loading: bool,
) -> String {
    let mut classes = vec!["btn".to_string()];
    
    // Variant classes
    match variant.unwrap_or(ButtonVariant::Default) {
        ButtonVariant::Default => classes.push("btn-default".to_string()),
        ButtonVariant::Destructive => classes.push("btn-destructive".to_string()),
        ButtonVariant::Outline => classes.push("btn-outline".to_string()),
        ButtonVariant::Secondary => classes.push("btn-secondary".to_string()),
        ButtonVariant::Ghost => classes.push("btn-ghost".to_string()),
        ButtonVariant::Link => classes.push("btn-link".to_string()),
    }
    
    // Size classes
    match size.unwrap_or(ButtonSize::Default) {
        ButtonSize::Default => classes.push("btn-md".to_string()),
        ButtonSize::Sm => classes.push("btn-sm".to_string()),
        ButtonSize::Lg => classes.push("btn-lg".to_string()),
        ButtonSize::Icon => classes.push("btn-icon".to_string()),
    }
    
    // State classes
    if disabled {
        classes.push("btn-disabled".to_string());
    }
    if loading {
        classes.push("btn-loading".to_string());
    }
    
    classes.join(" ")
}
```

### WASM-Specific Utilities

```rust
// src/utils/dom.rs
use wasm_bindgen::prelude::*;
use web_sys::*;
use uuid::Uuid;

pub struct DomUtils;

impl DomUtils {
    /// Generate a unique ID using WASM-compatible UUID
    pub fn generate_unique_id() -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Efficiently query DOM elements
    pub fn query_selector(selector: &str) -> Option<Element> {
        let document = window()?.document()?;
        document.query_selector(selector).ok().flatten()
    }
    
    /// Batch DOM operations for better performance
    pub fn batch_operations<F>(operations: F)
    where
        F: FnOnce(),
    {
        // Use requestAnimationFrame for optimal timing
        let closure = Closure::wrap(Box::new(operations) as Box<dyn FnOnce()>);
        window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    
    /// Add event listener with automatic cleanup
    pub fn add_event_listener_with_cleanup<F>(
        element: &EventTarget,
        event_type: &str,
        callback: F,
    ) -> Result<(), JsValue>
    where
        F: FnMut(Event) + 'static,
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);
        element.add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}
```

```rust
// src/utils/performance.rs
use wasm_bindgen::prelude::*;
use web_sys::*;
use std::collections::HashMap;

pub struct PerformanceMonitor {
    name: String,
    start_time: f64,
    measurements: HashMap<String, f64>,
}

impl PerformanceMonitor {
    pub fn new(name: &str) -> Self {
        let start_time = Self::get_current_time();
        Self {
            name: name.to_string(),
            start_time,
            measurements: HashMap::new(),
        }
    }
    
    pub fn mark(&mut self, label: &str) {
        let current_time = Self::get_current_time();
        let duration = current_time - self.start_time;
        self.measurements.insert(label.to_string(), duration);
    }
    
    pub fn finish(self) -> PerformanceReport {
        let total_time = Self::get_current_time() - self.start_time;
        PerformanceReport {
            name: self.name,
            total_time,
            measurements: self.measurements,
        }
    }
    
    fn get_current_time() -> f64 {
        window()
            .unwrap()
            .performance()
            .unwrap()
            .now()
    }
}

#[derive(Debug)]
pub struct PerformanceReport {
    pub name: String,
    pub total_time: f64,
    pub measurements: HashMap<String, f64>,
}

impl PerformanceReport {
    pub fn log(&self) {
        web_sys::console::log_2(
            &format!("Performance Report: {}", self.name).into(),
            &JsValue::from_serde(self).unwrap(),
        );
    }
}
```

## 🎨 Styling and Theming

### Theme Management

```rust
// src/styles/themes.rs
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub shadows: ShadowPalette,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub muted: String,
    pub border: String,
    pub destructive: String,
}

pub struct ThemeManager {
    current_theme: RwSignal<Theme>,
    available_themes: Vec<Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let default_theme = Self::get_default_theme();
        Self {
            current_theme: RwSignal::new(default_theme.clone()),
            available_themes: vec![default_theme],
        }
    }
    
    pub fn set_theme(&self, theme_name: &str) -> Result<(), String> {
        if let Some(theme) = self.available_themes.iter().find(|t| t.name == theme_name) {
            self.current_theme.set(theme.clone());
            self.apply_theme_to_document(theme);
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name))
        }
    }
    
    pub fn get_current_theme(&self) -> ReadSignal<Theme> {
        self.current_theme.read_only()
    }
    
    fn apply_theme_to_document(&self, theme: &Theme) {
        if let Some(document) = window().unwrap().document() {
            let root = document.document_element().unwrap();
            let style = root.style();
            
            // Apply CSS custom properties
            style.set_property("--primary", &theme.colors.primary).unwrap();
            style.set_property("--secondary", &theme.colors.secondary).unwrap();
            style.set_property("--accent", &theme.colors.accent).unwrap();
            style.set_property("--background", &theme.colors.background).unwrap();
            style.set_property("--foreground", &theme.colors.foreground).unwrap();
            style.set_property("--muted", &theme.colors.muted).unwrap();
            style.set_property("--border", &theme.colors.border).unwrap();
            style.set_property("--destructive", &theme.colors.destructive).unwrap();
        }
    }
    
    fn get_default_theme() -> Theme {
        Theme {
            name: "default".to_string(),
            colors: ColorPalette {
                primary: "#0f172a".to_string(),
                secondary: "#f1f5f9".to_string(),
                accent: "#3b82f6".to_string(),
                background: "#ffffff".to_string(),
                foreground: "#0f172a".to_string(),
                muted: "#f8fafc".to_string(),
                border: "#e2e8f0".to_string(),
                destructive: "#ef4444".to_string(),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            shadows: ShadowPalette::default(),
        }
    }
}
```

## 🧪 Testing Strategy

### WASM-Specific Test Suite

```rust
// tests/unit/button_test.rs
use wasm_bindgen_test::*;
use leptos_shadcn_ui_wasm::components::button::*;
use leptos::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_button_renders() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    
    // Create a test container
    let container = document.create_element("div").unwrap();
    container.set_id("test-container");
    body.append_child(&container).unwrap();
    
    // Render button component
    let button = Button {
        class: Some("test-button".to_string()),
        id: Some("test-button-id".to_string()),
        variant: Some(ButtonVariant::Default),
        size: Some(ButtonSize::Default),
        disabled: Some(false),
        loading: Some(false),
        on_click: None,
        children: move || view! { "Test Button" }.into_any(),
    };
    
    // Verify button exists
    let button_element = document.get_element_by_id("test-button-id");
    assert!(button_element.is_some());
    
    // Cleanup
    body.remove_child(&container).unwrap();
}

#[wasm_bindgen_test]
fn test_button_click_event() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    
    let container = document.create_element("div").unwrap();
    container.set_id("test-container");
    body.append_child(&container).unwrap();
    
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();
    
    let button = Button {
        class: None,
        id: Some("click-test-button".to_string()),
        variant: None,
        size: None,
        disabled: None,
        loading: None,
        on_click: Some(Callback::new(move |_| {
            *click_count_clone.borrow_mut() += 1;
        })),
        children: move || view! { "Click Me" }.into_any(),
    };
    
    // Simulate click
    let button_element = document.get_element_by_id("click-test-button").unwrap();
    let click_event = MouseEvent::new("click").unwrap();
    button_element.dispatch_event(&click_event).unwrap();
    
    // Verify click was handled
    assert_eq!(*click_count.borrow(), 1);
    
    // Cleanup
    body.remove_child(&container).unwrap();
}
```

## 📦 Bundle Optimization

### Build Configuration

```toml
# Cargo.toml - Build profiles
[profile.release]
# Optimize for size
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

# WASM-specific optimizations
[profile.release.package."*"]
opt-level = "z"
lto = true
codegen-units = 1
```

### Bundle Analysis

```rust
// src/utils/bundle_analyzer.rs
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct BundleAnalyzer;

impl BundleAnalyzer {
    /// Analyze current bundle size and performance
    pub fn analyze() -> BundleReport {
        let performance = window().unwrap().performance().unwrap();
        let navigation = performance.get_entries_by_type("navigation").get(0).unwrap();
        let navigation_timing = navigation.dyn_into::<PerformanceNavigationTiming>().unwrap();
        
        BundleReport {
            load_time: navigation_timing.load_event_end() - navigation_timing.load_event_start(),
            dom_content_loaded: navigation_timing.dom_content_loaded_event_end() - navigation_timing.dom_content_loaded_event_start(),
            first_paint: Self::get_first_paint_time(),
            memory_usage: Self::get_memory_usage(),
        }
    }
    
    fn get_first_paint_time() -> f64 {
        let performance = window().unwrap().performance().unwrap();
        let paint_entries = performance.get_entries_by_name("first-paint");
        if paint_entries.length() > 0 {
            paint_entries.get(0).unwrap().start_time()
        } else {
            0.0
        }
    }
    
    fn get_memory_usage() -> Option<f64> {
        // Memory API is not available in all browsers
        if let Ok(memory) = js_sys::Reflect::get(&window().unwrap(), &"memory".into()) {
            if let Ok(used) = js_sys::Reflect::get(&memory, &"usedJSHeapSize".into()) {
                return used.as_f64();
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleReport {
    pub load_time: f64,
    pub dom_content_loaded: f64,
    pub first_paint: f64,
    pub memory_usage: Option<f64>,
}
```

## 🚀 Performance Targets

### Bundle Size Targets

| Component Set | Target Size | Current Size | Status |
|---------------|-------------|--------------|--------|
| Essential (Button, Input, Label, Card) | < 50KB | TBD | 🟡 Pending |
| Extended (+ Dialog, Popover, Tooltip) | < 100KB | TBD | 🟡 Pending |
| Advanced (+ Table, Calendar, Form) | < 200KB | TBD | 🟡 Pending |
| All Components | < 500KB | TBD | 🟡 Pending |

### Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Initial Load Time | < 100ms | Performance API |
| First Paint | < 50ms | Performance API |
| Component Render Time | < 10ms | Custom timing |
| Memory Usage | < 10MB | Memory API |
| Bundle Parse Time | < 20ms | Performance API |

## 📋 Implementation Checklist

### Phase 1: Core Package Setup
- [ ] Create package structure
- [ ] Configure Cargo.toml with minimal dependencies
- [ ] Implement basic component architecture
- [ ] Set up WASM-specific utilities
- [ ] Create basic test suite

### Phase 2: Essential Components
- [ ] Implement Button component
- [ ] Implement Input component
- [ ] Implement Label component
- [ ] Implement Card component
- [ ] Add component tests
- [ ] Optimize bundle size

### Phase 3: Extended Components
- [ ] Implement Dialog component
- [ ] Implement Popover component
- [ ] Implement Tooltip component
- [ ] Add theme management
- [ ] Performance optimization

### Phase 4: Advanced Features
- [ ] Implement remaining components
- [ ] Add animation support
- [ ] Accessibility features
- [ ] Documentation and examples
- [ ] Performance benchmarking

## 🎯 Benefits

1. **Minimal Bundle Size:** Only WASM-compatible dependencies
2. **Optimized Performance:** WASM-specific optimizations
3. **Simplified Dependencies:** No conditional compilation complexity
4. **Faster Build Times:** Fewer dependencies to compile
5. **Better Tree Shaking:** Unused code elimination
6. **WASM-First Design:** Optimized for WebAssembly from the ground up

## ⚠️ Limitations

1. **No Native Support:** Cannot be used in native Rust applications
2. **Limited Testing:** No access to native testing frameworks
3. **File System Access:** No file system operations
4. **Threading Limitations:** Limited to single-threaded execution
5. **Memory Constraints:** Browser memory limitations

---

**Next Steps:**
1. Create the package structure
2. Implement essential components
3. Set up WASM-specific testing
4. Optimize bundle size and performance
5. Create documentation and examples
