//! Toast builder for fluent API
//! 
//! This module contains the ToastBuilder struct and its implementation
//! for creating toasts with a fluent API.

use leptos::prelude::*;
use crate::sonner::types::{ToastData, ToastVariant, ToastPosition, ToastTheme, ToastAction};
use crate::sonner::context::SonnerContextValue;

/// Toast builder for fluent API
#[derive(Debug, Clone)]
pub struct ToastBuilder {
    data: ToastData,
}

impl ToastBuilder {
    pub fn new(title: String) -> Self {
        Self {
            data: ToastData::new(title),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.data.description = Some(description);
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.data.variant = variant;
        self
    }

    pub fn duration(mut self, duration: std::time::Duration) -> Self {
        self.data.duration = Some(duration);
        self
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.data.position = position;
        self
    }

    pub fn theme(mut self, theme: ToastTheme) -> Self {
        self.data.theme = theme;
        self
    }

    pub fn action(mut self, action: ToastAction) -> Self {
        self.data.actions.push(action);
        self
    }

    pub fn progress(mut self, progress: f64) -> Self {
        self.data.progress = Some(progress);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.data.id = id;
        self
    }

    pub fn show(self) -> String {
        let toast_id = self.data.id.clone();
        if let Some(provider) = use_context::<SonnerContextValue>() {
            provider.add_toast.run(self.data);
        }
        toast_id
    }
}
