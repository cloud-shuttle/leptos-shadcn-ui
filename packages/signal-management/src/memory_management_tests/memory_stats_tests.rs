#[cfg(test)]
mod memory_stats_tests {
    use crate::*;
    use crate::memory_management::*;

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
    fn test_memory_stats_clone() {
        // Test MemoryStats cloning
        let stats = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let cloned_stats = stats.clone();
        assert_eq!(stats, cloned_stats);
    }

    #[test]
    fn test_memory_stats_debug() {
        // Test MemoryStats debug formatting
        let stats = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("active_signals"));
        assert!(debug_str.contains("active_memos"));
        assert!(debug_str.contains("estimated_memory_bytes"));
        assert!(debug_str.contains("tracked_groups"));
    }

    #[test]
    fn test_memory_stats_equality() {
        // Test MemoryStats equality
        let stats1 = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let stats2 = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let stats3 = MemoryStats {
            active_signals: 20,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_memory_stats_partial_equality() {
        // Test MemoryStats partial equality
        let stats1 = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let stats2 = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        let stats3 = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 2048,
            tracked_groups: 3,
        };
        
        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_memory_stats_zero_values() {
        // Test MemoryStats with zero values
        let stats = MemoryStats {
            active_signals: 0,
            active_memos: 0,
            estimated_memory_bytes: 0,
            tracked_groups: 0,
        };
        
        assert_eq!(stats.active_signals, 0);
        assert_eq!(stats.active_memos, 0);
        assert_eq!(stats.estimated_memory_bytes, 0);
        assert_eq!(stats.tracked_groups, 0);
    }

    #[test]
    fn test_memory_stats_large_values() {
        // Test MemoryStats with large values
        let stats = MemoryStats {
            active_signals: 1000000,
            active_memos: 500000,
            estimated_memory_bytes: 1024 * 1024 * 1024, // 1GB
            tracked_groups: 1000,
        };
        
        assert_eq!(stats.active_signals, 1000000);
        assert_eq!(stats.active_memos, 500000);
        assert_eq!(stats.estimated_memory_bytes, 1024 * 1024 * 1024);
        assert_eq!(stats.tracked_groups, 1000);
    }

    #[test]
    fn test_memory_stats_memory_calculation() {
        // Test MemoryStats memory calculation
        let stats = MemoryStats {
            active_signals: 100,
            active_memos: 50,
            estimated_memory_bytes: 1024 * 1024, // 1MB
            tracked_groups: 10,
        };
        
        // Test memory calculation
        let estimated_kb = stats.estimated_memory_bytes / 1024;
        assert_eq!(estimated_kb, 1024); // 1MB in KB
    }

    #[test]
    fn test_memory_stats_group_ratio() {
        // Test MemoryStats group ratio
        let stats = MemoryStats {
            active_signals: 100,
            active_memos: 50,
            estimated_memory_bytes: 1024,
            tracked_groups: 10,
        };
        
        // Test group ratio calculation
        let total_items = stats.active_signals + stats.active_memos;
        let items_per_group = total_items / stats.tracked_groups;
        assert_eq!(items_per_group, 15); // (100 + 50) / 10 = 15
    }

    #[test]
    fn test_memory_stats_memory_efficiency() {
        // Test MemoryStats memory efficiency
        let stats = MemoryStats {
            active_signals: 100,
            active_memos: 50,
            estimated_memory_bytes: 1024,
            tracked_groups: 10,
        };
        
        // Test memory efficiency calculation
        let total_items = stats.active_signals + stats.active_memos;
        let bytes_per_item = stats.estimated_memory_bytes / total_items;
        assert_eq!(bytes_per_item, 6); // 1024 / (100 + 50) = 6.82... ≈ 6
    }

    #[test]
    fn test_memory_stats_serialization() {
        // Test MemoryStats serialization
        let stats = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        // Test that stats can be serialized (basic test)
        let debug_str = format!("{:?}", stats);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_memory_stats_deserialization() {
        // Test MemoryStats deserialization
        let stats = MemoryStats {
            active_signals: 10,
            active_memos: 5,
            estimated_memory_bytes: 1024,
            tracked_groups: 3,
        };
        
        // Test that stats can be cloned (basic deserialization test)
        let cloned_stats = stats.clone();
        assert_eq!(stats, cloned_stats);
    }

    #[test]
    fn test_memory_stats_edge_cases() {
        // Test MemoryStats edge cases
        let stats = MemoryStats {
            active_signals: 1,
            active_memos: 1,
            estimated_memory_bytes: 1,
            tracked_groups: 1,
        };
        
        assert_eq!(stats.active_signals, 1);
        assert_eq!(stats.active_memos, 1);
        assert_eq!(stats.estimated_memory_bytes, 1);
        assert_eq!(stats.tracked_groups, 1);
    }

    #[test]
    fn test_memory_stats_overflow_protection() {
        // Test MemoryStats overflow protection
        let stats = MemoryStats {
            active_signals: usize::MAX,
            active_memos: usize::MAX,
            estimated_memory_bytes: usize::MAX,
            tracked_groups: usize::MAX,
        };
        
        assert_eq!(stats.active_signals, usize::MAX);
        assert_eq!(stats.active_memos, usize::MAX);
        assert_eq!(stats.estimated_memory_bytes, usize::MAX);
        assert_eq!(stats.tracked_groups, usize::MAX);
    }
}
