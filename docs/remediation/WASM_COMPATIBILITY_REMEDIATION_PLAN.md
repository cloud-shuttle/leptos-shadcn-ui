# WASM Compatibility Remediation Plan
## leptos-shadcn-ui v0.9.0+ WASM Support Strategy

**Document Version:** 1.0  
**Date:** 2025-01-27  
**Status:** DRAFT - Implementation Ready  

---

## 🎯 Executive Summary

This document outlines a comprehensive remediation plan to resolve WASM compatibility issues in leptos-shadcn-ui, specifically addressing dependency conflicts with `mio`, `uuid`, and `rustc-serialize` crates. The plan provides three strategic approaches to ensure full WASM support while maintaining backward compatibility.

## 🔍 Problem Analysis

### Current Issues Identified

1. **Dependency Conflicts:**
   - `packages/test-utils/Cargo.toml`: `uuid` missing `"js"` feature
   - `proptest` and `tempfile` dependencies not WASM-compatible
   - Mixed WASM/non-WASM dependencies in utility packages

2. **Architecture Gaps:**
   - No conditional compilation for WASM targets
   - Testing utilities assume file system access
   - Property-based testing framework incompatible with WASM

3. **Existing WASM Support:**
   - ✅ Core components (button, card, input) are WASM-compatible
   - ✅ Working WASM demos exist (`standalone-demo/`, `examples/leptos/`)
   - ✅ Proper WASM dependencies in demo configurations

## 🚀 Strategic Approaches

### Approach 1: Fix Existing test-utils Package (Recommended)

**Priority:** HIGH  
**Effort:** MEDIUM  
**Risk:** LOW  

#### Implementation Strategy

```toml
# packages/test-utils/Cargo.toml - WASM-Compatible Version
[package]
name = "shadcn-ui-test-utils"
version = "0.2.0"

[dependencies]
# WASM-compatible core dependencies
wasm-bindgen-test = "0.3"
web-sys = { workspace = true, features = ["console", "Document", "Element", "HtmlElement", "Window", "Performance", "PerformanceTiming"] }
js-sys = "0.3"
console_error_panic_hook = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

# ✅ FIXED: Add "js" feature for WASM compatibility
uuid = { version = "1.0", features = ["v4", "js"] }

# Framework-specific testing
leptos = { workspace = true }

# ❌ REMOVED: Non-WASM compatible dependencies
# proptest = "1.4"  # Not WASM-compatible
# tempfile = "3.0"  # File system operations not available in WASM

# ✅ ADDED: WASM-compatible alternatives
getrandom = { version = "0.2", features = ["js"] }

[features]
default = ["wasm-testing"]
wasm-testing = []
native-testing = []

# Conditional dependencies for different targets
[target.'cfg(target_arch = "wasm32")'.dependencies]
# WASM-specific testing utilities
wasm-bindgen-futures = "0.4"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
# Native-only testing utilities
proptest = "1.4"
tempfile = "3.0"
```

#### Code Changes Required

1. **Update `packages/test-utils/src/dom_testing.rs`:**
```rust
// Add conditional compilation for WASM vs native testing
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
use std::io::Write;

impl ComponentTestHarness {
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        // WASM-compatible UUID generation
        let mount_id = format!("test-mount-{}", uuid::Uuid::new_v4().to_string());
        Self { mount_point: mount_id }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        // Native UUID generation with additional features
        let mount_id = format!("test-mount-{}", uuid::Uuid::new_v4().to_string());
        Self { mount_point: mount_id }
    }
}
```

2. **Create WASM-compatible property testing:**
```rust
// packages/test-utils/src/wasm_property_testing.rs
#[cfg(target_arch = "wasm32")]
pub mod wasm_property_testing {
    use wasm_bindgen_test::*;
    
    /// WASM-compatible property testing using JavaScript
    pub fn wasm_proptest<F>(test_fn: F) 
    where
        F: Fn() + 'static,
    {
        wasm_bindgen_test_configure!(run_in_browser);
        test_fn();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native_property_testing {
    use proptest::prelude::*;
    
    /// Native property testing using proptest
    pub fn native_proptest<F>(test_fn: F) 
    where
        F: Fn() + 'static,
    {
        proptest! {
            test_fn();
        }
    }
}
```

### Approach 2: Create WASM-Only Minimal Version

**Priority:** MEDIUM  
**Effort:** HIGH  
**Risk:** MEDIUM  

#### Package Structure

```
packages/
├── leptos-shadcn-ui-wasm/          # New WASM-only package
│   ├── Cargo.toml                  # Minimal WASM dependencies
│   ├── src/
│   │   ├── lib.rs                  # Re-export core components
│   │   ├── components/             # WASM-compatible components only
│   │   └── utils/                  # WASM-specific utilities
│   └── README.md
```

#### Cargo.toml Design

```toml
[package]
name = "leptos-shadcn-ui-wasm"
version = "0.9.0"
edition = "2021"
description = "WASM-only version of leptos-shadcn-ui with minimal dependencies"

[dependencies]
# Core Leptos (WASM-compatible)
leptos = { version = "0.8", features = ["csr"] }
leptos_router = "0.8"
leptos-node-ref = "0.2"
leptos-struct-component = "0.2"
leptos-style = "0.2"

# WASM-specific dependencies only
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
console_error_panic_hook = "0.1"
getrandom = { version = "0.2", features = ["js"] }
uuid = { version = "1.0", features = ["v4", "js"] }

# Essential components only (no testing utilities)
leptos-shadcn-button = { version = "0.9.0", path = "../leptos/button" }
leptos-shadcn-input = { version = "0.9.0", path = "../leptos/input" }
leptos-shadcn-card = { version = "0.9.0", path = "../leptos/card" }
leptos-shadcn-label = { version = "0.9.0", path = "../leptos/label" }
# ... other core components

[features]
default = ["essential-components"]
essential-components = ["button", "input", "card", "label"]
extended-components = ["essential-components", "dialog", "popover", "tooltip"]

# No testing features - testing handled separately
```

