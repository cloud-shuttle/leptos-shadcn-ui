//! Sonner context and provider
//! 
//! This module contains the SonnerContextValue and SonnerProvider
//! for managing toast state and providing context to child components.

use leptos::prelude::*;
use std::collections::HashMap;
use crate::sonner::types::{ToastData, ToastPosition, ToastTheme};
use crate::sonner::toast::SonnerToast;

/// Sonner context value
#[derive(Clone)]
pub struct SonnerContextValue {
    pub toasts: RwSignal<HashMap<String, ToastData>>,
    pub add_toast: Callback<ToastData>,
    pub remove_toast: Callback<String>,
    pub dismiss_all: Callback<()>,
    pub position: RwSignal<ToastPosition>,
    pub theme: RwSignal<ToastTheme>,
    pub max_toasts: RwSignal<usize>,
}

impl SonnerContextValue {
    pub fn new() -> Self {
        let toasts = RwSignal::new(HashMap::<String, ToastData>::new());
        let position = RwSignal::new(ToastPosition::TopRight);
        let theme = RwSignal::new(ToastTheme::Auto);
        let max_toasts = RwSignal::new(5);

        let add_toast = {
            let toasts = toasts.clone();
            let max_toasts = max_toasts.clone();
            Callback::new(move |toast: ToastData| {
                let mut current_toasts = toasts.get();
                let max = max_toasts.get();
                
                // Remove oldest toasts if we exceed the limit
                if current_toasts.len() >= max {
                    let mut sorted_toasts: Vec<_> = current_toasts.iter().collect();
                    sorted_toasts.sort_by_key(|(_, data)| data.created_at);
                    
                    let to_remove: Vec<String> = sorted_toasts.iter()
                        .take(current_toasts.len() - max + 1)
                        .map(|(id, _)| (*id).clone())
                        .collect();
                    
                    for id in to_remove {
                        current_toasts.remove(&id);
                    }
                }
                
                current_toasts.insert(toast.id.clone(), toast);
                toasts.set(current_toasts);
            })
        };

        let remove_toast = {
            let toasts = toasts.clone();
            Callback::new(move |id: String| {
                let mut current_toasts = toasts.get();
                current_toasts.remove(&id);
                toasts.set(current_toasts);
            })
        };

        let dismiss_all = {
            let toasts = toasts.clone();
            Callback::new(move |_| {
                toasts.set(HashMap::new());
            })
        };

        Self {
            toasts,
            add_toast,
            remove_toast,
            dismiss_all,
            position,
            theme,
            max_toasts,
        }
    }
}

/// Sonner provider component
#[component]
pub fn SonnerProvider(
    #[prop(into, optional)] position: MaybeProp<ToastPosition>,
    #[prop(into, optional)] theme: MaybeProp<ToastTheme>,
    #[prop(into, optional)] max_toasts: MaybeProp<usize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = SonnerContextValue::new();
    
    // Set initial values
    if let Some(pos) = position.get() {
        context.position.set(pos);
    }
    if let Some(thm) = theme.get() {
        context.theme.set(thm);
    }
    if let Some(max) = max_toasts.get() {
        context.max_toasts.set(max);
    }

    provide_context(context);

    view! {
        <div>
            {children.map(|c| c())}
            <SonnerViewport />
        </div>
    }
}

/// Sonner viewport component that renders all toasts
#[component]
pub fn SonnerViewport() -> impl IntoView {
    let context = expect_context::<SonnerContextValue>();
    let toasts = context.toasts;
    let position = context.position;
    let theme = context.theme;

    let position_class = Signal::derive(move || {
        match position.get() {
            ToastPosition::TopLeft => "fixed top-4 left-4 z-[100]",
            ToastPosition::TopRight => "fixed top-4 right-4 z-[100]",
            ToastPosition::BottomLeft => "fixed bottom-4 left-4 z-[100]",
            ToastPosition::BottomRight => "fixed bottom-4 right-4 z-[100]",
            ToastPosition::TopCenter => "fixed top-4 left-1/2 transform -translate-x-1/2 z-[100]",
            ToastPosition::BottomCenter => "fixed bottom-4 left-1/2 transform -translate-x-1/2 z-[100]",
        }
    });

    let theme_class = Signal::derive(move || {
        match theme.get() {
            ToastTheme::Light => "light-theme",
            ToastTheme::Dark => "dark-theme",
            ToastTheme::Auto => "auto-theme",
        }
    });

    view! {
        <div class=move || format!("{} {}", position_class.get(), theme_class.get())>
            {move || {
                toasts.get().into_iter().map(|(id, toast_data)| {
                    let context = context.clone();
                    let on_dismiss = {
                        let context = context.clone();
                        let id = id.clone();
                        Callback::new(move |_| context.remove_toast.run(id.clone()))
                    };
                    view! {
                        <SonnerToast
                            id=id.clone()
                            data=toast_data.clone()
                            on_dismiss=on_dismiss
                        />
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}
