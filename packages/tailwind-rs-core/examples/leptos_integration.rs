//! Example integration of tailwind-rs-core with Leptos components.

use leptos::prelude::*;
use tailwind_rs_core::*;

/// Example button component using tailwind-rs-core for type-safe styling.
#[component]
pub fn EnhancedButton(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let size = size.unwrap_or(ButtonSize::Md);
    let disabled = disabled.unwrap_or(false);

    // Create reactive class signal using tailwind-rs-core
    let classes = create_class_signal(variant, size, disabled);

    view! {
        <button
            class=classes.get()
            disabled=disabled
        >
            {children.map(|c| c()).unwrap_or_else(|| "Click me".into())}
        </button>
    }
}

/// Button variant enum for type-safe styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Outline,
    Ghost,
    Link,
    Destructive,
}

/// Button size enum for type-safe styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

/// Create a reactive class signal for button styling.
fn create_class_signal(variant: ButtonVariant, size: ButtonSize, disabled: bool) -> ClassSignal {
    let base_classes = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Success => "bg-green-600 text-white hover:bg-green-700",
        ButtonVariant::Warning => "bg-yellow-600 text-white hover:bg-yellow-700",
        ButtonVariant::Error => "bg-red-600 text-white hover:bg-red-700",
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    };

    let size_classes = match size {
        ButtonSize::Xs => "h-8 px-2 text-xs",
        ButtonSize::Sm => "h-9 px-3 text-sm",
        ButtonSize::Md => "h-10 px-4 py-2",
        ButtonSize::Lg => "h-11 px-8 text-lg",
        ButtonSize::Xl => "h-12 px-10 text-xl",
    };

    let disabled_classes = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };

    let all_classes = format!("{} {} {} {}", base_classes, variant_classes, size_classes, disabled_classes);
    ClassSignal::new(all_classes)
}

/// Example card component using tailwind-rs-core for responsive design.
#[component]
pub fn ResponsiveCard(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create responsive classes using tailwind-rs-core
    let responsive_classes = Responsive::new()
        .sm("p-4")
        .md("p-6")
        .lg("p-8")
        .xl("p-10");

    let card_classes = format!("rounded-lg border bg-card text-card-foreground shadow-sm {}", responsive_classes.to_string());

    view! {
        <div class=card_classes>
            {title.map(|t| view! {
                <div class="flex flex-col space-y-1.5 p-6">
                    <h3 class="text-2xl font-semibold leading-none tracking-tight">{t}</h3>
                </div>
            })}
            <div class="p-6 pt-0">
                {children.map(|c| c()).unwrap_or_else(|| "Card content".into())}
            </div>
        </div>
    }
}

/// Example theme-aware component using tailwind-rs-core.
#[component]
pub fn ThemedComponent(
    #[prop(optional)] theme: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create theme manager
    let theme_manager = ReactiveThemeManager::new();
    
    // Get theme classes
    let primary_classes = theme_manager.get_classes_signal(&Variant::Primary, &Size::Md);
    let secondary_classes = theme_manager.get_classes_signal(&Variant::Secondary, &Size::Md);

    view! {
        <div class="space-y-4">
            <div class=primary_classes>
                "Primary themed content"
            </div>
            <div class=secondary_classes>
                "Secondary themed content"
            </div>
            {children.map(|c| c()).unwrap_or_else(|| "Themed content".into())}
        </div>
    }
}

/// Example color system component using tailwind-rs-core.
#[component]
pub fn ColorSystemComponent() -> impl IntoView {
    // Create reactive color system
    let color_system = ReactiveColor::new(Color::Blue);
    
    // Get color signals
    let background_signal = color_system.background_signal(600);
    let text_signal = color_system.text_signal(900);
    let hover_signal = color_system.hover_signal(700);

    view! {
        <div class="space-y-4">
            <div class=format!("{} {} {}", background_signal.get(), text_signal.get(), hover_signal.get())>
                "Dynamic color system"
            </div>
            <button
                on:click=move |_| {
                    // Switch to different color
                    color_system.set_color.set(Color::Green);
                }
            >
                "Switch to Green"
            </button>
        </div>
    }
}

/// Example form component using tailwind-rs-core for validation styling.
#[component]
pub fn ValidatedForm() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (is_valid, set_is_valid) = create_signal(false);

    // Create validation classes
    let input_classes = create_memo(move |_| {
        if is_valid.get() {
            "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-green-500"
        } else {
            "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-red-500"
        }
    });

    view! {
        <form class="space-y-4">
            <div>
                <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                    "Email"
                </label>
                <input
                    type="email"
                    class=input_classes
                    placeholder="Enter your email"
                    value=email
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_email.set(value.clone());
                        set_is_valid.set(value.contains('@'));
                    }
                />
            </div>
            <button
                type="submit"
                class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
            >
                "Submit"
            </button>
        </form>
    }
}

/// Example responsive grid component using tailwind-rs-core.
#[component]
pub fn ResponsiveGrid(
    #[prop(optional)] items: Option<Vec<String>>,
) -> impl IntoView {
    let items = items.unwrap_or_else(|| vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
        "Item 6".to_string(),
    ]);

    // Create responsive grid classes
    let grid_classes = Responsive::new()
        .sm("grid-cols-1")
        .md("grid-cols-2")
        .lg("grid-cols-3")
        .xl("grid-cols-4");

    let container_classes = format!("grid gap-4 {}", grid_classes.to_string());

    view! {
        <div class=container_classes>
            {items.into_iter().map(|item| view! {
                <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-4">
                    {item}
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Example component showcasing all tailwind-rs-core features.
#[component]
pub fn TailwindRsCoreDemo() -> impl IntoView {
    view! {
        <div class="container mx-auto p-8 space-y-8">
            <h1 class="text-4xl font-bold text-center mb-8">
                "Tailwind-RS-Core Integration Demo"
            </h1>
            
            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Enhanced Button Component"</h2>
                <div class="flex flex-wrap gap-4">
                    <EnhancedButton variant=ButtonVariant::Primary size=ButtonSize::Md>
                        "Primary Button"
                    </EnhancedButton>
                    <EnhancedButton variant=ButtonVariant::Secondary size=ButtonSize::Md>
                        "Secondary Button"
                    </EnhancedButton>
                    <EnhancedButton variant=ButtonVariant::Success size=ButtonSize::Md>
                        "Success Button"
                    </EnhancedButton>
                    <EnhancedButton variant=ButtonVariant::Error size=ButtonSize::Md>
                        "Error Button"
                    </EnhancedButton>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Responsive Card Component"</h2>
                <ResponsiveCard title="Responsive Card".to_string()>
                    "This card adapts to different screen sizes using tailwind-rs-core responsive utilities."
                </ResponsiveCard>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Theme-Aware Component"</h2>
                <ThemedComponent>
                    "This component uses the theme system for consistent styling."
                </ThemedComponent>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Color System Component"</h2>
                <ColorSystemComponent />
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Validated Form Component"</h2>
                <ValidatedForm />
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Responsive Grid Component"</h2>
                <ResponsiveGrid />
            </section>
        </div>
    }
}
