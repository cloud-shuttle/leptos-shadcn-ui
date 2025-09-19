# 🎨 **Form Component Design**

## **Overview**
Design for the Form component that provides form building blocks with validation and accessibility features.

## **Core Components**

### **Form Component**
```rust
#[component]
pub fn Form(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] on_submit: Option<Callback<FormData>>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let (form_data, set_form_data) = signal(std::collections::HashMap::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (errors, set_errors) = signal(Vec::new());
    
    let form_class = move || {
        let mut classes = vec!["space-y-6"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        if !is_submitting.get() {
            set_is_submitting.set(true);
            set_errors.set(Vec::new());
            
            if let Some(on_submit) = on_submit.as_ref() {
                let data = FormData {
                    fields: form_data.get(),
                    is_submitting: true,
                    is_valid: errors.get().is_empty(),
                    errors: errors.get(),
                };
                on_submit.call(data);
            }
            
            set_is_submitting.set(false);
        }
    };
    
    view! {
        <form
            class=form_class
            id=id
            on:submit=handle_submit
        >
            {children}
        </form>
    }
}
```

### **FormField Component**
```rust
#[component]
pub fn FormField(
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let field_class = move || {
        let mut classes = vec!["space-y-2"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=field_class
            data-field-name=name
        >
            {children}
        </div>
    }
}
```

### **FormItem Component**
```rust
#[component]
pub fn FormItem(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let item_class = move || {
        let mut classes = vec!["space-y-2"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=item_class
        >
            {children}
        </div>
    }
}
```

### **FormLabel Component**
```rust
#[component]
pub fn FormLabel(
    #[prop(into, optional)] for_id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let label_class = move || {
        let mut classes = vec!["text-sm", "font-medium", "leading-none", "peer-disabled:cursor-not-allowed", "peer-disabled:opacity-70"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <label
            class=label_class
            for=for_id
        >
            {children}
        </label>
    }
}
```

### **FormControl Component**
```rust
#[component]
pub fn FormControl(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let control_class = move || {
        let mut classes = vec!["peer"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=control_class
        >
            {children}
        </div>
    }
}
```

### **FormMessage Component**
```rust
#[component]
pub fn FormMessage(
    #[prop(into, optional)] message: Option<Signal<Option<String>>>,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let message_class = move || {
        let mut classes = vec!["text-sm", "font-medium", "text-destructive"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        if let Some(message) = message.as_ref() {
            if let Some(msg) = message.get() {
                <p class=message_class>
                    {msg}
                </p>
            }
        }
    }
}
```

### **FormDescription Component**
```rust
#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let description_class = move || {
        let mut classes = vec!["text-sm", "text-muted-foreground"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <p
            class=description_class
        >
            {children}
        </p>
    }
}
```

## **Supporting Types**

