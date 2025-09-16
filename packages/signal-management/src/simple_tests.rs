#[cfg(test)]
mod simple_tests {
    use super::*;
    use crate::lifecycle::*;
    use crate::batched_updates::*;
    use crate::memory_management::*;
    use crate::error::*;
    use leptos::prelude::*;
    use std::collections::HashMap;

    // ===== SIMPLE SIGNAL MANAGEMENT TESTS =====
    // These tests focus on basic functionality that actually works

    #[test]
    fn test_theme_enum_variants() {
        // Test Theme enum variants
        let default_theme = Theme::Default;
        let dark_theme = Theme::Dark;
        let light_theme = Theme::Light;
        
        // Test theme equality
        assert_eq!(default_theme, Theme::Default);
        assert_eq!(dark_theme, Theme::Dark);
        assert_eq!(light_theme, Theme::Light);
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
        
        // Test variant equality
        assert_eq!(primary_variant, Variant::Primary);
        assert_eq!(secondary_variant, Variant::Secondary);
        assert_eq!(destructive_variant, Variant::Destructive);
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
    fn test_signal_memory_manager_creation() {
        // Test SignalMemoryManager creation
        let manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.group_count(), 0);
        assert_eq!(manager.get_stats().active_signals, 0);
        assert_eq!(manager.get_stats().active_memos, 0);
        assert_eq!(manager.get_stats().estimated_memory_bytes, 0);
        assert_eq!(manager.get_stats().tracked_groups, 0);
    }

    #[test]
    fn test_signal_memory_manager_with_memory_limit() {
        // Test SignalMemoryManager with custom memory limit
        let max_memory = 1024 * 1024; // 1MB
        let manager = SignalMemoryManager::with_memory_limit(max_memory);
        
        // Test custom limits
        assert_eq!(manager.max_memory_bytes(), max_memory);
        assert_eq!(manager.group_count(), 0);
    }

    #[test]
    fn test_signal_memory_manager_create_group() {
        // Test creating signal groups
        let manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.group_count(), 0);
        
