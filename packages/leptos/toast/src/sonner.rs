use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Toast position variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}

impl Default for ToastPosition {
    fn default() -> Self {
        ToastPosition::TopRight
    }
}

impl From<String> for ToastPosition {
    fn from(s: String) -> Self {
        match s.as_str() {
            "top-left" => ToastPosition::TopLeft,
            "top-right" => ToastPosition::TopRight,
            "bottom-left" => ToastPosition::BottomLeft,
            "bottom-right" => ToastPosition::BottomRight,
            "top-center" => ToastPosition::TopCenter,
            "bottom-center" => ToastPosition::BottomCenter,
            _ => ToastPosition::TopRight,
        }
    }
}

/// Toast theme variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastTheme {
    Light,
    Dark,
    Auto,
}

impl Default for ToastTheme {
    fn default() -> Self {
        ToastTheme::Auto
    }
}

impl From<String> for ToastTheme {
    fn from(s: String) -> Self {
        match s.as_str() {
            "light" => ToastTheme::Light,
            "dark" => ToastTheme::Dark,
            "auto" => ToastTheme::Auto,
            _ => ToastTheme::Auto,
        }
    }
}

/// Toast variant types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

impl Default for ToastVariant {
    fn default() -> Self {
        ToastVariant::Default
    }
}

/// Toast action definition
#[derive(Debug, Clone)]
pub struct ToastAction {
    pub label: String,
    pub action: Callback<()>,
}

/// Toast data structure
#[derive(Debug, Clone)]
pub struct ToastData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration: Option<Duration>,
    pub position: ToastPosition,
    pub theme: ToastTheme,
    pub actions: Vec<ToastAction>,
    pub progress: Option<f64>,
    pub created_at: Instant,
}

impl ToastData {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            variant: ToastVariant::Default,
            duration: Some(Duration::from_millis(4000)),
            position: ToastPosition::TopRight,
            theme: ToastTheme::Auto,
            actions: Vec::new(),
            progress: None,
            created_at: Instant::now(),
        }
    }
}

/// Toast builder for fluent API
#[derive(Debug, Clone)]
pub struct ToastBuilder {
    data: ToastData,
}

impl ToastBuilder {
    pub fn new(title: String) -> Self {
        Self {
            data: ToastData::new(title),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.data.description = Some(description);
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.data.variant = variant;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.data.duration = Some(duration);
        self
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.data.position = position;
        self
    }

    pub fn theme(mut self, theme: ToastTheme) -> Self {
        self.data.theme = theme;
        self
    }

    pub fn action(mut self, action: ToastAction) -> Self {
        self.data.actions.push(action);
        self
    }

    pub fn progress(mut self, progress: f64) -> Self {
        self.data.progress = Some(progress);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.data.id = id;
        self
    }

    pub fn show(self) -> String {
        let toast_id = self.data.id.clone();
        if let Some(provider) = use_context::<SonnerContextValue>() {
            provider.add_toast.run(self.data);
        }
        toast_id
    }
}

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
        ToastVariant::Default => "bg-background text-foreground border",
        ToastVariant::Success => "bg-green-50 text-green-900 border-green-200 dark:bg-green-900 dark:text-green-100 dark:border-green-800",
        ToastVariant::Error => "bg-red-50 text-red-900 border-red-200 dark:bg-red-900 dark:text-red-100 dark:border-red-800",
        ToastVariant::Warning => "bg-yellow-50 text-yellow-900 border-yellow-200 dark:bg-yellow-900 dark:text-yellow-100 dark:border-yellow-800",
        ToastVariant::Info => "bg-blue-50 text-blue-900 border-blue-200 dark:bg-blue-900 dark:text-blue-100 dark:border-blue-800",
        ToastVariant::Loading => "bg-gray-50 text-gray-900 border-gray-200 dark:bg-gray-900 dark:text-gray-100 dark:border-gray-800",
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

/// Toast API functions
pub mod toast {
    use super::*;

    pub fn success(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Success)
    }

    pub fn error(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Error)
    }

    pub fn info(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Info)
    }

    pub fn warning(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Warning)
    }

    pub fn loading(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Loading)
    }

    pub fn custom(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string())
    }

    pub fn dismiss(id: String) {
        if let Some(context) = use_context::<SonnerContextValue>() {
            context.remove_toast.run(id);
        }
    }

    pub fn dismiss_all() {
        if let Some(context) = use_context::<SonnerContextValue>() {
            context.dismiss_all.run(());
        }
    }
}
