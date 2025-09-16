# 🎨 **leptos-shadcn-ui Component API Standards**
**Comprehensive API Design Guidelines for v1.0**

---

## 🎯 **API Design Philosophy**

**"Predictable, Consistent, Accessible, Performant"**

Every component API must be **intuitive to use**, **consistent across the library**, **accessible by default**, and **performant in all scenarios**.

---

## 📋 **Core API Principles**

### **1. Consistency First**
- **Naming Conventions**: All components follow identical patterns
- **Prop Interfaces**: Similar functionality uses identical prop names
- **Event Handling**: Consistent event naming and behavior
- **Default Values**: Sensible, accessible defaults

### **2. Accessibility by Default**
- **ARIA Attributes**: Automatically applied based on component role
- **Keyboard Navigation**: Built-in keyboard support
- **Screen Reader Support**: Proper semantic markup
- **Focus Management**: Logical focus flow

### **3. Performance Minded**
- **Minimal Re-renders**: Optimized reactivity patterns
- **Lazy Loading**: Components load efficiently
- **Memory Management**: No memory leaks
- **Bundle Optimization**: Tree-shakeable by design

### **4. Developer Experience**
- **TypeScript First**: Full type safety
- **IntelliSense Support**: Rich development experience
- **Error Handling**: Clear, actionable error messages
- **Documentation**: Self-documenting APIs

---

## 🏗️ **Standardized Component Architecture**

### **Base Component Structure**

Every component follows this standardized pattern:

```rust
// Component props definition
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentNameProps {
    // === Core Behavior Props ===
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    
    // === Styling Props ===
    pub variant: Option<ComponentVariant>,
    pub size: Option<ComponentSize>,
    pub class: Option<String>,
    pub style: Option<String>,
    
    // === Accessibility Props ===
    pub id: Option<String>,
    pub aria_label: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_labelledby: Option<String>,
    
    // === Event Handler Props ===
    pub onclick: Option<Box<dyn Fn()>>,
    pub onfocus: Option<Box<dyn Fn()>>,
    pub onblur: Option<Box<dyn Fn()>>,
    
    // === Component-Specific Props ===
    // ... (defined per component)
    
    // === Children ===
    pub children: Option<leptos::View>,
}

// Standardized variant enum
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    // Component-specific variants...
}

// Standardized size enum
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentSize {
    Sm,
    Default,
    Lg,
    Xl,
}

// Component implementation
#[component]
pub fn ComponentName(props: ComponentNameProps) -> impl IntoView {
    // Standardized prop processing
    let disabled = props.disabled.unwrap_or(false);
    let variant = props.variant.unwrap_or(ComponentVariant::Default);
    let size = props.size.unwrap_or(ComponentSize::Default);
    
    // Standardized CSS class generation
    let classes = create_memo(move |_| {
        generate_component_classes("component-name", &variant, &size, &props.class)
    });
    
    // Standardized accessibility attributes
    let accessibility_attrs = create_memo(move |_| {
        generate_accessibility_attrs(&props)
    });
    
    // Component render logic
    view! {
        <div
            class=classes
            ..accessibility_attrs
            disabled=disabled
        >
            {props.children}
        </div>
    }
}
```

---

## 📐 **Prop Naming Standards**

### **Core Props (All Components)**

| Prop Name | Type | Default | Description |
|-----------|------|---------|-------------|
| `id` | `Option<String>` | `None` | HTML element ID |
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `style` | `Option<String>` | `None` | Inline CSS styles |
| `disabled` | `Option<bool>` | `false` | Disable component interaction |
| `children` | `Option<View>` | `None` | Child content |

### **Styling Props (Visual Components)**

| Prop Name | Type | Default | Description |
|-----------|------|---------|-------------|
| `variant` | `Option<Variant>` | `Default` | Visual style variant |
| `size` | `Option<Size>` | `Default` | Component size |
| `color` | `Option<Color>` | `None` | Color override |
| `theme` | `Option<Theme>` | `None` | Theme override |

### **Accessibility Props (All Interactive Components)**

