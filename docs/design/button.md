# Button Component Design Specification

## Overview & Purpose

The Button component is the primary interactive element for triggering actions in the UI. It serves as the foundation for user interactions and must be highly reliable, accessible, and performant.

**Component Type**: Interactive/Action  
**Priority**: P0 (Critical - used everywhere)  
**Dependencies**: None (foundation component)

## API Specification

### Props Interface
```rust
#[derive(Props, PartialEq)]
pub struct ButtonProps {
    /// Visual style variant
    #[prop(into, optional)]
    pub variant: MaybeProp<ButtonVariant>,
    
    /// Size variant
    #[prop(into, optional)] 
    pub size: MaybeProp<ButtonSize>,
    
    /// Click event handler
    #[prop(optional)]
    pub on_click: Option<Callback<web_sys::MouseEvent>>,
    
    /// Disabled state
    #[prop(into, optional)]
    pub disabled: MaybeProp<bool>,
    
    /// Loading state with spinner
    #[prop(into, optional)]
    pub loading: MaybeProp<bool>,
    
    /// HTML type attribute
    #[prop(into, optional)]
    pub button_type: MaybeProp<String>,
    
    /// Custom CSS classes
    #[prop(into, optional)]
    pub class: MaybeProp<String>,
    
    /// HTML id attribute
    #[prop(into, optional)]
    pub id: MaybeProp<String>,
    
    /// Inline styles
    #[prop(into, optional)]
    pub style: MaybeProp<String>,
    
    /// Button content
    #[prop(optional)]
    pub children: Option<Children>,
}
```

### Enums
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,      // Primary action (blue)
    Destructive,  // Dangerous actions (red)  
    Outline,      // Secondary action (outlined)
    Secondary,    // Tertiary action (muted)
    Ghost,        // Minimal styling (transparent)
    Link,         // Link-style button (underlined)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Default,  // Standard size (px-4 py-2)
    Small,    // Compact size (px-3 py-1.5)  
    Large,    // Prominent size (px-8 py-3)
    Icon,     // Square icon button (p-2)
}
```

## Behavioral Requirements

### Core Behaviors
1. **Click Handling**: Executes `on_click` callback when activated
2. **Keyboard Support**: Responds to Enter and Space keys
3. **Focus Management**: Proper focus indicators and tab order
4. **Disabled State**: Prevents interaction and shows disabled styling
5. **Loading State**: Shows spinner and prevents additional clicks

### State Transitions
```
[Idle] --click--> [Processing] --complete--> [Idle]
[Idle] --disabled--> [Disabled] --enabled--> [Idle]  
[Any State] --loading--> [Loading] --complete--> [Previous State]
```

### Event Handling
- **Mouse Events**: click, mousedown, mouseup, mouseenter, mouseleave
- **Keyboard Events**: keydown (Enter/Space), keyup
- **Focus Events**: focus, blur, focusin, focusout
- **Touch Events**: touchstart, touchend (mobile support)

## Accessibility Requirements

### WCAG 2.1 AA Compliance
- **Role**: Implicit `button` role (or explicit if needed)
- **Labels**: Accessible name via content or `aria-label`
- **States**: `aria-disabled`, `aria-pressed` for toggle buttons
- **Focus**: Visible focus indicator (2px outline)
- **Contrast**: 4.5:1 minimum for text, 3:1 for non-text

### Keyboard Navigation
- **Tab**: Focuses the button
- **Enter/Space**: Activates the button  
- **Escape**: Cancels focus (in some contexts)

### Screen Reader Support
```html
<button 
  type="button"
  aria-label="Save changes"
  aria-disabled="false"
  aria-describedby="save-help-text">
  Save
