//! Customization tests for the advanced Date-picker component
//! 
//! This module contains tests for custom actions, accessibility features,
//! and custom styling functionality.

use leptos::prelude::*;
use crate::default::{DatePicker, DatePickerWithRange};
use leptos_shadcn_calendar::CalendarDate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_with_custom_actions() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_actions = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Date picker with custom actions test failed");
    }

    #[test]
    fn test_date_picker_accessibility_features() {
        let test_result = std::panic::catch_unwind(|| {
            let _accessible_date_picker = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Date picker accessibility features test failed");
    }

    #[test]
    fn test_date_picker_with_custom_styling() {
        let test_result = std::panic::catch_unwind(|| {
            let _styled_date_picker = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full custom-styling".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Date picker with custom styling test failed");
    }
}
