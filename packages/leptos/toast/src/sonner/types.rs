//! Types and enums for the Sonner toast component
//! 
//! This module contains the core types, enums, and data structures
//! used by the Sonner toast system.

use leptos::prelude::*;
use std::time::{Duration, Instant};

/// Toast position variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}

impl Default for ToastPosition {
    fn default() -> Self {
        ToastPosition::TopRight
    }
}

impl From<String> for ToastPosition {
    fn from(s: String) -> Self {
        match s.as_str() {
            "top-left" => ToastPosition::TopLeft,
            "top-right" => ToastPosition::TopRight,
            "bottom-left" => ToastPosition::BottomLeft,
            "bottom-right" => ToastPosition::BottomRight,
            "top-center" => ToastPosition::TopCenter,
            "bottom-center" => ToastPosition::BottomCenter,
            _ => ToastPosition::TopRight,
        }
    }
}

/// Toast theme variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastTheme {
    Light,
    Dark,
    Auto,
}

impl Default for ToastTheme {
    fn default() -> Self {
        ToastTheme::Auto
    }
}

impl From<String> for ToastTheme {
    fn from(s: String) -> Self {
        match s.as_str() {
            "light" => ToastTheme::Light,
            "dark" => ToastTheme::Dark,
            "auto" => ToastTheme::Auto,
            _ => ToastTheme::Auto,
        }
    }
}

/// Toast variant types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

impl Default for ToastVariant {
    fn default() -> Self {
        ToastVariant::Default
    }
}

/// Toast action definition
#[derive(Debug, Clone)]
pub struct ToastAction {
    pub label: String,
    pub action: Callback<()>,
}

/// Toast data structure
#[derive(Debug, Clone)]
pub struct ToastData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration: Option<Duration>,
    pub position: ToastPosition,
    pub theme: ToastTheme,
    pub actions: Vec<ToastAction>,
    pub progress: Option<f64>,
    pub created_at: Instant,
}

impl ToastData {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            variant: ToastVariant::Default,
            duration: Some(Duration::from_millis(4000)),
            position: ToastPosition::TopRight,
            theme: ToastTheme::Auto,
            actions: Vec::new(),
            progress: None,
            created_at: Instant::now(),
        }
    }
}
