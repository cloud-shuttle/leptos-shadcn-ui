//! Toast API functions
//! 
//! This module contains the public API functions for creating
//! and managing toast notifications.

use leptos::prelude::*;
use crate::sonner::builder::ToastBuilder;
use crate::sonner::types::ToastVariant;
use crate::sonner::context::SonnerContextValue;

/// Toast API functions
pub mod toast {
    use super::*;

    pub fn success(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Success)
    }

    pub fn error(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Error)
    }

    pub fn info(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Info)
    }

    pub fn warning(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Warning)
    }

    pub fn loading(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string()).variant(ToastVariant::Loading)
    }

    pub fn custom(title: &str) -> ToastBuilder {
        ToastBuilder::new(title.to_string())
    }

    pub fn dismiss(id: String) {
        if let Some(context) = use_context::<SonnerContextValue>() {
            context.remove_toast.run(id);
        }
    }

    pub fn dismiss_all() {
        if let Some(context) = use_context::<SonnerContextValue>() {
            context.dismiss_all.run(());
        }
    }
}
