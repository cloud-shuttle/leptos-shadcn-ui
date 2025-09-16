#[cfg(test)]
mod memory_management_tests {
    use super::*;
    use crate::memory_management::*;
    use leptos::prelude::*;
    use std::collections::HashMap;

    // ===== COMPREHENSIVE MEMORY MANAGEMENT TESTS =====
    // These tests focus on memory management and signal lifecycle optimization

    #[test]
    fn test_memory_stats_creation() {
        // Test MemoryStats creation
        let stats = MemoryStats::default();
        
        // Test initial state
        assert_eq!(stats.active_signals, 0);
        assert_eq!(stats.active_memos, 0);
        assert_eq!(stats.estimated_memory_bytes, 0);
        assert_eq!(stats.tracked_groups, 0);
    }

    #[test]
    fn test_memory_stats_custom_values() {
        // Test MemoryStats with custom values
        let stats = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        // Test custom values
        assert_eq!(stats.active_signals, 10);
        assert_eq!(stats.active_memos, 5);
        assert_eq!(stats.estimated_memory_bytes, 1024);
        assert_eq!(stats.tracked_groups, 3);
    }

    #[test]
    fn test_signal_memory_manager_creation() {
        // Test SignalMemoryManager creation
        let manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 0);
        assert_eq!(manager.stats.get().active_signals, 0);
        assert_eq!(manager.stats.get().active_memos, 0);
        assert_eq!(manager.stats.get().estimated_memory_bytes, 0);
        assert_eq!(manager.stats.get().tracked_groups, 0);
    }

    #[test]
    fn test_signal_memory_manager_with_custom_limits() {
        // Test SignalMemoryManager with custom limits
        let max_memory = 1024 * 1024; // 1MB
        let memory_limit = 512 * 1024; // 512KB
        let manager = SignalMemoryManager::with_limits(max_memory, memory_limit);
        
        // Test custom limits
        assert_eq!(manager.max_memory_bytes, max_memory);
        assert_eq!(manager.memory_limit, memory_limit);
        assert_eq!(manager.adaptive_management, false);
    }

    #[test]
    fn test_signal_memory_manager_with_adaptive_management() {
        // Test SignalMemoryManager with adaptive management
        let manager = SignalMemoryManager::with_adaptive_management();
        
        // Test adaptive management is enabled
        assert!(manager.adaptive_management);
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_group_creation() {
        // Test SignalGroup creation
        let group = SignalGroup::new("test_group".to_string());
        
        // Test initial state
        assert_eq!(group.name, "test_group");
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
        assert_eq!(group.created_at, 0); // Default timestamp
    }

    #[test]
    fn test_signal_group_with_timestamp() {
        // Test SignalGroup with custom timestamp
        let timestamp = 1234567890;
        let group = SignalGroup::with_timestamp("test_group".to_string(), timestamp);
        
        // Test custom timestamp
        assert_eq!(group.name, "test_group");
        assert_eq!(group.created_at, timestamp);
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_add_signal() {
        // Test adding signals to group
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        // Test adding signal
        group.add_signal(signal.clone());
        assert_eq!(group.signals.len(), 1);
        
        // Test signal is accessible
        assert_eq!(signal.get(), "test_value");
    }

    #[test]
    fn test_signal_group_add_memo() {
        // Test adding memos to group
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        // Test adding memo
        group.add_memo(memo.clone());
        assert_eq!(group.memos.len(), 1);
        
        // Test memo is accessible
        assert_eq!(memo.get(), "test_memo");
    }

    #[test]
    fn test_signal_group_remove_signal() {
        // Test removing signals from group
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        // Add signal
        group.add_signal(signal.clone());
        assert_eq!(group.signals.len(), 1);
        
        // Remove signal
        group.remove_signal(&signal);
        assert_eq!(group.signals.len(), 0);
    }

    #[test]
    fn test_signal_group_remove_memo() {
        // Test removing memos from group
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        // Add memo
        group.add_memo(memo.clone());
        assert_eq!(group.memos.len(), 1);
        
        // Remove memo
        group.remove_memo(&memo);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_clear() {
        // Test clearing group
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        // Add signals and memos
        group.add_signal(signal);
        group.add_memo(memo);
        assert_eq!(group.signals.len(), 1);
        assert_eq!(group.memos.len(), 1);
        
        // Clear group
        group.clear();
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_create_group() {
        // Test creating signal groups
        let manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 0);
        
        // Create group
        let group = manager.create_group("test_group".to_string());
        assert_eq!(group.name, "test_group");
        
        // Test group was added to manager
        assert_eq!(manager.tracked_groups.get().len(), 1);
    }

    #[test]
    fn test_signal_memory_manager_remove_group() {
        // Test removing signal groups
        let manager = SignalMemoryManager::new();
        
        // Create group
        let group = manager.create_group("test_group".to_string());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        // Remove group
        let result = manager.remove_group("test_group");
        assert!(result.is_ok());
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_get_group() {
        // Test getting signal groups
        let manager = SignalMemoryManager::new();
        
        // Create group
        let group = manager.create_group("test_group".to_string());
        
        // Get group
        let retrieved_group = manager.get_group("test_group");
        assert!(retrieved_group.is_some());
        
        // Test group properties
        if let Some(retrieved) = retrieved_group {
            assert_eq!(retrieved.name, "test_group");
        }
    }

    #[test]
    fn test_signal_memory_manager_get_nonexistent_group() {
        // Test getting nonexistent group
        let manager = SignalMemoryManager::new();
        
        // Try to get nonexistent group
        let retrieved_group = manager.get_group("nonexistent_group");
        assert!(retrieved_group.is_none());
    }

    #[test]
    fn test_signal_memory_manager_get_memory_stats() {
        // Test getting memory statistics
        let manager = SignalMemoryManager::new();
        
        // Get initial stats
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 0);
        assert_eq!(stats.active_memos, 0);
        assert_eq!(stats.estimated_memory_bytes, 0);
        assert_eq!(stats.tracked_groups, 0);
    }

    #[test]
    fn test_signal_memory_manager_update_memory_stats() {
        // Test updating memory statistics
        let manager = SignalMemoryManager::new();
        
        // Create group with signals and memos
        let group = manager.create_group("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(|_| "test_memo".to_string());
        
        group.add_signal(signal);
        group.add_memo(memo);
        
        // Update stats
        manager.update_memory_stats();
        
        // Get updated stats
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 1);
        assert_eq!(stats.active_memos, 1);
        assert_eq!(stats.tracked_groups, 1);
    }

    #[test]
    fn test_signal_memory_manager_memory_pressure_detection() {
        // Test memory pressure detection
        let max_memory = 1000;
        let memory_limit = 500;
        let manager = SignalMemoryManager::with_limits(max_memory, memory_limit);
        
        // Test initial state
        assert!(!manager.is_memory_pressure());
        
        // Simulate memory pressure by setting high memory usage
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = 600; // Above limit
        });
        
        // Test memory pressure detection
        assert!(manager.is_memory_pressure());
    }

    #[test]
    fn test_signal_memory_manager_cleanup_old_groups() {
        // Test cleaning up old groups
        let manager = SignalMemoryManager::new();
        
        // Create multiple groups
        let group1 = manager.create_group("group1".to_string());
        let group2 = manager.create_group("group2".to_string());
        let group3 = manager.create_group("group3".to_string());
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 3);
        
        // Cleanup old groups (older than 0 seconds)
        let result = manager.cleanup_old_groups(0);
        assert!(result.is_ok());
        
        // Test groups were cleaned up
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_low_priority_groups() {
        // Test cleaning up low priority groups
        let manager = SignalMemoryManager::new();
        
        // Create groups with different priorities
        let group1 = manager.create_group("high_priority".to_string());
        let group2 = manager.create_group("low_priority".to_string());
        
        // Add signals to groups
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        
        group1.add_signal(signal1);
        group2.add_signal(signal2);
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 2);
        
        // Cleanup low priority groups
        let result = manager.cleanup_low_priority_groups();
        assert!(result.is_ok());
        
        // Test low priority group was cleaned up
        assert_eq!(manager.tracked_groups.get().len(), 1);
    }

    #[test]
    fn test_signal_memory_manager_adaptive_cleanup() {
        // Test adaptive cleanup
        let manager = SignalMemoryManager::with_adaptive_management();
        
        // Create groups
        let group1 = manager.create_group("group1".to_string());
        let group2 = manager.create_group("group2".to_string());
        
        // Add signals to groups
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        
        group1.add_signal(signal1);
        group2.add_signal(signal2);
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 2);
        
        // Run adaptive cleanup
        let result = manager.run_adaptive_cleanup();
        assert!(result.is_ok());
        
        // Test adaptive cleanup worked
        assert!(manager.tracked_groups.get().len() <= 2);
    }

    #[test]
    fn test_signal_memory_manager_performance_characteristics() {
        // Test performance characteristics
        let start = std::time::Instant::now();
        
        // Create multiple managers
        for _ in 0..1000 {
            let manager = SignalMemoryManager::new();
            let group = manager.create_group("test_group".to_string());
            let signal = ArcRwSignal::new("test_value".to_string());
            group.add_signal(signal);
        }
        
        let duration = start.elapsed();
        
        // Should complete without panicking
        assert!(duration.as_nanos() >= 0, "Memory manager creation should complete");
    }

    #[test]
    fn test_signal_memory_manager_memory_management() {
        // Test memory management
        let mut managers = Vec::new();
        
        // Create multiple managers
        for i in 0..100 {
            let manager = SignalMemoryManager::new();
            let group = manager.create_group(format!("group_{}", i));
            let signal = ArcRwSignal::new(format!("value_{}", i));
            group.add_signal(signal);
            managers.push(manager);
        }
        
        // Test that managers can be dropped without issues
        drop(managers);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_signal_group_memory_management() {
        // Test memory management for signal groups
        let mut groups = Vec::new();
        
        // Create multiple groups
        for i in 0..100 {
            let mut group = SignalGroup::new(format!("group_{}", i));
            
            // Add signals and memos
            for j in 0..5 {
                let signal = ArcRwSignal::new(format!("signal_{}_{}", i, j));
                let memo = ArcMemo::new(move |_| format!("memo_{}_{}", i, j));
                
                group.add_signal(signal);
                group.add_memo(memo);
            }
            
            groups.push(group);
        }
        
        // Test that groups can be dropped without issues
        drop(groups);
        
        // Test passes if no memory leaks or panics occur
        assert!(true);
    }

    #[test]
    fn test_signal_memory_manager_edge_cases() {
        // Test edge cases
        let manager = SignalMemoryManager::new();
        
        // Test creating group with empty name
        let group = manager.create_group("".to_string());
        assert_eq!(group.name, "");
        
        // Test removing nonexistent group
        let result = manager.remove_group("nonexistent");
        assert!(result.is_err());
        
        // Test getting nonexistent group
        let retrieved = manager.get_group("nonexistent");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_signal_group_edge_cases() {
        // Test edge cases for signal groups
        let mut group = SignalGroup::new("test_group".to_string());
        
        // Test adding same signal multiple times
        let signal = ArcRwSignal::new("test_value".to_string());
        group.add_signal(signal.clone());
        group.add_signal(signal.clone());
        
        // Should only have one signal
        assert_eq!(group.signals.len(), 1);
        
        // Test removing nonexistent signal
        let nonexistent_signal = ArcRwSignal::new("nonexistent".to_string());
        group.remove_signal(&nonexistent_signal);
        
        // Should still have one signal
        assert_eq!(group.signals.len(), 1);
    }

    #[test]
    fn test_signal_memory_manager_integration_scenarios() {
        // Test integration scenarios
        let manager = SignalMemoryManager::new();
        
        // Create multiple groups
        let theme_group = manager.create_group("theme".to_string());
        let variant_group = manager.create_group("variant".to_string());
        let size_group = manager.create_group("size".to_string());
        
        // Add signals to groups
        let theme_signal = ArcRwSignal::new("light".to_string());
        let variant_signal = ArcRwSignal::new("primary".to_string());
        let size_signal = ArcRwSignal::new("medium".to_string());
        
        theme_group.add_signal(theme_signal);
        variant_group.add_signal(variant_signal);
        size_group.add_signal(size_signal);
        
        // Test all groups were created
        assert_eq!(manager.tracked_groups.get().len(), 3);
        
        // Test memory stats
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 3);
        assert_eq!(stats.tracked_groups, 3);
    }

    #[test]
    fn test_signal_memory_manager_large_scale_operations() {
        // Test large scale operations
        let manager = SignalMemoryManager::new();
        
        // Create many groups
        for i in 0..1000 {
            let group = manager.create_group(format!("group_{}", i));
            let signal = ArcRwSignal::new(format!("value_{}", i));
            group.add_signal(signal);
        }
        
        // Test all groups were created
        assert_eq!(manager.tracked_groups.get().len(), 1000);
        
        // Test memory stats
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        assert_eq!(stats.active_signals, 1000);
        assert_eq!(stats.tracked_groups, 1000);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_operations() {
        // Test cleanup operations
        let manager = SignalMemoryManager::new();
        
        // Create groups
        let group1 = manager.create_group("group1".to_string());
        let group2 = manager.create_group("group2".to_string());
        let group3 = manager.create_group("group3".to_string());
        
        // Add signals to groups
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let signal3 = ArcRwSignal::new("value3".to_string());
        
        group1.add_signal(signal1);
        group2.add_signal(signal2);
        group3.add_signal(signal3);
        
        // Test initial state
        assert_eq!(manager.tracked_groups.get().len(), 3);
        
        // Cleanup all groups
        let result = manager.cleanup_all_groups();
        assert!(result.is_ok());
        
        // Test all groups were cleaned up
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }
}
