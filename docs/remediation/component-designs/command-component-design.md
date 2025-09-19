# 🎨 **Command Component Design**

## **Overview**
Design for the Command component that provides a command palette interface with search, filtering, and keyboard navigation.

## **Core Components**

### **Command Component**
```rust
#[component]
pub fn Command(
    #[prop(into, optional)] value: Option<Signal<String>>,
    #[prop(into, optional)] on_select: Option<Callback<String>>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (search_value, set_search_value) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    
    let command_class = move || {
        let mut classes = vec!["flex", "h-full", "w-full", "flex-col", "overflow-hidden", "rounded-md", "border", "bg-popover", "text-popover-foreground"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_index.update(|i| *i += 1);
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|i| if *i > 0 { *i -= 1 });
            }
            "Enter" => {
                ev.prevent_default();
                if let Some(on_select) = on_select.as_ref() {
                    on_select.call(search_value.get());
                }
            }
            "Escape" => {
                ev.prevent_default();
                set_is_open.set(false);
            }
            _ => {}
        }
    };
    
    view! {
        <div
            class=command_class
            id=id
            on:keydown=handle_keydown
            role="combobox"
            aria-expanded=is_open
            aria-haspopup="listbox"
        >
            {children}
        </div>
    }
}
```

### **CommandInput Component**
```rust
#[component]
pub fn CommandInput(
    #[prop(into, optional)] value: Option<Signal<String>>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
) -> impl IntoView {
    let (input_value, set_input_value) = value.unwrap_or_else(|| signal(String::new()));
    
    let input_class = move || {
        let mut classes = vec!["flex", "h-11", "w-full", "rounded-md", "bg-transparent", "py-3", "text-sm", "outline-none", "placeholder:text-muted-foreground", "disabled:cursor-not-allowed", "disabled:opacity-50"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    let handle_input = move |ev: leptos::ev::InputEvent| {
        let value = event_target_value(&ev);
        set_input_value.set(value.clone());
        if let Some(on_change) = on_change.as_ref() {
            on_change.call(value);
        }
    };
    
    view! {
        <input
            value=input_value
            placeholder=placeholder.unwrap_or_else(|| "Search...".to_string())
            disabled=disabled.map(|d| d.get()).unwrap_or(false)
            class=input_class
            on:input=handle_input
            autocomplete="off"
            autocorrect="off"
            spellcheck="false"
        />
    }
}
```

### **CommandList Component**
```rust
#[component]
pub fn CommandList(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let list_class = move || {
        let mut classes = vec!["max-h-[300px]", "overflow-y-auto", "overflow-x-hidden"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=list_class
            role="listbox"
        >
            {children}
        </div>
    }
}
```

### **CommandEmpty Component**
```rust
#[component]
pub fn CommandEmpty(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let empty_class = move || {
        let mut classes = vec!["py-6", "text-center", "text-sm"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=empty_class
            role="status"
            aria-live="polite"
        >
            {children.unwrap_or_else(|| view! { "No results found." })}
        </div>
    }
}
```

### **CommandGroup Component**
```rust
#[component]
pub fn CommandGroup(
    #[prop(into, optional)] heading: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let group_class = move || {
        let mut classes = vec!["overflow-hidden", "p-1", "text-foreground"];
        
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
            if let Some(heading) = heading {
                <div class="px-2 py-1.5 text-xs font-semibold text-muted-foreground">
                    {heading}
                </div>
            }
            {children}
        </div>
    }
}
```

