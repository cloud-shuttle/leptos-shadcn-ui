# 🎨 **Input Component Design**

## **Overview**
Design for the Input component that provides a text input field with validation, styling, and accessibility features.

## **Core Component**

### **Input Component**
```rust
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
) -> impl IntoView {
    let input_class = move || {
        let mut classes = vec!["flex", "h-10", "w-full", "rounded-md", "border", "border-input", "bg-background", "px-3", "py-2", "text-sm", "ring-offset-background"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        if let Some(validation_error) = validation_error.as_ref() {
            if validation_error.get().is_some() {
                classes.push("border-destructive");
            }
        }
        
        classes.join(" ")
    };
    
    let handle_input = move |ev: leptos::ev::InputEvent| {
        let value = event_target_value(&ev);
        if let Some(on_change) = on_change.as_ref() {
            on_change.call(value);
        }
    };
    
    view! {
        <input
            type=input_type.unwrap_or_else(|| "text".to_string())
            value=value
            placeholder=placeholder
            disabled=disabled.map(|d| d.get()).unwrap_or(false)
            class=input_class
            id=id
            style=style.map(|s| s.get())
            on:input=handle_input
            aria-invalid=validation_error.map(|e| e.get().is_some()).unwrap_or(false)
            aria-describedby=validation_error.map(|e| e.get().is_some().then(|| format!("{}-error", id.unwrap_or_else(|| "input".to_string()))))
        />
    }
}
```

## **Supporting Types**

### **InputValidator**
```rust
#[derive(Debug, Clone)]
pub struct InputValidator {
    pub required: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub custom_validator: Option<Box<dyn Fn(&str) -> bool>>,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
            custom_validator: None,
        }
    }
    
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    pub fn min_length(mut self, length: usize) -> Self {
        self.min_length = Some(length);
        self
    }
    
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }
    
    pub fn pattern(mut self, pattern: String) -> Self {
        self.pattern = Some(pattern);
        self
    }
    
    pub fn custom_validator<F>(mut self, validator: F) -> Self 
    where 
        F: Fn(&str) -> bool + 'static 
    {
        self.custom_validator = Some(Box::new(validator));
        self
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        let mut errors = Vec::new();
        
        if self.required && value.is_empty() {
            errors.push("This field is required".to_string());
        }
        
        if let Some(min_len) = self.min_length {
            if value.len() < min_len {
                errors.push(format!("Minimum length is {} characters", min_len));
            }
        }
        
        if let Some(max_len) = self.max_length {
            if value.len() > max_len {
                errors.push(format!("Maximum length is {} characters", max_len));
            }
        }
        
        if let Some(pattern) = &self.pattern {
            let regex = regex::Regex::new(pattern).unwrap();
            if !regex.is_match(value) {
                errors.push("Invalid format".to_string());
            }
        }
        
        if let Some(validator) = &self.custom_validator {
            if !validator(value) {
                errors.push("Invalid value".to_string());
            }
        }
        
        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
        }
    }
}
```

### **ValidationResult**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }
    
    pub fn with_error(error: String) -> Self {
        Self {
            is_valid: false,
            errors: vec![error],
        }
    }
    
    pub fn first_error(&self) -> Option<&String> {
        self.errors.first()
    }
}
```

## **Enhanced Input Variants**

### **Input with Size Support**
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

impl InputSize {
    pub fn classes(&self) -> &'static str {
        match self {
            InputSize::Sm => "h-8 text-xs",
            InputSize::Default => "h-10 text-sm",
            InputSize::Lg => "h-12 text-base",
        }
    }
}
```

### **Input with Variant Support**
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

impl InputVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            InputVariant::Default => "border-input bg-background",
            InputVariant::Destructive => "border-destructive bg-destructive/10",
            InputVariant::Outline => "border-2 border-primary bg-transparent",
        }
    }
}
```

## **Usage Examples**

### **Basic Input**
```rust
let (value, set_value) = signal(String::new());

view! {
    <Input
        value=value
        on_change=move |new_value| set_value.set(new_value)
        placeholder="Enter your name"
    />
}
```

### **Input with Validation**
```rust
let (value, set_value) = signal(String::new());
let (error, set_error) = signal(None::<String>);

let validator = InputValidator::new()
    .required()
    .min_length(3)
    .max_length(50);

let handle_change = move |new_value: String| {
    set_value.set(new_value.clone());
    let result = validator.validate(&new_value);
    if !result.is_valid() {
        set_error.set(result.first_error().cloned());
    } else {
        set_error.set(None);
    }
};

view! {
    <Input
        value=value
        on_change=handle_change
        placeholder="Enter your name"
        validator=validator
        validation_error=error
        show_validation=true
    />
}
```

### **Input with Custom Styling**
```rust
let (value, set_value) = signal(String::new());
let style = leptos_style::Style::new()
    .background_color("#f0f0f0")
    .border("2px solid #ccc");

view! {
    <Input
        value=value
        on_change=move |new_value| set_value.set(new_value)
        placeholder="Custom styled input"
        style=style
        class="rounded-lg shadow-md"
    />
}
```

### **Input with Different Types**
```rust
let (email, set_email) = signal(String::new());
let (password, set_password) = signal(String::new());

view! {
    <div class="space-y-4">
        <Input
            value=email
            on_change=move |new_value| set_email.set(new_value)
            placeholder="Enter your email"
            input_type="email"
        />
        <Input
            value=password
            on_change=move |new_value| set_password.set(new_value)
            placeholder="Enter your password"
            input_type="password"
        />
    </div>
}
```

## **Accessibility Features**

### **ARIA Attributes**
- `aria-invalid`: Set based on validation state
- `aria-describedby`: Links to error messages
- `aria-required`: Set when field is required
- `aria-label`: Custom label for screen readers

### **Keyboard Navigation**
- Tab navigation support
- Enter key handling
- Escape key to clear (optional)

### **Screen Reader Support**
- Proper labeling
- Error message announcements
- State changes announced

---

**File Size**: 299 lines
**Priority**: 🔴 **P0 - CRITICAL**
**Dependencies**: leptos, leptos_style, regex
