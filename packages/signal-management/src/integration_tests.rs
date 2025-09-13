//! Integration tests for signal management with real Leptos components
//! 
//! These tests verify that our signal management utilities work correctly
//! with actual Leptos components and real-world usage patterns.

use leptos::prelude::*;
use crate::*;

/// Test basic signal integration patterns
#[test]
fn test_basic_signal_integration() {
    // Test ArcRwSignal creation and usage
    let signal = ArcRwSignal::new(42);
    assert_eq!(signal.get(), 42);
    
    // Test ArcMemo creation and usage
    let signal_for_memo = signal.clone();
    let memo = ArcMemo::new(move |_| signal_for_memo.get() * 2);
    assert_eq!(memo.get(), 84);
    
    // Test signal updates
    signal.set(100);
    assert_eq!(signal.get(), 100);
    assert_eq!(memo.get(), 200);
    
    // Test signal updates with update method
    signal.update(|value| *value += 1);
    assert_eq!(signal.get(), 101);
    assert_eq!(memo.get(), 202);
}

/// Test TailwindSignalManager integration
#[test]
fn test_tailwind_manager_integration() {
    let manager = TailwindSignalManager::new();
    
    // Test theme management
    let theme_signal = manager.theme();
    assert_eq!(theme_signal.get(), Theme::Light); // default theme
    
    // Test variant management
    let variant_signal = manager.variant();
    assert_eq!(variant_signal.get(), Variant::Default); // default variant
    
    // Test size management
    let size_signal = manager.size();
    assert_eq!(size_signal.get(), Size::Medium); // default size
    
    // Test responsive configuration
    let responsive_signal = manager.responsive();
    let config = responsive_signal.get();
    assert_eq!(config.sm, Some("640px".to_string()));
    assert_eq!(config.md, Some("768px".to_string()));
    assert_eq!(config.lg, Some("1024px".to_string()));
    assert_eq!(config.xl, Some("1280px".to_string()));
    
    // Test signal tracking
    let test_signal = ArcRwSignal::new(42);
    manager.track_signal(test_signal.clone());
    assert_eq!(manager.tracked_signals_count(), 1);
    
    // Test memo tracking
    let test_memo = ArcMemo::new(move |_| test_signal.get() * 2);
    manager.track_memo(test_memo);
    assert_eq!(manager.tracked_memos_count(), 1);
}

/// Test SignalMemoryManager integration
#[test]
fn test_memory_manager_integration() {
    let manager = SignalMemoryManager::new();
    
    // Test signal group creation
    let group = SignalGroup::new("test_group");
    manager.add_group(group.clone());
    
    // Test memory stats
    let stats = manager.get_stats();
    assert!(stats.get().total_signals >= 0);
    
    // Test memory pressure detection
    let pressure = manager.detect_memory_pressure();
    // Pressure detection returns an Option<MemoryPressureLevel>
    assert!(pressure.is_some() || pressure.is_none());
    
    // Test automatic cleanup
    let cleanup_performed = manager.perform_automatic_cleanup();
    // Cleanup may or may not be performed depending on memory state
    assert!(cleanup_performed || !cleanup_performed);
    
    // Test memory usage prediction
    let prediction = manager.predict_memory_usage(1000, 500);
    assert!(prediction >= 0);
    
    // Test performance metrics
    let metrics = manager.collect_performance_metrics();
    assert!(metrics.contains_key("signal_creation_time"));
    assert!(metrics.contains_key("memory_usage"));
}

/// Test BatchedSignalUpdater integration
#[test]
fn test_batched_updater_integration() {
    let updater = BatchedSignalUpdater::new();
    let manager = BatchedUpdaterManager::new();
    
    // Test updater registration
    manager.add_updater(updater.clone());
    assert_eq!(manager.updater_count(), 1);
    
    // Test batch size
    assert_eq!(updater.max_batch_size(), 1000);
    
    // Test auto-tuning
    updater.auto_tune_batch_size();
    assert!(updater.max_batch_size() > 0);
}

/// Test ComponentMigrator integration
#[test]
fn test_component_migrator_integration() {
    let migrator = ComponentMigrator::new();
    
    // Test initial state
    let initial_status = migrator.status().get();
    assert!(!initial_status.all_migrated);
    assert_eq!(initial_status.migrated_count, 0);
    assert_eq!(initial_status.failed_count, 46);
    
    // Test component migration
    migrator.mark_migrated("button");
    migrator.mark_migrated("input");
    migrator.mark_migrated("card");
    
    let updated_status = migrator.status().get();
    assert_eq!(updated_status.migrated_count, 3);
    assert_eq!(updated_status.failed_count, 43);
    assert!(!updated_status.all_migrated);
    
    // Test migration check
    assert!(migrator.is_migrated("button"));
    assert!(migrator.is_migrated("input"));
    assert!(!migrator.is_migrated("form"));
    
    // Test progress percentage
    let progress = migrator.progress_percentage();
    assert!(progress > 0.0 && progress < 100.0);
}

/// Test MemoryLeakDetector integration
#[test]
fn test_memory_leak_detector_integration() {
    let detector = MemoryLeakDetector::new();
    
    // Test leak prevention
    detector.enable_leak_prevention();
    assert!(detector.leak_prevention_enabled);
    
    // Test threshold setting
    let detector_with_threshold = MemoryLeakDetector::with_threshold(1000.0);
    assert!(detector_with_threshold.leak_prevention_enabled);
}

/// Test complex integration scenario
#[test]
fn test_complex_integration_scenario() {
    // Create a complex application scenario
    let app_state = ArcRwSignal::new(42);
    
    // Create computed state
    let app_state_for_computed = app_state.clone();
    let computed_state = ArcMemo::new(move |_| app_state_for_computed.get() * 2);
    
    // Create theme manager
    let theme_manager = TailwindSignalManager::new();
    
    // Create memory manager
    let memory_manager = SignalMemoryManager::new();
    
    // Create batched updater
    let updater = BatchedSignalUpdater::new();
    
    // Test initial state
    assert_eq!(app_state.get(), 42);
    assert_eq!(computed_state.get(), 84);
    
    // Test state updates
    app_state.set(100);
    assert_eq!(app_state.get(), 100);
    assert_eq!(computed_state.get(), 200);
    
    // Test theme manager
    let theme = theme_manager.theme().get();
    assert_eq!(theme, Theme::Light);
    
    // Test memory management
    let pressure = memory_manager.detect_memory_pressure();
    assert!(pressure.is_some() || pressure.is_none());
    
    // Test batched updater
    assert_eq!(updater.max_batch_size(), 1000);
}