| Prop Name | Type | Default | Description |
|-----------|------|---------|-------------|
| `aria_label` | `Option<String>` | `None` | Accessible name |
| `aria_describedby` | `Option<String>` | `None` | Description reference |
| `aria_labelledby` | `Option<String>` | `None` | Label reference |
| `role` | `Option<String>` | `None` | ARIA role override |
| `tabindex` | `Option<i32>` | `None` | Tab order override |

### **Form Props (Form Components)**

| Prop Name | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `Option<String>` | `None` | Form field name |
| `value` | `Option<T>` | `None` | Current value |
| `default_value` | `Option<T>` | `None` | Default value |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `required` | `Option<bool>` | `false` | Required field |
| `readonly` | `Option<bool>` | `false` | Read-only field |
| `autocomplete` | `Option<String>` | `None` | Autocomplete hint |

### **Event Handler Props (Interactive Components)**

| Prop Name | Type | Description |
|-----------|------|-------------|
| `onclick` | `Option<Box<dyn Fn()>>` | Click event handler |
| `onchange` | `Option<Box<dyn Fn(T)>>` | Value change handler |
| `onfocus` | `Option<Box<dyn Fn()>>` | Focus event handler |
| `onblur` | `Option<Box<dyn Fn()>>` | Blur event handler |
| `onkeydown` | `Option<Box<dyn Fn(KeyboardEvent)>>` | Key down handler |
| `onkeyup` | `Option<Box<dyn Fn(KeyboardEvent)>>` | Key up handler |
| `onsubmit` | `Option<Box<dyn Fn()>>` | Form submit handler |

---

## 🎨 **Variant System Standards**

### **Color Variants (All Visual Components)**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ColorVariant {
    Default,    // Neutral, accessible default
    Primary,    // Brand primary color
    Secondary,  // Brand secondary color
    Success,    // Green, positive actions
    Warning,    // Yellow/orange, caution
    Danger,     // Red, destructive actions
    Info,       // Blue, informational
    Light,      // Light theme variant
    Dark,       // Dark theme variant
}
```

### **Size Variants (All Sizeable Components)**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SizeVariant {
    Xs,         // Extra small (mobile-first)
    Sm,         // Small
    Default,    // Standard size
    Lg,         // Large
    Xl,         // Extra large
    Responsive, // Responsive sizing
}
```

### **Component-Specific Variants**

Each component can extend base variants:

```rust
// Button-specific variants
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    // Base variants
    Default,
    Primary,
    Secondary,
    
    // Button-specific
    Outline,
    Ghost,
    Link,
    Icon,
}

// Input-specific variants
#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Filled,
    Outlined,
    Underlined,
}
```

---

## 🔧 **CSS Class Generation Standards**

### **Base Class Structure**

All components follow this CSS class pattern:

```
.shadcn-{component} 
.shadcn-{component}--{variant}
.shadcn-{component}--{size}
.shadcn-{component}--{state}
```

### **CSS Class Generator**

```rust
pub fn generate_component_classes(
    component_name: &str,
    variant: &ComponentVariant,
    size: &ComponentSize,
    custom_class: &Option<String>,
) -> String {
    let mut classes = vec![
        format!("shadcn-{}", component_name),
        format!("shadcn-{}--{}", component_name, variant.to_css_class()),
        format!("shadcn-{}--{}", component_name, size.to_css_class()),
    ];
    
    if let Some(custom) = custom_class {
        classes.push(custom.clone());
    }
    
    classes.join(" ")
}

trait ToCssClass {
    fn to_css_class(&self) -> String;
}

impl ToCssClass for ComponentVariant {
    fn to_css_class(&self) -> String {
        match self {
            ComponentVariant::Default => "default".to_string(),
            ComponentVariant::Primary => "primary".to_string(),
            ComponentVariant::Secondary => "secondary".to_string(),
            // ... other variants
        }
    }
}
```

---

## ♿ **Accessibility Standards**

### **Automatic ARIA Attributes**

