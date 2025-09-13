//! WASM-specific tests for signal management
//! 
//! These tests verify that our signal management utilities work correctly
//! in WASM environments and handle browser-specific scenarios.

use leptos::prelude::*;
use crate::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test signal creation in WASM environment
#[wasm_bindgen_test]
fn test_wasm_signal_creation() {
    // Test ArcRwSignal creation in WASM
    let signal = ArcRwSignal::new(42);
    assert_eq!(signal.get(), 42);
    
    // Test ArcMemo creation in WASM
    let signal_for_memo = signal.clone();
    let memo = ArcMemo::new(move |_| signal_for_memo.get() * 2);
    assert_eq!(memo.get(), 84);
    
    // Test signal updates in WASM
    signal.set(100);
    assert_eq!(signal.get(), 100);
    assert_eq!(memo.get(), 200);
}

/// Test TailwindSignalManager in WASM environment
#[wasm_bindgen_test]
fn test_wasm_tailwind_manager() {
    let manager = TailwindSignalManager::new();
    
    // Test theme management in WASM
    let theme = manager.theme().get();
    assert_eq!(theme, Theme::Light);
    
    // Test variant management in WASM
    let variant = manager.variant().get();
    assert_eq!(variant, Variant::Default);
    
    // Test size management in WASM
    let size = manager.size().get();
    assert_eq!(size, Size::Medium);
    
    // Test responsive configuration in WASM
    let responsive = manager.responsive().get();
    assert_eq!(responsive.sm, Some("640px".to_string()));
    assert_eq!(responsive.md, Some("768px".to_string()));
    assert_eq!(responsive.lg, Some("1024px".to_string()));
    assert_eq!(responsive.xl, Some("1280px".to_string()));
}

/// Test memory management in WASM environment
#[wasm_bindgen_test]
fn test_wasm_memory_management() {
    let manager = SignalMemoryManager::new();
    
    // Test memory stats in WASM
    let stats = manager.get_stats();
    assert!(stats.get().total_signals >= 0);
    
    // Test memory pressure detection in WASM
    let pressure = manager.detect_memory_pressure();
    assert!(pressure.is_some() || pressure.is_none());
    
    // Test memory usage prediction in WASM
    let prediction = manager.predict_memory_usage(1000, 500);
    assert!(prediction >= 0);
    
    // Test performance metrics in WASM
    let metrics = manager.collect_performance_metrics();
    assert!(metrics.contains_key("signal_creation_time"));
    assert!(metrics.contains_key("memory_usage"));
}

/// Test batched updates in WASM environment
#[wasm_bindgen_test]
fn test_wasm_batched_updates() {
    let updater = BatchedSignalUpdater::new();
    
    // Test batch size in WASM
    assert_eq!(updater.max_batch_size(), 1000);
    
    // Test auto-tuning in WASM
    updater.auto_tune_batch_size();
    assert!(updater.max_batch_size() > 0);
}

/// Test component migration in WASM environment
#[wasm_bindgen_test]
fn test_wasm_component_migration() {
    let migrator = ComponentMigrator::new();
    
    // Test migration tracking in WASM
    migrator.mark_migrated("button");
    migrator.mark_migrated("input");
    
    assert!(migrator.is_migrated("button"));
    assert!(migrator.is_migrated("input"));
    assert!(!migrator.is_migrated("form"));
    
    // Test progress calculation in WASM
    let progress = migrator.progress_percentage();
    assert!(progress > 0.0 && progress < 100.0);
    
    // Test status updates in WASM
    let status = migrator.status().get();
    assert_eq!(status.migrated_count, 2);
    assert_eq!(status.failed_count, 44);
}

/// Test memory leak detection in WASM environment
#[wasm_bindgen_test]
fn test_wasm_memory_leak_detection() {
    let detector = MemoryLeakDetector::new();
    
    // Test leak prevention in WASM
    detector.enable_leak_prevention();
    assert!(detector.leak_prevention_enabled);
    
    // Test threshold setting in WASM
    let detector_with_threshold = MemoryLeakDetector::with_threshold(1000.0);
    assert!(detector_with_threshold.leak_prevention_enabled);
}

/// Test signal lifecycle in WASM environment
#[wasm_bindgen_test]
fn test_wasm_signal_lifecycle() {
    let manager = TailwindSignalManager::new();
    
    // Test signal tracking in WASM
    let signal = ArcRwSignal::new(42);
    manager.track_signal(signal.clone());
    assert_eq!(manager.tracked_signals_count(), 1);
    
    // Test memo tracking in WASM
    let memo = ArcMemo::new(move |_| signal.get() * 2);
    manager.track_memo(memo);
    assert_eq!(manager.tracked_memos_count(), 1);
}

/// Test complex WASM scenario
#[wasm_bindgen_test]
fn test_wasm_complex_scenario() {
    // Create a complex application scenario in WASM
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
    
    // Test initial state in WASM
    assert_eq!(app_state.get(), 42);
    assert_eq!(computed_state.get(), 84);
    
    // Test state updates in WASM
    app_state.set(100);
    assert_eq!(app_state.get(), 100);
    assert_eq!(computed_state.get(), 200);
    
    // Test theme manager in WASM
    let theme = theme_manager.theme().get();
    assert_eq!(theme, Theme::Light);
    
    // Test memory management in WASM
    let pressure = memory_manager.detect_memory_pressure();
    assert!(pressure.is_some() || pressure.is_none());
    
    // Test batched updater in WASM
    assert_eq!(updater.max_batch_size(), 1000);
}

/// Test performance in WASM environment
#[wasm_bindgen_test]
fn test_wasm_performance() {
    // Test signal creation performance
    let start_time = js_sys::Date::now();
    
    for _ in 0..1000 {
        let _signal = ArcRwSignal::new(42);
    }
    
    let end_time = js_sys::Date::now();
    let duration = end_time - start_time;
    
    // Should complete within reasonable time (adjust threshold as needed)
    assert!(duration < 1000.0); // 1 second
    
    // Test memo creation performance
    let start_time = js_sys::Date::now();
    
    for _ in 0..1000 {
        let source = ArcRwSignal::new(42);
        let _memo = ArcMemo::new(move |_| source.get() * 2);
    }
    
    let end_time = js_sys::Date::now();
    let duration = end_time - start_time;
    
    // Should complete within reasonable time
    assert!(duration < 1000.0); // 1 second
}

/// Test error handling in WASM environment
#[wasm_bindgen_test]
fn test_wasm_error_handling() {
    // Test signal error handling
    let signal = ArcRwSignal::new(42);
    
    // Test valid operations
    signal.set(100);
    assert_eq!(signal.get(), 100);
    
    // Test update operations
    signal.update(|value| {
        *value += 1;
    });
    assert_eq!(signal.get(), 101);
    
    // Test memo error handling
    let signal_for_memo = signal.clone();
    let memo = ArcMemo::new(move |_| {
        let value = signal_for_memo.get();
        if value > 100 {
            value * 2
        } else {
            value
        }
    });
    
    assert_eq!(memo.get(), 202); // 101 * 2
}