</button>
```

## Styling & Theming

### Base Styles
```rust
const BASE_CLASSES: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
```

### Variant Styles
```rust
fn variant_classes(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90", 
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    }
}
```

### Size Classes
```rust
fn size_classes(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Small => "h-9 rounded-md px-3 text-xs", 
        ButtonSize::Large => "h-11 rounded-md px-8",
        ButtonSize::Icon => "h-10 w-10",
    }
}
```

### Loading State
```rust
// Add spinner component when loading=true
view! {
    <button class=computed_classes disabled=is_loading_or_disabled>
        {move || if loading.get() {
            view! { <Spinner class="mr-2 h-4 w-4" /> }.into_any()
        } else {
            children.into_any()
        }}
    </button>
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
    fn renders_default_button() {
        let result = render_component(|| {
            view! { <Button>"Click me"</Button> }
        });
        
        assert_element_exists(&result, "button");
        assert_element_text(&result, "button", "Click me");
        assert_element_has_class(&result, "button", "bg-primary");
    }
    
    #[wasm_bindgen_test] 
    fn handles_click_events() {
        let clicked = create_rw_signal(false);
        
        let result = render_component(|| {
            view! {
                <Button on_click=move |_| clicked.set(true)>
                    "Click me"  
                </Button>
            }
        });
        
        click_element(&result, "button");
        assert!(clicked.get());
    }
    
    #[wasm_bindgen_test]
    fn disables_when_disabled_prop_true() {
        let result = render_component(|| {
            view! { <Button disabled=true>"Disabled"</Button> }
        });
        
        assert_element_disabled(&result, "button");
        assert_element_has_class(&result, "button", "opacity-50");
    }
}
```

### Integration Tests  
- Form submission integration
- Modal dialog integration
- Navigation integration
- Loading state management

### Accessibility Tests
```rust
#[wasm_bindgen_test]
async fn meets_accessibility_standards() {
    let result = render_component(|| {
        view! { <Button>"Accessible button"</Button> }
    });
    
    // Run axe-core accessibility checks
    assert_accessible(&result).await;
    
    // Test keyboard navigation
    assert_focusable(&result, "button");
    assert_activates_on_enter(&result, "button"); 
    assert_activates_on_space(&result, "button");
}
```

### Performance Tests
```rust
#[wasm_bindgen_test] 
fn renders_within_performance_budget() {
    let start = performance::now();
    
    let _result = render_component(|| {
        view! { <Button>"Performance test"</Button> }
    });
    
    let duration = performance::now() - start;
    assert!(duration < 16.0, "Button should render in <16ms");
}
```

## Implementation Notes

### State Management
- Use `create_rw_signal` for internal state (focus, hover)
- Props should be reactive via `MaybeProp<T>`
- Memoize computed classes with `create_memo`

### Event Handling Best Practices
```rust
let handle_click = move |event: web_sys::MouseEvent| {
    if disabled.get() || loading.get() {
        return;
    }
    
    if let Some(on_click) = on_click {
        on_click.call(event);
    }
};
```

### Bundle Size Considerations
- Import only necessary Tailwind classes
- Use const strings for common class combinations
- Avoid large dependency trees

### Performance Optimizations
- Memoize class computation
- Use `Signal::derive` for reactive styling
- Minimal re-renders on prop changes

## Examples & Usage

### Basic Usage
```rust
view! {
    <Button on_click=|_| console::log!("Clicked!")>
        "Click me"
    </Button>
}
```

### Variants Showcase
```rust
view! {
    <div class="space-y-2">
        <Button variant=ButtonVariant::Default>"Primary"</Button>
        <Button variant=ButtonVariant::Destructive>"Delete"</Button>  
        <Button variant=ButtonVariant::Outline>"Cancel"</Button>
        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
        <Button variant=ButtonVariant::Link>"Link style"</Button>
    </div>
}
```

### Loading State
```rust
fn LoadingExample() -> impl IntoView {
    let loading = create_rw_signal(false);
    
    view! {
        <Button 
            loading=loading.get()
            on_click=move |_| {
                loading.set(true);
                // Simulate async operation
                set_timeout(
                    move || loading.set(false),
                    Duration::from_secs(2)
                );
            }
        >
            "Save Changes"
        </Button>
    }
}
```

### Form Integration
```rust
view! {
    <form on_submit=handle_submit>
        <Button button_type="submit" disabled=form_invalid>
            "Submit Form"
        </Button>
    </form>
}
```

### Icon Button
```rust  
view! {
    <Button 
        size=ButtonSize::Icon
        variant=ButtonVariant::Ghost
        aria_label="Close dialog"
    >
        <Icon name="x" />
    </Button>
}
```

## Migration Notes

### From v0.3.x to v0.4.x
- `onClick` prop renamed to `on_click`
- `variant` prop now uses enum instead of string
- `loading` prop added for async operations

### Breaking Changes
- Removed `asChild` prop (use composition instead)
- Size prop values changed (sm/md/lg → Small/Default/Large)
