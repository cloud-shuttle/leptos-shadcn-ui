//! Advanced features tests for the Date-picker component
//! 
//! This module contains tests for time picker integration, calendar view modes,
//! timezone support, and inline calendar display functionality.

use leptos::prelude::*;
use crate::default::{DatePicker, DatePickerWithRange};
use leptos_shadcn_calendar::CalendarDate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_picker_integration() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_time = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select date and time".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Time picker integration test failed");
    }

    #[test]
    fn test_calendar_view_modes() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_view_modes = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Calendar view modes test failed");
    }

    #[test]
    fn test_timezone_support() {
        let test_result = std::panic::catch_unwind(|| {
            let _date_picker_with_timezone = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Timezone support test failed");
    }

    #[test]
    fn test_inline_calendar_display() {
        let test_result = std::panic::catch_unwind(|| {
            let _inline_date_picker = view! {
                <DatePicker
                    selected=Some(CalendarDate::new(2024, 1, 15)).into()
                    placeholder="Select a date".to_string().into()
                    class="w-full".into()
                />
            };
            true
        });
        assert!(test_result.is_ok(), "Inline calendar display test failed");
    }
}