### **CommandItem Component**
```rust
#[component]
pub fn CommandItem(
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] on_select: Option<Callback<String>>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_selected, set_is_selected) = signal(false);
    
    let item_class = move || {
        let mut classes = vec!["relative", "flex", "cursor-default", "select-none", "items-center", "rounded-sm", "px-2", "py-1.5", "text-sm", "outline-none", "aria-selected:bg-accent", "aria-selected:text-accent-foreground", "data-[disabled]:pointer-events-none", "data-[disabled]:opacity-50"];
        
        if is_selected.get() {
            classes.push("bg-accent", "text-accent-foreground");
        }
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    let handle_click = move |_| {
        if let Some(value) = value.as_ref() {
            if let Some(on_select) = on_select.as_ref() {
                on_select.call(value.clone());
            }
        }
    };
    
    view! {
        <div
            class=item_class
            on:click=handle_click
            role="option"
            aria-selected=is_selected
            data-disabled=disabled.map(|d| d.get()).unwrap_or(false)
        >
            {children}
        </div>
    }
}
```

### **CommandShortcut Component**
```rust
#[component]
pub fn CommandShortcut(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] children: Option<Children>,
) -> impl IntoView {
    let shortcut_class = move || {
        let mut classes = vec!["ml-auto", "text-xs", "tracking-widest", "opacity-60"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <span
            class=shortcut_class
        >
            {children}
        </span>
    }
}
```

### **CommandSeparator Component**
```rust
#[component]
pub fn CommandSeparator(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let separator_class = move || {
        let mut classes = vec!["-mx-1", "h-px", "bg-border"];
        
        if let Some(custom_class) = class.as_ref() {
            classes.push(custom_class);
        }
        
        classes.join(" ")
    };
    
    view! {
        <div
            class=separator_class
            role="separator"
        />
    }
}
```

## **Usage Examples**

### **Basic Command Palette**
```rust
let (search_value, set_search_value) = signal(String::new());
let (selected_value, set_selected_value) = signal(String::new());

let handle_select = move |value: String| {
    set_selected_value.set(value);
    println!("Selected: {}", value);
};

view! {
    <Command
        value=search_value
        on_select=handle_select
        class="w-96"
    >
        <CommandInput
            value=search_value
            on_change=move |value| set_search_value.set(value)
            placeholder="Search commands..."
        />
        <CommandList>
            <CommandItem
                value="new-file".to_string()
                on_select=handle_select
            >
                "New File"
                <CommandShortcut>"⌘N"</CommandShortcut>
            </CommandItem>
            <CommandItem
                value="save-file".to_string()
                on_select=handle_select
            >
                "Save File"
                <CommandShortcut>"⌘S"</CommandShortcut>
            </CommandItem>
        </CommandList>
    </Command>
}
```

### **Command with Groups**
```rust
view! {
    <Command class="w-96">
        <CommandInput placeholder="Search..." />
        <CommandList>
            <CommandGroup heading="File">
                <CommandItem value="new">"New File"</CommandItem>
                <CommandItem value="open">"Open File"</CommandItem>
                <CommandItem value="save">"Save File"</CommandItem>
            </CommandGroup>
            <CommandSeparator />
            <CommandGroup heading="Edit">
                <CommandItem value="undo">"Undo"</CommandItem>
                <CommandItem value="redo">"Redo"</CommandItem>
                <CommandItem value="cut">"Cut"</CommandItem>
            </CommandGroup>
        </CommandList>
    </Command>
}
```

### **Command with Empty State**
```rust
view! {
    <Command class="w-96">
        <CommandInput placeholder="Search..." />
        <CommandList>
            <CommandEmpty>
                "No commands found. Try a different search term."
            </CommandEmpty>
        </CommandList>
    </Command>
}
```

## **Accessibility Features**

### **Keyboard Navigation**
- **Arrow Keys**: Navigate through items
- **Enter**: Select current item
- **Escape**: Close command palette
- **Tab**: Focus management

### **ARIA Attributes**
- `role="combobox"`: Main command component
- `role="listbox"`: Command list
- `role="option"`: Command items
- `aria-expanded`: Open/closed state
- `aria-selected`: Selected item state

### **Screen Reader Support**
- Proper labeling and descriptions
- State announcements
- Focus management

---

**File Size**: 298 lines
**Priority**: 🔴 **P0 - CRITICAL**
**Dependencies**: leptos
