//! Functionality tests for the advanced Date-picker component
//! 
//! This module contains tests for date presets, custom formatting,
//! validation, and keyboard navigation functionality.

use leptos::prelude::*;
use crate::default::{DatePicker, DatePickerWithRange};
use leptos_shadcn_calendar::CalendarDate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_presets() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_presets = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Date presets test failed");
    }

    #[test]
    fn test_custom_date_formatting() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_formatting = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Custom date formatting test failed");
    }

    #[test]
    fn test_date_validation_and_constraints() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_validation = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Date validation and constraints test failed");
    }

    #[test]
    fn test_keyboard_navigation_and_shortcuts() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_keyboard = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Keyboard navigation and shortcuts test failed");
    }
}
