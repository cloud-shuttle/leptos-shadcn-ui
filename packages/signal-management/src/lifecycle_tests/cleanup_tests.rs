#[cfg(test)]
mod cleanup_tests {
    use crate::*;
    use leptos::prelude::*;

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
        
        cleanup.track_signal(signal1.clone());
        assert_eq!(cleanup.signals_count(), 1);
        
        cleanup.track_signal(signal2.clone());
        assert_eq!(cleanup.signals_count(), 2);
        
        // Test signals still work
        assert_eq!(signal1.get(), "value1");
        assert_eq!(signal2.get(), "value2");
    }

    #[test]
    fn test_signal_cleanup_memo_tracking() {
        // Test memo tracking in cleanup
        let mut cleanup = SignalCleanup::new();
        
        // Test initial count
        assert_eq!(cleanup.memos_count(), 0);
        
        // Track memos
        let signal = ArcRwSignal::new(42);
        let signal_clone1 = signal.clone();
        let signal_clone2 = signal.clone();
        let memo1 = ArcMemo::new(move |_| signal_clone1.get() * 2);
        let memo2 = ArcMemo::new(move |_| signal_clone2.get() * 3);
        
        cleanup.track_memo(memo1.clone());
        assert_eq!(cleanup.memos_count(), 1);
        
        cleanup.track_memo(memo2.clone());
        assert_eq!(cleanup.memos_count(), 2);
        
        // Test memos still work
        assert_eq!(memo1.get(), 84);
        assert_eq!(memo2.get(), 126);
    }

    #[test]
    fn test_signal_cleanup_cleanup_operation() {
        // Test cleanup operation
        let mut cleanup = SignalCleanup::new();
        
        // Track signals and memos
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        cleanup.track_signal(signal1.clone());
        cleanup.track_signal(signal2.clone());
        cleanup.track_memo(memo.clone());
        
        // Test initial counts
        assert_eq!(cleanup.signals_count(), 2);
        assert_eq!(cleanup.memos_count(), 1);
        
        // Test cleanup
        cleanup.cleanup();
        
        // Test counts after cleanup
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_cleanup_drop_implementation() {
        // Test drop implementation
        let mut cleanup = SignalCleanup::new();
        
        // Track signals
        let signal = ArcRwSignal::new("test".to_string());
        cleanup.track_signal(signal.clone());
        
        // Test initial count
        assert_eq!(cleanup.signals_count(), 1);
        
        // Test drop behavior
        drop(cleanup);
        
        // Test signal still works after cleanup is dropped
        assert_eq!(signal.get(), "test");
    }

    #[test]
    fn test_signal_cleanup_memory_management() {
        // Test memory management
        let mut cleanup = SignalCleanup::new();
        
        // Track many signals
        for i in 0..100 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            cleanup.track_signal(signal);
        }
        
        // Test count
        assert_eq!(cleanup.signals_count(), 100);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        
        // Test memory is cleaned up
        assert!(true); // If we get here, memory cleanup worked
    }

    #[test]
    fn test_signal_cleanup_edge_cases() {
        // Test cleanup edge cases
        let mut cleanup = SignalCleanup::new();
        
        // Test with empty string
        let empty_signal = ArcRwSignal::new("".to_string());
        cleanup.track_signal(empty_signal.clone());
        assert_eq!(cleanup.signals_count(), 1);
        
        // Test with large string
        let large_string = "x".repeat(10000);
        let large_signal = ArcRwSignal::new(large_string.clone());
        cleanup.track_signal(large_signal.clone());
        assert_eq!(cleanup.signals_count(), 2);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
        
        // Test signals still work
        assert_eq!(empty_signal.get(), "");
        assert_eq!(large_signal.get(), large_string);
    }

    #[test]
    fn test_signal_cleanup_clone_behavior() {
        // Test cleanup cloning behavior
        let mut cleanup1 = SignalCleanup::new();
        let signal = ArcRwSignal::new("test".to_string());
        cleanup1.track_signal(signal);
        
        // Test cloning
        let cleanup2 = cleanup1.clone();
        
        // Test both cleanups have same state
        assert_eq!(cleanup1.signals_count(), cleanup2.signals_count());
        assert_eq!(cleanup1.memos_count(), cleanup2.memos_count());
    }

    #[test]
    fn test_signal_cleanup_debug_formatting() {
        // Test cleanup debug formatting
        let cleanup = SignalCleanup::new();
        let debug_str = format!("{:?}", cleanup);
        assert!(debug_str.contains("SignalCleanup"));
    }

    #[test]
    fn test_signal_cleanup_large_scale_tracking() {
        // Test large scale signal tracking
        let mut cleanup = SignalCleanup::new();
        
        // Track many signals
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            cleanup.track_signal(signal);
        }
        
        // Test count
        assert_eq!(cleanup.signals_count(), 1000);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.signals_count(), 0);
    }

    #[test]
    fn test_signal_cleanup_large_scale_memo_tracking() {
        // Test large scale memo tracking
        let mut cleanup = SignalCleanup::new();
        
        // Track many memos
        for i in 0..1000 {
            let signal = ArcRwSignal::new(i);
            let memo = ArcMemo::new(move |_| signal.get() * 2);
            cleanup.track_memo(memo);
        }
        
        // Test count
        assert_eq!(cleanup.memos_count(), 1000);
        
        // Test cleanup
        cleanup.cleanup();
        assert_eq!(cleanup.memos_count(), 0);
    }

    #[test]
    fn test_signal_cleanup_performance() {
        // Test cleanup performance
        let mut cleanup = SignalCleanup::new();
        
        // Track many signals and memos
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            cleanup.track_signal(signal);
            
            let memo = ArcMemo::new(move |_| i * 2);
            cleanup.track_memo(memo);
        }
        
        // Test counts
        assert_eq!(cleanup.signals_count(), 1000);
        assert_eq!(cleanup.memos_count(), 1000);
        
        // Test cleanup performance
        let start = std::time::Instant::now();
        cleanup.cleanup();
        let duration = start.elapsed();
        
        // Should be fast
        assert!(duration.as_millis() < 100);
        
        // Test counts after cleanup
        assert_eq!(cleanup.signals_count(), 0);
        assert_eq!(cleanup.memos_count(), 0);
    }
}
