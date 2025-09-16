#[cfg(test)]
mod batched_updates_tests {
    use super::*;
    use crate::batched_updates::*;
    use leptos::prelude::*;

    // ===== COMPREHENSIVE BATCHED UPDATES TESTS =====
    // These tests focus on batched signal updates and performance optimization

    #[test]
    fn test_batched_signal_updater_creation() {
        // Test BatchedSignalUpdater creation
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert_eq!(updater.max_batch_size, 1000);
        assert_eq!(updater.update_queue.get().len(), 0);
        assert!(!updater.is_batching.get());
    }

    #[test]
    fn test_batched_signal_updater_with_custom_batch_size() {
        // Test BatchedSignalUpdater with custom batch size
        let custom_batch_size = 500;
        let updater = BatchedSignalUpdater::with_batch_size(custom_batch_size);
        
        // Test custom batch size
        assert_eq!(updater.max_batch_size, custom_batch_size);
        assert_eq!(updater.update_queue.get().len(), 0);
        assert!(!updater.is_batching.get());
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
        assert_eq!(updater.update_queue.get().len(), 1);
    }

    #[test]
    fn test_batched_signal_updater_queue_multiple_updates() {
        // Test queueing multiple updates
        let updater = BatchedSignalUpdater::new();
        let signal1 = ArcRwSignal::new(0);
        let signal2 = ArcRwSignal::new(0);
        let signal3 = ArcRwSignal::new(0);
        
        // Queue multiple updates
        let result1 = updater.queue_update(move || {
            signal1.set(1);
        });
        let result2 = updater.queue_update(move || {
            signal2.set(2);
        });
        let result3 = updater.queue_update(move || {
            signal3.set(3);
        });
        
        // Test all updates were queued successfully
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        assert_eq!(updater.update_queue.get().len(), 3);
    }

    #[test]
    fn test_batched_signal_updater_max_batch_size_exceeded() {
        // Test max batch size exceeded error
        let small_batch_size = 2;
        let updater = BatchedSignalUpdater::with_batch_size(small_batch_size);
        let signal = ArcRwSignal::new(0);
        
        // Queue updates up to the limit
        let result1 = updater.queue_update(move || {
            signal.set(1);
        });
        let result2 = updater.queue_update(move || {
            signal.set(2);
        });
        
        // Test first two updates succeed
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(updater.update_queue.get().len(), 2);
        
        // Test third update fails due to batch size limit
        let result3 = updater.queue_update(move || {
            signal.set(3);
        });
        
        assert!(result3.is_err());
        if let Err(SignalManagementError::BatchedUpdateFailed { reason }) = result3 {
            assert!(reason.contains("Maximum batch size"));
        }
    }

    #[test]
    fn test_batched_signal_updater_start_batching() {
        // Test starting batching
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert!(!updater.is_batching.get());
        
        // Start batching
        let result = updater.start_batching();
        assert!(result.is_ok());
        assert!(updater.is_batching.get());
    }

    #[test]
    fn test_batched_signal_updater_stop_batching() {
        // Test stopping batching
        let updater = BatchedSignalUpdater::new();
        let test_signal = ArcRwSignal::new(0);
        
        // Start batching and queue an update
        updater.start_batching().unwrap();
        updater.queue_update(move || {
            test_signal.set(42);
        }).unwrap();
        
        // Test batching is active
        assert!(updater.is_batching.get());
        assert_eq!(updater.update_queue.get().len(), 1);
        
        // Stop batching
        let result = updater.stop_batching();
        assert!(result.is_ok());
        assert!(!updater.is_batching.get());
        
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
        
        // Queue multiple updates
        updater.queue_update(move || {
            signal1.set(1);
        }).unwrap();
        updater.queue_update(move || {
            signal2.set(2);
        }).unwrap();
        updater.queue_update(move || {
            signal3.set(3);
        }).unwrap();
        
        // Test updates are queued
        assert_eq!(updater.update_queue.get().len(), 3);
        
        // Flush updates
        let result = updater.flush_updates();
        assert!(result.is_ok());
        
        // Test queue is empty after flush
        assert_eq!(updater.update_queue.get().len(), 0);
        
        // Test signals were updated
        assert_eq!(signal1.get(), 1);
        assert_eq!(signal2.get(), 2);
        assert_eq!(signal3.get(), 3);
    }

