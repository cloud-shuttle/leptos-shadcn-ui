use leptos::prelude::*;
use leptos_style::Style;

const AVATAR_CLASS: &str = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full";
const AVATAR_IMAGE_CLASS: &str = "aspect-square h-full w-full";
const AVATAR_FALLBACK_CLASS: &str = "flex h-full w-full items-center justify-center rounded-full bg-muted";
const AVATAR_GROUP_CLASS: &str = "flex -space-x-2";

#[component]
pub fn Avatar(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", AVATAR_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into, optional)] alt: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", AVATAR_IMAGE_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <img
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            src=src
            alt=alt.get().unwrap_or_default()
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", AVATAR_FALLBACK_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AvatarGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", AVATAR_GROUP_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}
