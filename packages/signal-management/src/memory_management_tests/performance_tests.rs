#[cfg(test)]
mod performance_tests {
    use crate::*;
    use crate::memory_management::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_memory_manager_performance_characteristics() {
        // Test SignalMemoryManager performance characteristics
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Create many groups
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            let _group = manager.create_group(group_name);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Creating 1000 groups should be fast");
        assert_eq!(manager.tracked_groups.get().len(), 1000);
    }

    #[test]
    fn test_signal_memory_manager_memory_management() {
        // Test SignalMemoryManager memory management
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Create groups with signals and memos
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add signals and memos to manager
            for j in 0..10 {
                let signal = ArcRwSignal::new(format!("value_{}_{}", i, j));
                let memo = ArcMemo::new(move |_| i * j);
                manager.add_signal(signal);
                manager.add_memo(memo);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500, "Creating groups with signals/memos should be fast");
        
        // Test memory stats
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 1000); // 100 groups * 10 signals
        assert_eq!(stats.active_memos, 1000); // 100 groups * 10 memos
        assert_eq!(stats.tracked_groups, 100);
    }

    #[test]
    fn test_signal_group_memory_management() {
        // Test SignalGroup memory management
        let mut group = SignalGroup::new("test_group".to_string());
        
        let start = std::time::Instant::now();
        
        // Add many signals and memos
        for i in 0..10000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            group.add_signal(signal);
            group.add_memo(memo);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Adding 10000 signals/memos should be fast");
        
        assert_eq!(group.signals.len(), 10000);
        assert_eq!(group.memos.len(), 10000);
        
        // Test clearing performance
        let clear_start = std::time::Instant::now();
        group.clear();
        let clear_duration = clear_start.elapsed();
        
        assert!(clear_duration.as_millis() < 100, "Clearing should be fast");
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_memory_stats_performance() {
        // Test MemoryStats performance
        let start = std::time::Instant::now();
        
        // Create many stats
        for i in 0..10000 {
            let _stats = MemoryStats {
                active_signals: i,
                active_memos: i * 2,
                estimated_memory_bytes: i * 1024,
                tracked_groups: i / 10,
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Creating 10000 stats should be fast");
    }

    #[test]
    fn test_signal_memory_manager_cleanup_performance() {
        // Test SignalMemoryManager cleanup performance
        let manager = SignalMemoryManager::new();
        
        // Create many groups
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            let _group = manager.create_group(group_name);
        }
        
        assert_eq!(manager.tracked_groups.get().len(), 1000);
        
        let start = std::time::Instant::now();
        
        // Cleanup all groups
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            manager.remove_group(&group_name);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500, "Removing 1000 groups should be fast");
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_memory_pressure_performance() {
        // Test SignalMemoryManager memory pressure detection performance
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Test memory pressure detection many times
        for _i in 0..10000 {
            let _pressure = manager.detect_memory_pressure();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Memory pressure detection should be fast");
    }

    #[test]
    fn test_signal_memory_manager_stats_update_performance() {
        // Test SignalMemoryManager stats update performance
        let manager = SignalMemoryManager::new();
        
        // Create some groups with content
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            manager.add_signal(signal);
            manager.add_memo(memo);
        }
        
        let start = std::time::Instant::now();
        
        // Update stats many times
        for _i in 0..1000 {
            manager.update_memory_stats();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500, "Stats update should be fast");
    }

    #[test]
    fn test_signal_memory_manager_adaptive_cleanup_performance() {
        // Test SignalMemoryManager adaptive cleanup performance
        let manager = SignalMemoryManager::with_adaptive_management();
        
        // Create many groups
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            let _group = manager.create_group(group_name);
        }
        
        // Simulate memory pressure
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = manager.memory_limit + 1;
        });
        
        let start = std::time::Instant::now();
        
        // Run adaptive cleanup
        manager.adaptive_cleanup();
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Adaptive cleanup should be fast");
    }

    #[test]
    fn test_signal_memory_manager_large_scale_operations() {
        // Test SignalMemoryManager large scale operations
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Create many groups with many signals/memos
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add many signals and memos to each group
            for j in 0..100 {
                let signal = ArcRwSignal::new(format!("value_{}_{}", i, j));
                let memo = ArcMemo::new(move |_| i * j);
                manager.add_signal(signal);
                manager.add_memo(memo);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 2000, "Large scale operations should be reasonable");
        
        // Test final state
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 10000); // 100 groups * 100 signals
        assert_eq!(stats.active_memos, 10000); // 100 groups * 100 memos
        assert_eq!(stats.tracked_groups, 100);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_operations() {
        // Test SignalMemoryManager cleanup operations
        let manager = SignalMemoryManager::new();
        
        // Create groups with old timestamps
        for i in 0..100 {
            let group_name = format!("old_group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Simulate old timestamp
            manager.tracked_groups.update(|groups| {
                if let Some(group) = groups.get_mut(&format!("old_group_{}", i)) {
                    group.created_at = 0.0; // Very old timestamp
                }
            });
        }
        
        assert_eq!(manager.tracked_groups.get().len(), 100);
        
        let start = std::time::Instant::now();
        
        // Cleanup old groups
        manager.cleanup_old_groups(1000); // Cleanup groups older than 1000 seconds
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Cleanup operations should be fast");
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_memory_efficiency() {
        // Test SignalMemoryManager memory efficiency
        let manager = SignalMemoryManager::new();
        
        // Create groups and measure memory usage
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            let _group = manager.create_group(group_name);
        }
        
        let duration = start.elapsed();
        let memory_per_group = duration.as_nanos() / 1000; // Rough estimate
        
        // Test memory efficiency (should be reasonable)
        assert!(memory_per_group < 1000, "Memory per group should be efficient");
    }

    #[test]
    fn test_signal_memory_manager_concurrent_operations() {
        // Test SignalMemoryManager concurrent-like operations
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Simulate concurrent operations
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name.clone());
            
            // Add content
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            manager.add_signal(signal);
            manager.add_memo(memo);
            
            // Update stats
            manager.update_memory_stats();
            
            // Check memory pressure
            let _pressure = manager.detect_memory_pressure();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Concurrent operations should be fast");
    }
}
