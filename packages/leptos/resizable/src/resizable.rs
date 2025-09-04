use leptos::prelude::*;
use std::collections::HashMap;

/// Resize direction for panels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeDirection {
    Horizontal,
    Vertical,
}

impl Default for ResizeDirection {
    fn default() -> Self {
        ResizeDirection::Horizontal
    }
}

/// Resizable state management
#[derive(Debug, Clone)]
pub struct ResizableState {
    pub panel_sizes: Vec<f64>,
    pub is_resizing: bool,
    pub resize_direction: ResizeDirection,
    pub collapsed_panels: Vec<usize>,
}

impl Default for ResizableState {
    fn default() -> Self {
        Self {
            panel_sizes: Vec::new(),
            is_resizing: false,
            resize_direction: ResizeDirection::Horizontal,
            collapsed_panels: Vec::new(),
        }
    }
}

/// Resizable configuration
#[derive(Debug, Clone)]
pub struct ResizableConfig {
    pub default_sizes: Vec<f64>,
    pub min_sizes: Vec<f64>,
    pub max_sizes: Vec<f64>,
    pub collapsible: Vec<bool>,
    pub collapsed_sizes: Vec<f64>,
    pub keyboard_resize: bool,
    pub touch_support: bool,
}

impl Default for ResizableConfig {
    fn default() -> Self {
        Self {
            default_sizes: vec![50.0, 50.0],
            min_sizes: vec![10.0, 10.0],
            max_sizes: vec![90.0, 90.0],
            collapsible: vec![false, false],
            collapsed_sizes: vec![0.0, 0.0],
            keyboard_resize: false,
            touch_support: false,
        }
    }
}

/// Resizable context for managing state across components
#[derive(Clone)]
pub struct ResizableContext {
    pub state: RwSignal<ResizableState>,
    pub config: RwSignal<ResizableConfig>,
    pub update_size: Callback<(usize, f64)>,
    pub toggle_collapse: Callback<usize>,
    pub start_resize: Callback<()>,
    pub end_resize: Callback<()>,
}

impl ResizableContext {
    pub fn new() -> Self {
        let state = RwSignal::new(ResizableState::default());
        let config = RwSignal::new(ResizableConfig::default());

        let update_size = {
            let state = state.clone();
            Callback::new(move |(panel_index, size): (usize, f64)| {
                state.update(|s| {
                    if panel_index < s.panel_sizes.len() {
                        s.panel_sizes[panel_index] = size;
                    }
                });
            })
        };

        let toggle_collapse = {
            let state = state.clone();
            Callback::new(move |panel_index: usize| {
                state.update(|s| {
                    if s.collapsed_panels.contains(&panel_index) {
                        s.collapsed_panels.retain(|&i| i != panel_index);
                    } else {
                        s.collapsed_panels.push(panel_index);
                    }
                });
            })
        };

        let start_resize = {
            let state = state.clone();
            Callback::new(move |_| {
                state.update(|s| s.is_resizing = true);
            })
        };

        let end_resize = {
            let state = state.clone();
            Callback::new(move |_| {
                state.update(|s| s.is_resizing = false);
            })
        };

        Self {
            state,
            config,
            update_size,
            toggle_collapse,
            start_resize,
            end_resize,
        }
    }
}

/// Hook for using resizable context
pub fn use_resizable_context() -> ResizableContext {
    expect_context::<ResizableContext>()
}

/// Utility functions for resizable panels
pub mod utils {
    use super::*;

    /// Calculate new panel sizes when resizing
    pub fn calculate_new_sizes(
        current_sizes: &[f64],
        panel_index: usize,
        new_size: f64,
        min_sizes: &[f64],
        max_sizes: &[f64],
    ) -> Vec<f64> {
        let mut new_sizes = current_sizes.to_vec();
        
        if panel_index >= new_sizes.len() {
            return new_sizes;
        }

        let old_size = new_sizes[panel_index];
        let size_diff = new_size - old_size;
        
        // Find the next panel to adjust
        let next_panel_index = if panel_index + 1 < new_sizes.len() {
            panel_index + 1
        } else if panel_index > 0 {
            panel_index - 1
        } else {
            return new_sizes;
        };

        // Clamp the new size to min/max constraints
        let clamped_size = new_size.clamp(
            min_sizes.get(panel_index).copied().unwrap_or(0.0),
            max_sizes.get(panel_index).copied().unwrap_or(100.0),
        );

        let actual_size_diff = clamped_size - old_size;
        new_sizes[panel_index] = clamped_size;
        new_sizes[next_panel_index] -= actual_size_diff;

        // Clamp the next panel size as well
        new_sizes[next_panel_index] = new_sizes[next_panel_index].clamp(
            min_sizes.get(next_panel_index).copied().unwrap_or(0.0),
            max_sizes.get(next_panel_index).copied().unwrap_or(100.0),
        );

        new_sizes
    }

    /// Check if a panel can be resized
    pub fn can_resize(
        panel_index: usize,
        direction: ResizeDirection,
        is_collapsed: bool,
    ) -> bool {
        !is_collapsed && panel_index > 0
    }

    /// Get the resize handle position
    pub fn get_handle_position(
        panel_index: usize,
        direction: ResizeDirection,
    ) -> String {
        match direction {
            ResizeDirection::Horizontal => "right".to_string(),
            ResizeDirection::Vertical => "bottom".to_string(),
        }
    }

    /// Calculate total size of all panels
    pub fn calculate_total_size(sizes: &[f64]) -> f64 {
        sizes.iter().sum()
    }

    /// Normalize panel sizes to ensure they sum to 100%
    pub fn normalize_sizes(sizes: &mut [f64]) {
        let total: f64 = sizes.iter().sum();
        if total > 0.0 {
            for size in sizes.iter_mut() {
                *size = (*size / total) * 100.0;
            }
        }
    }
}
