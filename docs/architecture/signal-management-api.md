# Signal Management API Documentation

## Overview

The `leptos-shadcn-signal-management` crate provides comprehensive utilities for managing Leptos 0.8.8 signals with advanced memory management, performance optimization, and component migration capabilities.

## Core Modules

### 1. Signal Lifecycle Management (`lifecycle`)

#### `TailwindSignalManager`

Central manager for Tailwind CSS signal lifecycle management.

```rust
use leptos_shadcn_signal_management::TailwindSignalManager;

let manager = TailwindSignalManager::new();
```

**Key Methods:**

- `theme() -> ArcRwSignal<Theme>` - Get theme signal (Light/Dark)
- `variant() -> ArcRwSignal<Variant>` - Get variant signal (Primary/Secondary/Destructive)
- `size() -> ArcRwSignal<Size>` - Get size signal (Small/Medium/Large)
- `responsive() -> ArcRwSignal<ResponsiveConfig>` - Get responsive configuration
- `track_signal<T>(signal: ArcRwSignal<T>)` - Track signal for lifecycle management
- `track_memo<T>(memo: ArcMemo<T>)` - Track memo for lifecycle management
- `tracked_signals_count() -> usize` - Get count of tracked signals
- `tracked_memos_count() -> usize` - Get count of tracked memos
- `apply_lifecycle_optimization()` - Apply lifecycle optimizations

**Example Usage:**

```rust
let manager = TailwindSignalManager::new();

// Track signals for lifecycle management
let button_state = ArcRwSignal::new(ButtonState::default());
manager.track_signal(button_state.clone());

// Track computed values
let button_class = ArcMemo::new(move |_| {
    format!("btn btn-{}", button_state.get().variant)
});
manager.track_memo(button_class);

// Apply optimizations
manager.apply_lifecycle_optimization();
```

#### `SignalCleanup`

Automatic cleanup utilities for signal lifecycle management.

```rust
use leptos_shadcn_signal_management::SignalCleanup;

let cleanup = SignalCleanup::new();
cleanup.cleanup_signals(&signals);
```

### 2. Memory Management (`memory_management`)

#### `SignalMemoryManager`

Advanced memory management for signal collections.

```rust
use leptos_shadcn_signal_management::SignalMemoryManager;

let manager = SignalMemoryManager::new();
```

**Key Methods:**

- `get_stats() -> ArcRwSignal<MemoryStats>` - Get memory statistics
- `detect_memory_pressure() -> Option<MemoryPressureLevel>` - Detect memory pressure
- `perform_automatic_cleanup() -> bool` - Perform automatic cleanup
- `predict_memory_usage(signal_count: usize, memo_count: usize) -> usize` - Predict memory usage
- `collect_performance_metrics() -> HashMap<String, f64>` - Collect performance metrics
- `deduplicate_signals<T>(signals: Vec<ArcRwSignal<T>>) -> Vec<ArcRwSignal<T>>` - Deduplicate signals
- `analyze_memory_fragmentation() -> f64` - Analyze memory fragmentation
- `enable_adaptive_management()` - Enable adaptive memory management

**Example Usage:**

```rust
let manager = SignalMemoryManager::new();

// Monitor memory pressure
if let Some(pressure) = manager.detect_memory_pressure() {
    if pressure > MemoryPressureLevel::High {
        manager.perform_automatic_cleanup();
    }
}

// Predict memory usage
let predicted_usage = manager.predict_memory_usage(1000, 500);
println!("Predicted memory usage: {} bytes", predicted_usage);

// Collect performance metrics
let metrics = manager.collect_performance_metrics();
println!("Signal creation time: {:?}", metrics.get("signal_creation_time"));
```

#### `SignalGroup`

Group signals for organized memory management.

```rust
use leptos_shadcn_signal_management::SignalGroup;

let group = SignalGroup::new("button_group".to_string());
```

#### `MemoryLeakDetector`

Detect and prevent memory leaks.

```rust
use leptos_shadcn_signal_management::MemoryLeakDetector;

let detector = MemoryLeakDetector::new();
detector.enable_leak_prevention();
```

### 3. Batched Updates (`batched_updates`)

#### `BatchedSignalUpdater`

Efficient batched signal updates for better performance.

```rust
use leptos_shadcn_signal_management::BatchedSignalUpdater;

let updater = BatchedSignalUpdater::new();
```

**Key Methods:**

- `max_batch_size() -> usize` - Get maximum batch size
- `auto_tune_batch_size()` - Auto-tune batch size for optimal performance

#### `BatchedUpdaterManager`

Manage multiple batched updaters.

