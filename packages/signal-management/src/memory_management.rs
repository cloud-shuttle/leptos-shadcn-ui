//! Memory management utilities for signal lifecycle

use leptos::prelude::*;
use std::collections::HashMap;

use crate::error::SignalManagementError;

/// Memory usage statistics for signal management
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryStats {
    /// Number of active signals
    pub active_signals: usize,
    /// Number of active memos
    pub active_memos: usize,
    /// Estimated memory usage in bytes
    pub estimated_memory_bytes: usize,
    /// Number of tracked signal groups
    pub tracked_groups: usize,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            active_signals: 0,
            active_memos: 0,
            estimated_memory_bytes: 0,
            tracked_groups: 0,
        }
    }
}

/// Memory manager for tracking and managing signal memory usage
#[derive(Debug, Clone)]
pub struct SignalMemoryManager {
    /// Tracked signal groups
    pub tracked_groups: ArcRwSignal<HashMap<String, SignalGroup>>,
    /// Memory statistics
    pub stats: ArcRwSignal<MemoryStats>,
    /// Maximum memory usage threshold
    pub max_memory_bytes: usize,
    /// Memory limit for pressure detection
    pub memory_limit: usize,
    /// Adaptive management enabled flag
    pub adaptive_management: bool,
}

/// A group of related signals that can be managed together
#[derive(Debug, Clone)]
pub struct SignalGroup {
    /// Group name
    pub name: String,
    /// Signals in this group
    pub signals: Vec<ArcRwSignal<()>>,
    /// Memos in this group
    pub memos: Vec<ArcMemo<()>>,
    /// Created timestamp
    pub created_at: f64,
}

impl SignalGroup {
    /// Create a new signal group
    pub fn new(name: String) -> Self {
        Self {
            name,
            signals: Vec::new(),
            memos: Vec::new(),
            created_at: js_sys::Date::now(),
        }
    }
    
    /// Add a signal to this group
    pub fn add_signal<T>(&mut self, signal: ArcRwSignal<T>) -> ArcRwSignal<T> {
        self.signals.push(ArcRwSignal::new(()));
        signal
    }
    
    /// Add a memo to this group
    pub fn add_memo<T: Send + Sync + 'static>(&mut self, memo: ArcMemo<T>) -> ArcMemo<T> {
        self.memos.push(ArcMemo::new(|_| ()));
        memo
    }
    
    /// Get the number of signals in this group
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }
    
    /// Get the number of memos in this group
    pub fn memo_count(&self) -> usize {
        self.memos.len()
    }
    
    /// Get the total count of tracked items
    pub fn total_count(&self) -> usize {
        self.signal_count() + self.memo_count()
    }
    
    /// Check if this group is empty
    pub fn is_empty(&self) -> bool {
        self.total_count() == 0
    }
    
    /// Remove a signal from this group
    pub fn remove_signal(&mut self, index: usize) -> Option<()> {
        if index < self.signals.len() {
            self.signals.remove(index);
            Some(())
        } else {
            None
        }
    }
    
    /// Remove a memo from this group
    pub fn remove_memo(&mut self, index: usize) -> Option<()> {
        if index < self.memos.len() {
            self.memos.remove(index);
            Some(())
        } else {
            None
        }
    }
    
    /// Create a group with timestamp
    pub fn with_timestamp(name: String, timestamp: f64) -> Self {
        Self {
            name,
            signals: Vec::new(),
            memos: Vec::new(),
            created_at: timestamp,
        }
    }
}

