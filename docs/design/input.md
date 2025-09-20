# Input Component Design Specification

## Overview & Purpose

The Input component is a foundational form element that handles text input with comprehensive validation, accessibility features, and integration with form libraries. It serves as the base for all text-based form interactions.

**Component Type**: Form/Input  
**Priority**: P0 (Critical - essential for forms)  
**Dependencies**: Label (optional), ValidationContext

## API Specification

### Props Interface
```rust
#[derive(Props, PartialEq)]
pub struct InputProps {
    /// Input type (text, email, password, etc.)
    #[prop(into, optional)]
    pub input_type: MaybeProp<InputType>,
    
    /// Current value
    #[prop(into, optional)]
    pub value: MaybeProp<String>,
    
    /// Placeholder text
    #[prop(into, optional)]
    pub placeholder: MaybeProp<String>,
    
    /// Disabled state
    #[prop(into, optional)]
    pub disabled: MaybeProp<bool>,
    
    /// Required field indicator
    #[prop(into, optional)]
    pub required: MaybeProp<bool>,
    
    /// Readonly state
    #[prop(into, optional)]
    pub readonly: MaybeProp<bool>,
    
    /// Input change handler
    #[prop(optional)]
    pub on_input: Option<Callback<String>>,
    
    /// Focus event handler
    #[prop(optional)]
    pub on_focus: Option<Callback<web_sys::FocusEvent>>,
    
    /// Blur event handler (validation trigger)
    #[prop(optional)]
    pub on_blur: Option<Callback<web_sys::FocusEvent>>,
    
    /// Validation rules
    #[prop(optional)]
    pub validator: Option<InputValidator>,
    
    /// Error state override
    #[prop(into, optional)]
    pub error: MaybeProp<Option<String>>,
    
    /// Success state override
    #[prop(into, optional)]
    pub success: MaybeProp<bool>,
    
    /// Custom CSS classes
    #[prop(into, optional)]
    pub class: MaybeProp<String>,
    
    /// HTML id attribute
    #[prop(into, optional)]
    pub id: MaybeProp<String>,
    
    /// HTML name attribute (form binding)
    #[prop(into, optional)]
    pub name: MaybeProp<String>,
    
    /// ARIA label for accessibility
    #[prop(into, optional)]
    pub aria_label: MaybeProp<String>,
    
    /// ARIA described by (error messages)
    #[prop(into, optional)]
    pub aria_describedby: MaybeProp<String>,
}
```

### Enums and Types
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Tel,
    Url,
    Search,
    Number,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Small,    // Compact form inputs
    Default,  // Standard size
    Large,    // Prominent inputs
}

