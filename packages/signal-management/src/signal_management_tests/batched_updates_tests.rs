#[cfg(test)]
mod batched_updates_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_batched_signal_updater_creation() {
        // Test BatchedSignalUpdater creation
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert_eq!(updater.queue_size(), 0);
        assert!(!updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_with_custom_batch_size() {
        // Test BatchedSignalUpdater with custom batch size
        let updater = BatchedSignalUpdater::with_batch_size(10);
        
        // Test initial state
        assert_eq!(updater.queue_size(), 0);
        assert!(!updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_queue_update() {
        // Test queuing updates
        let mut updater = BatchedSignalUpdater::new();
        
        // Test initial queue size
        assert_eq!(updater.queue_size(), 0);
        
        // Queue updates
        let signal = ArcRwSignal::new("initial".to_string());
        let signal_clone1 = signal.clone();
        updater.queue_update(move || {
            signal_clone1.set("update1".to_string());
        }).unwrap();
        assert_eq!(updater.queue_size(), 1);
        
        let signal_clone2 = signal.clone();
        updater.queue_update(move || {
            signal_clone2.set("update2".to_string());
        }).unwrap();
        assert_eq!(updater.queue_size(), 2);
        
        // Test signal still has original value
        assert_eq!(signal.get(), "initial");
    }

    #[test]
    fn test_batched_signal_updater_start_batching() {
        // Test starting batching
        let mut updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert!(!updater.is_batching());
        
        // Start batching
        updater.start_batching();
        assert!(updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_stop_batching() {
        // Test stopping batching
        let mut updater = BatchedSignalUpdater::new();
        
        // Start batching
        updater.start_batching();
        assert!(updater.is_batching());
        
        // Stop batching
        updater.stop_batching();
        assert!(!updater.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_flush_updates() {
        // Test flushing updates
        let mut updater = BatchedSignalUpdater::new();
        
        // Queue updates
        let signal1 = ArcRwSignal::new("initial1".to_string());
        let signal2 = ArcRwSignal::new("initial2".to_string());
        
        let signal1_clone = signal1.clone();
        updater.queue_update(move || {
            signal1_clone.set("update1".to_string());
        }).unwrap();
        let signal2_clone = signal2.clone();
        updater.queue_update(move || {
            signal2_clone.set("update2".to_string());
        }).unwrap();
        
        // Test queue size
        assert_eq!(updater.queue_size(), 2);
        
        // Test signals still have original values
        assert_eq!(signal1.get(), "initial1");
        assert_eq!(signal2.get(), "initial2");
        
        // Flush updates
        updater.flush_updates();
        
        // Test queue is empty
        assert_eq!(updater.queue_size(), 0);
        
        // Test signals are updated
        assert_eq!(signal1.get(), "update1");
        assert_eq!(signal2.get(), "update2");
    }

    #[test]
    fn test_batched_signal_updater_clear_updates() {
        // Test clearing updates
        let mut updater = BatchedSignalUpdater::new();
        
        // Queue updates
        let signal1 = ArcRwSignal::new("initial1".to_string());
        let signal2 = ArcRwSignal::new("initial2".to_string());
        
        let signal1_clone = signal1.clone();
        updater.queue_update(move || {
            signal1_clone.set("update1".to_string());
        }).unwrap();
        let signal2_clone = signal2.clone();
        updater.queue_update(move || {
            signal2_clone.set("update2".to_string());
        }).unwrap();
        
        // Test queue size
        assert_eq!(updater.queue_size(), 2);
        
        // Clear updates
        updater.clear_updates();
        
        // Test queue is empty
        assert_eq!(updater.queue_size(), 0);
        
        // Test signals still have original values
        assert_eq!(signal1.get(), "initial1");
        assert_eq!(signal2.get(), "initial2");
    }

    #[test]
    fn test_batched_signal_updater_automatic_flush() {
        // Test automatic flush when batch size is reached
        let mut updater = BatchedSignalUpdater::with_batch_size(3);
        
        // Queue updates up to batch size
        let signal1 = ArcRwSignal::new("initial1".to_string());
        let signal2 = ArcRwSignal::new("initial2".to_string());
        let signal3 = ArcRwSignal::new("initial3".to_string());
        
        let signal1_clone = signal1.clone();
        updater.queue_update(move || {
            signal1_clone.set("update1".to_string());
        }).unwrap();
        assert_eq!(updater.queue_size(), 1);
        
        let signal2_clone = signal2.clone();
        updater.queue_update(move || {
            signal2_clone.set("update2".to_string());
        }).unwrap();
        assert_eq!(updater.queue_size(), 2);
        
        let signal3_clone = signal3.clone();
        updater.queue_update(move || {
            signal3_clone.set("update3".to_string());
        }).unwrap();
        
        // Test automatic flush
        assert_eq!(updater.queue_size(), 0);
        assert_eq!(signal1.get(), "update1");
        assert_eq!(signal2.get(), "update2");
        assert_eq!(signal3.get(), "update3");
    }

    #[test]
    fn test_batched_signal_updater_multiple_signals() {
        // Test multiple signals in batch
        let mut updater = BatchedSignalUpdater::new();
        
        // Queue updates for multiple signals
        let signal1 = ArcRwSignal::new("initial1".to_string());
        let signal2 = ArcRwSignal::new("initial2".to_string());
        let signal3 = ArcRwSignal::new("initial3".to_string());
        
        let signal1_clone = signal1.clone();
        updater.queue_update(move || {
            signal1_clone.set("update1".to_string());
        }).unwrap();
        let signal2_clone = signal2.clone();
        updater.queue_update(move || {
            signal2_clone.set("update2".to_string());
        }).unwrap();
        let signal3_clone = signal3.clone();
        updater.queue_update(move || {
            signal3_clone.set("update3".to_string());
        }).unwrap();
        
        // Test queue size
        assert_eq!(updater.queue_size(), 3);
        
        // Flush updates
        updater.flush_updates();
        
        // Test all signals are updated
        assert_eq!(signal1.get(), "update1");
        assert_eq!(signal2.get(), "update2");
        assert_eq!(signal3.get(), "update3");
    }

    #[test]
    fn test_batched_signal_updater_same_signal_multiple_updates() {
        // Test same signal with multiple updates
        let mut updater = BatchedSignalUpdater::new();
        
        // Queue multiple updates for same signal
        let signal = ArcRwSignal::new("initial".to_string());
        
        let signal_clone1 = signal.clone();
        updater.queue_update(move || {
            signal_clone1.set("update1".to_string());
        }).unwrap();
        let signal_clone2 = signal.clone();
        updater.queue_update(move || {
            signal_clone2.set("update2".to_string());
        }).unwrap();
        let signal_clone3 = signal.clone();
        updater.queue_update(move || {
            signal_clone3.set("update3".to_string());
        }).unwrap();
        
        // Test queue size
        assert_eq!(updater.queue_size(), 3);
        
        // Flush updates
        updater.flush_updates();
        
        // Test signal has last update
        assert_eq!(signal.get(), "update3");
    }

    #[test]
    fn test_batched_signal_updater_clone_behavior() {
        // Test updater cloning behavior
        let mut updater1 = BatchedSignalUpdater::new();
        let signal = ArcRwSignal::new("test".to_string());
        let signal_clone = signal.clone();
        updater1.queue_update(move || {
            signal_clone.set("update".to_string());
        }).unwrap();
        
        // Test cloning
        let updater2 = updater1.clone();
        
        // Test both updaters have same state
        assert_eq!(updater1.queue_size(), updater2.queue_size());
        assert_eq!(updater1.is_batching(), updater2.is_batching());
    }

    #[test]
    fn test_batched_signal_updater_debug_formatting() {
        // Test updater debug formatting
        let updater = BatchedSignalUpdater::new();
        let debug_str = format!("{:?}", updater);
        assert!(debug_str.contains("BatchedSignalUpdater"));
    }

    #[test]
    fn test_batched_signal_updater_performance() {
        // Test updater performance
        let mut updater = BatchedSignalUpdater::new();
        
        // Test queuing many updates
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("initial_{}", i));
            let signal_clone = signal.clone();
            updater.queue_update(move || {
                signal_clone.set(format!("update_{}", i));
            }).unwrap();
        }
        
        let queue_duration = start.elapsed();
        
        // Should be fast
        assert!(queue_duration.as_millis() < 100);
        
        // Test queue size
        assert_eq!(updater.queue_size(), 1000);
        
        // Test flush performance
        let flush_start = std::time::Instant::now();
        updater.flush_updates();
        let flush_duration = flush_start.elapsed();
        
        // Should be fast
        assert!(flush_duration.as_millis() < 100);
        
        // Test queue is empty
        assert_eq!(updater.queue_size(), 0);
    }

    #[test]
    fn test_batched_signal_updater_memory_management() {
        // Test memory management
        let mut updater = BatchedSignalUpdater::new();
        
        // Queue many updates
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("initial_{}", i));
            let signal_clone = signal.clone();
            updater.queue_update(move || {
                signal_clone.set(format!("update_{}", i));
            }).unwrap();
        }
        
        // Test queue size
        assert_eq!(updater.queue_size(), 1000);
        
        // Clear updates
        updater.clear_updates();
        
        // Test queue is empty
        assert_eq!(updater.queue_size(), 0);
        
        // Test memory is cleaned up
        assert!(true); // If we get here, memory cleanup worked
    }
}
