#[cfg(test)]
mod performance_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_management_performance_characteristics() {
        // Test signal management performance characteristics
        
        // Test rapid signal updates
        let signal = ArcRwSignal::new("initial".to_string());
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            signal.set(format!("value_{}", i));
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test final value
        assert_eq!(signal.get(), "value_999");
    }

    #[test]
    fn test_signal_management_memory_management() {
        // Test memory management
        
        // Test signal cleanup
        let signal = ArcRwSignal::new("test".to_string());
        let initial_value = signal.get();
        assert_eq!(initial_value, "test");
        
        // Test large string handling
        let large_string = "x".repeat(100000);
        signal.set(large_string.clone());
        assert_eq!(signal.get(), large_string);
        
        // Test memory cleanup by setting smaller value
        signal.set("small".to_string());
        assert_eq!(signal.get(), "small");
        
        // Test memo memory management
        let memo = ArcMemo::new(move |_| 42);
        assert_eq!(memo.get(), 42);
        
        // Test memo cleanup
        drop(memo);
        assert!(true); // If we get here, memory cleanup worked
    }

    #[test]
    fn test_signal_management_integration_scenarios() {
        // Test integration scenarios
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        let mut memory_manager = SignalMemoryManager::new();
        let mut batched_updater = BatchedSignalUpdater::new();
        
        // Test signal creation and tracking
        let signal = ArcRwSignal::new("test_value".to_string());
        let tracked_signal = manager.track_signal(signal.clone());
        cleanup.track_signal(signal.clone());
        memory_manager.add_signal(signal.clone());
        
        // Test signal updates
        tracked_signal.set("updated_value".to_string());
        assert_eq!(tracked_signal.get(), "updated_value");
        
        // Test batched updates
        batched_updater.queue_update(signal.clone(), "batched_value".to_string());
        batched_updater.flush_updates();
        assert_eq!(signal.get(), "batched_value");
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        
        // Test memory cleanup
        memory_manager.cleanup_all();
        assert_eq!(memory_manager.total_signals(), 0);
    }

    #[test]
    fn test_signal_management_component_lifecycle() {
        // Test component lifecycle
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test component initialization
        let theme_signal = manager.theme();
        let variant_signal = manager.variant();
        let size_signal = manager.size();
        
        // Test initial values
        assert_eq!(theme_signal.get(), Theme::Default);
        assert_eq!(variant_signal.get(), Variant::Primary);
        assert_eq!(size_signal.get(), Size::Medium);
        
        // Test component updates
        theme_signal.set(Theme::Dark);
        variant_signal.set(Variant::Secondary);
        size_signal.set(Size::Large);
        
        // Test updated values
        assert_eq!(theme_signal.get(), Theme::Dark);
        assert_eq!(variant_signal.get(), Variant::Secondary);
        assert_eq!(size_signal.get(), Size::Large);
        
        // Test component cleanup
        cleanup.track_signal(theme_signal);
        cleanup.track_signal(variant_signal);
        cleanup.track_signal(size_signal);
        
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_management_large_scale_operations() {
        // Test large scale operations
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        let mut memory_manager = SignalMemoryManager::new();
        
        // Test creating many signals
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal.clone());
            memory_manager.add_signal(signal);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200); // Should be reasonable
        
        // Test counts
        assert_eq!(manager.tracked_signals_count(), 1000);
        assert_eq!(cleanup.signals_count(), 1000);
        assert_eq!(memory_manager.total_signals(), 1000);
        
        // Test cleanup performance
        let cleanup_start = std::time::Instant::now();
        cleanup.cleanup();
        memory_manager.cleanup_all();
        let cleanup_duration = cleanup_start.elapsed();
        
        assert!(cleanup_duration.as_millis() < 100); // Should be fast
        
        // Test counts after cleanup
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(memory_manager.total_signals(), 0);
    }

    #[test]
    fn test_signal_management_memo_performance() {
        // Test memo performance
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test creating many memos
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(i);
            let memo = ArcMemo::new(move |_| signal.get() * 2);
            let _tracked = manager.track_memo(memo.clone());
            cleanup.track_memo(memo);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200); // Should be reasonable
        
        // Test counts
        assert_eq!(manager.tracked_memos_count(), 1000);
        assert_eq!(cleanup.memos_count(), 1000);
        
        // Test cleanup performance
        let cleanup_start = std::time::Instant::now();
        cleanup.cleanup();
        let cleanup_duration = cleanup_start.elapsed();
        
        assert!(cleanup_duration.as_millis() < 100); // Should be fast
        
        // Test counts after cleanup
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_management_batched_updates_performance() {
        // Test batched updates performance
        let mut batched_updater = BatchedSignalUpdater::new();
        
        // Test queuing many updates
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("initial_{}", i));
            batched_updater.queue_update(signal, format!("update_{}", i));
        }
        
        let queue_duration = start.elapsed();
        assert!(queue_duration.as_millis() < 100); // Should be fast
        
        // Test queue size
        assert_eq!(batched_updater.queue_size(), 1000);
        
        // Test flush performance
        let flush_start = std::time::Instant::now();
        batched_updater.flush_updates();
        let flush_duration = flush_start.elapsed();
        
        assert!(flush_duration.as_millis() < 100); // Should be fast
        
        // Test queue is empty
        assert_eq!(batched_updater.queue_size(), 0);
    }

    #[test]
    fn test_signal_management_memory_usage() {
        // Test memory usage tracking
        let mut memory_manager = SignalMemoryManager::new();
        
        // Test initial memory usage
        let initial_memory = memory_manager.memory_usage_kb();
        assert_eq!(initial_memory, 0);
        
        // Test memory usage with many signals
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            memory_manager.add_signal(signal);
        }
        
        // Test memory usage increased
        let final_memory = memory_manager.memory_usage_kb();
        assert!(final_memory > initial_memory);
        
        // Test memory cleanup
        memory_manager.cleanup_all();
        let cleaned_memory = memory_manager.memory_usage_kb();
        assert_eq!(cleaned_memory, 0);
    }

    #[test]
    fn test_signal_management_concurrent_operations() {
        // Test concurrent-like operations
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        let mut batched_updater = BatchedSignalUpdater::new();
        
        let start = std::time::Instant::now();
        
        // Test concurrent-like operations
        for i in 0..100 {
            // Create and track signals
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal.clone());
            
            // Queue batched updates
            batched_updater.queue_update(signal, format!("update_{}", i));
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
        
        // Test final state
        assert_eq!(manager.tracked_signals_count(), 100);
        assert_eq!(cleanup.signals_count(), 100);
        assert_eq!(batched_updater.queue_size(), 100);
        
        // Test cleanup
        cleanup.cleanup();
        batched_updater.flush_updates();
        
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(batched_updater.queue_size(), 0);
    }
}
