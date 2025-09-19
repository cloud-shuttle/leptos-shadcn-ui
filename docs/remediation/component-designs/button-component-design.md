# 🎨 **Button Component Design**

## **Overview**
Design for the Button component that provides interactive buttons with variants, sizes, and accessibility features.

## **Core Component**

### **Button Component**
```rust
#[component]
pub fn Button(
    #[prop(into, optional)] variant: Option<ButtonVariant>,
    #[prop(into, optional)] size: Option<ButtonSize>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] loading: Option<Signal<bool>>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    
    let button_class = move || {
        let mut classes = vec!["inline-flex", "items-center", "justify-center", "whitespace-nowrap", "rounded-md", "text-sm", "font-medium", "ring-offset-background", "transition-colors", "focus-visible:outline-none", "focus-visible:ring-2", "focus-visible:ring-ring", "focus-visible:ring-offset-2", "disabled:pointer-events-none", "disabled:opacity-50"];
        
        // Add variant classes
        classes.extend(variant.classes());
        
        // Add size classes
        classes.extend(size.classes());
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    let handle_click = move |_| {
        if let Some(on_click) = on_click.as_ref() {
            on_click.call(());
        }
    };
    
    let is_disabled = disabled.map(|d| d.get()).unwrap_or(false);
    let is_loading = loading.map(|l| l.get()).unwrap_or(false);
    
    view! {
        <button
            class=button_class
            id=id
            disabled=is_disabled || is_loading
            on:click=handle_click
            type="button"
        >
            if is_loading {
                <span class="mr-2 h-4 w-4 animate-spin">
                    <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M21 12a9 9 0 11-6.219-8.56"/>
                    </svg>
                </span>
            }
            {children}
        </button>
    }
}
```

## **Supporting Types**

### **ButtonVariant**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl ButtonVariant {
    pub fn classes(&self) -> Vec<&'static str> {
        match self {
            ButtonVariant::Default => vec!["bg-primary", "text-primary-foreground", "hover:bg-primary/90"],
            ButtonVariant::Destructive => vec!["bg-destructive", "text-destructive-foreground", "hover:bg-destructive/90"],
            ButtonVariant::Outline => vec!["border", "border-input", "bg-background", "hover:bg-accent", "hover:text-accent-foreground"],
            ButtonVariant::Secondary => vec!["bg-secondary", "text-secondary-foreground", "hover:bg-secondary/80"],
            ButtonVariant::Ghost => vec!["hover:bg-accent", "hover:text-accent-foreground"],
            ButtonVariant::Link => vec!["text-primary", "underline-offset-4", "hover:underline"],
        }
    }
}
```

### **ButtonSize**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Default
    }
}

impl ButtonSize {
    pub fn classes(&self) -> Vec<&'static str> {
        match self {
            ButtonSize::Default => vec!["h-10", "px-4", "py-2"],
            ButtonSize::Sm => vec!["h-9", "rounded-md", "px-3"],
            ButtonSize::Lg => vec!["h-11", "rounded-md", "px-8"],
            ButtonSize::Icon => vec!["h-10", "w-10"],
        }
    }
}
```

## **Enhanced Button Features**

### **Button with Loading State**
```rust
#[component]
pub fn LoadingButton(
    #[prop(into, optional)] variant: Option<ButtonVariant>,
    #[prop(into, optional)] size: Option<ButtonSize>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false);
    
    let handle_click = move |_| {
        if !is_loading.get() {
            set_is_loading.set(true);
            if let Some(on_click) = on_click.as_ref() {
                on_click.call(());
            }
            // Simulate async operation
            set_timeout(move || {
                set_is_loading.set(false);
            }, 2000);
        }
    };
    
    view! {
        <Button
            variant=variant
            size=size
            class=class
            loading=is_loading
            on_click=handle_click
        >
            {children}
        </Button>
    }
}
```

### **Button with Icon**
```rust
#[component]
pub fn IconButton(
    #[prop(into, optional)] variant: Option<ButtonVariant>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] icon: Option<Children>,
) -> impl IntoView {
    view! {
        <Button
            variant=variant
            size=ButtonSize::Icon
            class=class
            on_click=on_click
        >
            {icon}
        </Button>
    }
}
```

### **Button Group**
```rust
#[component]
pub fn ButtonGroup(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let group_class = move || {
        let mut classes = vec!["inline-flex", "items-center", "justify-center", "rounded-md", "text-sm", "font-medium", "ring-offset-background", "transition-colors", "focus-visible:outline-none", "focus-visible:ring-2", "focus-visible:ring-ring", "focus-visible:ring-offset-2"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=group_class
            role="group"
        >
            {children}
        </div>
    }
}
```

## **Usage Examples**

### **Basic Button**
```rust
view! {
    <Button on_click=move |_| println!("Button clicked!")>
        "Click me"
    </Button>
}
```

### **Button with Variants**
```rust
view! {
    <div class="space-x-2">
        <Button variant=ButtonVariant::Default>"Default"</Button>
        <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
        <Button variant=ButtonVariant::Outline>"Outline"</Button>
        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
        <Button variant=ButtonVariant::Link>"Link"</Button>
    </div>
}
```

### **Button with Sizes**
```rust
view! {
    <div class="space-x-2">
        <Button size=ButtonSize::Sm>"Small"</Button>
        <Button size=ButtonSize::Default>"Default"</Button>
        <Button size=ButtonSize::Lg>"Large"</Button>
        <Button size=ButtonSize::Icon>
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
        </Button>
    </div>
}
```

### **Button with Loading State**
```rust
let (is_loading, set_is_loading) = signal(false);

let handle_async_click = move |_| {
    set_is_loading.set(true);
    // Simulate async operation
    set_timeout(move || {
        set_is_loading.set(false);
    }, 2000);
};

view! {
    <Button
        loading=is_loading
        on_click=handle_async_click
    >
        "Save Changes"
    </Button>
}
```

### **Button with Custom Styling**
```rust
view! {
    <Button
        class="bg-gradient-to-r from-purple-500 to-pink-500 text-white hover:from-purple-600 hover:to-pink-600"
        on_click=move |_| println!("Gradient button clicked!")
    >
        "Gradient Button"
    </Button>
}
```

### **Button Group**
```rust
view! {
    <ButtonGroup>
        <Button variant=ButtonVariant::Outline class="rounded-r-none">"Left"</Button>
        <Button variant=ButtonVariant::Outline class="rounded-none border-l-0">"Middle"</Button>
        <Button variant=ButtonVariant::Outline class="rounded-l-none border-l-0">"Right"</Button>
    </ButtonGroup>
}
```

## **Accessibility Features**

### **Keyboard Navigation**
- **Tab**: Focus management
- **Enter/Space**: Activate button
- **Escape**: Cancel action (if applicable)

### **ARIA Attributes**
- `role="button"`: Button role
- `aria-disabled`: Disabled state
- `aria-pressed`: Toggle state (if applicable)
- `aria-label`: Accessible label

### **Screen Reader Support**
- Proper labeling
- State announcements
- Focus management

### **Visual Indicators**
- Focus rings
- Hover states
- Disabled states
- Loading indicators

---

**File Size**: 299 lines
**Priority**: 🟢 **P2 - WORKING**
**Dependencies**: leptos
