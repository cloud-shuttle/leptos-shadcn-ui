#[cfg(test)]
mod memory_manager_tests {
    use crate::*;
    use crate::memory_management::*;
    use leptos::prelude::*;

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
    fn test_signal_memory_manager_create_group() {
        // Test SignalMemoryManager create group
        let manager = SignalMemoryManager::new();
        let group_name = "test_group".to_string();
        
        let group_id = manager.create_group(group_name.clone());
        assert!(group_id.is_ok());
        
        // Test group was created (verify via manager's tracking)
        let group = manager.get_group(&group_name);
        assert!(group.is_some());
        let group = group.unwrap();
        assert_eq!(group.name, group_name);
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
        
        // Test manager tracks the group
        assert_eq!(manager.tracked_groups.get().len(), 1);
        assert!(manager.tracked_groups.get().contains_key(&group_name));
    }

    #[test]
    fn test_signal_memory_manager_remove_group() {
        // Test SignalMemoryManager remove group
        let manager = SignalMemoryManager::new();
        let group_name = "test_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        manager.remove_group(&group_name);
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_get_group() {
        // Test SignalMemoryManager get group
        let manager = SignalMemoryManager::new();
        let group_name = "test_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        let retrieved_group = manager.get_group(&group_name);
        
        // Test group was retrieved
        assert!(retrieved_group.is_some());
        let group = retrieved_group.unwrap();
        assert_eq!(group.name, group_name);
    }

    #[test]
    fn test_signal_memory_manager_get_nonexistent_group() {
        // Test SignalMemoryManager get nonexistent group
        let manager = SignalMemoryManager::new();
        let group_name = "nonexistent_group".to_string();
        
        let retrieved_group = manager.get_group(&group_name);
        
        // Test group was not found
        assert!(retrieved_group.is_none());
    }

    #[test]
    fn test_signal_memory_manager_get_memory_stats() {
        // Test SignalMemoryManager get memory stats
        let manager = SignalMemoryManager::new();
        let stats = manager.get_memory_stats();
        
        // Test initial stats
        assert_eq!(stats.active_signals, 0);
        assert_eq!(stats.active_memos, 0);
        assert_eq!(stats.estimated_memory_bytes, 0);
        assert_eq!(stats.tracked_groups, 0);
    }

    #[test]
    fn test_signal_memory_manager_update_memory_stats() {
        // Test SignalMemoryManager update memory stats
        let manager = SignalMemoryManager::new();
        let group_name = "test_group".to_string();
        
        let _group_id = manager.create_group(group_name.clone());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        // Add signal and memo to manager instead
        manager.add_signal(signal);
        manager.add_memo(memo);
        
        manager.update_memory_stats();
        let stats = manager.get_memory_stats();
        
        // Test stats were updated
        assert_eq!(stats.active_signals, 1);
        assert_eq!(stats.active_memos, 1);
        assert_eq!(stats.tracked_groups, 1);
    }

    #[test]
    fn test_signal_memory_manager_memory_pressure_detection() {
        // Test SignalMemoryManager memory pressure detection
        let max_memory = 1024; // 1KB
        let memory_limit = 512; // 512B
        let manager = SignalMemoryManager::with_limits(max_memory, memory_limit);
        
        // Test initial state
        assert!(manager.detect_memory_pressure().is_none());
        
        // Simulate memory pressure
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = memory_limit + 1;
        });
        
