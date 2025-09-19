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
        let test_memo = ArcMemo::new(move |_| test_signal.get() * 2);
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
    fn test_tailwind_signal_manager_multiple_signals() {
        // Test multiple signal tracking
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
        
        // Test signal updates
        tracked1.set("updated1".to_string());
        tracked2.set("updated2".to_string());
        tracked3.set("updated3".to_string());
        
        assert_eq!(tracked1.get(), "updated1");
        assert_eq!(tracked2.get(), "updated2");
        assert_eq!(tracked3.get(), "updated3");
    }

    #[test]
    fn test_tailwind_signal_manager_multiple_memos() {
        // Test multiple memo tracking
        let manager = TailwindSignalManager::new();
        
        // Track multiple memos
        let signal1 = ArcRwSignal::new(10);
        let signal2 = ArcRwSignal::new(20);
        
        let memo1 = ArcMemo::new(move |_| signal1.get() * 2);
        let memo2 = ArcMemo::new(move |_| signal2.get() * 3);
        
        let tracked1 = manager.track_memo(memo1.clone());
        let tracked2 = manager.track_memo(memo2.clone());
        
        // Test tracking count
        assert_eq!(manager.tracked_memos_count(), 2);
        
        // Test all memos work
        assert_eq!(tracked1.get(), 20);
        assert_eq!(tracked2.get(), 60);
        
        // Test memo updates when signals change
        signal1.set(15);
        signal2.set(25);
        
        assert_eq!(tracked1.get(), 30);
        assert_eq!(tracked2.get(), 75);
    }

    #[test]
    fn test_tailwind_signal_manager_mixed_tracking() {
        // Test mixed signal and memo tracking
        let manager = TailwindSignalManager::new();
        
        // Track signals and memos
        let signal1 = ArcRwSignal::new("test".to_string());
        let signal2 = ArcRwSignal::new(42);
        
        let memo = ArcMemo::new(move |_| signal2.get() * 2);
        
        let tracked_signal = manager.track_signal(signal1.clone());
        let tracked_memo = manager.track_memo(memo.clone());
        
        // Test tracking counts
        assert_eq!(manager.tracked_signals_count(), 1);
        assert_eq!(manager.tracked_memos_count(), 1);
        
        // Test both work
        assert_eq!(tracked_signal.get(), "test");
        assert_eq!(tracked_memo.get(), 84);
        
        // Test updates
        tracked_signal.set("updated".to_string());
        signal2.set(100);
        
        assert_eq!(tracked_signal.get(), "updated");
        assert_eq!(tracked_memo.get(), 200);
    }

    #[test]
    fn test_tailwind_signal_manager_validity() {
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
    fn test_tailwind_signal_manager_clone_behavior() {
        // Test manager cloning behavior
        let manager1 = TailwindSignalManager::new();
        let signal = ArcRwSignal::new("test".to_string());
        let _tracked = manager1.track_signal(signal);
        
        // Test cloning
        let manager2 = manager1.clone();
        
        // Test both managers have same state
        assert_eq!(manager1.tracked_signals_count(), manager2.tracked_signals_count());
        assert_eq!(manager1.tracked_memos_count(), manager2.tracked_memos_count());
        assert_eq!(manager1.is_valid(), manager2.is_valid());
    }
}
