use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const COMMAND_GROUP_CLASS: &str = "overflow-hidden p-1 text-foreground";
const COMMAND_GROUP_HEADING_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
const COMMAND_ITEM_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const COMMAND_SHORTCUT_CLASS: &str = "ml-auto text-xs tracking-widest text-muted-foreground";
const COMMAND_SEPARATOR_CLASS: &str = "-mx-1 h-px bg-border";

#[component]
pub fn CommandGroup(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_GROUP_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            role="group"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroupHeading(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_GROUP_HEADING_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            role="heading"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItem(
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let context = expect_context::<crate::default_components::command_root::CommandContext>();
    let is_disabled = disabled.get().unwrap_or(false);
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_ITEM_CLASS,
        class.get().unwrap_or_default()
    ));
    
    let handle_click = move |_| {
        if !is_disabled {
            if let Some(value) = value.get() {
                context.selected_value.set(value.clone());
                if let Some(callback) = &context.on_value_change {
                    callback.run(value);
                }
            }
        }
    };
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            data-disabled=is_disabled
            on:click=handle_click
            role="option"
            aria-selected=move || context.selected_value.get() == value.get().unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandShortcut(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_SHORTCUT_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <span 
            class={merged_class}
            id=id.get()
            style=style.get()
        >
            {children()}
        </span>
    }
}

#[component]
pub fn CommandSeparator(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_SEPARATOR_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            role="separator"
        />
    }
}