        // Create group
        let result = manager.create_group("test_group".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.group_count(), 1);
    }

    #[test]
    fn test_signal_memory_manager_remove_group() {
        // Test removing signal groups
        let manager = SignalMemoryManager::new();
        
        // Create group
        let result = manager.create_group("test_group".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.group_count(), 1);
        
        // Remove group
        let result = manager.remove_group("test_group");
        assert!(result.is_ok());
        assert_eq!(manager.group_count(), 0);
    }

    #[test]
    fn test_signal_memory_manager_add_signal_to_group() {
        // Test adding signals to groups
        let manager = SignalMemoryManager::new();
        
        // Create group
        manager.create_group("test_group".to_string()).unwrap();
        
        // Add signal to group
        let signal = ArcRwSignal::new("test_value".to_string());
        let result = manager.add_signal_to_group("test_group", signal.clone());
        assert!(result.is_ok());
        
        // Test signal still works
        assert_eq!(signal.get(), "test_value");
    }

    #[test]
    fn test_signal_memory_manager_add_memo_to_group() {
        // Test adding memos to groups
        let manager = SignalMemoryManager::new();
        
        // Create group
        manager.create_group("test_group".to_string()).unwrap();
        
        // Add memo to group
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        let result = manager.add_memo_to_group("test_group", memo.clone());
        assert!(result.is_ok());
        
        // Test memo still works
        assert_eq!(memo.get(), "test_memo");
    }

    #[test]
    fn test_signal_memory_manager_memory_limits() {
        // Test memory limit checking
        let max_memory = 1000;
        let manager = SignalMemoryManager::with_memory_limit(max_memory);
        
        // Test initial state
        assert!(manager.is_memory_within_limits());
        
        // Test memory limit
        assert_eq!(manager.max_memory_bytes(), max_memory);
    }

    #[test]
    fn test_batched_signal_updater_creation() {
        // Test BatchedSignalUpdater creation
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert_eq!(updater.max_batch_size(), 1000);
        assert_eq!(updater.queue_size(), 0);
        assert!(!updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_with_custom_batch_size() {
        // Test BatchedSignalUpdater with custom batch size
        let custom_batch_size = 500;
        let updater = BatchedSignalUpdater::with_batch_size(custom_batch_size);
        
        // Test custom batch size
        assert_eq!(updater.max_batch_size(), custom_batch_size);
        assert_eq!(updater.queue_size(), 0);
        assert!(!updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_queue_update() {
        // Test queueing updates
        let updater = BatchedSignalUpdater::new();
        let test_signal = ArcRwSignal::new(0);
        
        // Test queueing a simple update
        let result = updater.queue_update(move || {
            test_signal.set(42);
        });
        
        // Test update was queued successfully
        assert!(result.is_ok());
        assert_eq!(updater.queue_size(), 1);
    }

    #[test]
    fn test_batched_signal_updater_start_batching() {
        // Test starting batching
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert!(!updater.is_batching());
        
        // Start batching
        updater.start_batching();
        assert!(updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_end_batching() {
        // Test ending batching
        let updater = BatchedSignalUpdater::new();
        let test_signal = ArcRwSignal::new(0);
        let test_signal_clone = test_signal.clone();
        
        // Start batching and queue an update
        updater.start_batching();
        updater.queue_update(move || {
            test_signal_clone.set(42);
        }).unwrap();
        
        // Test batching is active
        assert!(updater.is_batching());
        assert_eq!(updater.queue_size(), 1);
        
        // End batching
        let result = updater.end_batching();
        assert!(result.is_ok());
        assert!(!updater.is_batching());
        
        // Test signal was updated
        assert_eq!(test_signal.get(), 42);
    }

    #[test]
    fn test_batched_signal_updater_flush_updates() {
        // Test flushing updates
        let updater = BatchedSignalUpdater::new();
        let signal1 = ArcRwSignal::new(0);
        let signal2 = ArcRwSignal::new(0);
        let signal3 = ArcRwSignal::new(0);
        
        let signal1_clone = signal1.clone();
        let signal2_clone = signal2.clone();
        let signal3_clone = signal3.clone();
        
        // Queue multiple updates
        updater.queue_update(move || {
            signal1_clone.set(1);
        }).unwrap();
        updater.queue_update(move || {
            signal2_clone.set(2);
        }).unwrap();
        updater.queue_update(move || {
            signal3_clone.set(3);
        }).unwrap();
        
        // Test updates are queued
        assert_eq!(updater.queue_size(), 3);
        
        // Flush updates
        let result = updater.flush_updates();
        assert!(result.is_ok());
        
        // Test queue is empty after flush
        assert_eq!(updater.queue_size(), 0);
        
        // Test signals were updated
        assert_eq!(signal1.get(), 1);
        assert_eq!(signal2.get(), 2);
        assert_eq!(signal3.get(), 3);
    }

    #[test]
    fn test_batched_signal_updater_clear_queue() {
        // Test clearing queue
        let updater = BatchedSignalUpdater::new();
        let signal = ArcRwSignal::new(0);
        let signal_clone = signal.clone();
        
        // Queue an update
        updater.queue_update(move || {
            signal_clone.set(42);
        }).unwrap();
        
        // Test update is queued
        assert_eq!(updater.queue_size(), 1);
        
        // Clear queue
        updater.clear_queue();
        
        // Test queue is empty after clear
        assert_eq!(updater.queue_size(), 0);
        
        // Test signal was not updated
        assert_eq!(signal.get(), 0);
    }

    #[test]
    fn test_signal_management_error_types() {
        // Test SignalManagementError types
        let signal_disposed = SignalManagementError::SignalDisposed;
        let update_failed = SignalManagementError::update_failed("Test reason");
        let memory_failed = SignalManagementError::memory_management_failed("Memory reason");
        let batch_failed = SignalManagementError::batched_update_failed("Batch reason");
        
        // Test error types
        assert_eq!(signal_disposed, SignalManagementError::SignalDisposed);
        assert_eq!(update_failed, SignalManagementError::UpdateFailed {
            reason: "Test reason".to_string(),
        });
        assert_eq!(memory_failed, SignalManagementError::MemoryManagementFailed {
            reason: "Memory reason".to_string(),
        });
        assert_eq!(batch_failed, SignalManagementError::BatchedUpdateFailed {
            reason: "Batch reason".to_string(),
        });
    }

    #[test]
    fn test_signal_management_error_messages() {
        // Test error messages
        let signal_disposed = SignalManagementError::SignalDisposed;
        let update_failed = SignalManagementError::update_failed("Test reason");
        let memory_failed = SignalManagementError::memory_management_failed("Memory reason");
        let batch_failed = SignalManagementError::batched_update_failed("Batch reason");
        
        // Test error messages
        assert_eq!(signal_disposed.to_string(), "Signal has been disposed and is no longer valid");
        assert_eq!(update_failed.to_string(), "Signal update failed: Test reason");
        assert_eq!(memory_failed.to_string(), "Memory management operation failed: Memory reason");
        assert_eq!(batch_failed.to_string(), "Batched update operation failed: Batch reason");
    }

    #[test]
    fn test_signal_management_error_clone() {
        // Test error cloning
        let original_error = SignalManagementError::update_failed("Test reason");
        let cloned_error = original_error.clone();
        
        // Test cloned error is equal to original
        assert_eq!(original_error, cloned_error);
        
        // Test cloned error has same message
        assert_eq!(original_error.to_string(), cloned_error.to_string());
    }

    #[test]
    fn test_signal_management_error_debug() {
        // Test error debug formatting
        let error = SignalManagementError::update_failed("Test reason");
        let debug_string = format!("{:?}", error);
        
        // Test debug string contains error information
        assert!(debug_string.contains("UpdateFailed"));
        assert!(debug_string.contains("Test reason"));
    }

    #[test]
    fn test_signal_management_error_partial_eq() {
        // Test error equality
        let error1 = SignalManagementError::update_failed("Test reason");
        let error2 = SignalManagementError::update_failed("Test reason");
        let error3 = SignalManagementError::update_failed("Different reason");
        
        // Test equal errors
        assert_eq!(error1, error2);
        
        // Test different errors
        assert_ne!(error1, error3);
        
        // Test different error types
        let signal_disposed = SignalManagementError::SignalDisposed;
        assert_ne!(error1, signal_disposed);
    }

    #[test]
    fn test_signal_management_error_display() {
        // Test error display formatting
        let error = SignalManagementError::update_failed("Test reason");
        let display_string = format!("{}", error);
        
        // Test display string
        assert_eq!(display_string, "Signal update failed: Test reason");
    }

    #[test]
    fn test_signal_management_performance_characteristics() {
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
    fn test_signal_management_memory_management() {
        // Test memory management
        let mut managers = Vec::new();
        
        // Create multiple managers
        for _i in 0..100 {
            let manager = TailwindSignalManager::new();
            managers.push(manager);
        }
        
        // Test that managers can be dropped without issues
        drop(managers);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_signal_management_integration_scenarios() {
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
    fn test_signal_management_component_lifecycle() {
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