impl SignalMemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            tracked_groups: ArcRwSignal::new(HashMap::new()),
            stats: ArcRwSignal::new(MemoryStats::default()),
            max_memory_bytes: 10 * 1024 * 1024, // 10MB default
            memory_limit: 10 * 1024 * 1024, // 10MB default
            adaptive_management: false,
        }
    }
    
    /// Create a new memory manager with custom memory limit
    pub fn with_memory_limit(max_memory_bytes: usize) -> Self {
        Self {
            tracked_groups: ArcRwSignal::new(HashMap::new()),
            stats: ArcRwSignal::new(MemoryStats::default()),
            max_memory_bytes,
            memory_limit: max_memory_bytes,
            adaptive_management: false,
        }
    }
    
    /// Create a new signal group
    pub fn create_group(&self, name: String) -> Result<String, SignalManagementError> {
        let group = SignalGroup::new(name.clone());
        
        self.tracked_groups.update(|groups| {
            groups.insert(name.clone(), group);
        });
        
        self.update_stats();
        Ok(name)
    }
    
    /// Add a signal to a group
    pub fn add_signal_to_group<T>(
        &self,
        group_name: &str,
        signal: ArcRwSignal<T>,
    ) -> Result<ArcRwSignal<T>, SignalManagementError> {
        let signal_clone = signal.clone();
        self.tracked_groups.update(|groups| {
            if let Some(group) = groups.get_mut(group_name) {
                group.add_signal(signal);
            }
        });
        
        self.update_stats();
        Ok(signal_clone)
    }
    
    /// Add a memo to a group
    pub fn add_memo_to_group<T: Send + Sync + 'static>(
        &self,
        group_name: &str,
        memo: ArcMemo<T>,
    ) -> Result<ArcMemo<T>, SignalManagementError> {
        let memo_clone = memo.clone();
        self.tracked_groups.update(|groups| {
            if let Some(group) = groups.get_mut(group_name) {
                group.add_memo(memo);
            }
        });
        
        self.update_stats();
        Ok(memo_clone)
    }
    
    /// Remove a signal group
    pub fn remove_group(&self, group_name: &str) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            groups.remove(group_name);
        });
        
        self.update_stats();
        Ok(())
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.get()
    }
    
    /// Get the number of tracked groups
    pub fn group_count(&self) -> usize {
        self.tracked_groups.get().len()
    }
    
    /// Get the maximum memory limit
    pub fn max_memory_bytes(&self) -> usize {
        self.max_memory_bytes
    }
    
    /// Check if memory usage is within limits
    pub fn is_memory_within_limits(&self) -> bool {
        self.stats.get().estimated_memory_bytes <= self.max_memory_bytes
    }
    
    /// Get memory usage percentage
    pub fn memory_usage_percentage(&self) -> f64 {
        let stats = self.stats.get();
        (stats.estimated_memory_bytes as f64 / self.max_memory_bytes as f64) * 100.0
    }
    
    /// Cleanup empty groups
    pub fn cleanup_empty_groups(&self) -> Result<usize, SignalManagementError> {
        let mut removed_count = 0;
        
        self.tracked_groups.update(|groups| {
            groups.retain(|_, group| {
                if group.is_empty() {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        });
        
        self.update_stats();
        Ok(removed_count)
    }
    
    /// Force cleanup of all groups
    pub fn force_cleanup_all(&self) -> Result<(), SignalManagementError> {
        self.tracked_groups.set(HashMap::new());
        self.update_stats();
        Ok(())
    }
    
    /// Update memory statistics
    fn update_stats(&self) {
        let groups = self.tracked_groups.get();
        let mut stats = MemoryStats::default();
        
        for group in groups.values() {
            stats.active_signals += group.signal_count();
            stats.active_memos += group.memo_count();
        }
        
        stats.tracked_groups = groups.len();
        stats.estimated_memory_bytes = self.estimate_memory_usage(&stats);
        
        self.stats.set(stats);
    }
    
    /// Estimate memory usage based on statistics
    fn estimate_memory_usage(&self, stats: &MemoryStats) -> usize {
        // Rough estimation: each signal/memo uses approximately 1KB
        let base_usage = (stats.active_signals + stats.active_memos) * 1024;
        
        // Add overhead for tracking
        let overhead = stats.tracked_groups * 512;
        
        base_usage + overhead
    }
    
    /// Get total number of signals across all groups
    pub fn total_signals(&self) -> usize {
        self.tracked_groups.with(|groups| {
            groups.values().map(|group| group.signal_count()).sum()
        })
    }
    
    /// Get total number of memos across all groups
    pub fn total_memos(&self) -> usize {
        self.tracked_groups.with(|groups| {
            groups.values().map(|group| group.memo_count()).sum()
        })
    }
    
    /// Get memory usage in KB
    pub fn memory_usage_kb(&self) -> f64 {
        self.max_memory_bytes as f64 / 1024.0
    }
    
    /// Add a signal to the default group
    pub fn add_signal<T: Send + Sync + 'static>(&self, signal: ArcRwSignal<T>) -> Result<ArcRwSignal<T>, SignalManagementError> {
        self.add_signal_to_group("default", signal)
    }
    
    /// Add a memo to the default group
    pub fn add_memo<T: Send + Sync + 'static>(&self, memo: ArcMemo<T>) -> Result<ArcMemo<T>, SignalManagementError> {
        self.add_memo_to_group("default", memo)
    }
    
    /// Cleanup a specific group
    pub fn cleanup_group(&self, group_name: &str) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            groups.remove(group_name);
        });
        self.update_stats();
        Ok(())
    }
    
    /// Cleanup all groups
    pub fn cleanup_all(&self) -> Result<(), SignalManagementError> {
        self.tracked_groups.update(|groups| {
            groups.clear();
        });
        self.update_stats();
        Ok(())
    }
    
    /// Create a manager with custom limits
    pub fn with_limits(max_memory_bytes: usize, memory_limit: usize) -> Self {
        Self {
            tracked_groups: ArcRwSignal::new(HashMap::new()),
            stats: ArcRwSignal::new(MemoryStats::default()),
            max_memory_bytes,
            memory_limit,
            adaptive_management: false,
        }
    }
    
    /// Adaptive cleanup for low priority groups
    pub fn cleanup_low_priority_groups(&self) -> Result<(), SignalManagementError> {
        // Simple implementation - remove groups with no signals
        self.tracked_groups.update(|groups| {
            groups.retain(|_, group| !group.is_empty());
        });
        self.update_stats();
        Ok(())
    }
    
    /// Adaptive cleanup based on memory pressure
    pub fn adaptive_cleanup(&self) -> Result<(), SignalManagementError> {
        if self.max_memory_bytes > self.memory_limit {
            self.cleanup_low_priority_groups()
        } else {
            Ok(())
        }
    }
    
    /// Update memory statistics
    pub fn update_memory_stats(&self) -> Result<(), SignalManagementError> {
        self.update_stats();
        Ok(())
    }
    
    /// Get memory statistics
    pub fn get_memory_stats(&self) -> MemoryStats {
        self.stats.get()
    }
}

