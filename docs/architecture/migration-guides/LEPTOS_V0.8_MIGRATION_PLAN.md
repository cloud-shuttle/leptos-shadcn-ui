# 🚀 Leptos v0.8 Migration Plan

**Comprehensive plan to migrate all leptos-shadcn-ui components to Leptos v0.8's new attribute system**

## 🎯 **Problem Statement**

The current leptos-shadcn-ui v0.5.0 components are **NOT COMPATIBLE** with Leptos v0.8 due to:

1. **Signal Trait Bound Issues**
   - `Signal<String>: IntoClass` not satisfied
   - `Signal<bool>: IntoAttributeValue` not satisfied  
   - `RwSignal<String>: IntoProperty` not satisfied

2. **Missing Attribute Implementations**
   - `on:click` method trait bounds not satisfied
   - `id`, `type`, `disabled` method trait bounds not satisfied
   - HTML element attribute methods not working

3. **Affected Components**
   - `leptos-shadcn-input-otp` - 2 compilation errors
   - `leptos-shadcn-command` - 2 compilation errors
   - `leptos-shadcn-input` - 6 compilation errors
   - `leptos-shadcn-button` - 6 compilation errors
   - **All 46 components** need migration

## 🔧 **Migration Strategy**

### **Phase 1: Attribute System Migration**
Update all components to use Leptos v0.8's new attribute system:

#### **Old v0.7 Syntax → New v0.8 Syntax**
```rust
// OLD (v0.7) - ❌ NOT WORKING
<button
    class=computed_class
    id=id.get().unwrap_or_default()
    style=move || style.get().to_string()
    disabled=disabled
    on:click=handle_click
>

// NEW (v0.8) - ✅ WORKING
<button
    class=move || computed_class.get()
    id=move || id.get().unwrap_or_default()
    style=move || style.get().to_string()
    disabled=move || disabled.get()
    on:click=handle_click
>
```

#### **Key Changes Required:**
1. **Signal Access**: Wrap all signal access in `move ||` closures
2. **Class Attributes**: `class=computed_class` → `class=move || computed_class.get()`
3. **ID Attributes**: `id=id.get()` → `id=move || id.get()`
4. **Style Attributes**: Keep `style=move || style.get().to_string()`
5. **Disabled Attributes**: `disabled=disabled` → `disabled=move || disabled.get()`
6. **Event Handlers**: Keep `on:click=handle_click` (no changes needed)

### **Phase 2: Signal Handling Updates**
Update signal handling to work with new trait bounds:

#### **Signal to Attribute Conversion**
```rust
// OLD - Direct signal usage
class=computed_class
disabled=disabled

// NEW - Proper signal handling
class:move || computed_class.get()
disabled:move || disabled.get()
```

### **Phase 3: Component-by-Component Migration**
Systematically migrate all 46 components:

#### **Priority Order:**
1. **Core Form Components** (Button, Input, Label, Checkbox)
2. **Layout Components** (Card, Separator, Tabs, Accordion)
3. **Overlay Components** (Dialog, Popover, Tooltip)
4. **Navigation Components** (Breadcrumb, Navigation Menu)
5. **Feedback Components** (Alert, Badge, Toast)
6. **Advanced Components** (Table, Calendar, Form)

## 📋 **Detailed Migration Steps**

### **Step 1: Update Button Component**
**File**: `packages/leptos/button/src/default.rs`

**Changes Required:**
```rust
// BEFORE (v0.7)
<button
    class=computed_class
    id=id.get().unwrap_or_default()
    style=move || style.get().to_string()
    disabled=disabled
    on:click=handle_click
>

// AFTER (v0.8)
<button
    class:move || computed_class.get()
    id:move || id.get().unwrap_or_default()
    style:move || style.get().to_string()
    disabled:move || disabled.get()
    on:click=handle_click
>
```

### **Step 2: Update Input Component**
**File**: `packages/leptos/input/src/default.rs`

