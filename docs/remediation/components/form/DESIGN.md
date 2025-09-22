# 🎨 Form Component Design
**Component: Form | Priority: HIGH | Status: ⚠️ Needs Major Work**

## 📋 Component Overview

The Form component is a **critical form building component** but currently has **significant issues** with test coverage and implementation completeness. This is a **Priority 1** component requiring immediate attention.

### **Current Status**
- ⚠️ **1 test file** with basic implementations only
- ⚠️ **~30% test coverage** (estimated)
- ⚠️ **Missing accessibility tests**
- ⚠️ **Missing form validation tests**
- ⚠️ **Missing integration tests**
- ✅ **Basic structure exists**
- ✅ **Form validation system implemented**

---

## 🎯 Design Specifications

### **Visual Design**
```css
/* Form Container */
.form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  width: 100%;
}

/* Form Field */
.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* Form Item */
.form-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* Form Label */
.form-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: hsl(var(--foreground));
  line-height: 1.25rem;
}

/* Form Control */
.form-control {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

/* Form Message */
.form-message {
  font-size: 0.75rem;
  color: hsl(var(--destructive));
  margin-top: 0.25rem;
}

/* Form Description */
.form-description {
  font-size: 0.75rem;
  color: hsl(var(--muted-foreground));
  margin-top: 0.25rem;
}

/* Error State */
.form-field[data-invalid="true"] .form-control {
  border-color: hsl(var(--destructive));
}

.form-field[data-invalid="true"] .form-label {
  color: hsl(var(--destructive));
}
```

### **Animation Keyframes**
```css
@keyframes form-error-shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-4px); }
  75% { transform: translateX(4px); }
}

.form-field[data-invalid="true"] {
  animation: form-error-shake 0.3s ease-in-out;
}
```

---

## 🔧 API Design

