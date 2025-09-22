//! Individual Sonner toast component
//! 
//! This module contains the SonnerToast component that renders
//! individual toast notifications.

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::sonner::types::ToastData;

/// Individual Sonner toast component
#[component]
pub fn SonnerToast(
    id: String,
    data: ToastData,
    on_dismiss: Callback<()>,
) -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);
    let (progress, set_progress) = signal(data.progress.unwrap_or(0.0));

    // Auto-dismiss logic
    if let Some(duration) = data.duration {
        if duration.as_millis() > 0 {
            let set_is_visible = set_is_visible.clone();
            let on_dismiss = on_dismiss.clone();
            let id = id.clone();
            
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
                set_is_visible.set(false);
                // Small delay for animation
                gloo_timers::future::TimeoutFuture::new(300).await;
                on_dismiss.run(());
            });
        }
    }

    // Progress animation
    if data.progress.is_some() {
        let set_progress = set_progress.clone();
        spawn_local(async move {
            let mut current_progress = 0.0;
            let target_progress = data.progress.unwrap_or(0.0);
            let steps = 100;
            let step_size = target_progress / steps as f64;
            
            for _ in 0..steps {
                current_progress += step_size;
                set_progress.set(current_progress.min(1.0));
                gloo_timers::future::TimeoutFuture::new(20).await;
            }
        });
    }

    let variant_class = match data.variant {
        crate::sonner::types::ToastVariant::Default => "bg-background text-foreground border",
        crate::sonner::types::ToastVariant::Success => "bg-green-50 text-green-900 border-green-200 dark:bg-green-900 dark:text-green-100 dark:border-green-800",
        crate::sonner::types::ToastVariant::Error => "bg-red-50 text-red-900 border-red-200 dark:bg-red-900 dark:text-red-100 dark:border-red-800",
        crate::sonner::types::ToastVariant::Warning => "bg-yellow-50 text-yellow-900 border-yellow-200 dark:bg-yellow-900 dark:text-yellow-100 dark:border-yellow-800",
        crate::sonner::types::ToastVariant::Info => "bg-blue-50 text-blue-900 border-blue-200 dark:bg-blue-900 dark:text-blue-100 dark:border-blue-800",
        crate::sonner::types::ToastVariant::Loading => "bg-gray-50 text-gray-900 border-gray-200 dark:bg-gray-900 dark:text-gray-100 dark:border-gray-800",
    };

    let animation_class = if is_visible.get() {
        "animate-in slide-in-from-right-full"
    } else {
        "animate-out slide-out-to-right-full"
    };

    view! {
        <div
            class=format!("{} {} {} p-4 rounded-lg shadow-lg max-w-sm w-full mb-2", 
                variant_class, animation_class, 
                if data.actions.is_empty() { "" } else { "pb-2" }
            )
            role="alert"
            aria-live="polite"
            aria-atomic="true"
        >
            <div class="flex items-start justify-between">
                <div class="flex-1">
                    <div class="font-medium text-sm">
                        {data.title}
                    </div>
                    {if let Some(description) = &data.description {
                        view! {
                            <div class="text-sm opacity-90 mt-1">
                                {description.clone()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
                <button
                    class="ml-2 text-sm opacity-70 hover:opacity-100"
                    on:click=move |_| {
                        set_is_visible.set(false);
                        on_dismiss.run(());
                    }
                >
                    "×"
                </button>
            </div>
            
            {if let Some(_) = data.progress {
                view! {
                    <div class="w-full bg-gray-200 rounded-full h-1 mt-2">
                        <div 
                            class="bg-blue-600 h-1 rounded-full transition-all duration-300"
                            style=move || format!("width: {}%", (progress.get() * 100.0) as u32)
                        ></div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {if !data.actions.is_empty() {
                let actions = data.actions.clone();
                view! {
                    <div class="flex gap-2 mt-3">
                        {actions.into_iter().map(|action| {
                            view! {
                                <button
                                    class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700"
                                    on:click=move |_| action.action.run(())
                                >
                                    {action.label}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
