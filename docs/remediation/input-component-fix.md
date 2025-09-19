# 🔧 **Input Component Fix**

## **Critical Issues Identified**

### **Compilation Errors (73+)**
- **Non-existent Properties**: Tests use properties that don't exist in the actual Input component
- **Type Mismatches**: Style signal type incompatibilities
- **API Mismatches**: Tests written against hypothetical APIs

### **Root Cause Analysis**
The TDD test refactoring created tests for properties that were never implemented:
1. **Size/Variant Properties**: Not implemented in Input component
2. **Validation Properties**: Different API than expected
3. **Style Properties**: Type mismatches with `leptos_style::Style`

## **Fix Strategy**

### **Phase 1: Fix API Mismatches**

#### **1.1 Remove Non-existent Properties**
```rust
// REMOVE these non-existent properties from tests:
size=size                    // ❌ No such property
variant=variant              // ❌ No such property  
name="custom-name"           // ❌ No such property
animate=true                 // ❌ No such property
required=true                // ❌ No such property
validation=ValidationRule    // ❌ No such property
error="error message"        // ❌ No such property
```

#### **1.2 Fix Style Type Issues**
```rust
// BEFORE (BROKEN):
style="background-color: #f0f0f0; border: 2px solid #ccc;"

// AFTER (FIXED):
style=leptos_style::Style::new()
    .background_color("#f0f0f0")
    .border("2px solid #ccc")
```

#### **1.3 Align with Actual Input API**
```rust
// ACTUAL Input component properties:
#[component]
pub fn Input(
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] input_type: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] style: Option<Signal<Style>>,
    #[prop(into, optional)] validator: Option<InputValidator>,
    #[prop(into, optional)] validation_error: Option<Signal<Option<String>>>,
    #[prop(into, optional)] show_validation: Option<bool>,
) -> impl IntoView
```

### **Phase 2: Restructure Test Modules**

#### **2.1 Fix Test Module Structure**
```rust
// Update test modules to match actual API:
pub mod basic_rendering_tests {
    // Test actual properties: value, placeholder, disabled, input_type, class, id
}

pub mod validation_tests {
    // Test actual validation: validator, validation_error, show_validation
}

pub mod styling_tests {
    // Test actual styling: class, style, conditional styling
}

pub mod accessibility_tests {
    // Test accessibility: id, class, ARIA attributes
}
```

#### **2.2 Fix Import Issues**
```rust
// Add missing imports:
use leptos::prelude::*;
use leptos_style::Style;
use crate::default::{Input, InputValidator};
```

### **Phase 3: Implement Missing Features**

#### **3.1 Add Size Support (Optional)**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Sm,
    Default,
    Lg,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Default
    }
}
```

#### **3.2 Add Variant Support (Optional)**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Destructive,
    Outline,
}

impl Default for InputVariant {
    fn default() -> Self {
        Self::Default
    }
}
```

#### **3.3 Enhance Validation API**
```rust
#[derive(Debug, Clone)]
pub struct InputValidator {
    pub required: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub custom_validator: Option<Box<dyn Fn(&str) -> bool>>,
}
```

## **Implementation Plan**

### **Week 1: Critical Fixes**
- [ ] Remove all non-existent property tests
- [ ] Fix style type mismatches
- [ ] Align tests with actual Input API
- [ ] Fix import statements

### **Week 2: Feature Implementation**
- [ ] Implement size support (optional)
- [ ] Implement variant support (optional)
- [ ] Enhance validation API
- [ ] Add proper error handling

### **Week 3: Testing & Validation**
- [ ] Run full test suite
- [ ] Add integration tests
- [ ] Performance testing
- [ ] Documentation updates

## **Success Criteria**

### **Compilation**
- [ ] `cargo check` passes without errors
- [ ] `cargo test` runs successfully
- [ ] All test modules compile

### **Functionality**
- [ ] Input component works with actual API
- [ ] Validation works correctly
- [ ] Styling works as expected
- [ ] Accessibility features work

### **Testing**
- [ ] All tests pass
- [ ] Test coverage is accurate
- [ ] Integration tests work
- [ ] Performance benchmarks pass

## **Risk Mitigation**

### **High Risk**
- **API Changes**: Ensure backward compatibility
- **Validation**: Ensure validation works correctly
- **Styling**: Ensure styling doesn't break

### **Medium Risk**
- **Test Coverage**: Maintain comprehensive test coverage
- **Documentation**: Keep documentation up to date

### **Low Risk**
- **Import Issues**: Standardize import patterns
- **Code Style**: Maintain consistent code style

## **Files to Fix**

### **Critical Files**
1. `packages/leptos/input/src/tdd_tests/basic_rendering_tests.rs`
2. `packages/leptos/input/src/tdd_tests/validation_tests.rs`
3. `packages/leptos/input/src/tdd_tests/styling_tests.rs`
4. `packages/leptos/input/src/tdd_tests/accessibility_tests.rs`

### **Supporting Files**
1. `packages/leptos/input/src/default.rs`
2. `packages/leptos/input/src/lib.rs`
3. `packages/leptos/input/src/implementation_tests/mod.rs`

---

**Priority**: 🔴 **P0 - CRITICAL**
**Estimated Effort**: 3 weeks
**Dependencies**: None
