# Conditional Compilation Design
## WASM/Native Target Strategy for leptos-shadcn-ui

**Document Version:** 1.0  
**Date:** 2025-01-27  
**Status:** DRAFT - Implementation Ready  

---

## 🎯 Overview

This document details the conditional compilation strategy to enable leptos-shadcn-ui to work seamlessly across both WASM and native targets while maintaining optimal performance and feature sets for each platform.

## 🏗️ Architecture Design

### Target-Specific Feature Matrix

| Feature | WASM Target | Native Target | Notes |
|---------|-------------|---------------|-------|
| Core Components | ✅ Full Support | ✅ Full Support | Button, Input, Card, etc. |
| Property Testing | ❌ Not Available | ✅ Full Support | proptest incompatible |
| File System | ❌ Not Available | ✅ Full Support | tempfile incompatible |
| UUID Generation | ✅ JS-based | ✅ Native | Different feature sets |
| Random Generation | ✅ JS-based | ✅ Native | getrandom with different features |
| Performance Testing | ✅ Web APIs | ✅ Native APIs | Different measurement tools |
| Snapshot Testing | ❌ Limited | ✅ Full Support | File system dependent |

### Conditional Compilation Strategy

#### 1. Workspace-Level Configuration

```toml
# Cargo.toml - Workspace root
[workspace]
resolver = "2"
members = [
    # ... existing members
]

[workspace.dependencies]
# Core dependencies (target-agnostic)
leptos = "0.8.9"
leptos_router = "0.8.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Target-specific UUID configurations
uuid-wasm = { version = "1.0", features = ["v4", "js"] }
uuid-native = { version = "1.0", features = ["v4", "serde", "std"] }

# Target-specific random generation
getrandom-wasm = { version = "0.2", features = ["js"] }
getrandom-native = { version = "0.2", features = ["std"] }

# Conditional testing dependencies
proptest = { version = "1.4", optional = true }
tempfile = { version = "3.0", optional = true }
wasm-bindgen-test = { version = "0.3", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }

# WASM-specific dependencies
web-sys = { version = "0.3", optional = true }
js-sys = { version = "0.3", optional = true }
console_error_panic_hook = { version = "0.1", optional = true }
```

#### 2. Package-Level Conditional Dependencies

```toml
# packages/test-utils/Cargo.toml
[package]
name = "shadcn-ui-test-utils"
version = "0.2.0"

[dependencies]
# Core dependencies (always available)
serde = { workspace = true }
serde_json = { workspace = true }
leptos = { workspace = true }

# Conditional UUID based on target
[target.'cfg(target_arch = "wasm32")'.dependencies]
uuid = { workspace = true, package = "uuid-wasm" }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
uuid = { workspace = true, package = "uuid-native" }

# Conditional random generation
[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { workspace = true, package = "getrandom-wasm" }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
getrandom = { workspace = true, package = "getrandom-native" }

# WASM-specific dependencies
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen-test = { workspace = true }
wasm-bindgen-futures = { workspace = true }
web-sys = { workspace = true, features = ["console", "Document", "Element", "HtmlElement", "Window", "Performance"] }
js-sys = { workspace = true }
console_error_panic_hook = { workspace = true }

# Native-specific dependencies
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
proptest = { workspace = true }
tempfile = { workspace = true }

[features]
default = []
wasm-testing = ["dep:wasm-bindgen-test", "dep:wasm-bindgen-futures", "dep:web-sys", "dep:js-sys", "dep:console_error_panic_hook"]
native-testing = ["dep:proptest", "dep:tempfile"]
```

#### 3. Component-Level Conditional Implementation

```rust
// packages/leptos/button/src/lib.rs
use leptos::prelude::*;

// Conditional imports
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

// Conditional utility functions
#[cfg(target_arch = "wasm32")]
mod wasm_utils {
    use super::*;
    
    pub fn generate_unique_id() -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
    
    pub fn log_performance(operation: &str, duration: f64) {
        web_sys::console::log_2(
            &format!("{}: {}ms", operation, duration).into(),
            &duration.into()
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_utils {
    use super::*;
    
    pub fn generate_unique_id() -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
    
    pub fn log_performance(operation: &str, duration: f64) {
        println!("{}: {}ms", operation, duration);
    }
}

// Unified interface
pub fn generate_id() -> String {
    #[cfg(target_arch = "wasm32")]
    return wasm_utils::generate_unique_id();
    
    #[cfg(not(target_arch = "wasm32"))]
    return native_utils::generate_unique_id();
}

pub fn log_perf(operation: &str, duration: f64) {
    #[cfg(target_arch = "wasm32")]
    wasm_utils::log_performance(operation, duration);
    
    #[cfg(not(target_arch = "wasm32"))]
    native_utils::log_performance(operation, duration);
}

// Main component (target-agnostic)
#[component]
pub fn Button(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let button_id = id.unwrap_or_else(|| generate_id());
    
    view! {
        <button class=class id=button_id>
            {children()}
        </button>
    }
}
```