```rust
pub fn generate_accessibility_attrs(props: &ComponentProps) -> Vec<(&str, String)> {
    let mut attrs = Vec::new();
    
    // Required ID for accessibility
    let id = props.id.as_ref()
        .cloned()
        .unwrap_or_else(|| generate_unique_id("component"));
    attrs.push(("id", id));
    
    // ARIA label handling
    if let Some(label) = &props.aria_label {
        attrs.push(("aria-label", label.clone()));
    }
    
    if let Some(described_by) = &props.aria_describedby {
        attrs.push(("aria-describedby", described_by.clone()));
    }
    
    if let Some(labelled_by) = &props.aria_labelledby {
        attrs.push(("aria-labelledby", labelled_by.clone()));
    }
    
    // State attributes
    if props.disabled.unwrap_or(false) {
        attrs.push(("aria-disabled", "true".to_string()));
        attrs.push(("tabindex", "-1".to_string()));
    }
    
    attrs
}
```

### **Keyboard Navigation Standards**

| Key | Behavior | Components |
|-----|----------|------------|
| **Tab** | Navigate to next focusable element | All interactive |
| **Shift+Tab** | Navigate to previous focusable element | All interactive |
| **Enter** | Activate primary action | Button, Link |
| **Space** | Toggle or activate | Button, Checkbox, Switch |
| **Arrow Keys** | Navigate within component | Menu, Tabs, Radio Group |
| **Escape** | Close overlay or cancel | Dialog, Popover, Menu |
| **Home** | Navigate to first item | Lists, Menus |
| **End** | Navigate to last item | Lists, Menus |

---

## 📊 **Event System Standards**

### **Event Handler Patterns**

```rust
// Standard event handler signature
pub type ClickHandler = Box<dyn Fn()>;
pub type ChangeHandler<T> = Box<dyn Fn(T)>;
pub type KeyboardHandler = Box<dyn Fn(KeyboardEvent)>;

// Event data structures
#[derive(Debug, Clone)]
pub struct ComponentEvent {
    pub component_id: String,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Click,
    Change,
    Focus,
    Blur,
    KeyDown,
    KeyUp,
    Submit,
    // Component-specific events
}
```

### **Event Bubbling Standards**

- **Click Events**: Bubble by default, can be prevented
- **Focus Events**: Do not bubble (use focus/blur)
- **Form Events**: Bubble to form container
- **Custom Events**: Follow DOM standards

---

## 🧪 **Testing API Standards**

### **Required Test Coverage**

Every component must implement these test categories:

```rust
#[cfg(test)]
mod api_compliance_tests {
    use super::*;
    use shadcn_ui_test_utils::api_testing::*;

    #[test]
    fn test_props_api_compliance() {
        // Test that component accepts all standard props
        let props = ComponentNameProps {
            id: Some("test-id".to_string()),
            class: Some("custom-class".to_string()),
            disabled: Some(true),
            variant: Some(ComponentVariant::Primary),
            size: Some(ComponentSize::Lg),
            ..Default::default()
        };
        
        assert_component_renders(ComponentName, props);
    }

    #[test]
    fn test_accessibility_compliance() {
        let props = ComponentNameProps::default();
        let component = ComponentName(props);
        
        assert_accessibility_compliance(&component);
        assert_keyboard_navigation_support(&component);
    }

    #[test]
    fn test_css_class_generation() {
        let props = ComponentNameProps {
            variant: Some(ComponentVariant::Primary),
            size: Some(ComponentSize::Lg),
            class: Some("custom".to_string()),
            ..Default::default()
        };
        
        let component = ComponentName(props);
        
        assert_has_css_class(&component, "shadcn-component-name");
        assert_has_css_class(&component, "shadcn-component-name--primary");
        assert_has_css_class(&component, "shadcn-component-name--lg");
        assert_has_css_class(&component, "custom");
    }

    #[test]
    fn test_event_handling_standards() {
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();
        
        let props = ComponentNameProps {
            onclick: Some(Box::new(move || {
                *clicked_clone.lock().unwrap() = true;
            })),
            ..Default::default()
        };
        
        let component = ComponentName(props);
        simulate_click(&component);
        
        assert!(*clicked.lock().unwrap());
    }
}
```