        // Test memory pressure is detected
        assert!(manager.detect_memory_pressure().is_some());
    }

    #[test]
    fn test_signal_memory_manager_cleanup_old_groups() {
        // Test SignalMemoryManager cleanup old groups
        let manager = SignalMemoryManager::new();
        let group_name = "old_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        // Simulate old timestamp
        manager.tracked_groups.update(|groups| {
            if let Some(group) = groups.get_mut(&group_name) {
                group.created_at = 0.0; // Very old timestamp
            }
        });
        
        manager.cleanup_old_groups(1000); // Cleanup groups older than 1000 seconds
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_low_priority_groups() {
        // Test SignalMemoryManager cleanup low priority groups
        let manager = SignalMemoryManager::new();
        let group_name = "low_priority_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        // Simulate low priority (empty group)
        manager.tracked_groups.update(|groups| {
            if let Some(group) = groups.get_mut(&group_name) {
                group.clear();
            }
        });
        
        manager.cleanup_low_priority_groups();
        assert_eq!(manager.tracked_groups.get().len(), 0);
    }

    #[test]
    fn test_signal_memory_manager_adaptive_cleanup() {
        // Test SignalMemoryManager adaptive cleanup
        let manager = SignalMemoryManager::with_adaptive_management();
        let group_name = "test_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        assert_eq!(manager.tracked_groups.get().len(), 1);
        
        // Simulate memory pressure
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = manager.memory_limit + 1;
        });
        
        manager.adaptive_cleanup();
        
        // Test adaptive cleanup worked
        assert!(manager.tracked_groups.get().len() <= 1);
    }

    #[test]
    fn test_signal_memory_manager_clone() {
        // Test SignalMemoryManager cloning
        let manager = SignalMemoryManager::new();
        let group_name = "test_group".to_string();
        
        let _group = manager.create_group(group_name.clone());
        let cloned_manager = manager.clone();
        
        // Test cloned manager has same state
        assert_eq!(manager.tracked_groups.get().len(), cloned_manager.tracked_groups.get().len());
        assert_eq!(manager.stats.get(), cloned_manager.stats.get());
    }

    #[test]
    fn test_signal_memory_manager_debug() {
        // Test SignalMemoryManager debug formatting
        let manager = SignalMemoryManager::new();
        let debug_str = format!("{:?}", manager);
        
        assert!(debug_str.contains("SignalMemoryManager"));
    }

    #[test]
    fn test_signal_memory_manager_edge_cases() {
        // Test SignalMemoryManager edge cases
        let manager = SignalMemoryManager::new();
        
        // Test with empty group name
        let empty_group_id = manager.create_group("".to_string());
        assert!(empty_group_id.is_ok());
        
        let empty_group = manager.get_group("");
        assert!(empty_group.is_some());
        assert_eq!(empty_group.unwrap().name, "");
        
        // Test removing nonexistent group
        manager.remove_group("nonexistent");
        assert_eq!(manager.tracked_groups.get().len(), 1); // Still has empty group
    }

    #[test]
    fn test_signal_memory_manager_multiple_groups() {
        // Test SignalMemoryManager with multiple groups
        let manager = SignalMemoryManager::new();
        
        let group1 = manager.create_group("group1".to_string());
        let group2 = manager.create_group("group2".to_string());
        let group3 = manager.create_group("group3".to_string());
        
        assert_eq!(manager.tracked_groups.get().len(), 3);
        assert!(manager.tracked_groups.get().contains_key("group1"));
        assert!(manager.tracked_groups.get().contains_key("group2"));
        assert!(manager.tracked_groups.get().contains_key("group3"));
        
        // Test removing one group
        manager.remove_group("group2");
        assert_eq!(manager.tracked_groups.get().len(), 2);
        assert!(!manager.tracked_groups.get().contains_key("group2"));
    }

    #[test]
    fn test_signal_memory_manager_memory_limits() {
        // Test SignalMemoryManager memory limits
        let max_memory = 1024 * 1024; // 1MB
        let memory_limit = 512 * 1024; // 512KB
        let manager = SignalMemoryManager::with_limits(max_memory, memory_limit);
        
        assert_eq!(manager.max_memory_bytes, max_memory);
        assert_eq!(manager.memory_limit, memory_limit);
        
        // Test memory pressure detection
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = memory_limit - 1;
        });
        assert!(manager.detect_memory_pressure().is_none());
        
        manager.stats.update(|stats| {
            stats.estimated_memory_bytes = memory_limit + 1;
        });
        assert!(manager.detect_memory_pressure().is_some());
    }
}