**Changes Required:**
```rust
// BEFORE (v0.7)
<input
    r#type=input_type.get().unwrap_or_else(|| "text".to_string())
    value=value.get().unwrap_or_default()
    placeholder=placeholder.get().unwrap_or_default()
    disabled=move || disabled.get()
    class=move || computed_class.get()
    id=id.get().unwrap_or_default()
    style=move || style.get().to_string()
    on:input=handle_input
/>

// AFTER (v0.8)
<input
    type:move || input_type.get().unwrap_or_else(|| "text".to_string())
    value:move || value.get().unwrap_or_default()
    placeholder:move || placeholder.get().unwrap_or_default()
    disabled:move || disabled.get()
    class:move || computed_class.get()
    id:move || id.get().unwrap_or_default()
    style:move || style.get().to_string()
    on:input=handle_input
/>
```

### **Step 3: Update All Other Components**
Apply the same pattern to all remaining components.

## 🧪 **Testing Strategy**

### **Phase 1: Unit Tests**
- Update all component unit tests
- Ensure tests pass with new attribute system
- Verify signal handling works correctly

### **Phase 2: Integration Tests**
- Test components in real applications
- Verify event handling works
- Check styling and behavior

### **Phase 3: E2E Tests**
- Update Playwright tests if needed
- Verify browser compatibility
- Test user interactions

## 📦 **Version Strategy**

### **Version Bump Plan**
- **Current**: v0.5.0 (Performance Audit Edition)
- **Next**: v0.6.0 (Leptos v0.8 Compatibility Edition)

### **Breaking Changes**
- **MAJOR**: Attribute system changes
- **MINOR**: Signal handling updates
- **PATCH**: Bug fixes and improvements

## 🚀 **Implementation Plan**

### **Week 1: Core Components**
- [ ] Button component migration
- [ ] Input component migration
- [ ] Label component migration
- [ ] Checkbox component migration
- [ ] Testing and validation

### **Week 2: Layout Components**
- [ ] Card component migration
- [ ] Separator component migration
- [ ] Tabs component migration
- [ ] Accordion component migration
- [ ] Testing and validation

### **Week 3: Overlay Components**
- [ ] Dialog component migration
- [ ] Popover component migration
- [ ] Tooltip component migration
- [ ] Alert Dialog component migration
- [ ] Testing and validation

### **Week 4: Advanced Components**
- [ ] Table component migration
- [ ] Calendar component migration
- [ ] Form component migration
- [ ] Command component migration
- [ ] Final testing and validation

## 🔍 **Quality Assurance**

### **Testing Checklist**
- [ ] All components compile without errors
- [ ] All unit tests pass
- [ ] All E2E tests pass
- [ ] Components work in real applications
- [ ] Performance is maintained or improved
- [ ] Documentation is updated

### **Validation Steps**
1. **Compilation Test**: `cargo check --workspace`
2. **Unit Tests**: `cargo test --workspace`
3. **E2E Tests**: `npm run test:e2e`
4. **Integration Test**: Create test application
5. **Performance Test**: Run performance audit

## 📚 **Documentation Updates**

### **Required Updates**
- [ ] Update README.md with v0.8 compatibility
- [ ] Update component documentation
- [ ] Update migration guide
- [ ] Update examples
- [ ] Update API reference

## 🎯 **Success Criteria**

### **Definition of Done**
- [ ] All 46 components compile with Leptos v0.8
- [ ] All tests pass
- [ ] Components work in real applications
- [ ] Documentation is updated
- [ ] Performance is maintained
- [ ] v0.6.0 is published to crates.io

### **Acceptance Criteria**
- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] Example application works
- [ ] Performance audit passes
- [ ] No breaking changes for users (except attribute syntax)

## 🚨 **Risk Mitigation**

### **Potential Risks**
1. **Breaking Changes**: Users need to update their code
2. **Complex Migration**: Some components may be complex
3. **Testing Overhead**: Extensive testing required
4. **Timeline Pressure**: Migration may take longer than expected

### **Mitigation Strategies**
1. **Clear Documentation**: Provide migration guide
2. **Incremental Approach**: Migrate components in batches
3. **Comprehensive Testing**: Test thoroughly at each step
4. **Flexible Timeline**: Allow for additional time if needed

## 📅 **Timeline**

### **Total Duration**: 4 weeks
### **Start Date**: Today
### **Target Completion**: 4 weeks from start
### **Release Date**: v0.6.0 after completion

---

**🎯 Goal: Make leptos-shadcn-ui fully compatible with Leptos v0.8 while maintaining all existing functionality and performance.**
