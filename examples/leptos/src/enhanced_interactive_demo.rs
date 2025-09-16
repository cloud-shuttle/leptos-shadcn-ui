use leptos::*;
use leptos::prelude::*;

// Import all New York variants
use leptos_shadcn_button::new_york::{Button as ButtonNewYork, ButtonVariant as ButtonVariantNewYork, ButtonSize as ButtonSizeNewYork};
use leptos_shadcn_card::new_york::{Card as CardNewYork, CardHeader as CardHeaderNewYork, CardTitle as CardTitleNewYork, CardDescription as CardDescriptionNewYork, CardContent as CardContentNewYork, CardFooter as CardFooterNewYork};
use leptos_shadcn_input::new_york::Input as InputNewYork;

#[component]
pub fn EnhancedInteractiveDemo() -> impl IntoView {
    // State management for interactive features
    let (selected_theme, set_selected_theme) = signal("new_york".to_string());
    let (form_data, set_form_data) = signal(FormData::default());
    let (notifications, set_notifications) = signal(Vec::<Notification>::new());
    let (current_step, set_current_step) = signal(1);
    let (is_loading, set_is_loading) = signal(false);
    let (selected_items, set_selected_items) = signal(Vec::<String>::new());
    
    // Form validation state
    let (form_errors, set_form_errors) = signal(Vec::<String>::new());
    let (is_form_valid, set_is_form_valid) = signal(false);
    
    // Add notification function
    let add_notification = move |message: String, notification_type: NotificationType| {
        let new_notification = Notification {
            id: format!("notif_{}", chrono::Utc::now().timestamp_millis()),
            message,
            notification_type,
            timestamp: chrono::Utc::now(),
        };
        set_notifications.update(|notifications| {
            notifications.push(new_notification);
            if notifications.len() > 5 {
                notifications.remove(0);
            }
        });
    };
    
    // Form validation
    let validate_form = move || {
        let mut errors = Vec::new();
        let data = form_data.get();
        
        if data.name.is_empty() {
            errors.push("Name is required".to_string());
        }
        if data.email.is_empty() || !data.email.contains('@') {
            errors.push("Valid email is required".to_string());
        }
        if data.message.is_empty() {
            errors.push("Message is required".to_string());
        }
        
        set_form_errors.set(errors.clone());
        set_is_form_valid.set(errors.is_empty());
        errors.is_empty()
    };
    
    // Handle form submission
    let handle_form_submit = move |_| {
        if validate_form() {
            set_is_loading.set(true);
            
            // Simulate API call
            set_timeout(move || {
                set_is_loading.set(false);
                add_notification("Form submitted successfully!".to_string(), NotificationType::Success);
                set_form_data.set(FormData::default());
                set_form_errors.set(Vec::new());
            }, 2000);
        } else {
            add_notification("Please fix the form errors".to_string(), NotificationType::Error);
        }
    };
    
    // Handle item selection
    let toggle_item = move |item: String| {
        set_selected_items.update(|items| {
            if items.contains(&item) {
                items.retain(|i| i != &item);
            } else {
                items.push(item);
            }
        });
    };
    
    // Handle step navigation
    let next_step = move |_| {
        if current_step.get() < 3 {
            set_current_step.update(|step| *step += 1);
        }
    };
    
    let prev_step = move |_| {
        if current_step.get() > 1 {
            set_current_step.update(|step| *step -= 1);
        }
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 dark:from-slate-900 dark:to-blue-900">
            // Header with theme switcher
            <header class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <div class="flex items-center space-x-4">
                            <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                                <span class="text-white font-bold text-sm">"LS"</span>
                            </div>
                            <h1 class="text-xl font-bold text-gray-900 dark:text-white">
                                "Enhanced Interactive Demo - New York Theme"
                            </h1>
                        </div>
                        <div class="flex items-center space-x-4">
                            // Theme switcher
                            <div class="flex items-center space-x-2">
                                <span class="text-sm text-gray-600 dark:text-gray-300">"Theme:"</span>
                                <select 
                                    class="px-3 py-1 border border-gray-300 rounded-md text-sm bg-white dark:bg-gray-700 dark:border-gray-600"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_selected_theme.set(value);
                                    }
                                >
                                    <option value="new_york" selected=move || selected_theme.get() == "new_york">"New York"</option>
                                    <option value="default" selected=move || selected_theme.get() == "default">"Default"</option>
                                </select>
                            </div>
                        </div>
                    </div>
                </div>
            </header>

            // Main content
            <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                // Notifications
                <div class="fixed top-20 right-4 z-50 space-y-2">
                    {move || {
                        notifications.get().into_iter().map(|notification| {
                            let notification_class = match notification.notification_type {
                                NotificationType::Success => "bg-green-100 border-green-500 text-green-700",
                                NotificationType::Error => "bg-red-100 border-red-500 text-red-700",
                                NotificationType::Info => "bg-blue-100 border-blue-500 text-blue-700",
                                NotificationType::Warning => "bg-yellow-100 border-yellow-500 text-yellow-700",
                            };
                            
                            view! {
                                <div class=format!("p-4 border-l-4 rounded-md shadow-lg max-w-sm {}", notification_class)>
                                    <div class="flex justify-between items-start">
                                        <p class="text-sm font-medium">{notification.message}</p>
                                        <button 
                                            class="ml-2 text-lg leading-none"
                                            on:click=move |_| {
                                                set_notifications.update(|notifications| {
                                                    notifications.retain(|n| n.id != notification.id);
                                                });
                                            }
                                        >
                                            "×"
                                        </button>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>

                // Interactive components showcase
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    // Left column - Interactive Form
                    <div class="space-y-6">
                        <CardNewYork class="shadow-lg">
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"Interactive Contact Form"</CardTitleNewYork>
                                <CardDescriptionNewYork>
                                    "Fill out this form to see the New York theme components in action"
                                </CardDescriptionNewYork>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                <form class="space-y-4" on:submit=move |ev| {
                                    ev.prevent_default();
                                    handle_form_submit(());
                                }>
                                    // Name input
                                    <div class="space-y-2">
                                        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                            "Full Name"
                                        </label>
                                        <InputNewYork
                                            value=move || form_data.get().name.clone()
                                            on_change=move |value| {
                                                set_form_data.update(|data| data.name = value);
                                            }
                                            placeholder="Enter your full name"
                                            class="w-full"
                                        />
                                    </div>
                                    
                                    // Email input
                                    <div class="space-y-2">
                                        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                            "Email Address"
                                        </label>
                                        <InputNewYork
                                            value=move || form_data.get().email.clone()
                                            on_change=move |value| {
                                                set_form_data.update(|data| data.email = value);
                                            }
                                            placeholder="Enter your email"
                                            input_type="email"
                                            class="w-full"
                                        />
                                    </div>
                                    
                                    // Message textarea
                                    <div class="space-y-2">
                                        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                                            "Message"
                                        </label>
                                        <textarea
                                            class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                                            placeholder="Enter your message"
                                            prop:value=move || form_data.get().message.clone()
                                            on:input=move |ev| {
                                                let value = event_target_value(&ev);
                                                set_form_data.update(|data| data.message = value);
                                            }
                                        />
                                    </div>
                                    
                                    // Form errors
                                    {move || {
                                        if !form_errors.get().is_empty() {
                                            view! {
                                                <div class="space-y-1">
                                                    {form_errors.get().into_iter().map(|error| {
                                                        view! {
                                                            <p class="text-sm text-red-600 dark:text-red-400">"• " {error}</p>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        } else {
                                            view! { <div></div> }
                                        }
                                    }}
                                    
                                    // Submit button
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Default
                                        size=ButtonSizeNewYork::Default
                                        disabled=move || is_loading.get() || !is_form_valid.get()
                                        on_click=move |_| handle_form_submit(())
                                        class="w-full"
                                    >
                                        {move || if is_loading.get() { "Submitting..." } else { "Submit Form" }}
                                    </ButtonNewYork>
                                </form>
                            </CardContentNewYork>
                        </CardNewYork>

                        // Interactive selection demo
                        <CardNewYork class="shadow-lg">
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"Interactive Selection"</CardTitleNewYork>
                                <CardDescriptionNewYork>
                                    "Click items to select them and see the state management in action"
                                </CardDescriptionNewYork>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                <div class="space-y-3">
                                    {["Option 1", "Option 2", "Option 3", "Option 4", "Option 5"].into_iter().map(|item| {
                                        let item_clone = item.to_string();
                                        let is_selected = move || selected_items.get().contains(&item_clone);
                                        
                                        view! {
                                            <div 
                                                class=move || {
                                                    if is_selected() {
                                                        "p-3 border-2 border-blue-500 bg-blue-50 dark:bg-blue-900/20 rounded-lg cursor-pointer transition-all"
                                                    } else {
                                                        "p-3 border-2 border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 rounded-lg cursor-pointer transition-all"
                                                    }
                                                }
                                                on:click=move |_| toggle_item(item.to_string())
                                            >
                                                <div class="flex items-center justify-between">
                                                    <span class="font-medium text-gray-900 dark:text-white">{item}</span>
                                                    {move || if is_selected() {
                                                        view! { <span class="text-blue-600 text-sm">"✓ Selected"</span> }
                                                    } else {
                                                        view! { <span class="text-gray-400 text-sm">"Click to select"</span> }
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                
                                // Selection summary
                                <div class="mt-4 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                                    <p class="text-sm text-gray-600 dark:text-gray-300">
                                        "Selected: " {move || selected_items.get().len()} " items"
                                    </p>
                                    {move || if !selected_items.get().is_empty() {
                                        view! {
                                            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                                {selected_items.get().join(", ")}
                                            </p>
                                        }
                                    } else {
                                        view! { <div></div> }
                                    }}
                                </div>
                            </CardContentNewYork>
                        </CardNewYork>
                    </div>

                    // Right column - Component showcase
                    <div class="space-y-6">
                        // Button variants showcase
                        <CardNewYork class="shadow-lg">
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"Button Variants"</CardTitleNewYork>
                                <CardDescriptionNewYork>
                                    "All New York theme button variants with interactive states"
                                </CardDescriptionNewYork>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                <div class="grid grid-cols-2 gap-3">
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Default
                                        on_click=move |_| add_notification("Default button clicked!".to_string(), NotificationType::Info)
                                    >
                                        "Default"
                                    </ButtonNewYork>
                                    
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Destructive
                                        on_click=move |_| add_notification("Destructive action triggered!".to_string(), NotificationType::Warning)
                                    >
                                        "Destructive"
                                    </ButtonNewYork>
                                    
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Outline
                                        on_click=move |_| add_notification("Outline button clicked!".to_string(), NotificationType::Info)
                                    >
                                        "Outline"
                                    </ButtonNewYork>
                                    
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Secondary
                                        on_click=move |_| add_notification("Secondary button clicked!".to_string(), NotificationType::Info)
                                    >
                                        "Secondary"
                                    </ButtonNewYork>
                                    
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Ghost
                                        on_click=move |_| add_notification("Ghost button clicked!".to_string(), NotificationType::Info)
                                    >
                                        "Ghost"
                                    </ButtonNewYork>
                                    
                                    <ButtonNewYork
                                        variant=ButtonVariantNewYork::Link
                                        on_click=move |_| add_notification("Link button clicked!".to_string(), NotificationType::Info)
                                    >
                                        "Link"
                                    </ButtonNewYork>
                                </div>
                                
                                // Button sizes
                                <div class="mt-4 space-y-2">
                                    <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300">"Button Sizes"</h4>
                                    <div class="flex items-center space-x-2">
                                        <ButtonNewYork
                                            size=ButtonSizeNewYork::Sm
                                            on_click=move |_| add_notification("Small button clicked!".to_string(), NotificationType::Info)
                                        >
                                            "Small"
                                        </ButtonNewYork>
                                        
                                        <ButtonNewYork
                                            size=ButtonSizeNewYork::Default
                                            on_click=move |_| add_notification("Default size clicked!".to_string(), NotificationType::Info)
                                        >
                                            "Default"
                                        </ButtonNewYork>
                                        
                                        <ButtonNewYork
                                            size=ButtonSizeNewYork::Lg
                                            on_click=move |_| add_notification("Large button clicked!".to_string(), NotificationType::Info)
                                        >
                                            "Large"
                                        </ButtonNewYork>
                                        
                                        <ButtonNewYork
                                            size=ButtonSizeNewYork::Icon
                                            on_click=move |_| add_notification("Icon button clicked!".to_string(), NotificationType::Info)
                                        >
                                            "🚀"
                                        </ButtonNewYork>
                                    </div>
                                </div>
                            </CardContentNewYork>
                        </CardNewYork>

                        // Step-by-step demo
                        <CardNewYork class="shadow-lg">
                            <CardHeaderNewYork>
                                <CardTitleNewYork>"Step-by-Step Demo"</CardTitleNewYork>
                                <CardDescriptionNewYork>
                                    "Navigate through steps to see component interactions"
                                </CardDescriptionNewYork>
                            </CardHeaderNewYork>
                            <CardContentNewYork>
                                // Step indicator
                                <div class="flex items-center justify-between mb-6">
                                    {[1, 2, 3].into_iter().map(|step| {
                                        let is_active = move || current_step.get() == step;
                                        let is_completed = move || current_step.get() > step;
                                        
                                        view! {
                                            <div class="flex items-center">
                                                <div class=move || {
                                                    if is_active() {
                                                        "w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center text-sm font-medium"
                                                    } else if is_completed() {
                                                        "w-8 h-8 bg-green-600 text-white rounded-full flex items-center justify-center text-sm font-medium"
                                                    } else {
                                                        "w-8 h-8 bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full flex items-center justify-center text-sm font-medium"
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
                                                                "w-12 h-1 bg-gray-200 dark:bg-gray-700 mx-2"
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
                                
                                // Step content
                                <div class="min-h-[200px]">
                                    {move || match current_step.get() {
                                        1 => view! {
                                            <div class="text-center py-8">
                                                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                                                    "Welcome to Step 1"
                                                </h3>
                                                <p class="text-gray-600 dark:text-gray-300 mb-4">
                                                    "This is the first step of our interactive demo. Click 'Next' to continue."
                                                </p>
                                                <ButtonNewYork
                                                    variant=ButtonVariantNewYork::Default
                                                    on_click=next_step
                                                >
                                                    "Next Step"
                                                </ButtonNewYork>
                                            </div>
                                        },
                                        2 => view! {
                                            <div class="text-center py-8">
                                                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                                                    "Step 2: Interactive Elements"
                                                </h3>
                                                <p class="text-gray-600 dark:text-gray-300 mb-4">
                                                    "Here you can see various interactive elements. Try the buttons above!"
                                                </p>
                                                <div class="flex justify-center space-x-4">
                                                    <ButtonNewYork
                                                        variant=ButtonVariantNewYork::Outline
                                                        on_click=prev_step
                                                    >
                                                        "Previous"
                                                    </ButtonNewYork>
                                                    <ButtonNewYork
                                                        variant=ButtonVariantNewYork::Default
                                                        on_click=next_step
                                                    >
                                                        "Next Step"
                                                    </ButtonNewYork>
                                                </div>
                                            </div>
                                        },
                                        3 => view! {
                                            <div class="text-center py-8">
                                                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                                                    "Step 3: Completion"
                                                </h3>
                                                <p class="text-gray-600 dark:text-gray-300 mb-4">
                                                    "Congratulations! You've completed the interactive demo. You can go back or start over."
                                                </p>
                                                <div class="flex justify-center space-x-4">
                                                    <ButtonNewYork
                                                        variant=ButtonVariantNewYork::Outline
                                                        on_click=prev_step
                                                    >
                                                        "Previous"
                                                    </ButtonNewYork>
                                                    <ButtonNewYork
                                                        variant=ButtonVariantNewYork::Default
                                                        on_click=move |_| set_current_step.set(1)
                                                    >
                                                        "Start Over"
                                                    </ButtonNewYork>
                                                </div>
                                            </div>
                                        },
                                        _ => view! { <div></div> }
                                    }}
                                </div>
                            </CardContentNewYork>
                        </CardNewYork>
                    </div>
                </div>
            </main>
        </div>
    }
}

// Data structures
#[derive(Clone, Default)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

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