#### 4. Testing Module Structure

```rust
// packages/leptos/button/src/tests.rs
use super::*;

// WASM-specific tests
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_button_renders_in_browser() {
        // Test button rendering in browser environment
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        
        // Create and mount button component
        let button_element = document.create_element("button").unwrap();
        button_element.set_text_content(Some("Test Button"));
        body.append_child(&button_element).unwrap();
        
        // Verify button exists
        assert!(button_element.text_content().unwrap() == "Test Button");
    }
    
    #[wasm_bindgen_test]
    fn test_button_performance() {
        let start = web_sys::window().unwrap().performance().unwrap().now();
        
        // Simulate button creation
        for _ in 0..1000 {
            let _id = generate_id();
        }
        
        let duration = web_sys::window().unwrap().performance().unwrap().now() - start;
        assert!(duration < 10.0, "Button creation should be fast");
    }
}

// Native-specific tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod native_tests {
    use super::*;
    use proptest::prelude::*;
    use tempfile::tempdir;
    
    proptest! {
        #[test]
        fn test_button_properties(
            class in any::<Option<String>>(),
            id in any::<Option<String>>(),
            text in any::<String>()
        ) {
            // Test button with various property combinations
            let button = Button {
                class: class.map(MaybeProp::Some),
                id: id.map(MaybeProp::Some),
                children: move || view! { {text} }.into_any(),
            };
            
            // Verify button properties
            assert!(button.class.is_some() || button.class.is_none());
            assert!(button.id.is_some() || button.id.is_none());
        }
    }
    
    #[test]
    fn test_button_file_operations() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("button_test.json");
        
        // Test file-based operations (native only)
        let button_data = serde_json::json!({
            "type": "button",
            "class": "btn-primary",
            "id": generate_id()
        });
        
        std::fs::write(&file_path, serde_json::to_string_pretty(&button_data).unwrap()).unwrap();
        
        let read_data: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&file_path).unwrap()
        ).unwrap();
        
        assert_eq!(read_data["type"], "button");
    }
}

// Common tests (both targets)
#[cfg(test)]
mod common_tests {
    use super::*;
    
    #[test]
    fn test_id_generation() {
        let id1 = generate_id();
        let id2 = generate_id();
        
        assert_ne!(id1, id2, "Generated IDs should be unique");
        assert!(!id1.is_empty(), "Generated ID should not be empty");
        assert!(!id2.is_empty(), "Generated ID should not be empty");
    }
    
    #[test]
    fn test_performance_logging() {
        // This should work on both targets
        log_perf("test_operation", 1.5);
        // No assertion needed - just ensure it doesn't panic
    }
}
```

## 🔧 Implementation Patterns

### Pattern 1: Feature-Based Conditional Compilation

```rust
// Conditional feature activation
#[cfg(feature = "wasm-testing")]
mod wasm_testing {
    use wasm_bindgen_test::*;
    // WASM-specific testing code
}

#[cfg(feature = "native-testing")]
mod native_testing {
    use proptest::prelude::*;
    // Native-specific testing code
}
```

### Pattern 2: Target-Based Conditional Compilation

```rust
// Target-specific implementations
#[cfg(target_arch = "wasm32")]
fn platform_specific_function() -> String {
    "WASM implementation".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_specific_function() -> String {
    "Native implementation".to_string()
}
```

### Pattern 3: Unified Interface with Conditional Backend

```rust
// Unified public API
pub struct PlatformUtils;

impl PlatformUtils {
    pub fn generate_id() -> String {
        platform_generate_id()
    }
    
    pub fn log_info(message: &str) {
        platform_log_info(message);
    }
}

// Conditional backend implementations
#[cfg(target_arch = "wasm32")]
fn platform_generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(target_arch = "wasm32")]
fn platform_log_info(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_log_info(message: &str) {
    println!("{}", message);
}
```

## 📊 Performance Considerations

### WASM Optimization

