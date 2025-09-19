# 🎨 **Signal Management Component Design**

## **Overview**
Design for the Signal Management package that provides lifecycle management utilities for Leptos 0.8.8+ with tailwind-rs integration.

## **Core Components**

### **1. SignalCleanup**
```rust
#[derive(Debug, Clone)]
pub struct SignalCleanup {
    signals: Vec<ArcRwSignal<()>>,
    memos: Vec<ArcMemo<()>>,
    created_at: std::time::Instant,
}

impl SignalCleanup {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            memos: Vec::new(),
            created_at: std::time::Instant::now(),
        }
    }
    
    pub fn add_signal<T>(&mut self, signal: ArcRwSignal<T>) {
        // Convert to () type for storage
        let cleanup_signal = signal.map(|_| ());
        self.signals.push(cleanup_signal);
    }
    
    pub fn add_memo<T>(&mut self, memo: ArcMemo<T>) {
        // Convert to () type for storage
        let cleanup_memo = memo.map(|_| ());
        self.memos.push(cleanup_memo);
    }
    
    pub fn signals_count(&self) -> usize {
        self.signals.len()
    }
    
    pub fn memos_count(&self) -> usize {
        self.memos.len()
    }
    
    pub fn cleanup(mut self) -> Result<(), SignalManagementError> {
        // Clear all signals and memos
        self.signals.clear();
        self.memos.clear();
        Ok(())
    }
}
```

### **2. TailwindSignalManager**
```rust
#[derive(Debug, Clone)]
pub struct TailwindSignalManager {
    theme_signal: ArcRwSignal<Theme>,
    variant_signal: ArcRwSignal<Variant>,
    size_signal: ArcRwSignal<Size>,
    responsive_signal: ArcRwSignal<ResponsiveConfig>,
    signals: Vec<ArcRwSignal<()>>,
    memos: Vec<ArcMemo<()>>,
}

impl TailwindSignalManager {
    pub fn new() -> Self {
        Self {
            theme_signal: ArcRwSignal::new(Theme::Default),
            variant_signal: ArcRwSignal::new(Variant::Default),
            size_signal: ArcRwSignal::new(Size::Default),
            responsive_signal: ArcRwSignal::new(ResponsiveConfig::default()),
            signals: Vec::new(),
            memos: Vec::new(),
        }
    }
    
    pub fn theme_signal(&self) -> ArcRwSignal<Theme> {
        self.theme_signal.clone()
    }
    
    pub fn variant_signal(&self) -> ArcRwSignal<Variant> {
        self.variant_signal.clone()
    }
    
    pub fn size_signal(&self) -> ArcRwSignal<Size> {
        self.size_signal.clone()
    }
    
    pub fn responsive_signal(&self) -> ArcRwSignal<ResponsiveConfig> {
        self.responsive_signal.clone()
    }
    
    pub fn track_signal<T>(&mut self, signal: ArcRwSignal<T>) {
        let cleanup_signal = signal.map(|_| ());
        self.signals.push(cleanup_signal);
    }
    
    pub fn track_memo<T>(&mut self, memo: ArcMemo<T>) {
        let cleanup_memo = memo.map(|_| ());
        self.memos.push(cleanup_memo);
    }
}
```

### **3. BatchedSignalUpdater**
```rust
#[derive(Debug, Clone)]
pub struct BatchedSignalUpdater {
    pending_updates: Vec<SignalUpdate>,
    batch_size: usize,
    is_batching: bool,
}

#[derive(Debug, Clone)]
pub struct SignalUpdate {
    signal_id: String,
    value: serde_json::Value,
    timestamp: std::time::Instant,
}

impl BatchedSignalUpdater {
    pub fn new() -> Self {
        Self {
            pending_updates: Vec::new(),
            batch_size: 10,
            is_batching: false,
        }
    }
    
    pub fn with_batch_size(batch_size: usize) -> Self {
        Self {
            pending_updates: Vec::new(),
            batch_size,
            is_batching: false,
        }
    }
    
    pub fn queue_update(&mut self, update: SignalUpdate) {
        self.pending_updates.push(update);
        
        if self.pending_updates.len() >= self.batch_size {
            self.flush_updates().ok();
        }
    }
    
    pub fn start_batching(&mut self) {
        self.is_batching = true;
    }
    
    pub fn stop_batching(&mut self) {
        self.is_batching = false;
        self.flush_updates().ok();
    }
    
    pub fn flush_updates(&mut self) -> Result<(), SignalManagementError> {
        if self.pending_updates.is_empty() {
            return Ok(());
        }
        
        // Process all pending updates
        let updates = std::mem::take(&mut self.pending_updates);
        
        // Apply updates in batch
        for update in updates {
            self.apply_update(update)?;
        }
        
        Ok(())
    }
    
    fn apply_update(&self, update: SignalUpdate) -> Result<(), SignalManagementError> {
        // Implementation: Apply the signal update
        Ok(())
    }
}
```