### **Form Root Component**
```rust
#[component]
pub fn Form(
    #[prop(into, optional)] on_submit: Option<Callback<FormData>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Field Component**
```rust
#[component]
pub fn FormField(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Item Component**
```rust
#[component]
pub fn FormItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Label Component**
```rust
#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] r#for: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Control Component**
```rust
#[component]
pub fn FormControl(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Message Component**
```rust
#[component]
pub fn FormMessage(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

### **Form Description Component**
```rust
#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
```

---

## ♿ Accessibility Features

### **ARIA Attributes**
- `role="form"` on form element
- `aria-labelledby` linking to form title
- `aria-describedby` linking to form description
- `aria-invalid="true"` on invalid fields
- `aria-required="true"` on required fields
- `aria-describedby` linking to error messages

### **Keyboard Navigation**
- **Tab**: Navigate between form fields
- **Shift+Tab**: Reverse navigation
- **Enter**: Submit form
- **Escape**: Clear form or cancel

### **Focus Management**
- Focus on first invalid field after validation
- Focus on submit button after successful submission
- Focus restoration after form reset

### **Screen Reader Support**
- Form title announced when focused
- Field labels and descriptions provided
- Error messages announced immediately
- Validation state communicated

---

## 🧪 Testing Requirements

### **Current Test Status**
- ⚠️ **1 test file** with basic implementations
- ⚠️ **Missing form validation tests**
- ⚠️ **Missing accessibility tests**
- ⚠️ **Missing integration tests**
- ⚠️ **Missing error handling tests**

### **Required Test Categories**

#### **1. Basic Rendering Tests**
```rust
#[test]
fn test_form_renders_without_errors() {
    let form = Form::new(FormProps {
        on_submit: None,
        ..Default::default()
    });
    
    let rendered = form.render();
    assert!(rendered.contains("form"));
}

#[test]
fn test_form_field_renders() {
    let field = FormField::new(FormFieldProps {
        children: Some(Children::new()),
        ..Default::default()
    });
    
    let rendered = field.render();
    assert!(rendered.contains("form-field"));
}
```

#### **2. Form Validation Tests**
```rust
#[test]
fn test_form_validation_success() {
    let mut validation = FormValidation::new();
    
    // Test valid form
    assert!(validation.is_valid, "Form should be valid initially");
    assert!(validation.errors.is_empty(), "Should have no errors");
}

#[test]
fn test_form_validation_errors() {
    let mut validation = FormValidation::new();
    
    validation.add_error("email", "Email is required");
    validation.add_error("password", "Password is too short");
    
    assert!(!validation.is_valid, "Form should be invalid");
    assert_eq!(validation.errors.len(), 2, "Should have two errors");
    
    let email_error = validation.get_error("email");
    assert_eq!(email_error, Some("Email is required"));
}
```

#### **3. Form Submission Tests**
```rust
#[test]
fn test_form_submission() {
    let (submitted, set_submitted) = signal(false);
    let form_data = FormData::new();
    
    let form = Form::new(FormProps {
        on_submit: Some(Callback::new(move |data| {
            set_submitted.set(true);
        })),
        ..Default::default()
    });
    
    // Simulate form submission
    form.props.on_submit.unwrap().run(form_data);
    assert!(submitted.get(), "Form should be submitted");
}
```

#### **4. Accessibility Tests**
```rust
#[test]
fn test_form_aria_attributes() {
    let form = Form::new(FormProps {
        ..Default::default()
    });
    
    let rendered = form.render();
    assert!(rendered.contains("role=\"form\""));
}

#[test]
fn test_form_field_aria_attributes() {
    let field = FormField::new(FormFieldProps {
        invalid: Signal::derive(|| true),
        ..Default::default()
    });
    
    let rendered = field.render();
    assert!(rendered.contains("aria-invalid=\"true\""));
}
```

#### **5. Integration Tests**
```rust
#[test]
fn test_form_with_input_fields() {
    let form = Form::new(FormProps {
        children: Some(Children::new()),
        ..Default::default()
    });
    
    let rendered = form.render();
    // Test that form integrates with input fields
    assert!(rendered.contains("form"));
}

#[test]
fn test_form_with_validation() {
    let mut validation = FormValidation::new();
    validation.add_error("email", "Invalid email");
    
    let form = Form::new(FormProps {
        validation: Some(validation),
        ..Default::default()
    });
    
    let rendered = form.render();
    assert!(rendered.contains("form"));
}
```

---

## 🚀 Performance Considerations

### **Bundle Size**
- **Base component**: ~4KB
- **With validation**: ~6KB
- **WASM optimized**: ~5KB

### **Render Performance**
- **Initial render**: <3ms
- **Validation**: <1ms
- **Submission**: <2ms
- **Memory usage**: <3KB per instance

### **Optimization Strategies**
- Lazy validation on blur
- Debounced validation for real-time feedback
- Minimal re-renders on state changes
- Efficient error message updates

---

## 🔄 State Management

### **Form State**
```rust
#[derive(Clone, Debug)]
pub struct FormState {
    pub data: FormData,
    pub validation: FormValidation,
    pub is_submitting: bool,
    pub is_dirty: bool,
}

impl FormState {
    pub fn new() -> Self {
        Self {
            data: FormData::new(),
            validation: FormValidation::new(),
            is_submitting: false,
            is_dirty: false,
        }
    }
    
    pub fn update_field(&mut self, field: String, value: String) {
        self.data.fields.insert(field, value);
        self.is_dirty = true;
    }
    
    pub fn validate(&mut self) -> bool {
        self.validation = FormValidation::new();
        // Add validation logic here
        self.validation.is_valid
    }
}
```

### **Signal Integration**
```rust
// Form state management
let (form_state, set_form_state) = signal(FormState::new());

view! {
    <Form on_submit=move |data| {
        set_form_state.update(|state| {
            state.data = data;
            state.is_submitting = true;
        });
    }>
        <FormField>
            <FormLabel r#for="email">"Email"</FormLabel>
            <FormControl>
                <Input 
                    id="email"
                    on_change=move |value| {
                        set_form_state.update(|state| {
                            state.update_field("email".to_string(), value);
                        });
                    }
                />
            </FormControl>
            <FormMessage>
                {move || form_state.get().validation.get_error("email").unwrap_or("")}
            </FormMessage>
        </FormField>
    </Form>
}
```

---

## 📱 Responsive Design

### **Breakpoint Behavior**
- **Mobile**: Stacked layout, full width
- **Tablet**: Two-column layout for related fields
- **Desktop**: Multi-column layout with proper spacing

### **Touch Targets**
- **Minimum size**: 44px × 44px for interactive elements
- **Touch-friendly spacing**
- **Large tap targets for mobile**

---

## 🎨 Theming Support

### **CSS Custom Properties**
```css
:root {
  --form-gap: 1.5rem;
  --form-field-gap: 0.5rem;
  --form-label-color: hsl(var(--foreground));
  --form-message-color: hsl(var(--destructive));
  --form-description-color: hsl(var(--muted-foreground));
}
```

### **Dark Mode Support**
- Automatic theme switching
- High contrast mode support
- Custom theme overrides

---

## 🔧 Implementation Notes

### **Current Implementation**
- **File**: `packages/leptos/form/src/default.rs` (296 lines)
- **Status**: ⚠️ Basic implementation, needs enhancement
- **Tests**: `packages/leptos/form/src/tests.rs` (359 lines)

### **Required Implementation**
1. **Add comprehensive accessibility features**
2. **Implement form validation system**
3. **Add error handling and display**
4. **Create integration tests**
5. **Add performance optimizations**

---

## 📋 Action Items

### **Immediate (Week 1) - Priority 1**
- [ ] **Add accessibility features** (ARIA attributes, focus management)
- [ ] **Implement form validation system**
- [ ] **Create form submission tests**
- [ ] **Add error handling and display**

### **Short-term (Week 2-3)**
- [ ] **Implement comprehensive test suite**
- [ ] **Add integration tests**
- [ ] **Create performance benchmarks**
- [ ] **Add form state management**

### **Long-term (Week 4+)**
- [ ] **Advanced form features**
- [ ] **Custom validation rules**
- [ ] **Performance optimizations**
- [ ] **Advanced theming**

---

## 🎯 Success Metrics

### **Quality Metrics**
- **Test coverage**: 90%+ (current: ~30%)
- **Accessibility score**: AAA compliance
- **Performance**: <3ms render time
- **Bundle size**: <5KB optimized

### **Functionality Metrics**
- **Form validation**: 100% working
- **Accessibility compliance**: WCAG 2.1 AA
- **Form submission**: 100% functional
- **Error handling**: 100% working

---

## 🚨 Critical Issues

### **Must Fix Immediately**
1. **Missing accessibility features** - ARIA attributes, focus management
2. **No form validation tests** - Core functionality not tested
3. **Missing error handling** - Essential for form functionality
4. **No integration tests** - Form with other components not tested

### **Impact Assessment**
- **High impact** on user experience
- **Critical for form functionality**
- **Essential for accessibility compliance**
- **Required for production readiness**

---

This Form component is **critical** and requires **immediate attention** to bring it to production-ready status. It serves as the foundation for all form interactions in the library.

---

*Design document created: September 20, 2025*  
*Last updated: September 20, 2025*
