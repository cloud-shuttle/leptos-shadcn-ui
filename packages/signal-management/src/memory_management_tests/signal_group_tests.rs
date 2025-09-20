#[cfg(test)]
mod signal_group_tests {
    use crate::*;
    use crate::memory_management::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_group_creation() {
        // Test SignalGroup creation
        let group = SignalGroup::new("test_group".to_string());
        
        // Test initial state
        assert_eq!(group.name, "test_group");
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
        assert!(group.created_at > 0.0);
    }

    #[test]
    fn test_signal_group_with_timestamp() {
        // Test SignalGroup with custom timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64;
        
        let group = SignalGroup::with_timestamp("test_group".to_string(), timestamp);
        
        // Test custom timestamp
        assert_eq!(group.name, "test_group");
        assert_eq!(group.created_at, timestamp);
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_add_signal() {
        // Test SignalGroup add signal
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        group.add_signal(signal.clone());
        
        // Test signal was added
        assert_eq!(group.signals.len(), 1);
        assert_eq!(group.signal_count(), 1);
    }

    #[test]
    fn test_signal_group_add_memo() {
        // Test SignalGroup add memo
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_memo(memo.clone());
        
        // Test memo was added
        assert_eq!(group.memos.len(), 1);
        assert_eq!(group.memo_count(), 1);
    }

    #[test]
    fn test_signal_group_remove_signal() {
        // Test SignalGroup remove signal
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        group.add_signal(signal.clone());
        assert_eq!(group.signals.len(), 1);
        
        group.remove_signal(0);
        assert_eq!(group.signals.len(), 0);
    }

    #[test]
    fn test_signal_group_remove_memo() {
        // Test SignalGroup remove memo
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_memo(memo.clone());
        assert_eq!(group.memos.len(), 1);
        
        group.remove_memo(0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_clear() {
        // Test SignalGroup clear
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_signal(signal);
        group.add_memo(memo);
        assert_eq!(group.signals.len(), 1);
        assert_eq!(group.memos.len(), 1);
        
        group.clear();
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_clone() {
        // Test SignalGroup cloning
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_signal(signal);
        group.add_memo(memo);
        
        let cloned_group = group.clone();
        assert_eq!(group.name, cloned_group.name);
        assert_eq!(group.created_at, cloned_group.created_at);
        assert_eq!(group.signals.len(), cloned_group.signals.len());
        assert_eq!(group.memos.len(), cloned_group.memos.len());
    }

    #[test]
    fn test_signal_group_debug() {
        // Test SignalGroup debug formatting
        let group = SignalGroup::new("test_group".to_string());
        let debug_str = format!("{:?}", group);
        
        assert!(debug_str.contains("test_group"));
        assert!(debug_str.contains("SignalGroup"));
    }

    #[test]
    fn test_signal_group_equality() {
        // Test SignalGroup equality
        let group1 = SignalGroup::new("test_group".to_string());
        let group2 = SignalGroup::new("test_group".to_string());
        let group3 = SignalGroup::new("different_group".to_string());
        
        assert_eq!(group1.name, group2.name);
        assert_ne!(group1.name, group3.name);
    }

    #[test]
    fn test_signal_group_multiple_signals() {
        // Test SignalGroup with multiple signals
        let mut group = SignalGroup::new("test_group".to_string());
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let signal3 = ArcRwSignal::new("value3".to_string());
        
        group.add_signal(signal1.clone());
        group.add_signal(signal2.clone());
        group.add_signal(signal3.clone());
        
        assert_eq!(group.signals.len(), 3);
        assert_eq!(group.signal_count(), 3);
    }

    #[test]
    fn test_signal_group_multiple_memos() {
        // Test SignalGroup with multiple memos
        let mut group = SignalGroup::new("test_group".to_string());
        let memo1 = ArcMemo::new(move |_| 10);
        let memo2 = ArcMemo::new(move |_| 20);
        let memo3 = ArcMemo::new(move |_| 30);
        
        group.add_memo(memo1.clone());
        group.add_memo(memo2.clone());
        group.add_memo(memo3.clone());
        
        assert_eq!(group.memos.len(), 3);
        assert_eq!(group.memo_count(), 3);
    }

    #[test]
    fn test_signal_group_mixed_content() {
        // Test SignalGroup with mixed signals and memos
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_signal(signal.clone());
        group.add_memo(memo.clone());
        
        assert_eq!(group.signals.len(), 1);
        assert_eq!(group.memos.len(), 1);
        assert_eq!(group.signal_count(), 1);
        assert_eq!(group.memo_count(), 1);
    }

    #[test]
    fn test_signal_group_duplicate_signals() {
        // Test SignalGroup with duplicate signals
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        group.add_signal(signal.clone());
        group.add_signal(signal.clone()); // Add same signal twice
        
        // Should still only have one signal (HashSet behavior)
        assert_eq!(group.signals.len(), 1);
    }

    #[test]
    fn test_signal_group_duplicate_memos() {
        // Test SignalGroup with duplicate memos
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        group.add_memo(memo.clone());
        group.add_memo(memo.clone()); // Add same memo twice
        
        // Should still only have one memo (HashSet behavior)
        assert_eq!(group.memos.len(), 1);
    }

    #[test]
    fn test_signal_group_remove_nonexistent_signal() {
        // Test SignalGroup remove nonexistent signal
        let mut group = SignalGroup::new("test_group".to_string());
        let signal = ArcRwSignal::new("test_value".to_string());
        
        // Try to remove signal that was never added
        group.remove_signal(0);
        
        // Should still have 0 signals
        assert_eq!(group.signals.len(), 0);
    }

    #[test]
    fn test_signal_group_remove_nonexistent_memo() {
        // Test SignalGroup remove nonexistent memo
        let mut group = SignalGroup::new("test_group".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        // Try to remove memo that was never added
        group.remove_memo(0);
        
        // Should still have 0 memos
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_edge_cases() {
        // Test SignalGroup edge cases
        let group = SignalGroup::new("".to_string()); // Empty name
        assert_eq!(group.name, "");
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }

    #[test]
    fn test_signal_group_memory_management() {
        // Test SignalGroup memory management
        let mut group = SignalGroup::new("test_group".to_string());
        
        // Add many signals and memos
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            let memo = ArcMemo::new(move |_| i);
            group.add_signal(signal);
            group.add_memo(memo);
        }
        
        assert_eq!(group.signals.len(), 1000);
        assert_eq!(group.memos.len(), 1000);
        
        // Clear and verify memory is freed
        group.clear();
        assert_eq!(group.signals.len(), 0);
        assert_eq!(group.memos.len(), 0);
    }
}
