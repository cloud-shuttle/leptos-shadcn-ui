//! Basic Signal-Managed Input Component
//! 
//! This module contains the basic signal-managed input component with core functionality.

use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;
use leptos_shadcn_signal_management::*;
use crate::validation::{InputValidator, ValidationResult};
use super::types::{SignalManagedInputState, INPUT_CLASS, INPUT_ERROR_CLASS};

/// Signal-managed Input component
#[component]
pub fn SignalManagedInput(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] input_type: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] validator: Option<InputValidator>,
    #[prop(into, optional)] _validation_error: MaybeProp<String>,
    #[prop(into, optional)] show_validation: Signal<bool>,
) -> impl IntoView {
    // Create persistent state using ArcRwSignal
    let input_state = ArcRwSignal::new(SignalManagedInputState {
        value: value.get().unwrap_or_default(),
        placeholder: placeholder.get().unwrap_or_default(),
        disabled: disabled.get(),
        input_type: input_type.get().unwrap_or_else(|| "text".to_string()),
        validation_result: ValidationResult::new(),
        is_validating: false,
        has_error: false,
        focus_count: 0,
    });

    // Create computed class using ArcMemo
    let input_state_for_class = input_state.clone();
    let input_class = ArcMemo::new(move |_| {
        let state = input_state_for_class.get();
        let base_class = if state.has_error {
            format!("{} {}", INPUT_CLASS, INPUT_ERROR_CLASS)
        } else {
            INPUT_CLASS.to_string()
        };
        format!("{} {}", base_class, class.get().unwrap_or_default())
    });

    // Create validation status using ArcMemo
    let input_state_for_validation = input_state.clone();
    let validation_status = ArcMemo::new(move |_| {
        let state = input_state_for_validation.get();
        if state.is_validating {
            "validating".to_string()
        } else if state.has_error {
            "error".to_string()
        } else if state.validation_result.is_valid {
            "valid".to_string()
        } else {
            "neutral".to_string()
        }
    });

    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(input_state.clone());
    theme_manager.track_memo(input_class.clone());
    theme_manager.track_memo(validation_status.clone());

    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();

    // Create event handler with proper signal management
    let handle_input = {
        let input_state = input_state.clone();
        let on_change = on_change.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                let input_value = input.value();
                callback.run(input_value.clone());
                
                // Update state atomically
                input_state.update(|state| {
                    state.value = input_value.clone();
                    state.focus_count += 1;
                });
                
                // Real-time validation
                if let Some(validator) = &validator {
                    input_state.update(|state| {
                        state.is_validating = true;
                    });
                    
                    let result = validator.validate(&input_value);
                    
                    input_state.update(|state| {
                        state.validation_result = result.clone();
                        state.is_validating = false;
                        state.has_error = !result.is_valid;
                    });
                }
                
                // Check memory pressure and perform cleanup if needed
                if let Some(pressure) = memory_manager.detect_memory_pressure() {
                    match pressure {
                        MemoryPressureLevel::High | MemoryPressureLevel::Critical => {
                            memory_manager.perform_automatic_cleanup();
                        }
                        _ => {}
                    }
                }
            }
        }
    };

    // Create focus handler
    let handle_focus = {
        let input_state = input_state.clone();
        move |_event: leptos::ev::FocusEvent| {
            input_state.update(|state| {
                state.focus_count += 1;
            });
        }
    };

    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();

    let input_state_for_type = input_state.clone();
    let input_state_for_value = input_state.clone();
    let input_state_for_placeholder = input_state.clone();
    let input_state_for_disabled = input_state.clone();
    let input_state_for_validation_display = input_state.clone();
    let input_state_for_performance = input_state.clone();
    
    view! {
        <div class="signal-managed-input-container">
            <input
                type=move || input_state_for_type.get().input_type
                value=move || input_state_for_value.get().value
                placeholder=move || input_state_for_placeholder.get().placeholder
                disabled=move || input_state_for_disabled.get().disabled
                class=move || input_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                on:input=handle_input
                on:focus=handle_focus
            />
            
            // Validation display
            {move || if show_validation.get() && input_state_for_validation_display.get().has_error {
                let error_msg = input_state_for_validation_display.get().validation_result.get_error_message("input").unwrap_or_default().to_string();
                view! {
                    <div class="validation-error text-sm text-destructive mt-1">
                        {error_msg}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            
            // Performance monitoring (only in development)
            #[cfg(debug_assertions)]
            <div class="performance-monitor text-xs text-muted-foreground mt-1">
                {move || format!("Focus count: {}, Status: {}", 
                    input_state_for_performance.get().focus_count, 
                    validation_status.get()
                )}
            </div>
        </div>
    }
}
