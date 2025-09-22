use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;
use tailwind_fuse::*;

// Select Content
#[component]
pub fn SelectContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open_state = expect_context::<Signal<bool>>();
    let set_open = expect_context::<Callback<bool>>();
    
    let content_class = classes!(
        "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        class.get().unwrap_or_default()
    );

    let handle_click_outside = move |_e: MouseEvent| {
        set_open.run(false);
    };

    view! {
        <div
            class=content_class
            id=id.get()
            style=style.get()
            data-state=move || if open_state.get() { "open" } else { "closed" }
            role="listbox"
            aria-expanded=open_state
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Item
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let value_state = expect_context::<Signal<String>>();
    let set_value = expect_context::<Callback<String>>();
    let set_open = expect_context::<Callback<bool>>();
    
    let item_class = classes!(
        "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class.get().unwrap_or_default()
    );

    let is_selected = Signal::derive(move || value_state.get() == value);

    let handle_click = move |_e: MouseEvent| {
        set_value.run(value.clone());
        set_open.run(false);
    };

    view! {
        <div
            class=item_class
            id=id.get()
            style=style.get()
            on:click=handle_click
            role="option"
            aria-selected=is_selected
            data-value=value.clone()
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Separator
#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let separator_class = classes!(
        "-mx-1 my-1 h-px bg-muted",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=separator_class
            id=id.get()
            style=style.get()
            role="separator"
        />
    }
}

// Select Label
#[component]
pub fn SelectLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let label_class = classes!(
        "py-1.5 pl-8 pr-2 text-sm font-semibold",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=label_class
            id=id.get()
            style=style.get()
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Group
#[component]
pub fn SelectGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let group_class = classes!(
        "p-1 text-muted-foreground",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=group_class
            id=id.get()
            style=style.get()
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}