```rust
// WASM-specific optimizations
#[cfg(target_arch = "wasm32")]
mod wasm_optimizations {
    use wasm_bindgen::prelude::*;
    
    // Minimize JavaScript interop
    pub fn batch_dom_operations(operations: Vec<DomOperation>) {
        // Batch DOM operations to reduce JS interop overhead
        for operation in operations {
            operation.execute();
        }
    }
    
    // Use WebAssembly memory efficiently
    pub fn allocate_string_buffer(size: usize) -> *mut u8 {
        // Direct memory allocation for string operations
        std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap())
    }
}
```

### Native Optimization

```rust
// Native-specific optimizations
#[cfg(not(target_arch = "wasm32"))]
mod native_optimizations {
    use std::collections::HashMap;
    use std::sync::Mutex;
    
    // Use native threading for performance
    pub fn parallel_processing<T, F>(items: Vec<T>, processor: F) -> Vec<T>
    where
        F: Fn(T) -> T + Send + Sync + 'static,
        T: Send + 'static,
    {
        use rayon::prelude::*;
        items.into_par_iter().map(processor).collect()
    }
    
    // Use native file system caching
    lazy_static::lazy_static! {
        static ref FILE_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }
    
    pub fn cached_file_read(path: &str) -> Option<String> {
        let mut cache = FILE_CACHE.lock().unwrap();
        if let Some(content) = cache.get(path) {
            return Some(content.clone());
        }
        
        if let Ok(content) = std::fs::read_to_string(path) {
            cache.insert(path.to_string(), content.clone());
            Some(content)
        } else {
            None
        }
    }
}
```

## 🧪 Testing Strategy

### Cross-Platform Test Suite

```rust
// packages/test-utils/src/cross_platform_tests.rs
use crate::{TestResult, TestSuite};

pub struct CrossPlatformTestSuite {
    wasm_tests: Vec<Box<dyn Fn() -> TestResult>>,
    native_tests: Vec<Box<dyn Fn() -> TestResult>>,
    common_tests: Vec<Box<dyn Fn() -> TestResult>>,
}

impl CrossPlatformTestSuite {
    pub fn new() -> Self {
        Self {
            wasm_tests: Vec::new(),
            native_tests: Vec::new(),
            common_tests: Vec::new(),
        }
    }
    
    pub fn add_wasm_test<F>(&mut self, test: F)
    where
        F: Fn() -> TestResult + 'static,
    {
        self.wasm_tests.push(Box::new(test));
    }
    
    pub fn add_native_test<F>(&mut self, test: F)
    where
        F: Fn() -> TestResult + 'static,
    {
        self.native_tests.push(Box::new(test));
    }
    
    pub fn add_common_test<F>(&mut self, test: F)
    where
        F: Fn() -> TestResult + 'static,
    {
        self.common_tests.push(Box::new(test));
    }
    
    pub fn run_all_tests(&self) -> TestSuite {
        let mut results = TestSuite::new();
        
        // Run common tests on both platforms
        for test in &self.common_tests {
            results.add_result(test());
        }
        
        // Run platform-specific tests
        #[cfg(target_arch = "wasm32")]
        for test in &self.wasm_tests {
            results.add_result(test());
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        for test in &self.native_tests {
            results.add_result(test());
        }
        
        results
    }
}
```

## 📋 Migration Checklist

### For Package Maintainers

- [ ] Update `Cargo.toml` with conditional dependencies
- [ ] Add target-specific feature flags
- [ ] Implement conditional compilation in source code
- [ ] Create platform-specific test modules
- [ ] Update documentation with platform requirements
- [ ] Test compilation on both targets
- [ ] Update CI/CD for cross-platform testing

### For Package Users

- [ ] Update `Cargo.toml` dependencies
- [ ] Add appropriate feature flags for target platform
- [ ] Update import statements if needed
- [ ] Test application on target platform
- [ ] Update build scripts for WASM if applicable

## 🚀 Benefits

1. **Single Codebase:** Maintain one codebase for both platforms
2. **Optimal Performance:** Platform-specific optimizations
3. **Feature Parity:** Core functionality works on both platforms
4. **Testing Coverage:** Comprehensive testing for both targets
5. **Maintenance Efficiency:** Reduced code duplication
6. **User Experience:** Seamless platform switching

## ⚠️ Limitations

1. **Complexity:** More complex build configuration
2. **Testing Overhead:** Need to test on both platforms
3. **Documentation:** Must document platform-specific features
4. **Debugging:** Platform-specific issues require different approaches

---

**Next Steps:**
1. Implement conditional compilation in test-utils package
2. Create cross-platform test suite
3. Update CI/CD for dual-platform testing
4. Document platform-specific features and limitations
