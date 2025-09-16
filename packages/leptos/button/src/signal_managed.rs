//! Signal-managed version of the Button component using leptos-shadcn-signal-management

use leptos::prelude::*;
use leptos_style::Style;
use leptos_shadcn_signal_management::*;

pub const BUTTON_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

/// Signal-managed button state
#[derive(Debug, Clone, PartialEq)]
pub struct SignalManagedButtonState {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
    pub click_count: u32,
}

impl Default for SignalManagedButtonState {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            click_count: 0,
        }
    }
}


/// Props for child components when using as_child
#[derive(Debug, Clone)]
pub struct SignalManagedButtonChildProps {
    pub class: String,
    pub id: String,
    pub style: String,
    pub disabled: bool,
    pub r#type: String,
    pub onclick: Option<Callback<()>>,
}

/// Signal-managed Button component with advanced memory management and lifecycle optimization
#[component]
pub fn SignalManagedButton(
    #[prop(into, optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<ButtonSize>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] as_child: Option<Callback<SignalManagedButtonChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent button state using ArcRwSignal
    let button_state = ArcRwSignal::new(SignalManagedButtonState {
        variant: variant.get().unwrap_or_default(),
        size: size.get().unwrap_or_default(),
        disabled: disabled.get(),
        loading: false,
        click_count: 0,
    });
    
    // Create computed class using ArcMemo for better performance
    let button_state_for_class = button_state.clone();
    let button_class = ArcMemo::new(move |_| {
        let state = button_state_for_class.get();
        let variant_class = match state.variant {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        };
        
        let size_class = match state.size {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        };
        
        let loading_class = if state.loading { "loading" } else { "" };
        let disabled_class = if state.disabled { "disabled" } else { "" };
        
        format!("{} {} {} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            loading_class,
            disabled_class,
            class.get().unwrap_or_default()
        )
    });
    
    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(button_state.clone());
    theme_manager.track_memo(button_class.clone());
    
    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();
    
    // Create event handler with proper signal management
    let handle_click = {
        let button_state = button_state.clone();
        let on_click = on_click.clone();
        move |_event: leptos::ev::MouseEvent| {
            if !button_state.get().disabled && !button_state.get().loading {
                // Update state atomically
                button_state.update(|state| {
                    state.loading = true;
                    state.click_count += 1;
                });
                
                // Run the original callback if provided
                if let Some(callback) = &on_click {
                    callback.run(());
                }
                
                // Simulate async operation (in real usage, this would be an actual async operation)
                // For now, we'll just reset the loading state
                button_state.update(|state| {
                    state.loading = false;
                });
                
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
    
    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();
    
    // Implement as_child functionality using conditional rendering
    if let Some(as_child) = as_child {
        let child_props = SignalManagedButtonChildProps {
            class: button_class.get(),
            id: id.get().unwrap_or_default(),
            style: style.get().to_string(),
            disabled: button_state.get().disabled,
            r#type: "button".to_string(),
            onclick: Some(Callback::new(move |_| {
                // Create a dummy MouseEvent for the callback
                // In a real implementation, this would be the actual event
                handle_click(leptos::ev::MouseEvent::new("click").unwrap());
            })),
        };
        as_child.run(child_props).into_any()
        } else {
            let button_state_for_disabled = button_state.clone();
            let button_state_for_loading = button_state.clone();
            view! {
                <button
                    class=move || button_class.get()
                    id=move || id.get().unwrap_or_default()
                    style=move || style.get().to_string()
                    disabled=move || button_state_for_disabled.get().disabled
                    on:click=handle_click
                >
                    {move || if button_state_for_loading.get().loading {
                        view! { "Loading..." }.into_any()
                    } else {
                        view! { "Button" }.into_any()
                    }}
                </button>
            }.into_any()
        }
}

/// Enhanced Button component with signal management and performance monitoring
#[component]
pub fn EnhancedButton(
    #[prop(into, optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(into, optional)] size: MaybeProp<ButtonSize>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create persistent button state using ArcRwSignal
    let button_state = ArcRwSignal::new(SignalManagedButtonState {
        variant: variant.get().unwrap_or_default(),
        size: size.get().unwrap_or_default(),
        disabled: disabled.get(),
        loading: false,
        click_count: 0,
    });
    
    // Create computed class using ArcMemo
    let button_state_for_class = button_state.clone();
    let button_class = ArcMemo::new(move |_| {
        let state = button_state_for_class.get();
        let variant_class = match state.variant {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        };
        
        let size_class = match state.size {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        };
        
        format!("{} {} {} {}", 
            BUTTON_CLASS, 
            variant_class, 
            size_class, 
            class.get().unwrap_or_default()
        )
    });
    
    // Create performance monitoring
    let button_state_for_metrics = button_state.clone();
    let performance_metrics = ArcMemo::new(move |_| {
        let state = button_state_for_metrics.get();
        format!("Clicks: {}, Loading: {}", state.click_count, state.loading)
    });
    
    // Create theme manager for lifecycle management
    let theme_manager = TailwindSignalManager::new();
    theme_manager.track_signal(button_state.clone());
    theme_manager.track_memo(button_class.clone());
    theme_manager.track_memo(performance_metrics.clone());
    
    // Create memory manager for monitoring
    let memory_manager = SignalMemoryManager::new();
    
    // Create event handler with performance monitoring
    let handle_click = {
        let button_state = button_state.clone();
        let on_click = on_click.clone();
        move |_event: leptos::ev::MouseEvent| {
            if !button_state.get().disabled && !button_state.get().loading {
                // Update state atomically
                button_state.update(|state| {
                    state.loading = true;
                    state.click_count += 1;
                });
                
                // Run the original callback if provided
                if let Some(callback) = &on_click {
                    callback.run(());
                }
                
                // Simulate async operation
                button_state.update(|state| {
                    state.loading = false;
                });
                
                // Monitor memory usage
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
    
    // Apply lifecycle optimization
    theme_manager.apply_lifecycle_optimization();
    
    let button_state_for_disabled = button_state.clone();
    let button_state_for_loading = button_state.clone();
    view! {
        <div class="enhanced-button-container">
            <button
                class=move || button_class.get()
                id=move || id.get().unwrap_or_default()
                style=move || style.get().to_string()
                disabled=move || button_state_for_disabled.get().disabled
                on:click=handle_click
            >
                {move || if button_state_for_loading.get().loading {
                    view! { "Loading..." }.into_any()
                } else {
                    view! { "Enhanced Button" }.into_any()
                }}
            </button>
            
            // Performance monitoring (only in development)
            #[cfg(debug_assertions)]
            <div class="performance-monitor">
                <small>{move || performance_metrics.get()}</small>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use leptos::prelude::*; // Removed unused import
    
    #[test]
    fn test_signal_managed_button_creation() {
        let button_state = ArcRwSignal::new(SignalManagedButtonState::default());
        assert_eq!(button_state.get().click_count, 0);
        assert!(!button_state.get().loading);
    }
    
    #[test]
    fn test_button_state_updates() {
        let button_state = ArcRwSignal::new(SignalManagedButtonState::default());
        
        // Test state update
        button_state.update(|state| {
            state.click_count = 1;
            state.loading = true;
        });
        
        assert_eq!(button_state.get().click_count, 1);
        assert!(button_state.get().loading);
    }
    
    #[test]
    fn test_button_class_computation() {
        let button_state = ArcRwSignal::new(SignalManagedButtonState {
            variant: ButtonVariant::Default,
            size: ButtonSize::Lg,
            disabled: false,
            loading: false,
            click_count: 0,
        });
        
        let button_class = ArcMemo::new(move |_| {
            let state = button_state.get();
            format!("btn btn-{} btn-{}", 
                match state.variant {
                    ButtonVariant::Default => "default",
                    _ => "other",
                },
                match state.size {
                    ButtonSize::Lg => "lg",
                    _ => "other",
                }
            )
        });
        
        let class = button_class.get();
        assert!(class.contains("btn-default"));
        assert!(class.contains("btn-lg"));
    }
    
    #[test]
    fn test_theme_manager_integration() {
        let manager = TailwindSignalManager::new();
        let button_state = ArcRwSignal::new(SignalManagedButtonState::default());
        
        manager.track_signal(button_state.clone());
        assert_eq!(manager.tracked_signals_count(), 1);
        
        let button_class = ArcMemo::new(move |_| "btn".to_string());
        manager.track_memo(button_class);
        assert_eq!(manager.tracked_memos_count(), 1);
    }
    
    #[test]
    fn test_memory_management_integration() {
        let memory_manager = SignalMemoryManager::new();
        let button_state = ArcRwSignal::new(SignalManagedButtonState::default());
        
        // Test memory pressure detection
        let pressure = memory_manager.detect_memory_pressure();
        assert!(pressure.is_some() || pressure.is_none());
        
        // Test automatic cleanup
        let cleanup_performed = memory_manager.perform_automatic_cleanup();
        assert!(cleanup_performed || !cleanup_performed);
    }
}