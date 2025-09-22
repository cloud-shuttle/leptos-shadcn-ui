use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const COMMAND_LIST_CLASS: &str = "max-h-[300px] overflow-y-auto overflow-x-hidden";
const COMMAND_EMPTY_CLASS: &str = "py-6 text-center text-sm";

#[component]
pub fn CommandList(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_LIST_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            role="listbox"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandEmpty(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_EMPTY_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            id=id.get()
            style=style.get()
            role="status"
            aria-live="polite"
        >
            {children()}
        </div>
    }
}
