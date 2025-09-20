#[cfg(test)]
mod signal_manager_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_tailwind_signal_manager_creation() {
        // Test TailwindSignalManager creation
        let manager = TailwindSignalManager::new();
        
        // Test initial state
        assert_eq!(manager.tracked_signals_count(), 0);
        assert_eq!(manager.tracked_memos_count(), 0);
        assert!(manager.is_valid());
    }

    #[test]
    fn test_tailwind_signal_manager_default_implementation() {
        // Test TailwindSignalManager default implementation
        let manager = TailwindSignalManager::default();
        
        // Test default state
        assert_eq!(manager.tracked_signals_count(), 0);
        assert_eq!(manager.tracked_memos_count(), 0);
        assert!(manager.is_valid());
    }

    #[test]
    fn test_tailwind_signal_manager_theme_signal() {
        // Test theme signal management
        let manager = TailwindSignalManager::new();
        let theme_signal = manager.theme();
        
        // Test initial theme
        assert_eq!(theme_signal.get(), Theme::Default);
        
        // Test theme updates
        theme_signal.set(Theme::Dark);
        assert_eq!(theme_signal.get(), Theme::Dark);
        
        theme_signal.set(Theme::Light);
        assert_eq!(theme_signal.get(), Theme::Light);
    }

    #[test]
    fn test_tailwind_signal_manager_variant_signal() {
        // Test variant signal management
        let manager = TailwindSignalManager::new();
        let variant_signal = manager.variant();
        
        // Test initial variant
        assert_eq!(variant_signal.get(), Variant::Primary);
        
        // Test variant updates
        variant_signal.set(Variant::Secondary);
        assert_eq!(variant_signal.get(), Variant::Secondary);
        
        variant_signal.set(Variant::Destructive);
        assert_eq!(variant_signal.get(), Variant::Destructive);
    }

    #[test]
    fn test_tailwind_signal_manager_size_signal() {
        // Test size signal management
        let manager = TailwindSignalManager::new();
        let size_signal = manager.size();
        
        // Test initial size
        assert_eq!(size_signal.get(), Size::Medium);
        
        // Test size updates
        size_signal.set(Size::Small);
        assert_eq!(size_signal.get(), Size::Small);
        
        size_signal.set(Size::Large);
        assert_eq!(size_signal.get(), Size::Large);
    }

    #[test]
    fn test_tailwind_signal_manager_responsive_signal() {
        // Test responsive signal management
        let manager = TailwindSignalManager::new();
        let responsive_signal = manager.responsive();
        
        // Test initial responsive config
        let initial_config = responsive_signal.get();
        assert_eq!(initial_config.sm, None);
        assert_eq!(initial_config.md, None);
        assert_eq!(initial_config.lg, None);
        assert_eq!(initial_config.xl, None);
        
        // Test responsive config updates
        let new_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        responsive_signal.set(new_config.clone());
        
        let updated_config = responsive_signal.get();
        assert_eq!(updated_config.sm, Some("sm:text-sm".to_string()));
        assert_eq!(updated_config.md, Some("md:text-base".to_string()));
        assert_eq!(updated_config.lg, Some("lg:text-lg".to_string()));
        assert_eq!(updated_config.xl, Some("xl:text-xl".to_string()));
    }

    #[test]
    fn test_tailwind_signal_manager_signal_tracking() {
        // Test signal tracking functionality
        let manager = TailwindSignalManager::new();
        
        // Test initial tracking count
        assert_eq!(manager.tracked_signals_count(), 0);
        
        // Track a signal
        let test_signal = ArcRwSignal::new("test_value".to_string());
        let tracked_signal = manager.track_signal(test_signal.clone());
        
        // Test tracking count increased
        assert_eq!(manager.tracked_signals_count(), 1);
        
        // Test tracked signal still works
        assert_eq!(tracked_signal.get(), "test_value");
        
        // Test signal updates
        tracked_signal.set("updated_value".to_string());
        assert_eq!(tracked_signal.get(), "updated_value");
    }

    #[test]
    fn test_tailwind_signal_manager_memo_tracking() {
        // Test memo tracking functionality
        let manager = TailwindSignalManager::new();
        
        // Test initial tracking count
        assert_eq!(manager.tracked_memos_count(), 0);
        
        // Track a memo
        let test_signal = ArcRwSignal::new(42);
        let test_signal_clone = test_signal.clone();
        let test_memo = ArcMemo::new(move |_| test_signal_clone.get() * 2);
        let tracked_memo = manager.track_memo(test_memo.clone());
        
        // Test tracking count increased
        assert_eq!(manager.tracked_memos_count(), 1);
        
        // Test tracked memo still works
        assert_eq!(tracked_memo.get(), 84);
        
        // Test memo updates when signal changes
        test_signal.set(100);
        assert_eq!(tracked_memo.get(), 200);
    }

    #[test]
    fn test_tailwind_signal_manager_multiple_tracking() {
        // Test multiple signal and memo tracking
        let manager = TailwindSignalManager::new();
        
        // Track multiple signals
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let signal3 = ArcRwSignal::new("value3".to_string());
        
        let tracked1 = manager.track_signal(signal1.clone());
        let tracked2 = manager.track_signal(signal2.clone());
        let tracked3 = manager.track_signal(signal3.clone());
        
        // Test tracking count
        assert_eq!(manager.tracked_signals_count(), 3);
        
        // Test all signals work
        assert_eq!(tracked1.get(), "value1");
        assert_eq!(tracked2.get(), "value2");
        assert_eq!(tracked3.get(), "value3");
        
        // Track multiple memos
        let memo1 = ArcMemo::new(move |_| 10);
        let memo2 = ArcMemo::new(move |_| 20);
        let memo3 = ArcMemo::new(move |_| 30);
        
        let tracked_memo1 = manager.track_memo(memo1.clone());
        let tracked_memo2 = manager.track_memo(memo2.clone());
        let tracked_memo3 = manager.track_memo(memo3.clone());
        
        // Test tracking count
        assert_eq!(manager.tracked_memos_count(), 3);
        
        // Test all memos work
        assert_eq!(tracked_memo1.get(), 10);
        assert_eq!(tracked_memo2.get(), 20);
        assert_eq!(tracked_memo3.get(), 30);
    }

    #[test]
    fn test_tailwind_signal_manager_validity_check() {
        // Test manager validity
        let manager = TailwindSignalManager::new();
        
        // Test initial validity
        assert!(manager.is_valid());
        
        // Test validity after tracking
        let signal = ArcRwSignal::new("test".to_string());
        let _tracked = manager.track_signal(signal);
        
        assert!(manager.is_valid());
        
        // Test validity after multiple tracking
        let memo = ArcMemo::new(move |_| 42);
        let _tracked_memo = manager.track_memo(memo);
        
        assert!(manager.is_valid());
    }

    #[test]
    fn test_signal_manager_context_provision() {
        // Test signal manager context provision
        let manager = TailwindSignalManager::new();
        
        // Test that manager provides context for signals
        let theme_signal = manager.theme();
        let variant_signal = manager.variant();
        let size_signal = manager.size();
        let responsive_signal = manager.responsive();
        
        // Test all signals are valid
        assert!(theme_signal.get() == Theme::Default);
        assert!(variant_signal.get() == Variant::Primary);
        assert!(size_signal.get() == Size::Medium);
        assert!(responsive_signal.get().sm.is_none());
    }

    #[test]
    fn test_signal_manager_edge_cases() {
        // Test signal manager edge cases
        let manager = TailwindSignalManager::new();
        
        // Test with empty string
        let empty_signal = ArcRwSignal::new("".to_string());
        let _tracked = manager.track_signal(empty_signal.clone());
        assert_eq!(empty_signal.get(), "");
        
        // Test with large string
        let large_string = "x".repeat(10000);
        let large_signal = ArcRwSignal::new(large_string.clone());
        let _tracked = manager.track_signal(large_signal.clone());
        assert_eq!(large_signal.get(), large_string);
        
        // Test with zero value
        let zero_signal = ArcRwSignal::new(0);
        let _tracked = manager.track_signal(zero_signal.clone());
        assert_eq!(zero_signal.get(), 0);
    }
}
