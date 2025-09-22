use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const COMMAND_INPUT_CLASS: &str = "flex items-center border-b px-3";
const COMMAND_INPUT_WRAPPER_CLASS: &str = "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn CommandInput(
    #[prop(optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] id: MaybeProp<String>,
    #[prop(optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let context = expect_context::<crate::default_components::command_root::CommandContext>();
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_INPUT_CLASS,
        class.get().unwrap_or_default()
    ));
    
    let input_class = tw_merge!(&format!("{} {}", 
        COMMAND_INPUT_WRAPPER_CLASS,
        class.get().unwrap_or_default()
    ));
    
    let handle_input = move |ev: leptos::ev::InputEvent| {
        let value = event_target_value(&ev);
        context.search.set(value);
    };
    
    view! {
        <div class={merged_class}>
            <input
                class={input_class}
                id=id.get()
                style=style.get()
                placeholder=placeholder.get().unwrap_or_default()
                value=context.search
                on:input=handle_input
                type="text"
                role="searchbox"
                aria-label="Search commands"
            />
        </div>
    }
}