#[derive(Debug, Clone)]
pub struct InputValidator {
    pub rules: Vec<ValidationRule>,
    pub on_validation: Option<Callback<ValidationResult>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Email,
    Pattern(String),
    Custom(fn(&str) -> Result<(), String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub field_name: String,
}
```

## Behavioral Requirements

### Core Behaviors
1. **Text Input**: Accepts and displays user text input
2. **Real-time Validation**: Validates on input/blur based on rules
3. **State Management**: Tracks focus, error, success states
4. **Form Integration**: Works with form libraries and native forms
5. **Accessibility**: Full screen reader and keyboard support

### State Transitions
```
[Empty] --input--> [Filled] --validate--> [Valid/Invalid]
[Any] --focus--> [Focused] --blur--> [Unfocused + Validated]
[Any] --disabled--> [Disabled] --enabled--> [Previous State]
```

### Validation Timing
- **On Input**: Real-time for immediate feedback (debounced 300ms)
- **On Blur**: Comprehensive validation when field loses focus
- **On Submit**: Final validation before form submission
- **On Mount**: Initial validation if value provided

### Event Handling
```rust
// Input event with debouncing
let handle_input = move |event: web_sys::Event| {
    let input = event_target_value(&event);
    value_signal.set(input.clone());
    
    // Debounced validation
    debounced_validate.call(input);
    
    if let Some(on_input) = on_input {
        on_input.call(input);
    }
};

// Blur event for validation
let handle_blur = move |event: web_sys::FocusEvent| {
    set_focused(false);
    validate_field();
    
    if let Some(on_blur) = on_blur {
        on_blur.call(event);
    }
};
```

## Accessibility Requirements

### WCAG 2.1 AA Compliance
- **Labels**: Associated with `<label>` element or `aria-label`
- **Error Messages**: Linked via `aria-describedby`
- **Required Fields**: Indicated with `required` attribute and `aria-required`
- **Invalid State**: Marked with `aria-invalid="true"`
- **Focus Management**: Clear focus indicators and logical tab order

### Screen Reader Support
```html
<div>
  <label for="email-input" class="sr-only">Email Address</label>
  <input
    type="email"
    id="email-input"
    name="email"
    placeholder="Enter email address"
    required
    aria-required="true"
    aria-invalid="false"
    aria-describedby="email-error email-help"
  />
  <div id="email-help" class="sr-only">
    We'll never share your email address
  </div>
  <div id="email-error" role="alert" aria-live="polite">
    <!-- Error messages inserted here -->
  </div>
</div>
```

### Keyboard Navigation
- **Tab**: Focuses the input field
- **Enter**: Submits parent form (if applicable)
- **Escape**: Clears field (optional behavior)

## Styling & Theming

### Base Styles
```rust
const BASE_CLASSES: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
```

### State-based Styles
```rust
fn input_classes(
    has_error: bool,
    has_success: bool, 
    is_focused: bool,
    disabled: bool,
) -> String {
    let mut classes = vec![BASE_CLASSES];
    
    match (has_error, has_success, disabled) {
        (true, _, false) => classes.push("border-destructive focus-visible:ring-destructive"),
        (false, true, false) => classes.push("border-success focus-visible:ring-success"),
        (_, _, true) => classes.push("border-muted bg-muted"),
        _ => {}
    }
    
    if is_focused && !disabled {
        classes.push("ring-2 ring-ring ring-offset-2");
    }
    
    classes.join(" ")
}
```

### Size Variants
```rust
fn size_classes(size: InputSize) -> &'static str {
    match size {
        InputSize::Small => "h-8 px-2 py-1 text-xs",
        InputSize::Default => "h-10 px-3 py-2 text-sm", 
        InputSize::Large => "h-12 px-4 py-3 text-base",
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_testing::*;
    
    #[wasm_bindgen_test]
    fn renders_basic_input() {
        let result = render_component(|| {
            view! { <Input placeholder="Enter text" /> }
        });
        
        assert_element_exists(&result, "input");
        assert_element_attribute(&result, "input", "placeholder", "Enter text");
        assert_element_has_class(&result, "input", "border-input");
    }
    
    #[wasm_bindgen_test]
    fn handles_input_events() {
        let value = create_rw_signal(String::new());
        
        let result = render_component(|| {
            view! {
                <Input 
                    value=value.get()
                    on_input=move |v| value.set(v)
                />
            }
        });
        
        type_in_element(&result, "input", "Hello World");
        assert_eq!(value.get(), "Hello World");
    }
    
    #[wasm_bindgen_test]
    fn validates_required_field() {
        let validator = InputValidator::new()
            .required()
            .build();
            
        let result = render_component(|| {
            view! {
                <Input 
                    validator=validator
                    value=""
                />
            }
        });
        
        blur_element(&result, "input");
        
        assert_element_has_attribute(&result, "input", "aria-invalid", "true");
        assert_element_exists(&result, "[role='alert']");
    }
    
    #[wasm_bindgen_test]
    fn validates_email_format() {
        let validator = InputValidator::new()
            .email()
            .build();
            
        let result = render_component(|| {
            view! {
                <Input 
                    input_type=InputType::Email
                    validator=validator
                    value="invalid-email"
                />
            }
        });
        
        blur_element(&result, "input");
        
        assert_element_has_attribute(&result, "input", "aria-invalid", "true");
        assert_element_text_contains(&result, "[role='alert']", "valid email");
    }
}
```

### Integration Tests
```rust
#[wasm_bindgen_test]
async fn integrates_with_form() {
    let form_submitted = create_rw_signal(false);
    
    let result = render_component(|| {
        view! {
            <form on_submit=move |e| {
                e.prevent_default();
                form_submitted.set(true);
            }>
                <Input name="username" required=true />
                <button type="submit">"Submit"</button>
            </form>
        }
    });
    
    type_in_element(&result, "input", "testuser");
    click_element(&result, "button");
    
    assert!(form_submitted.get());
}
```

### Accessibility Tests
```rust
#[wasm_bindgen_test]
async fn meets_accessibility_standards() {
    let result = render_component(|| {
        view! {
            <div>
                <label for="test-input">"Test Input"</label>
                <Input id="test-input" required=true />
            </div>
        }
    });
    
    // Run axe-core checks
    assert_accessible(&result).await;
    
    // Test specific a11y requirements
    assert_element_has_attribute(&result, "input", "aria-required", "true");
    assert_elements_associated(&result, "label", "input");
}
```

### Performance Tests
```rust
#[wasm_bindgen_test]
fn handles_rapid_input_efficiently() {
    let result = render_component(|| {
        view! { <Input validator=complex_validator /> }
    });
    
    let start = performance::now();
    
    // Simulate rapid typing
    for i in 0..100 {
        type_in_element(&result, "input", &format!("text{}", i));
    }
    
    let duration = performance::now() - start;
    assert!(duration < 1000.0, "Should handle rapid input in <1s");
}
```

## Implementation Notes

### Debounced Validation
```rust
fn create_debounced_validator(
    validator: Option<InputValidator>,
    delay_ms: u32,
) -> impl Fn(String) {
    let timeout_handle = create_rw_signal(None::<i32>);
    
    move |value: String| {
        // Clear existing timeout
        if let Some(handle) = timeout_handle.get() {
            clear_timeout(handle);
        }
        
        // Set new timeout
        let new_handle = set_timeout(
            move || validate_value(value.clone(), &validator),
            delay_ms,
        );
        timeout_handle.set(Some(new_handle));
    }
}
```

### Form Integration
```rust
impl Input {
    // Register with form context on mount
    fn register_with_form(&self) {
        if let Some(form_context) = use_context::<FormContext>() {
            form_context.register_field(FormField {
                name: self.name.clone(),
                value: self.value_signal,
                validator: self.validator.clone(),
                errors: self.errors_signal,
            });
        }
    }
}
```

### Memory Management
- Use `create_memo` for computed validation state
- Clean up event listeners on unmount
- Debounce validation to prevent excessive calls

## Examples & Usage

### Basic Text Input
```rust
view! {
    <Input 
        placeholder="Enter your name"
        on_input=move |value| console::log!("Input: {}", value)
    />
}
```

### Email Input with Validation
```rust
let email_validator = InputValidator::new()
    .required()
    .email()
    .build();

view! {
    <Input 
        input_type=InputType::Email
        placeholder="your@email.com"
        validator=email_validator
        required=true
    />
}
```

### Password Input with Strength Validation
```rust
let password_validator = InputValidator::new()
    .required()
    .min_length(8)
    .pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])")
    .build();

view! {
    <Input 
        input_type=InputType::Password
        placeholder="Strong password"
        validator=password_validator
    />
}
```

### Controlled Input with External State
```rust
fn ControlledInput() -> impl IntoView {
    let value = create_rw_signal(String::new());
    let char_count = create_memo(move |_| value.get().len());
    
    view! {
        <div>
            <Input 
                value=value.get()
                on_input=move |v| value.set(v)
                placeholder="Type something..."
            />
            <div class="text-sm text-muted-foreground">
                {char_count} " characters"
            </div>
        </div>
    }
}
```

### Form Integration Example
```rust
view! {
    <form class="space-y-4">
        <div>
            <label for="username">"Username"</label>
            <Input 
                id="username"
                name="username" 
                required=true
                validator=username_validator
            />
        </div>
        
        <div>
            <label for="email">"Email"</label>
            <Input 
                id="email"
                name="email"
                input_type=InputType::Email
                required=true
                validator=email_validator
            />
        </div>
        
        <button type="submit">"Submit"</button>
    </form>
}
```