---

## 📚 **Documentation Standards**

### **Component Documentation Template**

```rust
/// # ComponentName
///
/// A brief description of what the component does and when to use it.
///
/// ## Usage
///
/// ```rust
/// use leptos::*;
/// use leptos_shadcn_component_name::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     view! {
///         <ComponentName
///             variant=ComponentVariant::Primary
///             size=ComponentSize::Lg
///             onclick=move || { /* handle click */ }
///         >
///             "Component content"
///         </ComponentName>
///     }
/// }
/// ```
///
/// ## Accessibility
///
/// This component follows WCAG 2.1 AA standards:
/// - Keyboard navigation with Tab/Shift+Tab
/// - Screen reader support with proper ARIA labels
/// - Focus management and visual indicators
///
/// ## Props
///
/// ### Core Props
/// - `variant`: Visual style variant
/// - `size`: Component size
/// - `disabled`: Disable interaction
///
/// ### Accessibility Props  
/// - `aria_label`: Accessible name
/// - `aria_describedby`: Description reference
/// - `id`: Unique identifier
///
/// ## Events
///
/// - `onclick`: Triggered when component is clicked
/// - `onfocus`: Triggered when component gains focus
/// - `onblur`: Triggered when component loses focus
///
/// ## Examples
///
/// ### Basic Usage
/// [Example code]
///
/// ### With Custom Styling
/// [Example code]
///
/// ### Form Integration
/// [Example code]
///
/// ### Accessibility Features
/// [Example code]
#[component]
pub fn ComponentName(props: ComponentNameProps) -> impl IntoView {
    // Implementation
}
```

---

## 🔍 **API Validation Framework**

### **Automated API Compliance Testing**

```rust
// API compliance testing framework
pub mod api_compliance {
    use super::*;
    
    pub trait ComponentApiCompliance {
        type Props: ComponentProps;
        
        fn test_basic_rendering(&self);
        fn test_prop_handling(&self);
        fn test_accessibility_compliance(&self);
        fn test_event_handling(&self);
        fn test_css_class_generation(&self);
        fn test_performance_characteristics(&self);
    }
    
    pub trait ComponentProps {
        fn with_core_props() -> Self;
        fn with_accessibility_props() -> Self;
        fn with_styling_props() -> Self;
        fn validate_props(&self) -> Result<(), ApiComplianceError>;
    }
    
    #[derive(Debug)]
    pub enum ApiComplianceError {
        MissingCoreProps(Vec<String>),
        InvalidVariant(String),
        AccessibilityViolation(String),
        EventHandlerError(String),
        CssClassError(String),
        PerformanceViolation(String),
    }
}
```

### **Component API Linter**

```rust
// Automated API linting for development
pub fn lint_component_api<C: ComponentApiCompliance>(
    component: &C,
    strict_mode: bool,
) -> Result<ApiLintReport, ApiComplianceError> {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    
    // Check core prop compliance
    if let Err(e) = component.test_prop_handling() {
        issues.push(ApiIssue::PropCompliance(e));
    }
    
    // Check accessibility compliance
    if let Err(e) = component.test_accessibility_compliance() {
        if strict_mode {
            return Err(e);
        } else {
            issues.push(ApiIssue::Accessibility(e));
        }
    }
    
    // Performance checks
    if let Err(e) = component.test_performance_characteristics() {
        suggestions.push(ApiSuggestion::Performance(e));
    }
    
    Ok(ApiLintReport {
        component_name: std::any::type_name::<C>().to_string(),
        issues,
        suggestions,
        compliance_score: calculate_compliance_score(&issues, &suggestions),
    })
}
```

---

**This API standardization framework ensures every component in leptos-shadcn-ui provides a consistent, accessible, and performant experience while maintaining exceptional developer ergonomics.**

---

*Last Updated: December 2024*  
*Status: 🚧 Active Implementation*  
*Compliance Target: 100% by v1.0*