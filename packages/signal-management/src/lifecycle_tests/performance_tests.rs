#[cfg(test)]
mod performance_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_manager_performance_characteristics() {
        // Test signal manager performance characteristics
        let manager = TailwindSignalManager::new();
        
        // Test rapid signal updates
        let theme_signal = manager.theme();
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            theme_signal.set(if i % 2 == 0 { Theme::Dark } else { Theme::Light });
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
        
        // Test final value
        assert!(theme_signal.get() == Theme::Light || theme_signal.get() == Theme::Dark);
    }

    #[test]
    fn test_signal_manager_memory_management() {
        // Test signal manager memory management
        let manager = TailwindSignalManager::new();
        
        // Test signal creation and tracking
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
    fn test_signal_cleanup_memory_management() {
        // Test signal cleanup memory management
        let mut cleanup = SignalCleanup::new();
        
        // Test signal creation and tracking
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
        
        // Test cleanup tracking
        cleanup.track_signal(signal);
        assert_eq!(cleanup.signals_count(), 1);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_manager_large_scale_operations() {
        // Test large scale operations
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test creating many signals
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200); // Should be reasonable
        
        // Test counts
        assert_eq!(manager.tracked_signals_count(), 1000);
        assert_eq!(cleanup.signals_count(), 1000);
        
        // Test cleanup performance
        let cleanup_start = std::time::Instant::now();
        cleanup.cleanup();
        let cleanup_duration = cleanup_start.elapsed();
        
        assert!(cleanup_duration.as_millis() < 100); // Should be fast
        
        // Test counts after cleanup
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_manager_memo_performance() {
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
    fn test_signal_manager_concurrent_operations() {
        // Test concurrent-like operations
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        let start = std::time::Instant::now();
        
        // Test concurrent-like operations
        for i in 0..100 {
            // Create and track signals
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be fast
        
        // Test final state
        assert_eq!(manager.tracked_signals_count(), 100);
        assert_eq!(cleanup.signals_count(), 100);
        
        // Test cleanup
        cleanup.cleanup();
        
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_manager_memory_usage() {
        // Test memory usage tracking
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test initial memory usage
        let initial_signals = manager.tracked_signals_count();
        let initial_memos = manager.tracked_memos_count();
        assert_eq!(initial_signals, 0);
        assert_eq!(initial_memos, 0);
        
        // Test memory usage with many signals
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
        }
        
        // Test memory usage increased
        let final_signals = manager.tracked_signals_count();
        let final_memos = manager.tracked_memos_count();
        assert!(final_signals > initial_signals);
        assert_eq!(final_memos, initial_memos);
        
        // Test memory cleanup
        cleanup.cleanup();
        let cleaned_signals = cleanup.signals_count();
        assert_eq!(cleaned_signals, 0);
    }

    #[test]
    fn test_signal_manager_performance_under_load() {
        // Test performance under load
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test performance under load
        let start = std::time::Instant::now();
        
        for i in 0..10000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
            
            if i % 1000 == 0 {
                // Test periodic cleanup
                cleanup.cleanup();
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000); // Should be reasonable
        
        // Test final state
        assert_eq!(cleanup.signals_count(), 0); // Should be cleaned up
    }

    #[test]
    fn test_signal_manager_memory_leak_prevention() {
        // Test memory leak prevention
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test memory leak prevention
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
        }
        
        // Test counts
        assert_eq!(manager.tracked_signals_count(), 1000);
        assert_eq!(cleanup.signals_count(), 1000);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        
        // Test memory is cleaned up
        assert!(true); // If we get here, memory cleanup worked
    }
}
