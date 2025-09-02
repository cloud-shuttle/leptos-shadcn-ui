# Leptos 0.8.8 Migration Guide

## 🚨 **CRITICAL ISSUE IDENTIFIED**

The project is currently experiencing compilation failures with Leptos 0.8.8 due to **version inconsistencies** in the dependency tree, not due to fundamental issues with Leptos 0.8.8 itself.

## 🔍 **Root Cause Analysis**

### **Version Mismatch in Cargo.lock**
The `Cargo.lock` file contains mixed Leptos versions:
- **Main packages**: `leptos 0.8.8` ✅
- **Some dependencies**: `leptos_config 0.7.8` ❌ (incompatible)
- **Other dependencies**: `leptos_dom 0.8.6` ❌ (version mismatch)

### **Compilation Error Details**
```
error[E0308]: mismatched types
   --> /Users/peterhanssens/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/leptos-0.8.8/src/hydration/mod.rs:138:5
    |
138 | /     view! {
139 |         <link rel="modulepreload" href=format!("{root}/{pkg_path}/{js_file_name}.js") crossorigin...
140 |         <link
141 |             rel="preload"
...   |
149 |         </script>
150 |     }
    |_____^ expected a tuple with 3 elements, found one with 5 elements
```

**This error occurs because:**
1. Some packages are compiled against Leptos 0.7.x APIs
2. Other packages are compiled against Leptos 0.8.x APIs
3. The type system cannot reconcile these different API expectations

## 🚀 **IMPLEMENTATION PLAN**

### **Phase 1: Fix Version Inconsistencies (CRITICAL)**

#### **Step 1.1: Update Workspace Dependencies**
```toml
[workspace.dependencies]
# BEFORE (causing issues)
leptos = "0.8.6"
leptos_router = "0.8.6"

# AFTER (fixed)
leptos = "0.8.8"
leptos_router = "0.8.8"
```

#### **Step 1.2: Clean Dependency Resolution**
```bash
# Remove existing lock file to force fresh resolution
rm Cargo.lock

# Clean build artifacts
cargo clean

# Rebuild with fresh dependencies
cargo check --workspace
```

### **Phase 2: Fix Component Package Dependencies**

#### **Step 2.1: Update All Component Cargo.toml Files**
Ensure all `packages/leptos/*/Cargo.toml` use workspace versions:

```toml
# BEFORE (hardcoded versions)
leptos = "0.8"
leptos = "0.8.6"

# AFTER (workspace inheritance)
leptos.workspace = true
leptos_router.workspace = true
```

#### **Step 2.2: Fix Specific Component Issues**

##### **Error Boundary Component**
**Problem**: Closure implements `FnOnce` instead of `FnMut`
**Solution**: Clone `children` before moving into closure

```rust
// BEFORE (causes FnOnce error)
move || {
    if has_error.get() {
        // ... error handling
    } else {
        children().into_any()  // ❌ moves children
    }
}

// AFTER (fixes FnMut requirement)
{
    let children = children.clone();
    move || {
        if has_error.get() {
            // ... error handling
        } else {
            children().into_any()  // ✅ uses cloned reference
        }
    }
}
```

##### **Lazy Loading Component**
**Problem**: Type mismatches between `View<()>` and `impl IntoView`
**Solution**: Consistent return type handling

```rust
// BEFORE (type mismatch)
pub fn LazyComponent() -> View<()> {
    view! { <div>...</div> }
}

// AFTER (consistent types)
pub fn LazyComponent() -> impl IntoView {
    view! { <div>...</div> }
}
```

### **Phase 3: Update Example Application**

#### **Step 3.1: Fix Example Dependencies**
Update `examples/leptos/Cargo.toml`:

```toml
[dependencies]
# Use workspace versions
leptos.workspace = true
leptos_router.workspace = true

# Ensure all component dependencies use workspace versions
shadcn-ui-leptos-button = { path = "../../packages/leptos/button", optional = true }
# ... other components
```

#### **Step 3.2: Fix Import Issues**
```rust
// BEFORE (incorrect imports)
use leptos_shadcn_ui::button::Button;

// AFTER (correct imports)
use shadcn_ui_leptos_button::Button;
```

### **Phase 4: Test and Validate**

#### **Step 4.1: Compilation Verification**
```bash
# Check entire workspace
cargo check --workspace

# Build example application
cd examples/leptos
cargo build

# Run tests
cargo test
```

#### **Step 4.2: Runtime Testing**
```bash
# Start development server
cd examples/leptos
trunk serve

# Verify components render correctly
# Test interactive functionality
# Check browser console for errors
```

## 🛠️ **TROUBLESHOOTING CHECKLIST**

### **Before Starting**
- [ ] Rust toolchain is up to date (1.89.0+)
- [ ] Cargo is up to date (1.89.0+)
- [ ] All changes are committed to version control

### **During Implementation**
- [ ] Workspace dependencies updated to 0.8.8
- [ ] Cargo.lock removed and regenerated
- [ ] All component packages use `leptos.workspace = true`
- [ ] Component compilation errors fixed
- [ ] Example application compiles successfully

### **After Implementation**
- [ ] `cargo check --workspace` passes
- [ ] Example application builds without errors
- [ ] Demo renders correctly in browser
- [ ] No console errors or warnings
- [ ] All components function as expected

## 🔧 **COMMON ISSUES AND SOLUTIONS**

### **Issue 1: "expected a tuple with 3 elements, found one with 5 elements"**
**Cause**: Mixed Leptos versions in dependency tree
**Solution**: Clean Cargo.lock and ensure all packages use workspace versions

### **Issue 2: "closure only implements FnOnce"**
**Cause**: Moving values into closures that need to be `FnMut`
**Solution**: Clone values before moving into closures

### **Issue 3: "mismatched types" in view! macros**
**Cause**: Inconsistent return types between components
**Solution**: Use consistent `impl IntoView` return types

### **Issue 4: "unresolved import" errors**
**Cause**: Incorrect import paths or missing dependencies
**Solution**: Verify import paths and ensure all dependencies are properly declared

## 📋 **VERIFICATION COMMANDS**

```bash
# Check current Leptos version in use
cargo tree -p leptos

# Verify all packages use workspace versions
grep -r "leptos = " packages/leptos/*/Cargo.toml

# Check for version conflicts
cargo check --workspace 2>&1 | grep -i "version"

# Verify example compiles
cd examples/leptos && cargo check
```

## 🎯 **SUCCESS CRITERIA**

The migration is successful when:
1. ✅ `cargo check --workspace` completes without errors
2. ✅ Example application compiles successfully
3. ✅ Demo renders correctly in browser
4. ✅ All components function as expected
5. ✅ No version conflicts in dependency tree
6. ✅ Consistent Leptos 0.8.8 usage throughout project

## 📚 **ADDITIONAL RESOURCES**

- [Leptos 0.8 Migration Guide](https://leptos-rs.github.io/leptos/upgrading/0.8.html)
- [Leptos GitHub Repository](https://github.com/leptos-rs/leptos)
- [Cargo Workspace Documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html)

---

**Last Updated**: $(date)
**Status**: In Progress
**Target Completion**: Next development session
