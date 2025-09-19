# 🔧 API Standardization Plan

**Priority**: 🔴 **CRITICAL**  
**Timeline**: Week 1-2  
**Impact**: Ensures consistent, type-safe APIs across all components

## 🚨 Current API Inconsistencies

### **Type System Issues**
```rust
// Inconsistent prop types across components:
<Button variant="primary"/>           // Some components use &str
<Input placeholder="Enter text"/>     // Others use MaybeProp<String>

// Inconsistent callback patterns:
on_click=Some(callback)               // Some use Option<Callback<T>>
on_change=callback                    // Others use Callback<T> directly
```

### **Signal Management Inconsistencies**
```rust
// Mixed signal creation patterns:
let (value, set_value) = create_signal(initial);  // Deprecated
let (value, set_value) = signal(initial);         // Current
```

## 🎯 Standardization Strategy

### **Phase 1: Prop Type Standardization**

#### **1.1 Standardize String Props**
```rust
// Standard pattern for all string props:
#[component]
pub fn ComponentName(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    // Implementation
}

// Usage with helper macros:
<ComponentName 
    placeholder=prop_string!("Enter text")
    label=prop_string!("Label")
    class=prop_string!("custom-class")
/>
```

#### **1.2 Standardize Boolean Props**
```rust
// Standard pattern for all boolean props:
#[component]
pub fn ComponentName(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
) -> impl IntoView {
    // Implementation
}

// Usage with helper macros:
<ComponentName 
    disabled=prop_bool!(false)
    required=prop_bool!(true)
    checked=prop_bool!(false)
/>
```

#### **1.3 Standardize Numeric Props**
```rust
// Standard pattern for all numeric props:
#[component]
pub fn ComponentName(
    #[prop(into, optional)] min: MaybeProp<i32>,
    #[prop(into, optional)] max: MaybeProp<i32>,
    #[prop(into, optional)] step: MaybeProp<f64>,
) -> impl IntoView {
    // Implementation
}

// Usage:
<ComponentName 
    min=prop_number!(0)
    max=prop_number!(100)
    step=prop_number!(1.0)
/>
```

### **Phase 2: Callback Standardization**

#### **2.1 Standardize Callback Patterns**
```rust
// Standard pattern for all callbacks:
#[component]
pub fn ComponentName(
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_submit: Option<Callback<FormData>>,
) -> impl IntoView {
    // Implementation
}

// Usage:
<ComponentName 
    on_click=Some(callback)
    on_change=Some(change_callback)
    on_submit=Some(submit_callback)
/>
```

#### **2.2 Create Callback Helper Macros**
```rust
// Helper macros for common callback patterns:
#[macro_export]
macro_rules! callback {
    ($closure:expr) => {
        Some(Callback::new($closure))
    };
}

// Usage:
<ComponentName 
    on_click=callback!(|_| println!("Clicked"))
    on_change=callback!(|value| println!("Changed: {}", value))
/>
```

### **Phase 3: Signal Management Standardization**

#### **3.1 Standardize Signal Creation**
```rust
// Standard pattern for all signal creation:
use leptos::prelude::*;

// Always use signal() instead of create_signal():
let (value, set_value) = signal(initial_value);
let (is_loading, set_is_loading) = signal(false);
let (items, set_items) = signal(Vec::<String>::new());
```

#### **3.2 Create Signal Helper Functions**
```rust
// Helper functions for common signal patterns:
pub fn create_string_signal(initial: &str) -> (Signal<String>, WriteSignal<String>) {
    signal(initial.to_string())
}

pub fn create_bool_signal(initial: bool) -> (Signal<bool>, WriteSignal<bool>) {
    signal(initial)
}

pub fn create_vec_signal<T>(initial: Vec<T>) -> (Signal<Vec<T>>, WriteSignal<Vec<T>>)
where
    T: Clone + 'static,
{
    signal(initial)
}
```

## 📋 Implementation Checklist

### **Week 1: Core Standardization**

#### **Day 1-2: Prop Type Standardization**
- [ ] Create prop helper macros (`prop_string!`, `prop_bool!`, `prop_number!`)
- [ ] Update all component prop definitions
- [ ] Standardize string prop usage across components
- [ ] Test prop type consistency

#### **Day 3-4: Callback Standardization**
- [ ] Create callback helper macros
- [ ] Standardize callback patterns across components
- [ ] Update all callback prop definitions
- [ ] Test callback consistency

#### **Day 5: Signal Management**
- [ ] Replace all `create_signal` with `signal()`
- [ ] Create signal helper functions
- [ ] Update all signal creation patterns
- [ ] Test signal management consistency

### **Week 2: Component Updates**

#### **Day 6-7: Core Components**
- [ ] Update button component API
- [ ] Update input component API
- [ ] Update card component API
- [ ] Test core component consistency

#### **Day 8-9: Form Components**
- [ ] Update form component API
- [ ] Update select component API
- [ ] Update checkbox component API
- [ ] Test form component consistency

#### **Day 10: Advanced Components**
- [ ] Update dialog component API
- [ ] Update popover component API
- [ ] Update tooltip component API
- [ ] Test advanced component consistency

## 🧪 Testing Strategy

### **API Consistency Tests**
```rust
#[cfg(test)]
mod api_consistency_tests {
    use super::*;
    
    #[test]
    fn test_prop_type_consistency() {
        // Test that all components use consistent prop types
        // Verify MaybeProp<T> usage for optional props
        // Check string literal conversion
    }
    
    #[test]
    fn test_callback_consistency() {
        // Test that all components use consistent callback patterns
        // Verify Option<Callback<T>> usage
        // Check callback creation patterns
    }
    
    #[test]
    fn test_signal_consistency() {
        // Test that all components use signal() instead of create_signal()
        // Verify signal creation patterns
        // Check signal management consistency
    }
}
```

### **Integration Tests**
```rust
#[test]
fn test_component_api_integration() {
    // Test that components work together with standardized APIs
    // Verify prop passing between components
    // Check callback communication
}
```

## 📊 Success Criteria

- ✅ **Consistent prop types** across all components
- ✅ **Standardized callback patterns** throughout codebase
- ✅ **Unified signal management** using `signal()` only
- ✅ **Helper macros** for common patterns
- ✅ **Comprehensive test coverage** for API consistency

## 🚨 Risk Mitigation

### **Backward Compatibility**
- Maintain existing component functionality
- Provide migration guide for API changes
- Test all existing usage patterns

### **Type Safety**
- Ensure all prop types are properly validated
- Test type conversion edge cases
- Verify compile-time type checking

### **Performance**
- Ensure helper macros don't impact performance
- Test signal management efficiency
- Verify callback performance

## 📁 Related Documents

- [Build System Remediation](./build-system-remediation.md) - Fix compilation issues
- [Command Component Fix](./component-fixes/command-component-fix.md) - Example implementation
- [Component API Guidelines](./component-api-guidelines.md) - Detailed API standards

---

**Next Steps**: After completing API standardization, proceed to [Stub Implementation Plan](./stub-implementation.md) for completing todo! items.