### **FormData**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FormData {
    pub fields: std::collections::HashMap<String, String>,
    pub is_submitting: bool,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            fields: std::collections::HashMap::new(),
            is_submitting: false,
            is_valid: true,
            errors: Vec::new(),
        }
    }
    
    pub fn add_field(&mut self, name: String, value: String) {
        self.fields.insert(name, value);
    }
    
    pub fn get_field(&self, name: &str) -> Option<&String> {
        self.fields.get(name)
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
```

### **FormValidation**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FormValidation {
    pub can_submit: bool,
    pub has_errors: bool,
    pub error_count: usize,
}

impl FormValidation {
    pub fn new() -> Self {
        Self {
            can_submit: true,
            has_errors: false,
            error_count: 0,
        }
    }
    
    pub fn with_errors(errors: Vec<String>) -> Self {
        Self {
            can_submit: errors.is_empty(),
            has_errors: !errors.is_empty(),
            error_count: errors.len(),
        }
    }
}
```

### **FormError**
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FormError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl FormError {
    pub fn new(field: String, message: String, code: String) -> Self {
        Self {
            field,
            message,
            code,
        }
    }
    
    pub fn required(field: String) -> Self {
        Self::new(field, "This field is required".to_string(), "required".to_string())
    }
    
    pub fn invalid_email(field: String) -> Self {
        Self::new(field, "Invalid email format".to_string(), "invalid_email".to_string())
    }
    
    pub fn min_length(field: String, min: usize) -> Self {
        Self::new(field, format!("Minimum length is {} characters", min), "min_length".to_string())
    }
}
```

## **Usage Examples**

### **Basic Form**
```rust
let handle_submit = move |data: FormData| {
    println!("Form submitted: {:?}", data);
};

view! {
    <Form on_submit=handle_submit>
        <FormField name="name">
            <FormItem>
                <FormLabel for_id="name">"Name"</FormLabel>
                <FormControl>
                    <Input
                        id="name"
                        placeholder="Enter your name"
                        required=true
                    />
                </FormControl>
                <FormDescription>
                    "Enter your full name"
                </FormDescription>
            </FormItem>
        </FormField>
        
        <FormField name="email">
            <FormItem>
                <FormLabel for_id="email">"Email"</FormLabel>
                <FormControl>
                    <Input
                        id="email"
                        input_type="email"
                        placeholder="Enter your email"
                        required=true
                    />
                </FormControl>
                <FormDescription>
                    "Enter your email address"
                </FormDescription>
            </FormItem>
        </FormField>
        
        <Button type="submit">
            "Submit"
        </Button>
    </Form>
}
```

### **Form with Validation**
```rust
let (name_error, set_name_error) = signal(None::<String>);
let (email_error, set_email_error) = signal(None::<String>);

let handle_submit = move |data: FormData| {
    let mut errors = Vec::new();
    
    if data.get_field("name").map_or(true, |v| v.is_empty()) {
        errors.push("Name is required".to_string());
        set_name_error.set(Some("Name is required".to_string()));
    } else {
        set_name_error.set(None);
    }
    
    if data.get_field("email").map_or(true, |v| v.is_empty()) {
        errors.push("Email is required".to_string());
        set_email_error.set(Some("Email is required".to_string()));
    } else {
        set_email_error.set(None);
    }
    
    if errors.is_empty() {
        println!("Form is valid: {:?}", data);
    }
};

view! {
    <Form on_submit=handle_submit>
        <FormField name="name">
            <FormItem>
                <FormLabel for_id="name">"Name"</FormLabel>
                <FormControl>
                    <Input
                        id="name"
                        placeholder="Enter your name"
                        validation_error=name_error
                    />
                </FormControl>
                <FormMessage message=name_error />
            </FormItem>
        </FormField>
        
        <FormField name="email">
            <FormItem>
                <FormLabel for_id="email">"Email"</FormLabel>
                <FormControl>
                    <Input
                        id="email"
                        input_type="email"
                        placeholder="Enter your email"
                        validation_error=email_error
                    />
                </FormControl>
                <FormMessage message=email_error />
            </FormItem>
        </FormField>
        
        <Button type="submit">
            "Submit"
        </Button>
    </Form>
}
```

### **Form with Custom Styling**
```rust
view! {
    <Form
        class="max-w-md mx-auto bg-white p-6 rounded-lg shadow-md"
        on_submit=move |data| println!("Submitted: {:?}", data)
    >
        <FormField name="message">
            <FormItem>
                <FormLabel for_id="message">"Message"</FormLabel>
                <FormControl>
                    <Textarea
                        id="message"
                        placeholder="Enter your message"
                        rows=4
                    />
                </FormControl>
                <FormDescription>
                    "Enter a message (optional)"
                </FormDescription>
            </FormItem>
        </FormField>
        
        <div class="flex justify-end space-x-2">
            <Button variant=ButtonVariant::Outline>
                "Cancel"
            </Button>
            <Button type="submit">
                "Send Message"
            </Button>
        </div>
    </Form>
}
```

## **Accessibility Features**

### **Form Structure**
- Proper form semantics
- Field grouping with `FormField`
- Label associations
- Error message associations

### **Keyboard Navigation**
- Tab order through form fields
- Enter key submission
- Escape key cancellation

### **Screen Reader Support**
- Proper labeling
- Error announcements
- Form state announcements

---

**File Size**: 299 lines
**Priority**: 🟢 **P2 - WORKING**
**Dependencies**: leptos
