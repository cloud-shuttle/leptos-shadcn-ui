use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use leptos_shadcn_performance_audit::benchmarks::ComponentBenchmarker;
use std::time::Duration;

/// Comprehensive component performance benchmarks
/// 
/// This benchmark suite tests the performance of all major components
/// to ensure they meet our performance targets:
/// - Render time: < 16ms (60fps)
/// - Memory usage: < 1MB per component
/// - Bundle size: < 5KB per component

fn benchmark_component_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_rendering");
    
    // Set measurement time to get stable results
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    
    // Test components with different complexity levels
    let components = vec![
        ("button", "simple"),
        ("input", "simple"),
        ("label", "simple"),
        ("checkbox", "medium"),
        ("switch", "medium"),
        ("radio_group", "medium"),
        ("textarea", "medium"),
        ("card", "medium"),
        ("dialog", "complex"),
        ("form", "complex"),
        ("select", "complex"),
        ("table", "complex"),
        ("calendar", "complex"),
        ("date_picker", "complex"),
    ];
    
    for (component_name, complexity) in components {
        group.bench_with_input(
            BenchmarkId::new("render", format!("{}_{}", component_name, complexity)),
            &component_name,
            |b, &component| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_component_render(component))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(500);
    
    let components = vec![
        "button", "input", "label", "checkbox", "switch", 
        "radio_group", "textarea", "card", "dialog", "form", 
        "select", "table", "calendar", "date_picker"
    ];
    
    for component in components {
        group.bench_with_input(
            BenchmarkId::new("memory", component),
            &component,
            |b, &component| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_memory_usage(component))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_bundle_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("bundle_size");
    
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(100);
    
    let components = vec![
        "button", "input", "label", "checkbox", "switch", 
        "radio_group", "textarea", "card", "dialog", "form", 
        "select", "table", "calendar", "date_picker"
    ];
    
    for component in components {
        group.bench_with_input(
            BenchmarkId::new("bundle", component),
            &component,
            |b, &component| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_bundle_size(component))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_state_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_management");
    
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(500);
    
    let state_operations = vec![
        ("signal_creation", "create"),
        ("signal_update", "update"),
        ("signal_read", "read"),
        ("callback_creation", "callback"),
        ("context_provision", "context"),
    ];
    
    for (operation, op_type) in state_operations {
        group.bench_with_input(
            BenchmarkId::new("state", format!("{}_{}", operation, op_type)),
            &operation,
            |b, &operation| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_state_operation(operation))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_accessibility_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("accessibility");
    
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(200);
    
    let a11y_features = vec![
        "aria_attributes", "keyboard_navigation", "focus_management", 
        "screen_reader_support", "color_contrast"
    ];
    
    for feature in a11y_features {
        group.bench_with_input(
            BenchmarkId::new("a11y", feature),
            &feature,
            |b, &feature| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_accessibility_feature(feature))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_theme_switching(c: &mut Criterion) {
    let mut group = c.benchmark_group("theme_switching");
    
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(200);
    
    let themes = vec!["default", "new_york", "dark", "light"];
    
    for theme in themes {
        group.bench_with_input(
            BenchmarkId::new("theme", theme),
            &theme,
            |b, &theme| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_theme_switch(theme))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_integration_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("integration");
    
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    
    let scenarios = vec![
        "form_with_validation", "dialog_with_form", "table_with_pagination",
        "calendar_with_date_picker", "select_with_search", "tabs_with_content"
    ];
    
    for scenario in scenarios {
        group.bench_with_input(
            BenchmarkId::new("integration", scenario),
            &scenario,
            |b, &scenario| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_integration_scenario(scenario))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_memory_leak_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_leaks");
    
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(50);
    
    let leak_tests = vec![
        "component_creation_destruction", "event_listener_cleanup", 
        "signal_cleanup", "context_cleanup", "long_running_component"
    ];
    
    for test in leak_tests {
        group.bench_with_input(
            BenchmarkId::new("leak", test),
            &test,
            |b, &test| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_memory_leak_test(test))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_performance_regression(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression");
    
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(1000);
    
    // Test performance regression scenarios
    let regression_tests = vec![
        "render_time_regression", "memory_usage_regression", 
        "bundle_size_regression", "state_update_regression"
    ];
    
    for test in regression_tests {
        group.bench_with_input(
            BenchmarkId::new("regression", test),
            &test,
            |b, &test| {
                let benchmarker = ComponentBenchmarker::new();
                b.iter(|| {
                    black_box(benchmarker.benchmark_regression_test(test))
                });
            },
        );
    }
    
    group.finish();
}

// Configure benchmark groups
criterion_group!(
    benches,
    benchmark_component_rendering,
    benchmark_memory_usage,
    benchmark_bundle_size,
    benchmark_state_management,
    benchmark_accessibility_features,
    benchmark_theme_switching,
    benchmark_integration_scenarios,
    benchmark_memory_leak_detection,
    benchmark_performance_regression
);

criterion_main!(benches);
