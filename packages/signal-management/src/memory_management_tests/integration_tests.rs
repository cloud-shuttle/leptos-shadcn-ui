#[cfg(test)]
mod integration_tests {
    use crate::*;
    use crate::memory_management::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_memory_manager_integration_scenarios() {
        // Test SignalMemoryManager integration scenarios
        let manager = SignalMemoryManager::new();
        
        // Create multiple groups with different content
        let group1 = manager.create_group("group1".to_string());
        let group2 = manager.create_group("group2".to_string());
        let group3 = manager.create_group("group3".to_string());
        
        // Add different content to each group
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let signal3 = ArcRwSignal::new("value3".to_string());
        
        let memo1 = ArcMemo::new(move |_| 10);
        let memo2 = ArcMemo::new(move |_| 20);
        let memo3 = ArcMemo::new(move |_| 30);
        
        let _group1_id = group1;
        let _group2_id = group2;
        let _group3_id = group3;
        
        manager.add_signal(signal1);
        manager.add_memo(memo1);
        
        manager.add_signal(signal2);
        manager.add_memo(memo2);
        
        manager.add_signal(signal3);
        manager.add_memo(memo3);
        
        // Test integration
        assert_eq!(manager.tracked_groups.get().len(), 3);
        
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 3);
        assert_eq!(stats.active_memos, 3);
        assert_eq!(stats.tracked_groups, 3);
    }

    #[test]
    fn test_signal_memory_manager_large_scale_operations() {
        // Test SignalMemoryManager large scale operations
        let manager = SignalMemoryManager::new();
        
        // Create many groups
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add content to each group
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            manager.add_signal(signal);
            manager.add_memo(memo);
        }
        
        // Test large scale state
        assert_eq!(manager.tracked_groups.get().len(), 100);
        
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 100);
        assert_eq!(stats.active_memos, 100);
        assert_eq!(stats.tracked_groups, 100);
        
        // Test cleanup
        for i in 0..50 {
            let group_name = format!("group_{}", i);
            manager.remove_group(&group_name);
        }
        
        assert_eq!(manager.tracked_groups.get().len(), 50);
        
        manager.update_memory_stats();
        let stats_after_cleanup = manager.get_memory_stats();
        assert_eq!(stats_after_cleanup.active_signals, 50);
        assert_eq!(stats_after_cleanup.active_memos, 50);
        assert_eq!(stats_after_cleanup.tracked_groups, 50);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_operations() {
        // Test SignalMemoryManager cleanup operations
        let manager = SignalMemoryManager::new();
        
        // Create groups with different ages
        for i in 0..10 {
            let group_name = format!("old_group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Make some groups old
            if i < 5 {
                manager.tracked_groups.update(|groups| {
                    if let Some(group) = groups.get_mut(&format!("old_group_{}", i)) {
                        group.created_at = 0.0; // Very old timestamp
                    }
                });
            }
        }
        
        assert_eq!(manager.tracked_groups.get().len(), 10);
        
        // Cleanup old groups
        manager.cleanup_old_groups(1000); // Cleanup groups older than 1000 seconds
        
        // Should have removed 5 old groups
        assert_eq!(manager.tracked_groups.get().len(), 5);
        
        // Test low priority cleanup
        manager.tracked_groups.update(|groups| {
            for (_, group) in groups.iter_mut() {
                group.clear(); // Make all groups low priority
            }
        });
        
        manager.cleanup_low_priority_groups();
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_memory_pressure_integration() {
        // Test SignalMemoryManager memory pressure integration
        let max_memory = 1024 * 1024; // 1MB
        let memory_limit = 512 * 1024; // 512KB
        let manager = SignalMemoryManager::with_limits(max_memory, memory_limit);
        
        // Create groups until memory pressure
        for i in 0..100 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add content to increase memory usage
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            manager.add_signal(signal);
            manager.add_memo(memo);
            
            // Update stats
            manager.update_memory_stats();
            
            // Simulate memory usage
            manager.stats.update(|stats| {
                stats.estimated_memory_bytes += 1024; // Add 1KB per group
            });
            
            // Check for memory pressure
            if manager.detect_memory_pressure().is_some() {
                // Should detect pressure after exceeding limit
                assert!(i > 0);
                break;
            }
        }
        
        // Test adaptive cleanup
        manager.adaptive_cleanup();
        
        // Should have cleaned up some groups
        assert!(manager.tracked_groups.get().len() < 100);
    }

    #[test]
    fn test_signal_memory_manager_adaptive_management_integration() {
        // Test SignalMemoryManager adaptive management integration
        let manager = SignalMemoryManager::with_adaptive_management();
        
        // Create groups with different priorities
        for i in 0..20 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add content to some groups
            if i % 2 == 0 {
                let signal = ArcRwSignal::new(format!("value_{}", i));
                let memo = ArcMemo::new(move |_| i);
                manager.add_signal(signal);
                manager.add_memo(memo);
            }
        }
        
        assert_eq!(manager.tracked_groups.get().len(), 20);
        
        // Simulate memory pressure
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = manager.memory_limit + 1;
        });
        
        // Run adaptive cleanup
        manager.adaptive_cleanup();
        
        // Should have cleaned up low priority groups
        assert!(manager.tracked_groups.get().len() < 20);
    }

    #[test]
    fn test_signal_memory_manager_stats_integration() {
        // Test SignalMemoryManager stats integration
        let manager = SignalMemoryManager::new();
        
        // Create groups with content
        for i in 0..10 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            // Add different amounts of content
            for j in 0..i {
                let signal = ArcRwSignal::new(format!("value_{}_{}", i, j));
                let memo = ArcMemo::new(move |_| i * j);
                manager.add_signal(signal);
                manager.add_memo(memo);
            }
        }
        
        // Update stats
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        
        // Calculate expected values
        let expected_signals: usize = (0..10).sum(); // 0+1+2+...+9 = 45
        let expected_memos: usize = (0..10).sum(); // 0+1+2+...+9 = 45
        let expected_groups = 10;
        
        assert_eq!(stats.active_signals, expected_signals);
        assert_eq!(stats.active_memos, expected_memos);
        assert_eq!(stats.tracked_groups, expected_groups);
    }

    #[test]
    fn test_signal_memory_manager_group_lifecycle_integration() {
        // Test SignalMemoryManager group lifecycle integration
        let manager = SignalMemoryManager::new();
        
        // Create group
        let group_name = "lifecycle_group".to_string();
        let _group_id = manager.create_group(group_name.clone());
        
        // Add content
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(move |_| 42);
        manager.add_signal(signal.clone());
        manager.add_memo(memo.clone());
        
        // Verify group exists
        assert!(manager.get_group(&group_name).is_some());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        // Update stats
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 1);
        assert_eq!(stats.active_memos, 1);
        assert_eq!(stats.tracked_groups, 1);
        
        // Remove content
        manager.remove_signal(&signal);
        manager.remove_memo(&memo);
        
        // Update stats
        manager.update_memory_stats();
        let stats_after_removal = manager.get_memory_stats();
        assert_eq!(stats_after_removal.active_signals, 0);
        assert_eq!(stats_after_removal.active_memos, 0);
        assert_eq!(stats_after_removal.tracked_groups, 1);
        
        // Remove group
        manager.remove_group(&group_name);
        assert!(manager.get_group(&group_name).is_none());
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_edge_cases_integration() {
        // Test SignalMemoryManager edge cases integration
        let manager = SignalMemoryManager::new();
        
        // Test with empty group name
        let _empty_group_id = manager.create_group("".to_string());
        
        // Test removing nonexistent group
        manager.remove_group("nonexistent");
        assert_eq!(manager.tracked_groups.get().len(), 1); // Still has empty group
        
        // Test getting nonexistent group
        assert!(manager.get_group("nonexistent").is_none());
        
        // Test memory pressure with no groups
        assert!(manager.detect_memory_pressure().is_none());
        
        // Test cleanup with no groups
        manager.cleanup_old_groups(1000);
        manager.cleanup_low_priority_groups();
        assert_eq!(manager.tracked_groups.get().len(), 1); // Still has empty group
    }

    #[test]
    fn test_signal_memory_manager_performance_integration() {
        // Test SignalMemoryManager performance integration
        let manager = SignalMemoryManager::new();
        
        let start = std::time::Instant::now();
        
        // Create many groups with content
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            let _group_id = manager.create_group(group_name);
            
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            manager.add_signal(signal);
            manager.add_memo(memo);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 2000, "Creating 1000 groups should be reasonable");
        
        // Test stats update performance
        let stats_start = std::time::Instant::now();
        manager.update_memory_stats();
        let stats_duration = stats_start.elapsed();
        
        assert!(stats_duration.as_millis() < 100, "Stats update should be fast");
        
        // Test cleanup performance
        let cleanup_start = std::time::Instant::now();
        for i in 0..1000 {
            let group_name = format!("group_{}", i);
            manager.remove_group(&group_name);
        }
        let cleanup_duration = cleanup_start.elapsed();
        
        assert!(cleanup_duration.as_millis() < 1000, "Cleanup should be reasonable");
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }
}