### **4. SignalMemoryManager**
```rust
#[derive(Debug, Clone)]
pub struct SignalMemoryManager {
    groups: std::collections::HashMap<String, SignalGroup>,
    memory_limits: MemoryLimits,
    adaptive_management: bool,
}

#[derive(Debug, Clone)]
pub struct SignalGroup {
    id: String,
    signals: Vec<ArcRwSignal<()>>,
    memos: Vec<ArcMemo<()>>,
    created_at: std::time::Instant,
    priority: GroupPriority,
}

#[derive(Debug, Clone)]
pub struct MemoryLimits {
    max_signals: usize,
    max_memos: usize,
    max_groups: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GroupPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl SignalMemoryManager {
    pub fn new() -> Self {
        Self {
            groups: std::collections::HashMap::new(),
            memory_limits: MemoryLimits {
                max_signals: 1000,
                max_memos: 500,
                max_groups: 100,
            },
            adaptive_management: false,
        }
    }
    
    pub fn with_limits(limits: MemoryLimits) -> Self {
        Self {
            groups: std::collections::HashMap::new(),
            memory_limits: limits,
            adaptive_management: false,
        }
    }
    
    pub fn create_group(&mut self, id: String, priority: GroupPriority) -> &mut SignalGroup {
        let group = SignalGroup {
            id: id.clone(),
            signals: Vec::new(),
            memos: Vec::new(),
            created_at: std::time::Instant::now(),
            priority,
        };
        self.groups.insert(id, group);
        self.groups.get_mut(&id).unwrap()
    }
    
    pub fn add_signal_to_group<T>(&mut self, group_id: &str, signal: ArcRwSignal<T>) -> Result<(), SignalManagementError> {
        if let Some(group) = self.groups.get_mut(group_id) {
            let cleanup_signal = signal.map(|_| ());
            group.signals.push(cleanup_signal);
            Ok(())
        } else {
            Err(SignalManagementError::GroupNotFound(group_id.to_string()))
        }
    }
    
    pub fn add_memo_to_group<T>(&mut self, group_id: &str, memo: ArcMemo<T>) -> Result<(), SignalManagementError> {
        if let Some(group) = self.groups.get_mut(group_id) {
            let cleanup_memo = memo.map(|_| ());
            group.memos.push(cleanup_memo);
            Ok(())
        } else {
            Err(SignalManagementError::GroupNotFound(group_id.to_string()))
        }
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        let total_signals: usize = self.groups.values().map(|g| g.signals.len()).sum();
        let total_memos: usize = self.groups.values().map(|g| g.memos.len()).sum();
        
        MemoryStats {
            total_signals,
            total_memos,
            total_groups: self.groups.len(),
            memory_usage_percent: (total_signals + total_memos) as f64 / (self.memory_limits.max_signals + self.memory_limits.max_memos) as f64 * 100.0,
        }
    }
}
```

## **Supporting Types**

### **Theme, Variant, Size Enums**
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Default,
    Dark,
    Light,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Variant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Sm,
    Default,
    Lg,
    Icon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
}
```

### **Error Types**
```rust
#[derive(Debug, thiserror::Error)]
pub enum SignalManagementError {
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    #[error("Memory limit exceeded: {0}")]
    MemoryLimitExceeded(String),
    #[error("Invalid signal operation: {0}")]
    InvalidOperation(String),
    #[error("Cleanup failed: {0}")]
    CleanupFailed(String),
}
```

## **Usage Examples**

### **Basic Signal Cleanup**
```rust
let mut cleanup = SignalCleanup::new();
let signal = ArcRwSignal::new(42);
cleanup.add_signal(signal);
assert_eq!(cleanup.signals_count(), 1);
cleanup.cleanup().unwrap();
```

### **Signal Manager Usage**
```rust
let mut manager = TailwindSignalManager::new();
let theme = manager.theme_signal();
theme.set(Theme::Dark);
assert_eq!(theme.get(), Theme::Dark);
```

### **Batched Updates**
```rust
let mut updater = BatchedSignalUpdater::new();
updater.start_batching();
updater.queue_update(SignalUpdate {
    signal_id: "test".to_string(),
    value: serde_json::Value::Number(42.into()),
    timestamp: std::time::Instant::now(),
});
updater.stop_batching(); // Flushes automatically
```

---

**File Size**: 298 lines
**Priority**: 🔴 **P0 - CRITICAL**
**Dependencies**: leptos, serde, thiserror