```rust
use leptos_shadcn_signal_management::BatchedUpdaterManager;

let manager = BatchedUpdaterManager::new();
manager.add_updater(updater);
```

### 4. Component Migration (`component_migration`)

#### `ComponentMigrator`

Migrate existing components to new signal patterns.

```rust
use leptos_shadcn_signal_management::ComponentMigrator;

let migrator = ComponentMigrator::new();
```

**Key Methods:**

- `mark_migrated(component_name: &str)` - Mark component as migrated
- `is_migrated(component_name: &str) -> bool` - Check if component is migrated
- `status() -> ArcRwSignal<MigrationStatus>` - Get migration status
- `progress_percentage() -> f64` - Get migration progress percentage

**Example Usage:**

```rust
let migrator = ComponentMigrator::new();

// Mark components as migrated
migrator.mark_migrated("button");
migrator.mark_migrated("input");

// Check migration status
let status = migrator.status().get();
println!("Migrated: {}, Failed: {}", status.migrated_count, status.failed_count);

// Get progress
let progress = migrator.progress_percentage();
println!("Migration progress: {:.1}%", progress);
```

#### Migration Helper Functions

- `create_migrated_button_component() -> Option<()>` - Create migrated button component
- `create_migrated_input_component() -> Option<()>` - Create migrated input component
- `create_migrated_card_component() -> Option<()>` - Create migrated card component
- `validate_all_component_migrations() -> MigrationStatus` - Validate all migrations

## Data Types

### Enums

#### `Theme`
```rust
pub enum Theme {
    Light,
    Dark,
}
```

#### `Variant`
```rust
pub enum Variant {
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}
```

#### `Size`
```rust
pub enum Size {
    Small,
    Medium,
    Large,
}
```

#### `MemoryPressureLevel`
```rust
pub enum MemoryPressureLevel {
    Low,
    Medium,
    High,
    Critical,
}
```

### Structs

#### `ResponsiveConfig`
```rust
pub struct ResponsiveConfig {
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
}
```

#### `MemoryStats`
```rust
pub struct MemoryStats {
    pub total_signals: usize,
    pub total_memos: usize,
    pub memory_usage: usize,
    pub peak_memory_usage: usize,
    pub signal_creation_time: f64,
    pub memo_creation_time: f64,
}
```

#### `MigrationStatus`
```rust
pub struct MigrationStatus {
    pub all_migrated: bool,
    pub migrated_count: usize,
    pub failed_count: usize,
}
```

## Performance Considerations

### Signal Creation Performance
- **ArcRwSignal**: ~226ns (very fast)
- **ArcMemo**: ~336ns (fast)
- **Regular Signal**: ~294ns (fast)

### Signal Access Performance
- **ArcRwSignal get/set**: ~70ns (extremely fast)
- **ArcMemo access**: ~187ns (fast)
- **Regular Signal access**: ~120ns (fast)

### Memory Management
- Automatic cleanup when memory pressure is detected
- Signal deduplication to reduce memory usage
- Adaptive memory management for optimal performance

## Best Practices

### 1. Signal Lifecycle Management
```rust
// Always track signals for lifecycle management
let manager = TailwindSignalManager::new();
manager.track_signal(my_signal);

// Apply lifecycle optimizations
manager.apply_lifecycle_optimization();
```

### 2. Memory Management
```rust
// Monitor memory pressure
let manager = SignalMemoryManager::new();
if let Some(pressure) = manager.detect_memory_pressure() {
    if pressure > MemoryPressureLevel::High {
        manager.perform_automatic_cleanup();
    }
}
```

### 3. Component Migration
```rust
// Use migration utilities for systematic migration
let migrator = ComponentMigrator::new();
migrator.mark_migrated("component_name");

// Validate migration progress
let status = validate_all_component_migrations();
```

### 4. Performance Optimization
```rust
// Use batched updates for better performance
let updater = BatchedSignalUpdater::new();
updater.auto_tune_batch_size();
```

## Error Handling

The crate uses `SignalManagementError` for error handling:

```rust
pub enum SignalManagementError {
    MemoryLimitExceeded,
    InvalidSignal,
    MigrationFailed,
    CleanupFailed,
}
```

## Testing

The crate includes comprehensive tests:
- **42 total tests** covering all functionality
- **Performance benchmarks** with criterion
- **cargo nextest integration** for fast testing
- **WASM-specific tests** for browser environments

## Dependencies

- `leptos = "0.8"` - Core Leptos framework
- `serde = "1.0"` - Serialization support
- `chrono = "0.4"` - Date/time handling
- `js-sys = "0.3"` - WASM bindings
- `criterion = "0.5"` - Performance benchmarking
- `wasm-bindgen-test = "0.3"` - WASM testing
