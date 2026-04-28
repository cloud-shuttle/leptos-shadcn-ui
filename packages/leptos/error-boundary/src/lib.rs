//! Production-Ready Error Handling for Leptos
//!
//! This module provides comprehensive error handling for production applications.
//! It includes rich error context, graceful degradation, and enhanced user experience.

mod context;

pub mod default;
pub mod new_york;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;

pub use context::{
    ErrorContext, ErrorContextBuilder, ErrorSeverity, ErrorSource, RecoverySuggestion,
};

use leptos::prelude::*;
use std::panic::PanicHookInfo;

/// Simple error information for production use (legacy, consider using ErrorContext instead)
#[derive(Clone, Debug)]
pub struct ErrorInfo {
    /// User-friendly error message
    pub message: String,
    /// Technical error details (for logging)
    pub technical_details: Option<String>,
}

/// Simple error fallback component
#[component]
pub fn ErrorFallback(
    #[prop(into)] error_info: ErrorInfo,
) -> impl IntoView {
    view! {
        <div class="error-fallback">
            <div class="error-content">
                <div class="error-icon">!</div>
                <h2 class="error-title">Something went wrong</h2>
                <p class="error-message">
                    {error_info.message}
                </p>
                <div class="error-actions">
                    <button
                        class="error-retry"
                        on:click=move |_| {
                            // Simple page reload for now
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    >
                        "Try Again"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Simple error boundary wrapper
#[component]
pub fn ErrorBoundary(
    #[prop(into)] children: Children,
) -> impl IntoView {
    let (has_error, set_has_error) = signal(false);
    let (_error_info, set_error_info) = signal(None::<ErrorInfo>);

    // Set up panic hook for production error handling
    std::panic::set_hook(Box::new(move |panic_info: &PanicHookInfo<'_>| {
        log::error!("Panic caught: {:?}", panic_info);

        let error = ErrorInfo {
            message: "An unexpected error occurred. Please try refreshing the page.".to_string(),
            technical_details: Some(format!("{:?}", panic_info)),
        };

        set_error_info.set(Some(error));
        set_has_error.set(true);
    }));

    // Render children or error fallback using a different approach
    if has_error.get() {
        if let Some(error) = _error_info.get() {
            view! { <ErrorFallback error_info=error /> }.into_any()
        } else {
            view! { <ErrorFallback error_info=ErrorInfo { message: "An error occurred".to_string(), technical_details: None } /> }.into_any()
        }
    } else {
        children()
    }
}

/// Rich error fallback component with enhanced debugging context
#[component]
pub fn RichErrorFallback(
    #[prop(into)] error_context: ErrorContext,
    #[prop(into, optional)] show_technical: MaybeProp<bool>,
) -> impl IntoView {
    let (show_details, set_show_details) = signal(show_technical.get().unwrap_or(false));

    let severity_class = move || match error_context.severity {
        ErrorSeverity::Info => "info",
        ErrorSeverity::Warning => "warning",
        ErrorSeverity::Error => "error",
        ErrorSeverity::Critical => "critical",
    };

    let icon = match error_context.severity {
        ErrorSeverity::Info => "i",
        ErrorSeverity::Warning => "⚠",
        ErrorSeverity::Error => "!",
        ErrorSeverity::Critical => "!!",
    };

    view! {
        <div class="rich-error-fallback" data-severity=severity_class>
            <div class="error-content">
                <div class="error-header">
                    <div class="error-icon">{icon}</div>
                    <div class="error-title-section">
                        <h2 class="error-title">
                            {match error_context.severity {
                                ErrorSeverity::Info => "Information",
                                ErrorSeverity::Warning => "Warning",
                                ErrorSeverity::Error => "Something went wrong",
                                ErrorSeverity::Critical => "Critical Error",
                            }.to_string()}
                        </h2>
                        {if let Some(error_code) = &error_context.error_code {
                            view! {
                                <span class="error-code">{error_code.clone()}</span>
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                </div>

                <p class="error-message">
                    {error_context.message.clone()}
                </p>

                {if !error_context.recovery_suggestions.is_empty() {
                    view! {
                        <div class="error-recovery">
                            <h3>"What you can do:"</h3>
                            <ul class="recovery-suggestions">
                                {error_context.recovery_suggestions.iter().map(|suggestion| {
                                    view! {
                                        <li class="recovery-item">
                                            <span class="recovery-action">{suggestion.action.clone()}</span>
                                            {if let Some(explanation) = &suggestion.explanation {
                                                view! {
                                                    <span class="recovery-explanation">{explanation.clone()}</span>
                                                }.into_any()
                                            } else {
                                                ().into_any()
                                            }}
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="error-actions">
                            <button
                                class="error-retry"
                                on:click=move |_| {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.location().reload();
                                    }
                                }
                            >
                                "Try Again"
                            </button>
                        </div>
                    }.into_any()
                }}

                <div class="error-details-toggle">
                    <button
                        class="toggle-details"
                        on:click=move |_| {
                            set_show_details.set(!show_details.get());
                        }
                    >
                        {move || if show_details.get() { "Hide" } else { "Show" }}
                        " technical details"
                    </button>
                </div>

                {move || if show_details.get() {
                    view! {
                        <div class="error-technical-details">
                            {if let Some(technical) = &error_context.technical_details {
                                view! {
                                    <div class="technical-section">
                                        <h4>"Technical Details:"</h4>
                                        <pre>{technical.clone()}</pre>
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {if let Some(cause) = &error_context.cause {
                                view! {
                                    <div class="technical-section">
                                        <h4>"Underlying Cause:"</h4>
                                        <pre>{cause.clone()}</pre>
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {if error_context.source.file.is_some() {
                                view! {
                                    <div class="technical-section">
                                        <h4>"Source Location:"</h4>
                                        <code>{error_context.source.to_string()}</code>
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {if !error_context.metadata.is_empty() {
                                view! {
                                    <div class="technical-section">
                                        <h4>"Additional Context:"</h4>
                                        <dl class="metadata-list">
                                            {error_context.metadata.iter().map(|(key, value)| {
                                                view! {
                                                    <div class="metadata-item">
                                                        <dt>{key.clone()}</dt>
                                                        <dd>{value.clone()}</dd>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </dl>
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {if let Some(timestamp) = &error_context.timestamp {
                                view! {
                                    <div class="technical-section">
                                        <h4>"Timestamp:"</h4>
                                        <code>{timestamp.to_rfc3339()}</code>
                                    </div>
                                }.into_any()
                            } else {
                                ().into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </div>
        </div>
    }
}

/// Enhanced error boundary wrapper with rich context
#[component]
pub fn RichErrorBoundary(
    #[prop(into)] children: Children,
    #[prop(into, optional)] fallback: Option<Callback<ErrorContext>>,
) -> impl IntoView {
    let (has_error, set_has_error) = signal(false);
    let (error_context, set_error_context) = signal(None::<ErrorContext>);

    // Set up panic hook for enhanced error handling
    std::panic::set_hook(Box::new(move |panic_info: &PanicHookInfo<'_>| {
        log::error!("Panic caught with enhanced context: {:?}", panic_info);

        let context = ErrorContext::from_panic(panic_info);
        set_error_context.set(Some(context));
        set_has_error.set(true);
    }));

    // Render children or error fallback
    if has_error.get() {
        if let Some(context) = error_context.get() {
            if let Some(fallback_callback) = &fallback {
                fallback_callback.run(context).into_any()
            } else {
                view! { <RichErrorFallback error_context=context /> }.into_any()
            }
        } else {
            let default_context = ErrorContext::new(
                "An error occurred".to_string()
            ).with_severity(ErrorSeverity::Error);
            view! { <RichErrorFallback error_context=default_context /> }.into_any()
        }
    } else {
        children()
    }
}

/// Hook for manual error handling
pub fn use_error_handler() -> (ReadSignal<bool>, WriteSignal<bool>, WriteSignal<Option<ErrorInfo>>) {
    let (has_error, set_has_error) = signal(false);
    let (_error_info, set_error_info) = signal(None::<ErrorInfo>);

    (has_error, set_has_error, set_error_info)
}

/// Hook for enhanced error handling with rich context
pub fn use_rich_error_handler() -> (
    ReadSignal<bool>,
    WriteSignal<bool>,
    ReadSignal<Option<ErrorContext>>,
    WriteSignal<Option<ErrorContext>>,
) {
    let (has_error, set_has_error) = signal(false);
    let (error_context, set_error_context) = signal(None::<ErrorContext>);

    (has_error, set_has_error, error_context, set_error_context)
}

/// Utility function to create rich error context
#[track_caller]
pub fn create_rich_error(message: String, technical: Option<String>) -> ErrorContext {
    ErrorContext::new(message).with_technical_details(
        technical.unwrap_or_else(|| "No technical details available".to_string())
    )
}

/// Utility function to create error context with severity
#[track_caller]
pub fn create_error_with_severity(
    message: String,
    severity: ErrorSeverity,
    technical: Option<String>,
) -> ErrorContext {
    ErrorContext::new(message)
        .with_severity(severity)
        .with_technical_details(technical.unwrap_or_default())
}

/// Utility function to handle errors with rich context
pub fn handle_error_with_context<T, E>(
    result: Result<T, E>,
    user_message: &str,
) -> Result<T, ErrorContext>
where
    E: std::fmt::Debug,
{
    result.map_err(|error| {
        log::error!("Error occurred: {:?}", error);
        ErrorContext::new(user_message.to_string())
            .with_technical_details(format!("{:?}", error))
            .with_severity(ErrorSeverity::Error)
    })
}

/// Utility function to wrap errors with additional context
pub fn wrap_error_with_context(
    original: ErrorContext,
    additional_message: String,
    additional_metadata: Option<std::collections::HashMap<String, String>>,
) -> ErrorContext {
    let mut wrapped = ErrorContext::new(additional_message)
        .with_severity(original.severity)
        .with_cause(original.message.clone())
        .with_technical_details(original.debug_string());

    if let Some(metadata) = additional_metadata {
        wrapped = wrapped.with_metadata_map(metadata);
    }

    // Preserve original error code and append wrapped context
    if let Some(code) = original.error_code {
        wrapped = wrapped.with_error_code(format!("WRAPPED_{}", code));
    }

    wrapped
}

/// Utility function to create user-friendly error messages
pub fn create_user_error(message: &str, technical: Option<&str>) -> ErrorInfo {
    ErrorInfo {
        message: message.to_string(),
        technical_details: technical.map(|s| s.to_string()),
    }
}

/// Utility function to handle errors gracefully
pub fn handle_error<T>(result: Result<T, impl std::fmt::Debug>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            log::error!("Error occurred: {:?}", error);
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_error_info_creation() {
        let error = ErrorInfo {
            message: "Test error".to_string(),
            technical_details: Some("Technical details".to_string()),
        };

        assert_eq!(error.message, "Test error");
        assert_eq!(error.technical_details, Some("Technical details".to_string()));
    }

    #[test]
    fn test_create_user_error() {
        let error = create_user_error("User message", Some("Technical"));
        assert_eq!(error.message, "User message");
        assert_eq!(error.technical_details, Some("Technical".to_string()));
    }

    #[test]
    fn test_handle_error() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(handle_error(result), Some(42));

        let error_result: Result<i32, &str> = Err("Error");
        assert_eq!(handle_error(error_result), None);
    }

    #[test]
    fn test_error_info_without_technical_details() {
        let error = ErrorInfo {
            message: "Simple error".to_string(),
            technical_details: None,
        };

        assert_eq!(error.message, "Simple error");
        assert_eq!(error.technical_details, None);
    }

    #[test]
    fn test_create_user_error_without_technical() {
        let error = create_user_error("User message", None);
        assert_eq!(error.message, "User message");
        assert_eq!(error.technical_details, None);
    }

    #[test]
    fn test_use_error_handler() {
        let (has_error, set_has_error, set_error_info) = use_error_handler();

        // Initially no error
        assert!(!has_error.get());

        // Set an error
        let error = ErrorInfo {
            message: "Test error".to_string(),
            technical_details: None,
        };
        set_error_info.set(Some(error));
        set_has_error.set(true);

        // Check error is set
        assert!(has_error.get());
    }

    #[test]
    fn test_error_info_clone_and_debug() {
        let error = ErrorInfo {
            message: "Test error".to_string(),
            technical_details: Some("Technical".to_string()),
        };

        let cloned = error.clone();
        assert_eq!(error.message, cloned.message);
        assert_eq!(error.technical_details, cloned.technical_details);

        // Test debug formatting
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Test error"));
        assert!(debug_str.contains("Technical"));
    }
}
