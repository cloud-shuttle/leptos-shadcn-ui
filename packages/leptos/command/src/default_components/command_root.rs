use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const COMMAND_CLASS: &str = "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground";

#[derive(Clone)]
pub struct CommandContext {
    pub search: RwSignal<String>,
    pub selected_value: RwSignal<String>,
    pub on_value_change: Option<Callback<String>>,
}

#[component]
pub fn Command(
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let search = RwSignal::new(String::new());
    let selected_value = RwSignal::new(value.get().unwrap_or_default());
    
    // Update selected value when prop changes
    Effect::new(move |_| {
        if let Some(new_value) = value.get() {
            selected_value.set(new_value);
        }
    });
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_CLASS,
        class.get().unwrap_or_default()
    ));
    
    // Create context for child components
    provide_context(CommandContext {
        search,
        selected_value,
        on_value_change,
    });
    
    view! {
        <div 
            class={merged_class}
            role="combobox"
            aria-expanded="true"
        >
            {children()}
        </div>
    }
}
