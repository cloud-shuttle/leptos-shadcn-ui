//! # Leptos ShadCN UI
//! 
//! A comprehensive collection of beautiful, accessible UI components built for [Leptos](https://leptos.dev/) v0.8+, 
//! inspired by [shadcn/ui](https://ui.shadcn.com/).
//! 
//! ## Features
//! 
//! - **25+ Components**: Button, Input, Card, Alert, and many more
//! - **Leptos 0.8+**: Built specifically for Leptos v0.8+ compatibility
//! - **Accessibility First**: All components follow accessibility best practices
//! - **Tailwind CSS**: Seamless integration with Tailwind CSS
//! - **Type Safety**: Full Rust type safety with proper error handling
//! 
//! ## Usage
//! 
//! See the [README.md](../README.md) for complete installation and usage instructions.
//! 
//! **Note**: Make sure to enable the features for the components you want to use:
//! 
//! ```toml
//! [dependencies]
//! leptos-shadcn-ui = { path = "path/to/leptos-shadcn-ui/packages/leptos-shadcn-ui", features = ["button", "input", "card"] }
//! ```
//! 
//! ## Components
//! 
//! ### Form Components
//! - Button, Input, Label, Checkbox, Switch, Radio Group, Select, Textarea
//! 
//! ### Layout Components  
//! - Card, Separator, Tabs, Accordion, Dialog, Popover, Tooltip
//! 
//! ### Feedback & Status
//! - Alert, Badge, Skeleton, Progress, Toast, Table, Calendar, Date Picker, Pagination
//! 
//! ### Interactive Components
//! - Slider, Toggle
//! 
//! ### Performance Monitoring
//! - Performance Audit System - Comprehensive performance monitoring and optimization
//! - Bundle Size Analysis - Component size tracking and optimization recommendations
//! - Real-time Monitoring - Performance metrics collection and analysis
//! - CLI Tool - Command-line interface for running audits and generating reports
//! 
//! ## License
//! 
//! MIT License - see the [LICENSE](../LICENSE) file for details.

// Re-export all components (conditionally based on features)
#[cfg(feature = "button")]
pub use leptos_shadcn_button::default::*;
#[cfg(feature = "input")]
pub use leptos_shadcn_input::default::*;
#[cfg(feature = "label")]
pub use leptos_shadcn_label::default::*;
#[cfg(feature = "checkbox")]
pub use leptos_shadcn_checkbox::default::*;
#[cfg(feature = "switch")]
pub use leptos_shadcn_switch::default::*;
#[cfg(feature = "radio-group")]
pub use leptos_shadcn_radio_group::default::*;
#[cfg(feature = "select")]
pub use leptos_shadcn_select::default::*;
#[cfg(feature = "textarea")]
pub use leptos_shadcn_textarea::default::*;
#[cfg(feature = "card")]
pub use leptos_shadcn_card::default::*;
#[cfg(feature = "separator")]
pub use leptos_shadcn_separator::default::*;
#[cfg(feature = "tabs")]
pub use leptos_shadcn_tabs::default::*;
#[cfg(feature = "accordion")]
pub use leptos_shadcn_accordion::default::*;
#[cfg(feature = "dialog")]
pub use leptos_shadcn_dialog::default::*;
#[cfg(feature = "popover")]
pub use leptos_shadcn_popover::default::*;
#[cfg(feature = "tooltip")]
pub use leptos_shadcn_tooltip::default::*;
#[cfg(feature = "alert")]
pub use leptos_shadcn_alert::default::*;
#[cfg(feature = "badge")]
pub use leptos_shadcn_badge::default::*;
#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::default::*;
#[cfg(feature = "progress")]
pub use leptos_shadcn_progress::default::*;
#[cfg(feature = "toast")]
pub use leptos_shadcn_toast::default::*;
#[cfg(feature = "table")]
pub use leptos_shadcn_table::default::*;
#[cfg(feature = "calendar")]
pub use leptos_shadcn_calendar::*;
#[cfg(feature = "date-picker")]
pub use leptos_shadcn_date_picker::*;
#[cfg(feature = "pagination")]
pub use leptos_shadcn_pagination::*;
#[cfg(feature = "slider")]
pub use leptos_shadcn_slider::default::*;
#[cfg(feature = "toggle")]
pub use leptos_shadcn_toggle::default::*;