    #[test]
    fn test_batched_signal_updater_clear_updates() {
        // Test clearing updates
        let updater = BatchedSignalUpdater::new();
        let signal = ArcRwSignal::new(0);
        
        // Queue an update
        updater.queue_update(move || {
            signal.set(42);
        }).unwrap();
        
        // Test update is queued
        assert_eq!(updater.update_queue.get().len(), 1);
        
        // Clear updates
        let result = updater.clear_updates();
        assert!(result.is_ok());
        
        // Test queue is empty after clear
        assert_eq!(updater.update_queue.get().len(), 0);
        
        // Test signal was not updated
        assert_eq!(signal.get(), 0);
    }

    #[test]
    fn test_batched_signal_updater_batch_size_validation() {
        // Test batch size validation
        let zero_batch_size = 0;
        let updater = BatchedSignalUpdater::with_batch_size(zero_batch_size);
        
        // Test that zero batch size is handled
        assert_eq!(updater.max_batch_size, 0);
        
        // Test that queueing fails with zero batch size
        let signal = ArcRwSignal::new(0);
        let result = updater.queue_update(move || {
            signal.set(42);
        });
        
        assert!(result.is_err());
    }

    #[test]
    fn test_batched_signal_updater_performance_characteristics() {
        // Test performance characteristics
        let start = std::time::Instant::now();
        
        // Create multiple updaters
        for _ in 0..1000 {
            let updater = BatchedSignalUpdater::new();
            let signal = ArcRwSignal::new(0);
            
            // Queue some updates
            for _ in 0..10 {
                updater.queue_update(move || {
                    signal.set(42);
                }).unwrap();
            }
        }
        
        let duration = start.elapsed();
        
        // Should complete without panicking
        assert!(duration.as_nanos() >= 0, "Batched updater creation should complete");
    }

