#[cfg(test)]
mod integration_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_manager_integration_scenarios() {
        // Test signal manager integration scenarios
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test signal creation and tracking
        let signal = ArcRwSignal::new("test_value".to_string());
        let tracked_signal = manager.track_signal(signal.clone());
        cleanup.track_signal(signal.clone());
        
        // Test signal updates
        tracked_signal.set("updated_value".to_string());
        assert_eq!(tracked_signal.get(), "updated_value");
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        
        // Test signal still works after cleanup
        assert_eq!(signal.get(), "updated_value");
    }

    #[test]
    fn test_signal_manager_component_lifecycle() {
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
    fn test_signal_manager_mixed_operations() {
        // Test mixed operations
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test mixed signal and memo operations
        let signal1 = ArcRwSignal::new("test1".to_string());
        let signal2 = ArcRwSignal::new("test2".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        let tracked_signal1 = manager.track_signal(signal1.clone());
        let tracked_signal2 = manager.track_signal(signal2.clone());
        let tracked_memo = manager.track_memo(memo.clone());
        
        cleanup.track_signal(signal1.clone());
        cleanup.track_signal(signal2.clone());
        cleanup.track_memo(memo.clone());
        
        // Test all work
        assert_eq!(tracked_signal1.get(), "test1");
        assert_eq!(tracked_signal2.get(), "test2");
        assert_eq!(tracked_memo.get(), 42);
        
        // Test updates
        tracked_signal1.set("updated1".to_string());
        tracked_signal2.set("updated2".to_string());
        
        assert_eq!(tracked_signal1.get(), "updated1");
        assert_eq!(tracked_signal2.get(), "updated2");
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_manager_large_scale_integration() {
        // Test large scale integration
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test large scale operations
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
    }

    #[test]
    fn test_signal_manager_memory_integration() {
        // Test memory integration
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test memory integration
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let _tracked = manager.track_signal(signal.clone());
            cleanup.track_signal(signal);
        }
        
        // Test counts
        assert_eq!(manager.tracked_signals_count(), 1000);
        assert_eq!(cleanup.signals_count(), 1000);
        
        // Test memory cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_manager_performance_integration() {
        // Test performance integration
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test performance integration
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
    fn test_signal_manager_edge_case_integration() {
        // Test edge case integration
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test edge cases
        let empty_signal = ArcRwSignal::new("".to_string());
        let large_signal = ArcRwSignal::new("x".repeat(10000));
        let zero_signal = ArcRwSignal::new(0);
        
        let tracked_empty = manager.track_signal(empty_signal.clone());
        let tracked_large = manager.track_signal(large_signal.clone());
        let tracked_zero = manager.track_signal(zero_signal.clone());
        
        cleanup.track_signal(empty_signal.clone());
        cleanup.track_signal(large_signal.clone());
        cleanup.track_signal(zero_signal.clone());
        
        // Test all work
        assert_eq!(tracked_empty.get(), "");
        assert_eq!(tracked_large.get(), "x".repeat(10000));
        assert_eq!(tracked_zero.get(), 0);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_manager_complex_integration() {
        // Test complex integration
        let manager = TailwindSignalManager::new();
        let mut cleanup = SignalCleanup::new();
        
        // Test complex integration
        let signal1 = ArcRwSignal::new("test1".to_string());
        let signal2 = ArcRwSignal::new("test2".to_string());
        let signal3 = ArcRwSignal::new("test3".to_string());
        
        let memo1 = ArcMemo::new(move |_| 10);
        let memo2 = ArcMemo::new(move |_| 20);
        let memo3 = ArcMemo::new(move |_| 30);
        
        let tracked_signal1 = manager.track_signal(signal1.clone());
        let tracked_signal2 = manager.track_signal(signal2.clone());
        let tracked_signal3 = manager.track_signal(signal3.clone());
        
        let tracked_memo1 = manager.track_memo(memo1.clone());
        let tracked_memo2 = manager.track_memo(memo2.clone());
        let tracked_memo3 = manager.track_memo(memo3.clone());
        
        cleanup.track_signal(signal1.clone());
        cleanup.track_signal(signal2.clone());
        cleanup.track_signal(signal3.clone());
        
        cleanup.track_memo(memo1.clone());
        cleanup.track_memo(memo2.clone());
        cleanup.track_memo(memo3.clone());
        
        // Test all work
        assert_eq!(tracked_signal1.get(), "test1");
        assert_eq!(tracked_signal2.get(), "test2");
        assert_eq!(tracked_signal3.get(), "test3");
        
        assert_eq!(tracked_memo1.get(), 10);
        assert_eq!(tracked_memo2.get(), 20);
        assert_eq!(tracked_memo3.get(), 30);
        
        // Test updates
        tracked_signal1.set("updated1".to_string());
        tracked_signal2.set("updated2".to_string());
        tracked_signal3.set("updated3".to_string());
        
        assert_eq!(tracked_signal1.get(), "updated1");
        assert_eq!(tracked_signal2.get(), "updated2");
        assert_eq!(tracked_signal3.get(), "updated3");
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }
}