// Advanced components (newly fixed)
#[cfg(feature = "form")]
pub use leptos_shadcn_form::default::*;
#[cfg(feature = "combobox")]
pub use leptos_shadcn_combobox::default::*;
#[cfg(feature = "command")]
pub use leptos_shadcn_command::*;
#[cfg(feature = "input-otp")]
pub use leptos_shadcn_input_otp::*;
#[cfg(feature = "breadcrumb")]
pub use leptos_shadcn_breadcrumb::*;
#[cfg(feature = "lazy-loading")]
pub use leptos_shadcn_lazy_loading::*;
#[cfg(feature = "error-boundary")]
pub use leptos_shadcn_error_boundary::*;
#[cfg(feature = "registry")]
pub use leptos_shadcn_registry::*;

// Re-export common types and utilities
pub use tailwind_fuse::tw_merge;

// Module documentation
#[cfg(feature = "all-components")]
pub mod prelude {
    //! # Leptos ShadCN UI Prelude
    //! 
    //! This module re-exports the most commonly used components and types.
    //! 
    //! ```rust
    //! use leptos_shadcn_ui::prelude::*;
    //! ```
    
    // Form components
    #[cfg(feature = "button")]
    pub use super::{Button, ButtonVariant, ButtonSize};
    #[cfg(feature = "input")]
    pub use super::{Input, InputProps};
    #[cfg(feature = "label")]
    pub use super::{Label, LabelProps};
    #[cfg(feature = "checkbox")]
    pub use super::{Checkbox, CheckboxProps};
    #[cfg(feature = "switch")]
    pub use super::{Switch, SwitchProps};
    #[cfg(feature = "radio-group")]
    pub use super::{RadioGroup, RadioGroupProps};
    #[cfg(feature = "select")]
    pub use super::{Select, SelectProps};
    #[cfg(feature = "textarea")]
    pub use super::{Textarea, TextareaProps};
    
    // Layout components
    #[cfg(feature = "card")]
    pub use super::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
    #[cfg(feature = "separator")]
    pub use super::{Separator, SeparatorProps};
    #[cfg(feature = "tabs")]
    pub use super::{Tabs, TabsList, TabsTrigger, TabsContent};
    #[cfg(feature = "accordion")]
    pub use super::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
    #[cfg(feature = "dialog")]
    pub use super::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter};
    #[cfg(feature = "popover")]
    pub use super::Popover;
    #[cfg(feature = "tooltip")]
    pub use super::{Tooltip, TooltipContent, TooltipTrigger, TooltipProvider};
    
    // Feedback components
    #[cfg(feature = "alert")]
    pub use super::{Alert, AlertTitle, AlertDescription, AlertVariant};
    #[cfg(feature = "badge")]
    pub use super::{Badge, BadgeProps, BadgeVariant};
    #[cfg(feature = "skeleton")]
    pub use super::{Skeleton, SkeletonProps};
    #[cfg(feature = "progress")]
    pub use super::{Progress, ProgressProps};
    #[cfg(feature = "toast")]
    pub use super::{Toast, ToastProps};
    #[cfg(feature = "table")]
    pub use super::Table;
    #[cfg(feature = "calendar")]
    pub use super::{Calendar, CalendarDate};
    #[cfg(feature = "date-picker")]
    pub use super::DatePicker;
    #[cfg(feature = "pagination")]
    pub use super::Pagination;
    
    // Interactive components
    #[cfg(feature = "slider")]
    pub use super::{Slider, SliderProps};
    #[cfg(feature = "toggle")]
    pub use super::{Toggle, ToggleProps};
    
    // Utilities
    pub use super::tw_merge;
}
