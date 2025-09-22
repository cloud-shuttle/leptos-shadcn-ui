use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;
use tailwind_fuse::*;

// Select Root Provider
#[component]
pub fn Select(
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_open = RwSignal::new(false);
    let internal_value = RwSignal::new(default_value.get().unwrap_or_default());
    
    let open_state = Signal::derive(move || {
        if open.get() != internal_open.get() {
            open.get()
        } else {
            internal_open.get()
        }
    });
    
    let value_state = Signal::derive(move || {
        if !value.get().is_empty() && value.get() != internal_value.get() {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_open = Callback::new(move |new_open: bool| {
        internal_open.set(new_open);
        if let Some(callback) = &on_open_change {
            callback.run(new_open);
        }
    });

    let set_value = Callback::new(move |new_value: String| {
        internal_value.set(new_value.clone());
        if let Some(callback) = &on_value_change {
            callback.run(new_value);
        }
    });

    provide_context(open_state);
    provide_context(value_state);
    provide_context(set_open);
    provide_context(set_value);
    provide_context(disabled);
    provide_context(required);
    provide_context(name);

    children.map(|c| c())
}

// Select Trigger
#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<Signal<bool>>();
    let set_open = expect_context::<Callback<bool>>();
    let disabled = expect_context::<Signal<bool>>();
    
    let trigger_class = classes!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
        class.get().unwrap_or_default()
    );

    let handle_click = move |_e: MouseEvent| {
        if !disabled.get() {
            set_open.run(!open_state.get());
        }
    };

    view! {
        <button
            class=trigger_class
            id=id.get()
            style=style.get()
            on:click=handle_click
            disabled=disabled
            aria-expanded=open_state
            aria-haspopup="listbox"
            role="combobox"
        >
            {children.map(|c| c())}
        </button>
    }
}

// Select Value
#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let value_state = expect_context::<Signal<String>>();
    
    let value_class = classes!(
        "placeholder:text-muted-foreground",
        class.get().unwrap_or_default()
    );

    view! {
        <span
            class=value_class
            id=id.get()
            style=style.get()
        >
            {move || {
                let current_value = value_state.get();
                if current_value.is_empty() {
                    placeholder.get().unwrap_or_default()
                } else {
                    current_value
                }
            }}
        </span>
    }
}
