use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;
use crate::validation::{InputValidator, ValidationResult};

pub const INPUT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

pub const INPUT_ERROR_CLASS: &str = "border-destructive focus-visible:ring-destructive";

#[component]
pub fn Input(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] input_type: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    // TDD Enhancement: Validation props
    #[prop(into, optional)] validator: Option<InputValidator>,
    #[prop(into, optional)] validation_error: MaybeProp<String>,
    #[prop(into, optional)] show_validation: Signal<bool>,
) -> impl IntoView {
    let (validation_result, set_validation_result) = signal(ValidationResult::new());
    let (_is_validating, _set_is_validating) = signal(false);

    let handle_input = {
        let on_change = on_change.clone();
        let set_validation_result = set_validation_result.clone();
        let _set_is_validating = _set_is_validating.clone();
        
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                let input_value = input.value();
                callback.run(input_value.clone());
                
                // TDD Enhancement: Real-time validation
                if let Some(validator) = &validator {
                    _set_is_validating.set(true);
                    let result = validator.validate(&input_value);
                    set_validation_result.set(result);
                    _set_is_validating.set(false);
                }
            }
        }
    };

    let computed_class = Signal::derive(move || {
        let base_class = INPUT_CLASS;
        let custom_class = class.get().unwrap_or_default();
        let error_class = if validation_result.get().has_errors() || validation_error.get().is_some() {
            INPUT_ERROR_CLASS
        } else {
            ""
        };
        
        format!("{} {} {}", base_class, custom_class, error_class).trim().to_string()
    });

    let display_error = Signal::derive(move || {
        if let Some(error) = validation_error.get() {
            Some(error)
        } else if validation_result.get().has_errors() {
            validation_result.get().errors.first().map(|e| e.message.clone())
        } else {
            None
        }
    });

    view! {
        <div class="space-y-1">
            <input
                r#type=move || input_type.get().unwrap_or_else(|| "text".to_string())
                value=move || value.get().unwrap_or_default()
                placeholder=move || placeholder.get().unwrap_or_default()
                disabled=move || disabled.get()
                class=move || computed_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                on:input=handle_input
                aria-invalid=move || (validation_result.get().has_errors() || validation_error.get().is_some()).to_string()
                aria-describedby=move || {
                    if display_error.get().is_some() {
                        format!("{}-error", id.get().unwrap_or_default())
                    } else {
                        String::new()
                    }
                }
            />
            <Show when=move || display_error.get().is_some() && show_validation.get()>
                <p 
                    id=move || format!("{}-error", id.get().unwrap_or_default())
                    class="text-sm text-destructive"
                    role="alert"
                >
                    {move || display_error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
