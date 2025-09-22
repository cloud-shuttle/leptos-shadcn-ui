use leptos::prelude::*;
use leptos::ev::{MouseEvent, KeyboardEvent};
use leptos_style::Style;

pub const CARD_CLASS: &str = "rounded-lg border bg-card text-card-foreground shadow-sm";

/// Card variant for different visual states
#[derive(Clone, Debug, PartialEq)]
pub enum CardVariant {
    Default,
    Destructive,
    Warning,
    Success,
}
pub const CARD_HEADER_CLASS: &str = "flex flex-col space-y-1.5 p-6";
pub const CARD_TITLE_CLASS: &str = "text-2xl font-semibold leading-none tracking-tight";
pub const CARD_DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";
pub const CARD_CONTENT_CLASS: &str = "p-6 pt-0";
pub const CARD_FOOTER_CLASS: &str = "flex items-center p-6 pt-0";

#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] variant: MaybeProp<CardVariant>,
    #[prop(into, optional)] interactive: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = CARD_CLASS;
        let variant_class = match variant.get() {
            Some(CardVariant::Destructive) => " border-destructive bg-destructive/5",
            Some(CardVariant::Warning) => " border-warning bg-warning/5",
            Some(CardVariant::Success) => " border-success bg-success/5",
            _ => "",
        };
        let interactive_class = if interactive.get() { " cursor-pointer hover:shadow-md transition-shadow" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{}{} {}", base_class, variant_class, interactive_class, custom_class)
    });

    view! {
        <article
            class=computed_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            role="article"
            tabindex=move || if interactive.get() { "0" } else { "-1" }
        >
            {children.map(|c| c())}
        </article>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", CARD_HEADER_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] level: MaybeProp<u8>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", CARD_TITLE_CLASS, class.get().unwrap_or_default())
    });

    // Default to h2 for semantic structure
    view! {
        <h2
            class=computed_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", CARD_DESCRIPTION_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <p
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", CARD_CONTENT_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", CARD_FOOTER_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Interactive Card component with click handling
#[component]
pub fn InteractiveCard(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] variant: MaybeProp<CardVariant>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_hovered, set_is_hovered) = signal(false);
    let (is_focused, set_is_focused) = signal(false);
    
    let computed_class = Signal::derive(move || {
        let base_class = CARD_CLASS;
        let variant_class = match variant.get() {
            Some(CardVariant::Destructive) => " border-destructive bg-destructive/5",
            Some(CardVariant::Warning) => " border-warning bg-warning/5",
            Some(CardVariant::Success) => " border-success bg-success/5",
            _ => "",
        };
        let interactive_class = " cursor-pointer hover:shadow-md transition-shadow focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";
        let custom_class = class.get().unwrap_or_default();
        format!("{}{}{} {}", base_class, variant_class, interactive_class, custom_class)
    });

    let handle_click = move |_: MouseEvent| {
        if let Some(callback) = &on_click {
            callback.run(());
        }
    };

    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == "Enter" || event.key() == " " {
            event.prevent_default();
            if let Some(callback) = &on_click {
                callback.run(());
            }
        }
    };

    view! {
        <article
            class=computed_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            role="button"
            tabindex="0"
            on:click=handle_click
            on:keydown=handle_keydown
            on:mouseenter=move |_| set_is_hovered.set(true)
            on:mouseleave=move |_| set_is_hovered.set(false)
            on:focus=move |_| set_is_focused.set(true)
            on:blur=move |_| set_is_focused.set(false)
        >
            {children.map(|c| c())}
        </article>
    }
}