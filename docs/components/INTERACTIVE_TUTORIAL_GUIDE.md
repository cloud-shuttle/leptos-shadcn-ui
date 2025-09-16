# Interactive Tutorial Guide - New York Theme Components

## 🎯 Overview

This comprehensive tutorial guide will walk you through building interactive applications using the New York theme variants of our Leptos shadcn/ui components. You'll learn how to create engaging user interfaces with proper state management, form validation, and component interactions.

## 📚 Table of Contents

1. [Getting Started](#getting-started)
2. [Component Basics](#component-basics)
3. [State Management](#state-management)
4. [Form Handling](#form-handling)
5. [Interactive Features](#interactive-features)
6. [Advanced Patterns](#advanced-patterns)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## 🚀 Getting Started

### Prerequisites

Before starting this tutorial, make sure you have:

- Rust 1.70+ installed
- Leptos 0.8+ framework
- Basic understanding of Rust and Leptos
- Familiarity with HTML/CSS concepts

### Project Setup

1. **Create a new Leptos project:**
```bash
cargo new my-leptos-app --bin
cd my-leptos-app
```

2. **Add dependencies to Cargo.toml:**
```toml
[dependencies]
leptos = "0.8"
leptos-shadcn-button = "0.8"
leptos-shadcn-card = "0.8"
leptos-shadcn-input = "0.8"
leptos-shadcn-form = "0.8"
```

3. **Import New York theme components:**
```rust
use leptos_shadcn_button::new_york::{Button as ButtonNewYork, ButtonVariant as ButtonVariantNewYork};
use leptos_shadcn_card::new_york::{Card as CardNewYork, CardHeader as CardHeaderNewYork};
use leptos_shadcn_input::new_york::Input as InputNewYork;
```

## 🧩 Component Basics

### Button Components

The New York theme provides several button variants with consistent styling:

```rust
#[component]
pub fn ButtonShowcase() -> impl IntoView {
    view! {
        <div class="space-y-4">
            // Default button
            <ButtonNewYork variant=ButtonVariantNewYork::Default>
                "Default Button"
            </ButtonNewYork>
            
            // Destructive button
            <ButtonNewYork variant=ButtonVariantNewYork::Destructive>
                "Delete Item"
            </ButtonNewYork>
            
            // Outline button
            <ButtonNewYork variant=ButtonVariantNewYork::Outline>
                "Cancel"
            </ButtonNewYork>
            
            // Secondary button
            <ButtonNewYork variant=ButtonVariantNewYork::Secondary>
                "Secondary Action"
            </ButtonNewYork>
            
            // Ghost button
            <ButtonNewYork variant=ButtonVariantNewYork::Ghost>
                "Ghost Button"
            </ButtonNewYork>
            
            // Link button
            <ButtonNewYork variant=ButtonVariantNewYork::Link>
                "Learn More"
            </ButtonNewYork>
        </div>
    }
}
```

### Card Components

Cards provide structured content containers:

```rust
#[component]
pub fn CardShowcase() -> impl IntoView {
    view! {
        <CardNewYork class="max-w-md">
            <CardHeaderNewYork>
                <CardTitleNewYork>"Card Title"</CardTitleNewYork>
                <CardDescriptionNewYork>
                    "This is a description of the card content."
                </CardDescriptionNewYork>
            </CardHeaderNewYork>
            <CardContentNewYork>
                <p>"This is the main content of the card."</p>
            </CardContentNewYork>
            <CardFooterNewYork>
                <ButtonNewYork variant=ButtonVariantNewYork::Default>
                    "Action"
                </ButtonNewYork>
            </CardFooterNewYork>
        </CardNewYork>
    }
}
```

### Input Components

Input components handle user data entry:

```rust
#[component]
pub fn InputShowcase() -> impl IntoView {
    let (value, set_value) = signal("".to_string());
    
    view! {
        <div class="space-y-4">
            <InputNewYork
                value=move || value.get()
                on_change=move |new_value| set_value.set(new_value)
                placeholder="Enter text here"
            />
            
            <InputNewYork
                value=move || value.get()
                on_change=move |new_value| set_value.set(new_value)
                placeholder="Email address"
                input_type="email"
            />
            
            <InputNewYork
                value=move || value.get()
                on_change=move |new_value| set_value.set(new_value)
                placeholder="Password"
                input_type="password"
            />
        </div>
    }
}
```

## 🔄 State Management

### Basic State with Signals

Leptos uses signals for reactive state management:

```rust
#[component]
pub fn StateManagementExample() -> impl IntoView {
    // Create reactive signals
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("".to_string());
    let (is_visible, set_is_visible) = signal(true);
    
    // Derived state
    let double_count = Signal::derive(move || count.get() * 2);
    
    view! {
        <div class="space-y-4">
            // Counter example
            <div class="p-4 border rounded-lg">
                <h3 class="text-lg font-semibold mb-2">"Counter: " {count}</h3>
                <p class="text-sm text-gray-600">"Double: " {double_count}</p>
                <div class="mt-2 space-x-2">
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Default
                        on_click=move |_| set_count.update(|c| *c += 1)
                    >
                        "Increment"
                    </ButtonNewYork>
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Outline
                        on_click=move |_| set_count.update(|c| *c -= 1)
                    >
                        "Decrement"
                    </ButtonNewYork>
                </div>
            </div>
            
            // Name input
            <div class="p-4 border rounded-lg">
                <label class="block text-sm font-medium mb-2">"Name:"</label>
                <InputNewYork
                    value=move || name.get()
                    on_change=move |new_name| set_name.set(new_name)
                    placeholder="Enter your name"
                />
                <p class="mt-2 text-sm text-gray-600">
                    "Hello, " {move || if name.get().is_empty() { "Anonymous".to_string() } else { name.get() }} "!"
                </p>
            </div>
            
            // Visibility toggle
            <div class="p-4 border rounded-lg">
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Default
                    on_click=move |_| set_is_visible.update(|v| *v = !*v)
                >
                    {move || if is_visible.get() { "Hide" } else { "Show" }} " Content"
                </ButtonNewYork>
                
                {move || if is_visible.get() {
                    view! {
                        <div class="mt-2 p-2 bg-blue-50 rounded">
                            "This content is visible!"
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }}
            </div>
        </div>
    }
}
```

### Complex State Management

For more complex applications, use structured state:

```rust
#[derive(Clone, Default)]
struct AppState {
    user: Option<User>,
    notifications: Vec<Notification>,
    theme: String,
    loading: bool,
}

#[derive(Clone)]
struct User {
    name: String,
    email: String,
    preferences: UserPreferences,
}

#[derive(Clone)]
struct UserPreferences {
    theme: String,
    notifications_enabled: bool,
}

#[component]
pub fn ComplexStateExample() -> impl IntoView {
    let (app_state, set_app_state) = signal(AppState::default());
    
    // State update functions
    let update_user = move |user: User| {
        set_app_state.update(|state| state.user = Some(user));
    };
    
    let add_notification = move |notification: Notification| {
        set_app_state.update(|state| {
            state.notifications.push(notification);
            if state.notifications.len() > 10 {
                state.notifications.remove(0);
            }
        });
    };
    
    let set_loading = move |loading: bool| {
        set_app_state.update(|state| state.loading = loading);
    };
    
    view! {
        <div class="space-y-6">
            // User info display
            {move || {
                if let Some(user) = app_state.get().user.clone() {
                    view! {
                        <CardNewYork>
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"User Profile"</CardTitleNewYork>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                <p>"Name: " {user.name}</p>
                                <p>"Email: " {user.email}</p>
                                <p>"Theme: " {user.preferences.theme}</p>
                            </CardContentNewYork>
                        </CardNewYork>
                    }
                } else {
                    view! {
                        <CardNewYork>
                            <CardContentNewYork>
                                <p class="text-gray-500">"No user logged in"</p>
                            </CardContentNewYork>
                        </CardNewYork>
                    }
                }
            }}
            
            // Loading state
            {move || if app_state.get().loading {
                view! {
                    <div class="flex items-center justify-center p-8">
                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                        <span class="ml-2">"Loading..."</span>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
```

## 📝 Form Handling

### Basic Form with Validation

```rust
#[derive(Clone, Default)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[derive(Clone)]
struct FormErrors {
    name: Option<String>,
    email: Option<String>,
    message: Option<String>,
}

impl Default for FormErrors {
    fn default() -> Self {
        Self {
            name: None,
            email: None,
            message: None,
        }
    }
}

#[component]
pub fn ContactFormExample() -> impl IntoView {
    let (form_data, set_form_data) = signal(ContactForm::default());
    let (errors, set_errors) = signal(FormErrors::default());
    let (is_submitting, set_is_submitting) = signal(false);
    let (is_submitted, set_is_submitted) = signal(false);
    
    // Validation function
    let validate_form = move || {
        let mut new_errors = FormErrors::default();
        let data = form_data.get();
        
        if data.name.trim().is_empty() {
            new_errors.name = Some("Name is required".to_string());
        }
        
        if data.email.trim().is_empty() {
            new_errors.email = Some("Email is required".to_string());
        } else if !data.email.contains('@') {
            new_errors.email = Some("Please enter a valid email".to_string());
        }
        
        if data.message.trim().is_empty() {
            new_errors.message = Some("Message is required".to_string());
        }
        
        set_errors.set(new_errors.clone());
        new_errors.name.is_none() && new_errors.email.is_none() && new_errors.message.is_none()
    };
    
    // Form submission
    let handle_submit = move |_| {
        if validate_form() {
            set_is_submitting.set(true);
            
            // Simulate API call
            set_timeout(move || {
                set_is_submitting.set(false);
                set_is_submitted.set(true);
                set_form_data.set(ContactForm::default());
            }, 2000);
        }
    };
    
    view! {
        <CardNewYork class="max-w-2xl mx-auto">
            <CardHeaderNewYork>
                <CardTitleNewYork>"Contact Us"</CardTitleNewYork>
                <CardDescriptionNewYork>
                    "Send us a message and we'll get back to you soon."
                </CardDescriptionNewYork>
            </CardHeaderNewYork>
            <CardContentNewYork>
                {move || if is_submitted.get() {
                    view! {
                        <div class="text-center py-8">
                            <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                <span class="text-green-600 text-2xl">"✓"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-2">"Message Sent!"</h3>
                            <p class="text-gray-600 mb-4">"Thank you for your message. We'll get back to you soon."</p>
                            <ButtonNewYork
                                variant=ButtonVariantNewYork::Default
                                on_click=move |_| set_is_submitted.set(false)
                            >
                                "Send Another Message"
                            </ButtonNewYork>
                        </div>
                    }
                } else {
                    view! {
                        <form class="space-y-4" on:submit=move |ev| {
                            ev.prevent_default();
                            handle_submit(());
                        }>
                            // Name field
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-gray-700">"Full Name"</label>
                                <InputNewYork
                                    value=move || form_data.get().name.clone()
                                    on_change=move |value| {
                                        set_form_data.update(|data| data.name = value);
                                    }
                                    placeholder="Enter your full name"
                                    class=move || {
                                        if errors.get().name.is_some() {
                                            "border-red-500 focus:border-red-500"
                                        } else {
                                            ""
                                        }
                                    }
                                />
                                {move || if let Some(error) = errors.get().name.clone() {
                                    view! { <p class="text-sm text-red-600">{error}</p> }
                                } else {
                                    view! { <div></div> }
                                }}
                            </div>
                            
                            // Email field
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-gray-700">"Email Address"</label>
                                <InputNewYork
                                    value=move || form_data.get().email.clone()
                                    on_change=move |value| {
                                        set_form_data.update(|data| data.email = value);
                                    }
                                    placeholder="Enter your email"
                                    input_type="email"
                                    class=move || {
                                        if errors.get().email.is_some() {
                                            "border-red-500 focus:border-red-500"
                                        } else {
                                            ""
                                        }
                                    }
                                />
                                {move || if let Some(error) = errors.get().email.clone() {
                                    view! { <p class="text-sm text-red-600">{error}</p> }
                                } else {
                                    view! { <div></div> }
                                }}
                            </div>
                            
                            // Message field
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-gray-700">"Message"</label>
                                <textarea
                                    class=format!("flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}", 
                                        if errors.get().message.is_some() { "border-red-500 focus:border-red-500" } else { "" }
                                    )
                                    placeholder="Enter your message"
                                    prop:value=move || form_data.get().message.clone()
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_form_data.update(|data| data.message = value);
                                    }
                                />
                                {move || if let Some(error) = errors.get().message.clone() {
                                    view! { <p class="text-sm text-red-600">{error}</p> }
                                } else {
                                    view! { <div></div> }
                                }}
                            </div>
                            
                            // Submit button
                            <ButtonNewYork
                                variant=ButtonVariantNewYork::Default
                                disabled=move || is_submitting.get()
                                class="w-full"
                            >
                                {move || if is_submitting.get() { "Sending..." } else { "Send Message" }}
                            </ButtonNewYork>
                        </form>
                    }
                }}
            </CardContentNewYork>
        </CardNewYork>
    }
}
```

### Dynamic Form Fields

```rust
#[derive(Clone)]
struct DynamicField {
    id: String,
    label: String,
    value: String,
    field_type: String,
}

#[component]
pub fn DynamicFormExample() -> impl IntoView {
    let (fields, set_fields) = signal(Vec::<DynamicField>::new());
    let (next_id, set_next_id) = signal(1);
    
    let add_field = move |field_type: String| {
        let id = format!("field_{}", next_id.get());
        let new_field = DynamicField {
            id: id.clone(),
            label: format!("Field {}", next_id.get()),
            value: "".to_string(),
            field_type: field_type.clone(),
        };
        
        set_fields.update(|fields| fields.push(new_field));
        set_next_id.update(|id| *id += 1);
    };
    
    let update_field_value = move |field_id: String, value: String| {
        set_fields.update(|fields| {
            if let Some(field) = fields.iter_mut().find(|f| f.id == field_id) {
                field.value = value;
            }
        });
    };
    
    let remove_field = move |field_id: String| {
        set_fields.update(|fields| {
            fields.retain(|f| f.id != field_id);
        });
    };
    
    view! {
        <CardNewYork class="max-w-4xl mx-auto">
            <CardHeaderNewYork>
                <CardTitleNewYork>"Dynamic Form Builder"</CardTitleNewYork>
                <CardDescriptionNewYork>
                    "Add and remove form fields dynamically"
                </CardDescriptionNewYork>
            </CardHeaderNewYork>
            <CardContentNewYork>
                // Add field buttons
                <div class="flex flex-wrap gap-2 mb-6">
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Default
                        on_click=move |_| add_field("text".to_string())
                    >
                        "Add Text Field"
                    </ButtonNewYork>
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Default
                        on_click=move |_| add_field("email".to_string())
                    >
                        "Add Email Field"
                    </ButtonNewYork>
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Default
                        on_click=move |_| add_field("number".to_string())
                    >
                        "Add Number Field"
                    </ButtonNewYork>
                </div>
                
                // Dynamic fields
                <div class="space-y-4">
                    {move || {
                        fields.get().into_iter().map(|field| {
                            let field_id = field.id.clone();
                            let field_type = field.field_type.clone();
                            
                            view! {
                                <div class="flex items-center space-x-4 p-4 border rounded-lg">
                                    <div class="flex-1 space-y-2">
                                        <label class="text-sm font-medium text-gray-700">
                                            {field.label}
                                        </label>
                                        <InputNewYork
                                            value=move || field.value.clone()
                                            on_change=move |value| update_field_value(field_id.clone(), value)
                                            placeholder=format!("Enter {}", field_type)
                                            input_type=field_type.clone()
                                        />
                                    </div>
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Destructive
                                        size=ButtonSizeNewYork::Sm
                                        on_click=move |_| remove_field(field_id.clone())
                                    >
                                        "Remove"
                                    </ButtonNewYork>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
                
                // Form data preview
                {move || if !fields.get().is_empty() {
                    view! {
                        <div class="mt-6 p-4 bg-gray-50 rounded-lg">
                            <h4 class="text-sm font-medium text-gray-700 mb-2">"Form Data Preview:"</h4>
                            <pre class="text-xs text-gray-600 overflow-auto">
                                {move || {
                                    let data: Vec<String> = fields.get().into_iter()
                                        .map(|f| format!("  {}: \"{}\"", f.label, f.value))
                                        .collect();
                                    format!("{{\n{}\n}}", data.join(",\n"))
                                }}
                            </pre>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }}
            </CardContentNewYork>
        </CardNewYork>
    }
}
```

## 🎮 Interactive Features

### Modal and Dialog Management

```rust
#[component]
pub fn ModalExample() -> impl IntoView {
    let (is_modal_open, set_is_modal_open) = signal(false);
    let (modal_content, set_modal_content) = signal("".to_string());
    
    let open_modal = move |content: String| {
        set_modal_content.set(content);
        set_is_modal_open.set(true);
    };
    
    let close_modal = move |_| {
        set_is_modal_open.set(false);
    };
    
    view! {
        <div class="space-y-4">
            // Modal triggers
            <div class="flex space-x-4">
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Default
                    on_click=move |_| open_modal("This is a simple modal with some content.".to_string())
                >
                    "Open Simple Modal"
                </ButtonNewYork>
                
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Outline
                    on_click=move |_| open_modal("This modal contains a form for user input.".to_string())
                >
                    "Open Form Modal"
                </ButtonNewYork>
            </div>
            
            // Modal overlay
            {move || if is_modal_open.get() {
                view! {
                    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                        <CardNewYork class="max-w-md w-full mx-4">
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"Modal Dialog"</CardTitleNewYork>
                                <button
                                    class="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
                                    on:click=close_modal
                                >
                                    "×"
                                </button>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                <p class="mb-4">{modal_content.get()}</p>
                                
                                // Modal form (if needed)
                                {move || if modal_content.get().contains("form") {
                                    view! {
                                        <div class="space-y-4">
                                            <InputNewYork
                                                placeholder="Enter your name"
                                            />
                                            <InputNewYork
                                                placeholder="Enter your email"
                                                input_type="email"
                                            />
                                        </div>
                                    }
                                } else {
                                    view! { <div></div> }
                                }}
                            </CardContentNewYork>
                            <CardFooterNewYork>
                                <div class="flex justify-end space-x-2">
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Outline
                                        on_click=close_modal
                                    >
                                        "Cancel"
                                    </ButtonNewYork>
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Default
                                        on_click=close_modal
                                    >
                                        "Confirm"
                                    </ButtonNewYork>
                                </div>
                            </CardFooterNewYork>
                        </CardNewYork>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
```

### Notification System

```rust
#[derive(Clone)]
struct Notification {
    id: String,
    message: String,
    notification_type: NotificationType,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
enum NotificationType {
    Success,
    Error,
    Info,
    Warning,
}

#[component]
pub fn NotificationExample() -> impl IntoView {
    let (notifications, set_notifications) = signal(Vec::<Notification>::new());
    
    let add_notification = move |message: String, notification_type: NotificationType| {
        let notification = Notification {
            id: format!("notif_{}", chrono::Utc::now().timestamp_millis()),
            message,
            notification_type,
            timestamp: chrono::Utc::now(),
        };
        
        set_notifications.update(|notifications| {
            notifications.push(notification);
            if notifications.len() > 5 {
                notifications.remove(0);
            }
        });
        
        // Auto-remove after 5 seconds
        set_timeout(move || {
            set_notifications.update(|notifications| {
                notifications.retain(|n| n.id != notification.id);
            });
        }, 5000);
    };
    
    let remove_notification = move |id: String| {
        set_notifications.update(|notifications| {
            notifications.retain(|n| n.id != id);
        });
    };
    
    view! {
        <div class="space-y-4">
            // Notification triggers
            <div class="flex flex-wrap gap-2">
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Default
                    on_click=move |_| add_notification("Operation completed successfully!".to_string(), NotificationType::Success)
                >
                    "Success Notification"
                </ButtonNewYork>
                
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Destructive
                    on_click=move |_| add_notification("Something went wrong!".to_string(), NotificationType::Error)
                >
                    "Error Notification"
                </ButtonNewYork>
                
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Outline
                    on_click=move |_| add_notification("Here's some useful information.".to_string(), NotificationType::Info)
                >
                    "Info Notification"
                </ButtonNewYork>
                
                <ButtonNewYork
                    variant=ButtonVariantNewYork::Secondary
                    on_click=move |_| add_notification("Please be careful with this action.".to_string(), NotificationType::Warning)
                >
                    "Warning Notification"
                </ButtonNewYork>
            </div>
            
            // Notification display
            <div class="fixed top-4 right-4 z-50 space-y-2">
                {move || {
                    notifications.get().into_iter().map(|notification| {
                        let notification_class = match notification.notification_type {
                            NotificationType::Success => "bg-green-100 border-green-500 text-green-700",
                            NotificationType::Error => "bg-red-100 border-red-500 text-red-700",
                            NotificationType::Info => "bg-blue-100 border-blue-500 text-blue-700",
                            NotificationType::Warning => "bg-yellow-100 border-yellow-500 text-yellow-700",
                        };
                        
                        let id = notification.id.clone();
                        
                        view! {
                            <div class=format!("p-4 border-l-4 rounded-md shadow-lg max-w-sm transform transition-all duration-300 {}", notification_class)>
                                <div class="flex justify-between items-start">
                                    <div class="flex-1">
                                        <p class="text-sm font-medium">{notification.message}</p>
                                        <p class="text-xs opacity-75 mt-1">
                                            {notification.timestamp.format("%H:%M:%S").to_string()}
                                        </p>
                                    </div>
                                    <button
                                        class="ml-2 text-lg leading-none hover:opacity-75"
                                        on:click=move |_| remove_notification(id)
                                    >
                                        "×"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
```

## 🚀 Advanced Patterns

### Component Composition

```rust
#[component]
pub fn ComposedForm() -> impl IntoView {
    let (form_data, set_form_data) = signal(FormData::default());
    let (current_step, set_current_step) = signal(1);
    
    view! {
        <CardNewYork class="max-w-2xl mx-auto">
            <CardHeaderNewYork>
                <CardTitleNewYork>"Multi-Step Form"</CardTitleNewYork>
                <CardDescriptionNewYork>
                    "Complete the form step by step"
                </CardDescriptionNewYork>
            </CardHeaderNewYork>
            <CardContentNewYork>
                // Step indicator
                <StepIndicator current=current_step />
                
                // Form content
                <div class="mt-6">
                    {move || match current_step.get() {
                        1 => view! { <PersonalInfoStep data=form_data set_data=set_form_data /> },
                        2 => view! { <ContactInfoStep data=form_data set_data=set_form_data /> },
                        3 => view! { <ReviewStep data=form_data /> },
                        _ => view! { <div></div> }
                    }}
                </div>
                
                // Navigation
                <div class="flex justify-between mt-6">
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Outline
                        disabled=move || current_step.get() <= 1
                        on_click=move |_| set_current_step.update(|s| *s -= 1)
                    >
                        "Previous"
                    </ButtonNewYork>
                    
                    <ButtonNewYork
                        variant=ButtonVariantNewYork::Default
                        disabled=move || current_step.get() >= 3
                        on_click=move |_| set_current_step.update(|s| *s += 1)
                    >
                        "Next"
                    </ButtonNewYork>
                </div>
            </CardContentNewYork>
        </CardNewYork>
    }
}

#[component]
pub fn StepIndicator(current: Signal<usize>) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between">
            {[1, 2, 3].into_iter().map(|step| {
                let is_active = move || current.get() == step;
                let is_completed = move || current.get() > step;
                
                view! {
                    <div class="flex items-center">
                        <div class=move || {
                            if is_active() {
                                "w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center text-sm font-medium"
                            } else if is_completed() {
                                "w-8 h-8 bg-green-600 text-white rounded-full flex items-center justify-center text-sm font-medium"
                            } else {
                                "w-8 h-8 bg-gray-200 text-gray-600 rounded-full flex items-center justify-center text-sm font-medium"
                            }
                        }>
                            {move || if is_completed() { "✓" } else { step.to_string() }}
                        </div>
                        {move || if step < 3 {
                            view! {
                                <div class=move || {
                                    if is_completed() {
                                        "w-12 h-1 bg-green-600 mx-2"
                                    } else {
                                        "w-12 h-1 bg-gray-200 mx-2"
                                    }
                                }></div>
                            }
                        } else {
                            view! { <div></div> }
                        }}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn PersonalInfoStep(
    data: Signal<FormData>,
    set_data: WriteSignal<FormData>
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h3 class="text-lg font-semibold">"Personal Information"</h3>
            <InputNewYork
                value=move || data.get().name.clone()
                on_change=move |value| set_data.update(|d| d.name = value)
                placeholder="Full Name"
            />
        </div>
    }
}

#[component]
pub fn ContactInfoStep(
    data: Signal<FormData>,
    set_data: WriteSignal<FormData>
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h3 class="text-lg font-semibold">"Contact Information"</h3>
            <InputNewYork
                value=move || data.get().email.clone()
                on_change=move |value| set_data.update(|d| d.email = value)
                placeholder="Email Address"
                input_type="email"
            />
        </div>
    }
}

#[component]
pub fn ReviewStep(data: Signal<FormData>) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h3 class="text-lg font-semibold">"Review Information"</h3>
            <div class="p-4 bg-gray-50 rounded-lg">
                <p>"Name: " {move || data.get().name}</p>
                <p>"Email: " {move || data.get().email}</p>
            </div>
        </div>
    }
}
```

## ✅ Best Practices

### 1. State Management
- Use signals for reactive state
- Keep state as close to where it's used as possible
- Use derived signals for computed values
- Structure complex state with custom types

### 2. Component Design
- Keep components focused and single-purpose
- Use props for configuration
- Compose components for complex UIs
- Follow the New York theme consistently

### 3. Performance
- Use `move` closures to avoid unnecessary re-renders
- Implement proper loading states
- Use timeouts for async operations
- Clean up resources when components unmount

### 4. Accessibility
- Use semantic HTML elements
- Provide proper labels and descriptions
- Ensure keyboard navigation works
- Test with screen readers

### 5. Error Handling
- Validate user input
- Provide clear error messages
- Handle network errors gracefully
- Use proper loading states

## 🔧 Troubleshooting

### Common Issues

1. **Components not updating:**
   - Check that you're using signals correctly
   - Ensure you're calling `set_*` functions to update state
   - Verify that your closures are properly capturing signals

2. **Form validation not working:**
   - Make sure validation functions are called
   - Check that error state is properly managed
   - Verify that form submission is prevented when invalid

3. **Styling issues:**
   - Ensure Tailwind CSS is properly configured
   - Check that New York theme classes are applied
   - Verify that custom classes don't conflict

4. **Performance problems:**
   - Use `move` closures to avoid unnecessary re-renders
   - Implement proper loading states
   - Consider using `memo` for expensive computations

### Debug Tips

1. **Use browser dev tools:**
   - Check the console for errors
   - Inspect DOM elements
   - Monitor network requests

2. **Add logging:**
   ```rust
   let debug_signal = signal("initial".to_string());
   Effect::new(move |_| {
       console_log!("Signal value: {}", debug_signal.get());
   });
   ```

3. **Test incrementally:**
   - Build components one at a time
   - Test each feature separately
   - Use simple examples first

## 📚 Additional Resources

- [Leptos Documentation](https://leptos.dev/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [shadcn/ui Documentation](https://ui.shadcn.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 🎉 Conclusion

This tutorial has covered the essential patterns for building interactive applications with the New York theme components. You've learned about:

- Component basics and variants
- State management with signals
- Form handling and validation
- Interactive features and notifications
- Advanced composition patterns
- Best practices and troubleshooting

Continue experimenting with these patterns and building more complex applications. The New York theme provides a solid foundation for creating beautiful, interactive user interfaces with Leptos and Rust.

Happy coding! 🚀
