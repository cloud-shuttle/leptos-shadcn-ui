use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos::prelude::*;
use leptos_shadcn_signal_management::*;
use std::time::Duration;

/// Benchmark signal creation performance
fn benchmark_signal_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_creation");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark ArcRwSignal creation
    group.bench_function("arc_rw_signal_creation", |b| {
        b.iter(|| {
            let _signal = ArcRwSignal::new(black_box(42));
        });
    });
    
    // Benchmark ArcMemo creation
    group.bench_function("arc_memo_creation", |b| {
        let source = ArcRwSignal::new(42);
        b.iter(|| {
            let source_clone = source.clone();
            let _memo = ArcMemo::new(move |_| source_clone.get() * 2);
        });
    });
    
    // Benchmark regular Signal creation (for comparison)
    group.bench_function("regular_signal_creation", |b| {
        b.iter(|| {
            let _signal = signal(black_box(42));
        });
    });
    
    group.finish();
}

/// Benchmark signal access performance
fn benchmark_signal_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_access");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark ArcRwSignal get/set operations
    group.bench_function("arc_rw_signal_get_set", |b| {
        let signal = ArcRwSignal::new(42);
        b.iter(|| {
            let value = signal.get();
            signal.set(black_box(value + 1));
        });
    });
    
    // Benchmark ArcMemo access
    group.bench_function("arc_memo_access", |b| {
        let source = ArcRwSignal::new(42);
        let memo = ArcMemo::new(move |_| source.get() * 2);
        b.iter(|| {
            let _value = memo.get();
        });
    });
    
    // Benchmark regular Signal access (for comparison)
    group.bench_function("regular_signal_access", |b| {
        let (read, write) = signal(42);
        b.iter(|| {
            let value = read.get();
            write.set(black_box(value + 1));
        });
    });
    
    group.finish();
}

/// Benchmark TailwindSignalManager performance
fn benchmark_tailwind_manager(c: &mut Criterion) {
    let mut group = c.benchmark_group("tailwind_manager");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark manager creation
    group.bench_function("manager_creation", |b| {
        b.iter(|| {
            let _manager = TailwindSignalManager::new();
        });
    });
    
    // Benchmark theme access
    group.bench_function("theme_access", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let _theme = manager.theme().get();
        });
    });
    
    // Benchmark variant access
    group.bench_function("variant_access", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let _variant = manager.variant().get();
        });
    });
    
    // Benchmark size access
    group.bench_function("size_access", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let _size = manager.size().get();
        });
    });
    
    // Benchmark responsive config access
    group.bench_function("responsive_config_access", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let _config = manager.responsive().get();
        });
    });
    
    group.finish();
}

/// Benchmark memory management performance
fn benchmark_memory_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_management");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark signal group creation
    group.bench_function("signal_group_creation", |b| {
        b.iter(|| {
            let _group = SignalGroup::new("test_group".to_string());
        });
    });
    
    // Benchmark memory stats collection
    group.bench_function("memory_stats_collection", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _stats = manager.get_stats();
        });
    });
    
    // Benchmark memory pressure detection
    group.bench_function("memory_pressure_detection", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _pressure = manager.detect_memory_pressure();
        });
    });
    
    // Benchmark automatic cleanup
    group.bench_function("automatic_cleanup", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _cleanup = manager.perform_automatic_cleanup();
        });
    });
    
    // Benchmark memory usage prediction
    group.bench_function("memory_usage_prediction", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _prediction = manager.predict_memory_usage(black_box(1000), black_box(500));
        });
    });
    
    // Benchmark performance metrics
    group.bench_function("performance_metrics", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _metrics = manager.collect_performance_metrics();
        });
    });
    
    group.finish();
}

/// Benchmark batched updates performance
fn benchmark_batched_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("batched_updates");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark batched updater creation
    group.bench_function("batched_updater_creation", |b| {
        b.iter(|| {
            let _updater = BatchedSignalUpdater::new();
        });
    });
    
    // Benchmark batch size access
    group.bench_function("batch_size_access", |b| {
        let updater = BatchedSignalUpdater::new();
        b.iter(|| {
            let _size = updater.max_batch_size();
        });
    });
    
    // Benchmark auto-tuning
    group.bench_function("batch_size_auto_tuning", |b| {
        let mut updater = BatchedSignalUpdater::new();
        b.iter(|| {
            updater.auto_tune_batch_size();
        });
    });
    
    group.finish();
}

/// Benchmark component migration performance
fn benchmark_component_migration(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_migration");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark component migrator creation
    group.bench_function("migrator_creation", |b| {
        b.iter(|| {
            let _migrator = ComponentMigrator::new();
        });
    });
    
    // Benchmark migration validation
    group.bench_function("migration_validation", |b| {
        b.iter(|| {
            let _status = validate_all_component_migrations();
        });
    });
    
    // Benchmark individual component migrations
    group.bench_function("migrate_button", |b| {
        b.iter(|| {
            let _result = create_migrated_button_component();
        });
    });
    
    group.bench_function("migrate_input", |b| {
        b.iter(|| {
            let _result = create_migrated_input_component();
        });
    });
    
    group.bench_function("migrate_card", |b| {
        b.iter(|| {
            let _result = create_migrated_card_component();
        });
    });
    
    group.finish();
}

/// Benchmark signal lifecycle performance
fn benchmark_signal_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_lifecycle");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark signal tracking
    group.bench_function("signal_tracking", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let signal = ArcRwSignal::new(42);
            manager.track_signal(signal);
        });
    });
    
    // Benchmark memo tracking
    group.bench_function("memo_tracking", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            let signal = ArcRwSignal::new(42);
            let memo = ArcMemo::new(move |_| signal.get() * 2);
            manager.track_memo(memo);
        });
    });
    
    // Benchmark lifecycle optimization
    group.bench_function("lifecycle_optimization", |b| {
        let manager = TailwindSignalManager::new();
        b.iter(|| {
            manager.apply_lifecycle_optimization();
        });
    });
    
    group.finish();
}

/// Benchmark memory pressure scenarios
fn benchmark_memory_pressure(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pressure");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark memory pressure detection
    group.bench_function("memory_pressure_detection", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _pressure = manager.detect_memory_pressure();
        });
    });
    
    // Benchmark automatic cleanup
    group.bench_function("automatic_cleanup", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _cleanup = manager.perform_automatic_cleanup();
        });
    });
    
    // Benchmark memory usage prediction
    group.bench_function("memory_usage_prediction", |b| {
        let manager = SignalMemoryManager::new();
        b.iter(|| {
            let _prediction = manager.predict_memory_usage(black_box(1000), black_box(500));
        });
    });
    
    group.finish();
}

/// Benchmark signal deduplication
fn benchmark_signal_deduplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_deduplication");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark deduplication with varying signal counts
    for signal_count in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("deduplication", signal_count),
            signal_count,
            |b, &count| {
                let manager = SignalMemoryManager::new();
                // Create signals for deduplication
                let signals: Vec<_> = (0..count)
                    .map(|i| ArcRwSignal::new(i))
                    .collect();
                
                b.iter(|| {
                    let _deduplicated = manager.deduplicate_signals(signals.clone());
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_signal_creation,
    benchmark_signal_access,
    benchmark_tailwind_manager,
    benchmark_memory_management,
    benchmark_batched_updates,
    benchmark_component_migration,
    benchmark_signal_lifecycle,
    benchmark_memory_pressure,
    benchmark_signal_deduplication
);

criterion_main!(benches);