### Approach 3: Conditional Compilation Strategy

**Priority:** HIGH  
**Effort:** HIGH  
**Risk:** LOW  

#### Workspace-Level Configuration

```toml
# Cargo.toml - Workspace level
[workspace]
resolver = "2"
members = [
    # ... existing members
    "packages/leptos-shadcn-ui-wasm",  # New WASM package
]

[workspace.dependencies]
# WASM-compatible versions
uuid-wasm = { version = "1.0", features = ["v4", "js"] }
uuid-native = { version = "1.0", features = ["v4", "serde"] }
getrandom-wasm = { version = "0.2", features = ["js"] }
getrandom-native = { version = "0.2", features = ["std"] }

# Conditional testing dependencies
proptest = { version = "1.4", optional = true }
tempfile = { version = "3.0", optional = true }
```

#### Component-Level Conditional Compilation

```rust
// packages/leptos/button/src/lib.rs
use leptos::prelude::*;

// Conditional imports based on target
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;

#[component]
pub fn Button(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    // Component implementation works for both targets
    view! {
        <button class=class id=id>
            {children()}
        </button>
    }
}

// Conditional testing modules
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;
    use super::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_button_wasm() {
        // WASM-specific tests
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod native_tests {
    use proptest::prelude::*;
    use super::*;
    
    proptest! {
        #[test]
        fn test_button_properties(props in any::<ButtonProps>()) {
            // Native property-based tests
        }
    }
}
```

## 📋 Implementation Roadmap

### Phase 1: Immediate Fixes (Week 1)
- [ ] Fix `packages/test-utils/Cargo.toml` UUID dependency
- [ ] Add conditional compilation for WASM targets
- [ ] Update workspace dependencies
- [ ] Test WASM compilation

### Phase 2: Enhanced WASM Support (Week 2-3)
- [ ] Create WASM-compatible property testing utilities
- [ ] Implement conditional testing modules
- [ ] Add WASM-specific documentation
- [ ] Create WASM-only package (Approach 2)

### Phase 3: Full Integration (Week 4)
- [ ] Update CI/CD for WASM testing
- [ ] Create WASM-specific examples
- [ ] Performance optimization for WASM
- [ ] Documentation and release

## 🧪 Testing Strategy

### WASM Testing Framework

```rust
// packages/test-utils/src/wasm_testing.rs
use wasm_bindgen_test::*;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

pub struct WASMTestRunner {
    test_results: Vec<TestResult>,
}

impl WASMTestRunner {
    pub fn new() -> Self {
        Self {
            test_results: Vec::new(),
        }
    }
    
    pub fn run_component_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: FnOnce() -> bool,
    {
        let start = performance().unwrap().now();
        let result = test_fn();
        let duration = performance().unwrap().now() - start;
        
        self.test_results.push(TestResult {
            name: name.to_string(),
            passed: result,
            duration_ms: duration,
        });
    }
}

#[derive(Debug)]
struct TestResult {
    name: String,
    passed: bool,
    duration_ms: f64,
}
```

### CI/CD Integration

```yaml
# .github/workflows/wasm-tests.yml
name: WASM Compatibility Tests

on: [push, pull_request]

jobs:
  wasm-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust with WASM target
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
          
      - name: Test WASM compilation
        run: |
          cargo check --target wasm32-unknown-unknown
          wasm-pack test --headless --firefox
          
      - name: Test WASM demos
        run: |
          cd standalone-demo
          wasm-pack build --target web
```

## 📊 Success Metrics

### Technical Metrics
- [ ] 100% WASM compilation success rate
- [ ] < 2MB total WASM bundle size
- [ ] < 100ms component initialization time
- [ ] 0 WASM-specific runtime errors

### Quality Metrics
- [ ] All core components work in WASM
- [ ] WASM tests pass in all supported browsers
- [ ] Performance within 10% of native benchmarks
- [ ] Documentation coverage > 90%

## 🔧 Migration Guide

### For Existing Users

1. **Update Dependencies:**
```toml
# Before
leptos-shadcn-ui = "0.8.0"

# After (WASM-compatible)
leptos-shadcn-ui = "0.9.0"
# OR for WASM-only projects
leptos-shadcn-ui-wasm = "0.9.0"
```

2. **Update Cargo.toml:**
```toml
[dependencies]
# Add WASM-compatible features
uuid = { version = "1.0", features = ["v4", "js"] }
getrandom = { version = "0.2", features = ["js"] }
```

3. **Update Test Configuration:**
```rust
// Use conditional compilation in tests
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(not(target_arch = "wasm32"))]
use proptest::prelude::*;
```

## 🚨 Risk Assessment

### High Risk
- **Breaking Changes:** Conditional compilation may require code changes
- **Testing Coverage:** WASM testing infrastructure needs validation

### Medium Risk
- **Performance:** WASM bundle size optimization required
- **Browser Compatibility:** Need to test across all target browsers

### Low Risk
- **Dependency Conflicts:** Well-understood and documented
- **Backward Compatibility:** Native functionality preserved

## 📚 References

- [WebAssembly Rust Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Leptos WASM Documentation](https://leptos-rs.github.io/leptos/appendix_wasm.html)
- [UUID WASM Features](https://docs.rs/uuid/latest/uuid/features/index.html)

---

**Next Steps:**
1. Review and approve this remediation plan
2. Begin Phase 1 implementation
3. Set up WASM testing infrastructure
4. Create migration documentation for users