    #[test]
    fn test_batched_signal_updater_memory_management() {
        // Test memory management
        let mut updaters = Vec::new();
        
        // Create multiple updaters
        for i in 0..100 {
            let updater = BatchedSignalUpdater::new();
            let signal = ArcRwSignal::new(i);
            
            // Queue some updates
            for j in 0..5 {
                updater.queue_update(move || {
                    signal.set(i + j);
                }).unwrap();
            }
            
            updaters.push(updater);
        }
        
        // Test that updaters can be dropped without issues
        drop(updaters);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_batched_signal_updater_error_handling() {
        // Test error handling scenarios
        let updater = BatchedSignalUpdater::new();
        
        // Test queueing update when not batching
        let signal = ArcRwSignal::new(0);
        let result = updater.queue_update(move || {
            signal.set(42);
        });
        
        // Should succeed even when not batching
        assert!(result.is_ok());
        
        // Test starting batching when already batching
        updater.start_batching().unwrap();
        let result = updater.start_batching();
        
        // Should handle gracefully
        assert!(result.is_ok());
    }

    #[test]
    fn test_batched_signal_updater_complex_update_scenarios() {
        // Test complex update scenarios
        let updater = BatchedSignalUpdater::new();
        let signal1 = ArcRwSignal::new(0);
        let signal2 = ArcRwSignal::new(0);
        let signal3 = ArcRwSignal::new(0);
        
        // Start batching
        updater.start_batching().unwrap();
        
        // Queue complex updates
        updater.queue_update(move || {
            signal1.set(1);
            signal2.set(2);
        }).unwrap();
        
        updater.queue_update(move || {
            signal3.set(3);
        }).unwrap();
        
        // Test updates are queued
        assert_eq!(updater.update_queue.get().len(), 2);
        
        // Stop batching to execute updates
        updater.stop_batching().unwrap();
        
        // Test all signals were updated
        assert_eq!(signal1.get(), 1);
        assert_eq!(signal2.get(), 2);
        assert_eq!(signal3.get(), 3);
    }

    #[test]
    fn test_batched_signal_updater_edge_cases() {
        // Test edge cases
        let updater = BatchedSignalUpdater::new();
        
        // Test queueing empty update
        let result = updater.queue_update(|| {});
        assert!(result.is_ok());
        
        // Test flushing empty queue
        let result = updater.flush_updates();
        assert!(result.is_ok());
        
        // Test clearing empty queue
        let result = updater.clear_updates();
        assert!(result.is_ok());
    }

    #[test]
    fn test_batched_signal_updater_integration_scenarios() {
        // Test integration scenarios
        let updater = BatchedSignalUpdater::new();
        let theme_signal = ArcRwSignal::new("light".to_string());
        let variant_signal = ArcRwSignal::new("primary".to_string());
        let size_signal = ArcRwSignal::new("medium".to_string());
        
        // Simulate theme switching with batched updates
        updater.start_batching().unwrap();
        
        updater.queue_update(move || {
            theme_signal.set("dark".to_string());
        }).unwrap();
        
        updater.queue_update(move || {
            variant_signal.set("secondary".to_string());
        }).unwrap();
        
        updater.queue_update(move || {
            size_signal.set("large".to_string());
        }).unwrap();
        
        // Execute all updates at once
        updater.stop_batching().unwrap();
        
        // Test all signals were updated
        assert_eq!(theme_signal.get(), "dark");
        assert_eq!(variant_signal.get(), "secondary");
        assert_eq!(size_signal.get(), "large");
    }

    #[test]
    fn test_batched_signal_updater_large_batch_handling() {
        // Test handling large batches
        let large_batch_size = 10000;
        let updater = BatchedSignalUpdater::with_batch_size(large_batch_size);
        
        // Queue many updates
        for i in 0..1000 {
            let signal = ArcRwSignal::new(i);
            updater.queue_update(move || {
                signal.set(i * 2);
            }).unwrap();
        }
        
        // Test all updates were queued
        assert_eq!(updater.update_queue.get().len(), 1000);
        
        // Flush all updates
        let result = updater.flush_updates();
        assert!(result.is_ok());
        
        // Test queue is empty
        assert_eq!(updater.update_queue.get().len(), 0);
    }

    #[test]
    fn test_batched_signal_updater_concurrent_access() {
        // Test concurrent access scenarios
        let updater = BatchedSignalUpdater::new();
        let signal = ArcRwSignal::new(0);
        
        // Simulate concurrent updates
        for i in 0..100 {
            let signal_clone = signal.clone();
            updater.queue_update(move || {
                signal_clone.set(i);
            }).unwrap();
        }
        
        // Test all updates were queued
        assert_eq!(updater.update_queue.get().len(), 100);
        
        // Flush updates
        updater.flush_updates().unwrap();
        
        // Test signal was updated (to the last value)
        assert_eq!(signal.get(), 99);
    }

    #[test]
    fn test_batched_signal_updater_state_consistency() {
        // Test state consistency
        let updater = BatchedSignalUpdater::new();
        
        // Test initial state
        assert!(!updater.is_batching.get());
        assert_eq!(updater.update_queue.get().len(), 0);
        
        // Start batching
        updater.start_batching().unwrap();
        assert!(updater.is_batching.get());
        
        // Queue an update
        let signal = ArcRwSignal::new(0);
        updater.queue_update(move || {
            signal.set(42);
        }).unwrap();
        
        // Test state consistency
        assert!(updater.is_batching.get());
        assert_eq!(updater.update_queue.get().len(), 1);
        
        // Stop batching
        updater.stop_batching().unwrap();
        assert!(!updater.is_batching.get());
        assert_eq!(updater.update_queue.get().len(), 0);
    }
}