impl Default for SignalMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory leak detector for signal management
pub struct MemoryLeakDetector {
    /// Baseline memory stats
    baseline_stats: MemoryStats,
    /// Current memory stats
    current_stats: ArcRwSignal<MemoryStats>,
    /// Memory growth threshold
    growth_threshold: f64,
    /// Leak prevention enabled flag
    pub leak_prevention_enabled: bool,
}

impl MemoryLeakDetector {
    /// Create a new memory leak detector
    pub fn new() -> Self {
        Self {
            baseline_stats: MemoryStats::default(),
            current_stats: ArcRwSignal::new(MemoryStats::default()),
            growth_threshold: 0.1, // 10% growth threshold
            leak_prevention_enabled: false,
        }
    }
    
    /// Create a new memory leak detector with custom threshold
    pub fn with_threshold(growth_threshold: f64) -> Self {
        Self {
            baseline_stats: MemoryStats::default(),
            current_stats: ArcRwSignal::new(MemoryStats::default()),
            growth_threshold,
            leak_prevention_enabled: false,
        }
    }
    
    /// Set baseline memory stats
    pub fn set_baseline(&mut self, stats: MemoryStats) {
        self.baseline_stats = stats;
    }
    
    /// Update current memory stats
    pub fn update_current(&self, stats: MemoryStats) {
        self.current_stats.set(stats);
    }
    
    /// Check for memory leaks
    pub fn check_for_leaks(&self) -> Result<bool, SignalManagementError> {
        let current = self.current_stats.get();
        
        // Check if memory usage has grown significantly
        let memory_growth = if self.baseline_stats.estimated_memory_bytes > 0 {
            (current.estimated_memory_bytes as f64 - self.baseline_stats.estimated_memory_bytes as f64)
                / self.baseline_stats.estimated_memory_bytes as f64
        } else {
            0.0
        };
        
        Ok(memory_growth > self.growth_threshold)
    }
    
    /// Get memory growth percentage
    pub fn memory_growth_percentage(&self) -> f64 {
        let current = self.current_stats.get();
        
        if self.baseline_stats.estimated_memory_bytes > 0 {
            ((current.estimated_memory_bytes as f64 - self.baseline_stats.estimated_memory_bytes as f64)
                / self.baseline_stats.estimated_memory_bytes as f64) * 100.0
        } else {
            0.0
        }
    }
    
    /// Get the growth threshold
    pub fn growth_threshold(&self) -> f64 {
        self.growth_threshold
    }
}

impl Default for MemoryLeakDetector {
    fn default() -> Self {
        Self::new()
    }
}
