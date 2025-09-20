#[cfg(test)]
mod memory_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_signal_memory_manager_creation() {
        // Test SignalMemoryManager creation
        let manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.total_signals(), 0);
        assert_eq!(manager.total_memos(), 0);
        assert_eq!(manager.memory_usage_kb(), 0.0);
    }

    #[test]
    fn test_signal_memory_manager_default_implementation() {
        // Test SignalMemoryManager default implementation
        let manager = SignalMemoryManager::default();
        
        // Test default state
        assert_eq!(manager.total_signals(), 0);
        assert_eq!(manager.total_memos(), 0);
        assert_eq!(manager.memory_usage_kb(), 0.0);
    }

    #[test]
    fn test_signal_memory_manager_add_signal() {
        // Test adding signals to memory manager
        let mut manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.total_signals(), 0);
        
        // Add signals
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        
        manager.add_signal(signal1.clone());
        assert_eq!(manager.total_signals(), 1);
        
        manager.add_signal(signal2.clone());
        assert_eq!(manager.total_signals(), 2);
        
        // Test signals still work
        assert_eq!(signal1.get(), "value1");
        assert_eq!(signal2.get(), "value2");
    }

    #[test]
    fn test_signal_memory_manager_add_memo() {
        // Test adding memos to memory manager
        let mut manager = SignalMemoryManager::new();
        
        // Test initial state
        assert_eq!(manager.total_memos(), 0);
        
        // Add memos
        let signal = ArcRwSignal::new(42);
        let signal_clone1 = signal.clone();
        let signal_clone2 = signal.clone();
        let memo1 = ArcMemo::new(move |_| signal_clone1.get() * 2);
        let memo2 = ArcMemo::new(move |_| signal_clone2.get() * 3);
        
        manager.add_memo(memo1.clone());
        assert_eq!(manager.total_memos(), 1);
        
        manager.add_memo(memo2.clone());
        assert_eq!(manager.total_memos(), 2);
        
        // Test memos still work
        assert_eq!(memo1.get(), 84);
        assert_eq!(memo2.get(), 126);
    }

    #[test]
    fn test_signal_memory_manager_add_signal_to_group() {
        // Test adding signals to groups
        let mut manager = SignalMemoryManager::new();
        
        // Add signals to different groups
        let signal1 = ArcRwSignal::new("group1_signal1".to_string());
        let signal2 = ArcRwSignal::new("group1_signal2".to_string());
        let signal3 = ArcRwSignal::new("group2_signal1".to_string());
        
        manager.add_signal_to_group("group1", signal1.clone());
        manager.add_signal_to_group("group1", signal2.clone());
        manager.add_signal_to_group("group2", signal3.clone());
        
        // Test total signals
        assert_eq!(manager.total_signals(), 3);
        
        // Test signals still work
        assert_eq!(signal1.get(), "group1_signal1");
        assert_eq!(signal2.get(), "group1_signal2");
        assert_eq!(signal3.get(), "group2_signal1");
    }

    #[test]
    fn test_signal_memory_manager_add_memo_to_group() {
        // Test adding memos to groups
        let mut manager = SignalMemoryManager::new();
        
        // Add memos to different groups
        let signal1 = ArcRwSignal::new(10);
        let signal2 = ArcRwSignal::new(20);
        let signal3 = ArcRwSignal::new(30);
        
        let memo1 = ArcMemo::new(move |_| signal1.get() * 2);
        let memo2 = ArcMemo::new(move |_| signal2.get() * 3);
        let memo3 = ArcMemo::new(move |_| signal3.get() * 4);
        
        manager.add_memo_to_group("group1", memo1.clone());
        manager.add_memo_to_group("group1", memo2.clone());
        manager.add_memo_to_group("group2", memo3.clone());
        
        // Test total memos
        assert_eq!(manager.total_memos(), 3);
        
        // Test memos still work
        assert_eq!(memo1.get(), 20);
        assert_eq!(memo2.get(), 60);
        assert_eq!(memo3.get(), 120);
    }

    #[test]
    fn test_signal_memory_manager_memory_limits() {
        // Test memory limits
        let mut manager = SignalMemoryManager::new();
        
        // Test initial memory usage
        assert_eq!(manager.memory_usage_kb(), 0.0);
        
        // Add signals and test memory usage
        for i in 0..100 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            manager.add_signal(signal);
        }
        
        // Test memory usage increased
        assert!(manager.memory_usage_kb() > 0.0);
        
        // Test total signals
        assert_eq!(manager.total_signals(), 100);
    }

    #[test]
    fn test_signal_memory_manager_cleanup_group() {
        // Test cleaning up specific groups
        let mut manager = SignalMemoryManager::new();
        
        // Add signals to different groups
        let signal1 = ArcRwSignal::new("group1_signal1".to_string());
        let signal2 = ArcRwSignal::new("group1_signal2".to_string());
        let signal3 = ArcRwSignal::new("group2_signal1".to_string());
        
        manager.add_signal_to_group("group1", signal1.clone());
        manager.add_signal_to_group("group1", signal2.clone());
        manager.add_signal_to_group("group2", signal3.clone());
        
        // Test initial state
        assert_eq!(manager.total_signals(), 3);
        
        // Cleanup group1
        manager.cleanup_group("group1");
        
        // Test group1 signals are cleaned up
        assert_eq!(manager.total_signals(), 1);
        
        // Test group2 signal still works
        assert_eq!(signal3.get(), "group2_signal1");
    }

    #[test]
    fn test_signal_memory_manager_cleanup_all() {
        // Test cleaning up all signals and memos
        let mut manager = SignalMemoryManager::new();
        
        // Add signals and memos
        let signal1 = ArcRwSignal::new("value1".to_string());
        let signal2 = ArcRwSignal::new("value2".to_string());
        let memo = ArcMemo::new(move |_| 42);
        
        manager.add_signal(signal1.clone());
        manager.add_signal(signal2.clone());
        manager.add_memo(memo.clone());
        
        // Test initial state
        assert_eq!(manager.total_signals(), 2);
        assert_eq!(manager.total_memos(), 1);
        
        // Cleanup all
        manager.cleanup_all();
        
        // Test all are cleaned up
        assert_eq!(manager.total_signals(), 0);
        assert_eq!(manager.total_memos(), 0);
    }

    #[test]
    fn test_signal_memory_manager_clone_behavior() {
        // Test memory manager cloning behavior
        let mut manager1 = SignalMemoryManager::new();
        let signal = ArcRwSignal::new("test".to_string());
        manager1.add_signal(signal);
        
        // Test cloning
        let manager2 = manager1.clone();
        
        // Test both managers have same state
        assert_eq!(manager1.total_signals(), manager2.total_signals());
        assert_eq!(manager1.total_memos(), manager2.total_memos());
        assert_eq!(manager1.memory_usage_kb(), manager2.memory_usage_kb());
    }

    #[test]
    fn test_signal_memory_manager_debug_formatting() {
        // Test memory manager debug formatting
        let manager = SignalMemoryManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("SignalMemoryManager"));
    }

    #[test]
    fn test_signal_memory_manager_performance() {
        // Test memory manager performance
        let mut manager = SignalMemoryManager::new();
        
        // Test adding many signals
        let start = std::time::Instant::now();
        
        for i in 0..1000 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            manager.add_signal(signal);
        }
        
        let duration = start.elapsed();
        
        // Should be fast
        assert!(duration.as_millis() < 100);
        
        // Test final state
        assert_eq!(manager.total_signals(), 1000);
    }

    #[test]
    fn test_signal_memory_manager_memory_tracking() {
        // Test memory tracking accuracy
        let mut manager = SignalMemoryManager::new();
        
        // Test initial memory usage
        let initial_memory = manager.memory_usage_kb();
        assert_eq!(initial_memory, 0.0);
        
        // Add signals and track memory usage
        let mut memory_usage = Vec::new();
        
        for i in 0..100 {
            let signal = ArcRwSignal::new(format!("value_{}", i));
            manager.add_signal(signal);
            memory_usage.push(manager.memory_usage_kb());
        }
        
        // Test memory usage increases
        for i in 1..memory_usage.len() {
            assert!(memory_usage[i] >= memory_usage[i-1]);
        }
        
        // Test final memory usage
        assert!(manager.memory_usage_kb() > 0.0);
    }
}
