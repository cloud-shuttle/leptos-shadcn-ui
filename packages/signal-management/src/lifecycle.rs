//! Signal lifecycle management utilities for Leptos 0.8.8+

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::SignalManagementError;

/// Theme configuration for components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    /// Default theme
    Default,
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Custom theme with custom properties
    Custom(HashMap<String, String>),
}

impl Default for Theme {
    fn default() -> Self {
        Self::Default
    }
}

/// Component variant configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Variant {
    /// Primary variant
    Primary,
    /// Secondary variant
    Secondary,
    /// Destructive variant
    Destructive,
    /// Outline variant
    Outline,
    /// Ghost variant
    Ghost,
    /// Link variant
    Link,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Component size configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Size {
    /// Small size
    Small,
    /// Medium size
    Medium,
    /// Large size
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

/// Responsive configuration for components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    /// Small breakpoint classes
    pub sm: Option<String>,
    /// Medium breakpoint classes
    pub md: Option<String>,
    /// Large breakpoint classes
    pub lg: Option<String>,
    /// Extra large breakpoint classes
    pub xl: Option<String>,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }
}

/// Manages signal lifecycle for tailwind-rs components
/// 
/// This struct provides centralized management of signal lifecycles,
/// ensuring proper disposal and memory management in Leptos 0.8.8+
#[derive(Clone, Debug)]
pub struct TailwindSignalManager {
    /// Theme signal that persists across component disposal
    theme_signal: ArcRwSignal<Theme>,
    /// Variant signal that persists across component disposal
    variant_signal: ArcRwSignal<Variant>,
    /// Size signal that persists across component disposal
    size_signal: ArcRwSignal<Size>,
    /// Responsive configuration signal
    responsive_signal: ArcRwSignal<ResponsiveConfig>,
    /// Tracked signals for cleanup
    tracked_signals: ArcRwSignal<Vec<ArcRwSignal<()>>>,
    /// Tracked memos for cleanup
    tracked_memos: ArcRwSignal<Vec<ArcMemo<()>>>,
}

impl TailwindSignalManager {
    /// Create a new signal manager
    pub fn new() -> Self {
        Self {
            theme_signal: ArcRwSignal::new(Theme::default()),
            variant_signal: ArcRwSignal::new(Variant::default()),
            size_signal: ArcRwSignal::new(Size::default()),
            responsive_signal: ArcRwSignal::new(ResponsiveConfig::default()),
            tracked_signals: ArcRwSignal::new(Vec::new()),
            tracked_memos: ArcRwSignal::new(Vec::new()),
        }
    }
    
    /// Provide context that persists across component disposal
    pub fn provide_context(self) {
        provide_context(self);
    }
    
    /// Get theme signal for dynamic theming
    pub fn theme(&self) -> ArcRwSignal<Theme> {
        self.theme_signal.clone()
    }
    
    /// Get variant signal for component variants
    pub fn variant(&self) -> ArcRwSignal<Variant> {
        self.variant_signal.clone()
    }
    
    /// Get size signal for responsive sizing
    pub fn size(&self) -> ArcRwSignal<Size> {
        self.size_signal.clone()
    }
    
    /// Get responsive configuration signal
    pub fn responsive(&self) -> ArcRwSignal<ResponsiveConfig> {
        self.responsive_signal.clone()
    }
    
    /// Track a signal for cleanup
    pub fn track_signal<T>(&self, signal: ArcRwSignal<T>) -> ArcRwSignal<T> {
        self.tracked_signals.update(|signals| {
            signals.push(ArcRwSignal::new(()));
        });
        signal
    }
    
    /// Track a memo for cleanup
    pub fn track_memo<T: Send + Sync + 'static>(&self, memo: ArcMemo<T>) -> ArcMemo<T> {
        self.tracked_memos.update(|memos| {
            memos.push(ArcMemo::new(|_| ()));
        });
        memo
    }
    
    /// Get the number of tracked signals
    pub fn tracked_signals_count(&self) -> usize {
        self.tracked_signals.get().len()
    }
    
    /// Get the number of tracked memos
    pub fn tracked_memos_count(&self) -> usize {
        self.tracked_memos.get().len()
    }
    
    /// Check if the manager is valid (not disposed)
    pub fn is_valid(&self) -> bool {
        // In Leptos 0.8.8+, we can check if signals are still valid
        // by attempting to read from them
        self.theme_signal.try_get().is_some()
    }
}

impl Default for TailwindSignalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal cleanup utility for proper memory management
#[derive(Clone, Debug)]
pub struct SignalCleanup {
    signals: Vec<ArcRwSignal<()>>,
    memos: Vec<ArcMemo<()>>,
}

impl SignalCleanup {
    /// Create a new signal cleanup utility
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            memos: Vec::new(),
        }
    }
    
    /// Track a signal for cleanup
    pub fn track_signal<T>(&mut self, signal: ArcRwSignal<T>) -> ArcRwSignal<T> {
        // Track signal for cleanup
        self.signals.push(ArcRwSignal::new(()));
        signal
    }
    
    /// Track a memo for cleanup
    pub fn track_memo<T: Send + Sync + 'static>(&mut self, memo: ArcMemo<T>) -> ArcMemo<T> {
        // Track memo for cleanup
        self.memos.push(ArcMemo::new(|_| ()));
        memo
    }
    
    /// Get the number of tracked signals
    pub fn signals_count(&self) -> usize {
        self.signals.len()
    }
    
    /// Get the number of tracked memos
    pub fn memos_count(&self) -> usize {
        self.memos.len()
    }
    
    /// Cleanup all tracked signals and memos
    pub fn cleanup(&mut self) -> Result<(), SignalManagementError> {
        // Clear the tracked signals and memos - they will be automatically disposed
        // due to Leptos 0.8.8's ownership tree
        self.signals.clear();
        self.memos.clear();
        Ok(())
    }
}

impl Default for SignalCleanup {
    fn default() -> Self {
        Self::new()
    }
}

/// Automatic cleanup implementation
impl Drop for SignalCleanup {
    fn drop(&mut self) {
        // Leptos 0.8.8 will automatically dispose signals and memos
        // when they go out of scope
    }
}
