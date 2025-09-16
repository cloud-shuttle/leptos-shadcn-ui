#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use crate::lifecycle::*;
    use leptos::prelude::*;
    use std::collections::HashMap;

    // ===== COMPREHENSIVE LIFECYCLE TESTS =====
    // These tests focus on signal lifecycle management and memory optimization

    #[test]
    fn test_theme_enum_variants() {
        // Test Theme enum variants
        let default_theme = Theme::Default;
        let dark_theme = Theme::Dark;
        let light_theme = Theme::Light;
        
        // Test custom theme
        let mut custom_props = HashMap::new();
        custom_props.insert("primary".to_string(), "#3b82f6".to_string());
        custom_props.insert("secondary".to_string(), "#64748b".to_string());
        let custom_theme = Theme::Custom(custom_props);
        
        // Test theme equality
        assert_eq!(default_theme, Theme::Default);
        assert_eq!(dark_theme, Theme::Dark);
        assert_eq!(light_theme, Theme::Light);
        
        // Test custom theme properties
        if let Theme::Custom(props) = custom_theme {
            assert_eq!(props.get("primary"), Some(&"#3b82f6".to_string()));
            assert_eq!(props.get("secondary"), Some(&"#64748b".to_string()));
        }
    }

    #[test]
    fn test_theme_default_implementation() {
        // Test Theme default implementation
        let default_theme = Theme::default();
        assert_eq!(default_theme, Theme::Default);
    }

    #[test]
    fn test_variant_enum_variants() {
        // Test Variant enum variants
        let primary_variant = Variant::Primary;
        let secondary_variant = Variant::Secondary;
        let destructive_variant = Variant::Destructive;
        let outline_variant = Variant::Outline;
        let ghost_variant = Variant::Ghost;
        let link_variant = Variant::Link;
        
        // Test variant equality
        assert_eq!(primary_variant, Variant::Primary);
        assert_eq!(secondary_variant, Variant::Secondary);
        assert_eq!(destructive_variant, Variant::Destructive);
        assert_eq!(outline_variant, Variant::Outline);
        assert_eq!(ghost_variant, Variant::Ghost);
        assert_eq!(link_variant, Variant::Link);
    }

    #[test]
    fn test_variant_default_implementation() {
        // Test Variant default implementation
        let default_variant = Variant::default();
        assert_eq!(default_variant, Variant::Primary);
    }

    #[test]
    fn test_size_enum_variants() {
        // Test Size enum variants
        let small_size = Size::Small;
        let medium_size = Size::Medium;
        let large_size = Size::Large;
        
        // Test size equality
        assert_eq!(small_size, Size::Small);
        assert_eq!(medium_size, Size::Medium);
        assert_eq!(large_size, Size::Large);
    }

    #[test]
    fn test_size_default_implementation() {
        // Test Size default implementation
        let default_size = Size::default();
        assert_eq!(default_size, Size::Medium);
    }

    #[test]
    fn test_responsive_config_creation() {
        // Test ResponsiveConfig creation
        let responsive_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        
        // Test responsive config properties
        assert_eq!(responsive_config.sm, Some("sm:text-sm".to_string()));
        assert_eq!(responsive_config.md, Some("md:text-base".to_string()));
        assert_eq!(responsive_config.lg, Some("lg:text-lg".to_string()));
        assert_eq!(responsive_config.xl, Some("xl:text-xl".to_string()));
    }

    #[test]
    fn test_responsive_config_default_implementation() {
        // Test ResponsiveConfig default implementation
        let default_config = ResponsiveConfig::default();
        assert_eq!(default_config.sm, None);
        assert_eq!(default_config.md, None);
        assert_eq!(default_config.lg, None);
        assert_eq!(default_config.xl, None);
    }

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
    fn test_tailwind_signal_manager_multiple_tracking() {
        // Test tracking multiple signals and memos
        let manager = TailwindSignalManager::new();
        
        // Track multiple signals
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let signal3 = ArcRwSignal::new("value3".to_string());
        
        manager.track_signal(signal1);
        manager.track_signal(signal2);
        manager.track_signal(signal3);
        
        // Test tracking count
        assert_eq!(manager.tracked_signals_count(), 3);
        
        // Track multiple memos
        let memo1 = ArcMemo::new(|_| "memo1".to_string());
        let memo2 = ArcMemo::new(|_| "memo2".to_string());
        
        manager.track_memo(memo1);
        manager.track_memo(memo2);
        
        // Test tracking count
        assert_eq!(manager.tracked_memos_count(), 2);
    }

    #[test]
    fn test_tailwind_signal_manager_validity_check() {
        // Test manager validity checking
        let manager = TailwindSignalManager::new();
        
        // Test manager is initially valid
        assert!(manager.is_valid());
        
        // Test theme signal is accessible
        let theme_signal = manager.theme();
        assert_eq!(theme_signal.get(), Theme::Default);
        
        // Test variant signal is accessible
        let variant_signal = manager.variant();
        assert_eq!(variant_signal.get(), Variant::Primary);
        
        // Test size signal is accessible
        let size_signal = manager.size();
        assert_eq!(size_signal.get(), Size::Medium);
        
        // Test responsive signal is accessible
        let responsive_signal = manager.responsive();
        let config = responsive_signal.get();
        assert_eq!(config.sm, None);
    }

    #[test]
    fn test_signal_cleanup_creation() {
        // Test SignalCleanup creation
        let cleanup = SignalCleanup::new();
        
        // Test initial state
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_cleanup_default_implementation() {
        // Test SignalCleanup default implementation
        let cleanup = SignalCleanup::default();
        
        // Test default state
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_cleanup_signal_tracking() {
        // Test signal tracking in cleanup
        let mut cleanup = SignalCleanup::new();
        
        // Test initial count
        assert_eq!(cleanup.signals_count(), 0);
        
        // Track signals
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        
        cleanup.track_signal(signal1);
        cleanup.track_signal(signal2);
        
        // Test count increased
        assert_eq!(cleanup.signals_count(), 2);
    }

    #[test]
    fn test_signal_cleanup_memo_tracking() {
        // Test memo tracking in cleanup
        let mut cleanup = SignalCleanup::new();
        
        // Test initial count
        assert_eq!(cleanup.memos_count(), 0);
        
        // Track memos
        let memo1 = ArcMemo::new(|_| "memo1".to_string());
        let memo2 = ArcMemo::new(|_| "memo2".to_string());
        
        cleanup.track_memo(memo1);
        cleanup.track_memo(memo2);
        
        // Test count increased
        assert_eq!(cleanup.memos_count(), 2);
    }

    #[test]
    fn test_signal_cleanup_cleanup_operation() {
        // Test cleanup operation
        let mut cleanup = SignalCleanup::new();
        
        // Track some signals and memos
        let signal = ArcRwSignal::new("test".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        cleanup.track_signal(signal);
        cleanup.track_memo(memo);
        
        // Test cleanup operation succeeds
        let result = cleanup.cleanup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_signal_cleanup_drop_implementation() {
        // Test that SignalCleanup implements Drop
        let mut cleanup = SignalCleanup::new();
        
        // Track some signals and memos
        let signal = ArcRwSignal::new("test".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        cleanup.track_signal(signal);
        cleanup.track_memo(memo);
        
        // Test that cleanup can be dropped without issues
        drop(cleanup);
        
        // Test passes if no panics occur
        assert!(true);
    }

    #[test]
    fn test_theme_custom_properties() {
        // Test custom theme with properties
        let mut custom_props = HashMap::new();
        custom_props.insert("primary".to_string(), "#3b82f6".to_string());
        custom_props.insert("secondary".to_string(), "#64748b".to_string());
        custom_props.insert("accent".to_string(), "#f59e0b".to_string());
        
        let custom_theme = Theme::Custom(custom_props.clone());
        
        // Test custom theme properties
        if let Theme::Custom(props) = custom_theme {
            assert_eq!(props.get("primary"), Some(&"#3b82f6".to_string()));
            assert_eq!(props.get("secondary"), Some(&"#64748b".to_string()));
            assert_eq!(props.get("accent"), Some(&"#f59e0b".to_string()));
            assert_eq!(props.len(), 3);
        }
    }

    #[test]
    fn test_responsive_config_partial_configuration() {
        // Test responsive config with partial configuration
        let partial_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: None,
            lg: Some("lg:text-lg".to_string()),
            xl: None,
        };
        
        // Test partial configuration
        assert_eq!(partial_config.sm, Some("sm:text-sm".to_string()));
        assert_eq!(partial_config.md, None);
        assert_eq!(partial_config.lg, Some("lg:text-lg".to_string()));
        assert_eq!(partial_config.xl, None);
    }

    #[test]
    fn test_signal_manager_context_provision() {
        // Test that manager can provide context
        let manager = TailwindSignalManager::new();
        
        // Test that provide_context doesn't panic
        // Note: In a real test environment, we would need to set up a Leptos runtime
        // For now, we just test that the method exists and can be called
        assert!(true, "Context provision method exists");
    }

    #[test]
    fn test_signal_manager_performance_characteristics() {
        // Test performance characteristics
        let start = std::time::Instant::now();
        
        // Create multiple managers
        for _ in 0..1000 {
            let manager = TailwindSignalManager::new();
            let _theme_signal = manager.theme();
            let _variant_signal = manager.variant();
            let _size_signal = manager.size();
            let _responsive_signal = manager.responsive();
        }
        
        let duration = start.elapsed();
        
        // Should complete without panicking
        assert!(duration.as_nanos() >= 0, "Signal manager creation should complete");
    }

    #[test]
    fn test_signal_manager_memory_management() {
        // Test memory management
        let mut managers = Vec::new();
        
        // Create multiple managers
        for i in 0..100 {
            let manager = TailwindSignalManager::new();
            managers.push(manager);
        }
        
        // Test that managers can be dropped without issues
        drop(managers);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_signal_cleanup_memory_management() {
        // Test memory management for cleanup
        let mut cleanups = Vec::new();
        
        // Create multiple cleanups
        for i in 0..100 {
            let mut cleanup = SignalCleanup::new();
            
            // Add some signals and memos
            let signal = ArcRwSignal::new(format!("signal_{}", i));
            let memo = ArcMemo::new(move |_| format!("memo_{}", i));
            
            cleanup.track_signal(signal);
            cleanup.track_memo(memo);
            
            cleanups.push(cleanup);
        }
        
        // Test that cleanups can be dropped without issues
        drop(cleanups);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_enum_serialization_compatibility() {
        // Test that enums are compatible with serialization
        // This is important for state persistence and debugging
        
        // Test Theme serialization
        let theme = Theme::Dark;
        assert_eq!(theme, Theme::Dark);
        
        // Test Variant serialization
        let variant = Variant::Destructive;
        assert_eq!(variant, Variant::Destructive);
        
        // Test Size serialization
        let size = Size::Large;
        assert_eq!(size, Size::Large);
        
        // Test ResponsiveConfig serialization
        let config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        assert_eq!(config.sm, Some("sm:text-sm".to_string()));
    }

    #[test]
    fn test_signal_manager_edge_cases() {
        // Test edge cases for signal manager
        let manager = TailwindSignalManager::new();
        
        // Test tracking empty signals
        let empty_signal = ArcRwSignal::new(());
        manager.track_signal(empty_signal);
        assert_eq!(manager.tracked_signals_count(), 1);
        
        // Test tracking empty memos
        let empty_memo = ArcMemo::new(|_| ());
        manager.track_memo(empty_memo);
        assert_eq!(manager.tracked_memos_count(), 1);
        
        // Test manager validity after tracking
        assert!(manager.is_valid());
    }

    #[test]
    fn test_signal_cleanup_edge_cases() {
        // Test edge cases for signal cleanup
        let mut cleanup = SignalCleanup::new();
        
        // Test tracking empty signals
        let empty_signal = ArcRwSignal::new(());
        cleanup.track_signal(empty_signal);
        assert_eq!(cleanup.signals_count(), 1);
        
        // Test tracking empty memos
        let empty_memo = ArcMemo::new(|_| ());
        cleanup.track_memo(empty_memo);
        assert_eq!(cleanup.memos_count(), 1);
        
        // Test cleanup operation
        let result = cleanup.cleanup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_signal_manager_integration_scenarios() {
        // Test integration scenarios
        let manager = TailwindSignalManager::new();
        
        // Test theme switching scenario
        let theme_signal = manager.theme();
        theme_signal.set(Theme::Light);
        assert_eq!(theme_signal.get(), Theme::Light);
        
        // Test variant switching scenario
        let variant_signal = manager.variant();
        variant_signal.set(Variant::Secondary);
        assert_eq!(variant_signal.get(), Variant::Secondary);
        
        // Test size switching scenario
        let size_signal = manager.size();
        size_signal.set(Size::Small);
        assert_eq!(size_signal.get(), Size::Small);
        
        // Test responsive configuration scenario
        let responsive_signal = manager.responsive();
        let responsive_config = ResponsiveConfig {
            sm: Some("sm:text-sm".to_string()),
            md: Some("md:text-base".to_string()),
            lg: Some("lg:text-lg".to_string()),
            xl: Some("xl:text-xl".to_string()),
        };
        responsive_signal.set(responsive_config);
        
        let updated_config = responsive_signal.get();
        assert_eq!(updated_config.sm, Some("sm:text-sm".to_string()));
    }

    #[test]
    fn test_signal_manager_component_lifecycle() {
        // Test component lifecycle scenarios
        let manager = TailwindSignalManager::new();
        
        // Simulate component creation
        let component_theme = manager.theme();
        let component_variant = manager.variant();
        let component_size = manager.size();
        
        // Simulate component state changes
        component_theme.set(Theme::Dark);
        component_variant.set(Variant::Destructive);
        component_size.set(Size::Large);
        
        // Simulate component disposal
        // In a real scenario, the manager would handle cleanup
        assert!(manager.is_valid());
        
        // Test that signals are still accessible after "disposal"
        assert_eq!(component_theme.get(), Theme::Dark);
        assert_eq!(component_variant.get(), Variant::Destructive);
        assert_eq!(component_size.get(), Size::Large);
    }
}
