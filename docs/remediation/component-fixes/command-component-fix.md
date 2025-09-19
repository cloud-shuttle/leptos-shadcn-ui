# 🔧 Command Component Fix Design

**Component**: `leptos-shadcn-command`  
**Priority**: 🔴 **CRITICAL**  
**Issues**: 68 compilation errors, type mismatches  
**Timeline**: 2-3 days

## 🚨 Critical Issues

### **Type Mismatch Errors (68 total)**
```rust
// Error pattern 1: String literal vs MaybeProp<String>
<CommandInput placeholder="Search..."/>  // ❌ &str
// Expected:
<CommandInput placeholder="Search...".into()/>  // ✅ MaybeProp<String>

// Error pattern 2: Boolean literal vs MaybeProp<bool>
<CommandItem disabled=true>"Disabled Item"</CommandItem>  // ❌ bool
// Expected:
<CommandItem disabled=true.into()>"Disabled Item"</CommandItem>  // ✅ MaybeProp<bool>

// Error pattern 3: Option<Callback> vs Callback
on_value_change=Some(callback)  // ❌ Option<Callback<String>>
// Expected:
on_value_change=callback  // ✅ Callback<String>
```

## 🎯 Fix Strategy

### **Phase 1: Prop Type Standardization**

#### **1.1 String Props Fix**
```rust
// Create helper macro for string literals
macro_rules! prop_string {
    ($value:literal) => {
        $value.into()
    };
}

// Usage in tests:
<CommandInput placeholder=prop_string!("Search...")/>
<CommandGroup heading=prop_string!("Suggestions")>
```

#### **1.2 Boolean Props Fix**
```rust
// Create helper macro for boolean literals
macro_rules! prop_bool {
    ($value:literal) => {
        $value.into()
    };
}

// Usage in tests:
<CommandItem disabled=prop_bool!(true)>
```

#### **1.3 Callback Props Fix**
```rust
// Remove Option wrapper where not needed
// Before:
on_value_change=Some(callback)
// After:
on_value_change=callback
```

### **Phase 2: Test Case Updates**

#### **2.1 Update All Test Cases**
```rust
// File: packages/leptos/command/src/tdd_tests.rs
// Update all 68 error locations with proper type conversions

// Example fix:
#[test]
fn test_command_basic_functionality() {
    view! {
        <Command on_value_change=callback>
            <CommandInput placeholder=prop_string!("Search...")/>
            <CommandGroup heading=prop_string!("Suggestions")>
                <CommandItem disabled=prop_bool!(false)>"Item 1"</CommandItem>
            </CommandGroup>
        </Command>
    }
}
```

#### **2.2 Create Type-Safe Test Helpers**
```rust
// Add to test module:
mod test_helpers {
    use leptos::prelude::*;
    
    pub fn string_prop(s: &str) -> MaybeProp<String> {
        s.into()
    }
    
    pub fn bool_prop(b: bool) -> MaybeProp<bool> {
        b.into()
    }
    
    pub fn callback_prop<T, F>(f: F) -> Callback<T>
    where
        F: Fn(T) + 'static,
    {
        Callback::new(f)
    }
}
```

### **Phase 3: Component API Review**

#### **3.1 Verify Component Definitions**
```rust
// Check component prop definitions in:
// packages/leptos/command/src/default.rs
// packages/leptos/command/src/new_york.rs

// Ensure all props use MaybeProp<T> consistently:
#[component]
pub fn CommandInput(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    // ... other props
) -> impl IntoView {
    // Implementation
}
```

#### **3.2 Update Component Documentation**
```rust
// Add prop type documentation:
/// Command input component with type-safe props
/// 
/// # Props
/// - `placeholder`: MaybeProp<String> - Input placeholder text
/// - `disabled`: MaybeProp<bool> - Whether input is disabled
/// - `on_value_change`: Callback<String> - Value change callback
```

## 📋 Implementation Steps

### **Step 1: Create Helper Macros (Day 1)**
```rust
// Add to packages/leptos/command/src/lib.rs
#[macro_export]
macro_rules! prop_string {
    ($value:literal) => {
        $value.into()
    };
}

#[macro_export]
macro_rules! prop_bool {
    ($value:literal) => {
        $value.into()
    };
}
```

### **Step 2: Fix Test Cases (Day 2)**
```bash
# Fix all 68 errors in tdd_tests.rs
# Use find/replace with regex patterns:
# Find: placeholder="([^"]*)"
# Replace: placeholder=prop_string!("$1")

# Find: disabled=([^>]*)
# Replace: disabled=prop_bool!($1)
```

### **Step 3: Verify and Test (Day 3)**
```bash
# Test compilation:
cargo check --package leptos-shadcn-command

# Run tests:
cargo test --package leptos-shadcn-command

# Verify functionality:
cargo test --package leptos-shadcn-command --lib
```

## 🧪 Testing Strategy

### **Compilation Tests**
```bash
# Verify no compilation errors:
cargo check --package leptos-shadcn-command --lib
cargo check --package leptos-shadcn-command --tests
```

### **Functionality Tests**
```bash
# Run all command component tests:
cargo test --package leptos-shadcn-command --lib
cargo test --package leptos-shadcn-command --test tdd_tests
```

### **Integration Tests**
```bash
# Test command component in example app:
cargo run --example leptos-demo
```

## 📊 Success Criteria

- ✅ **Zero compilation errors** in command component
- ✅ **All 68 type mismatch errors resolved**
- ✅ **All tests passing** for command component
- ✅ **Type-safe prop usage** throughout
- ✅ **Consistent API patterns** with other components

## 🚨 Risk Mitigation

### **Backup Strategy**
```bash
# Create backup branch:
git checkout -b fix/command-component-types
git add -A && git commit -m "Backup before command component fixes"
```

### **Incremental Testing**
- Fix 10-15 errors at a time
- Test compilation after each batch
- Commit working fixes immediately

### **Reference Implementation**
- Use button component as reference for correct patterns
- Compare prop definitions with working components
- Maintain consistency with established patterns

## 📁 Related Files

- `packages/leptos/command/src/tdd_tests.rs` - Main file with 68 errors
- `packages/leptos/command/src/default.rs` - Component implementation
- `packages/leptos/command/src/new_york.rs` - Alternative implementation
- `packages/leptos/button/src/` - Reference implementation

---

**Next Steps**: After fixing command component, proceed to [Tailwind Core Fix](./tailwind-core-fix.md) for remaining compilation issues.
