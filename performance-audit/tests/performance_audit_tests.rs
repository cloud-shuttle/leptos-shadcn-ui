//! Comprehensive Performance Audit Tests
//! 
//! This test suite implements TDD for the performance audit system.
//! All tests start as failing tests (Red Phase) and will be made to pass
//! in the Green Phase.

use leptos_shadcn_performance_audit::*;
use std::time::Duration;
// use std::collections::HashMap;

/// Test bundle size analysis functionality
#[tokio::test]
async fn test_bundle_size_analysis_comprehensive() {
    // This test will fail initially - we need to implement the functionality
    
    let mut bundle_results = bundle_analysis::BundleAnalysisResults::default();
    
    // Test adding components with various sizes
    let small_component = bundle_analysis::ComponentBundleAnalysis::new("button".to_string(), 2048); // 2KB
    let medium_component = bundle_analysis::ComponentBundleAnalysis::new("input".to_string(), 4096); // 4KB
    let large_component = bundle_analysis::ComponentBundleAnalysis::new("table".to_string(), 8192); // 8KB
    
    bundle_results.add_component(small_component);
    bundle_results.add_component(medium_component);
    bundle_results.add_component(large_component);
    
    // Verify bundle analysis results
    assert_eq!(bundle_results.component_analyses.len(), 3);
    assert_eq!(bundle_results.total_bundle_size_bytes, 14336); // 2KB + 4KB + 8KB
    assert_eq!(bundle_results.total_bundle_size_kb, 14.0);
    assert_eq!(bundle_results.average_component_size_kb, 14.0 / 3.0);
    assert_eq!(bundle_results.largest_component_size_kb, 8.0);
    assert_eq!(bundle_results.oversized_components.len(), 1);
    assert_eq!(bundle_results.oversized_components[0], "table");
    
    // Test performance scores
    let button_score = bundle_results.component_analyses["button"].performance_score();
    let table_score = bundle_results.component_analyses["table"].performance_score();
    assert!(button_score > table_score); // Smaller component should have better score
    
    // Test optimization recommendations
    let recommendations = bundle_results.get_optimization_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("table"));
}

/// Test performance monitoring functionality
#[tokio::test]
async fn test_performance_monitoring_comprehensive() {
    // This test will fail initially - we need to implement the functionality
    
    let config = performance_monitoring::PerformanceConfig::default();
    let mut monitor = performance_monitoring::PerformanceMonitor::new(config);
    
    // Start monitoring
    monitor.start_monitoring();
    assert!(monitor.is_monitoring());
    
    // Record various performance metrics
    monitor.record_render_time("button", Duration::from_millis(8));
    monitor.record_render_time("button", Duration::from_millis(12));
    monitor.record_memory_usage("button", 512 * 1024); // 512KB
    
    monitor.record_render_time("slow-component", Duration::from_millis(32));
    monitor.record_memory_usage("slow-component", 2 * 1024 * 1024); // 2MB
    
    // Stop monitoring and get results
    let results = monitor.stop_monitoring();
    assert!(!monitor.is_monitoring());
    
    // Verify monitoring results
    assert_eq!(results.component_metrics.len(), 2);
    
    let button_metrics = &results.component_metrics["button"];
    assert_eq!(button_metrics.average_render_time_ms, 10.0); // (8 + 12) / 2
    assert_eq!(button_metrics.max_render_time_ms, 12.0);
    assert_eq!(button_metrics.min_render_time_ms, 8.0);
    assert_eq!(button_metrics.rerender_count, 2);
    assert_eq!(button_metrics.memory_usage_bytes, 512 * 1024);
    assert!(button_metrics.meets_targets); // Good performance
    
    let slow_metrics = &results.component_metrics["slow-component"];
    assert_eq!(slow_metrics.average_render_time_ms, 32.0);
    assert_eq!(slow_metrics.memory_usage_bytes, 2 * 1024 * 1024);
    assert!(!slow_metrics.meets_targets); // Poor performance
    
    // Verify overall results
    assert_eq!(results.failing_components.len(), 1);
    assert_eq!(results.failing_components[0], "slow-component");
    assert!(!results.performance_bottlenecks.is_empty());
    
    // Test performance recommendations
    let recommendations = results.get_performance_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("slow-component"));
}

/// Test optimization roadmap generation
#[tokio::test]
async fn test_optimization_roadmap_generation() {
    // This test will fail initially - we need to implement the functionality
    
    // Create mock bundle analysis results
    let mut bundle_results = bundle_analysis::BundleAnalysisResults::default();
    let large_component = bundle_analysis::ComponentBundleAnalysis::new("table".to_string(), 8192); // 8KB
    bundle_results.add_component(large_component);
    
    // Create mock performance monitoring results
    let mut performance_results = performance_monitoring::PerformanceMonitoringResults::default();
    let mut slow_metrics = performance_monitoring::ComponentPerformanceMetrics::new("slow-button".to_string());
    slow_metrics.update_render_time(Duration::from_millis(32));
    performance_results.add_component_metrics(slow_metrics);
    
    // Generate optimization roadmap
    let roadmap = optimization_roadmap::OptimizationRoadmapGenerator::generate_roadmap(
        &bundle_results,
        &performance_results,
    );
    
    // Verify roadmap generation
    assert!(!roadmap.recommendations.is_empty());
    assert!(roadmap.total_estimated_effort_hours > 0.0);
    assert!(roadmap.overall_expected_impact > 0.0);
    
    // Test priority-based recommendations
    let high_priority = roadmap.get_high_priority_recommendations();
    assert!(!high_priority.is_empty());
    
    // Test ROI-based recommendations
    let by_roi = roadmap.get_recommendations_by_roi();
    assert!(!by_roi.is_empty());
    
    // Test implementation plan generation
    let implementation_plan = roadmap.generate_implementation_plan();
    assert!(!implementation_plan.phases.is_empty());
    assert!(implementation_plan.total_effort_hours > 0.0);
    assert!(implementation_plan.total_expected_impact > 0.0);
}

/// Test benchmark functionality
#[tokio::test]
async fn test_benchmark_system_comprehensive() {
    // This test will fail initially - we need to implement the functionality
    
    let config = benchmarks::BenchmarkConfig::default();
    let mut runner = benchmarks::BenchmarkRunner::new(config);
    
    // Register mock benchmarks
    let fast_benchmark = Box::new(benchmarks::MockBenchmark {
        name: "fast-render".to_string(),
        component_name: "button".to_string(),
        execution_time: Duration::from_millis(8),
        memory_usage: 1024,
    });
    
    let slow_benchmark = Box::new(benchmarks::MockBenchmark {
        name: "slow-render".to_string(),
        component_name: "table".to_string(),
        execution_time: Duration::from_millis(32),
        memory_usage: 4096,
    });
    
    runner.register_benchmark(fast_benchmark);
    runner.register_benchmark(slow_benchmark);
    
    // Run all benchmarks
    let results = runner.run_all_benchmarks().await;
    
    // Verify benchmark results
    assert_eq!(results.benchmark_results.len(), 2);
    assert_eq!(results.failing_components.len(), 1);
    assert_eq!(results.failing_components[0], "table");
    
    // Test individual benchmark results
    let fast_result = &results.benchmark_results["fast-render"];
    assert_eq!(fast_result.average_time, Duration::from_millis(8));
    assert_eq!(fast_result.memory_usage_bytes, 1024);
    assert!(fast_result.meets_target);
    
    let slow_result = &results.benchmark_results["slow-render"];
    assert_eq!(slow_result.average_time, Duration::from_millis(32));
    assert_eq!(slow_result.memory_usage_bytes, 4096);
    assert!(!slow_result.meets_target);
    
    // Test performance recommendations
    let recommendations = results.get_performance_recommendations();
    assert!(!recommendations.is_empty());
    assert!(recommendations[0].contains("table"));
}

/// Test complete performance audit workflow
#[tokio::test]
async fn test_complete_performance_audit_workflow() {
    // This test will fail initially - we need to implement the functionality
    
    let config = PerformanceConfig::default();
    
    // Run complete performance audit
    let results = run_performance_audit(config).await.unwrap();
    
    // Verify audit results structure
    assert!(results.overall_score >= 0.0 && results.overall_score <= 100.0);
    assert!(results.bundle_analysis.overall_efficiency_score >= 0.0);
    assert!(results.performance_monitoring.overall_performance_score >= 0.0);
    assert!(!results.optimization_roadmap.recommendations.is_empty());
    
    // Test performance grade calculation
    let grade = results.get_grade();
    assert!(matches!(grade, 'A' | 'B' | 'C' | 'D' | 'F'));
    
    // Test targets meeting
    let meets_targets = results.meets_targets();
    assert!(meets_targets == (results.overall_score >= 80.0));
}

/// Test performance audit with real component data
#[tokio::test]
async fn test_performance_audit_with_real_components() {
    // This test will fail initially - we need to implement the functionality
    
    // Test with actual leptos-shadcn-ui components
    let component_names = vec![
        "button", "input", "card", "dialog", "table", "calendar", 
        "date-picker", "resizable", "toast", "tooltip"
    ];
    
    let mut bundle_results = bundle_analysis::BundleAnalysisResults::default();
    let mut performance_results = performance_monitoring::PerformanceMonitoringResults::default();
    
    // Simulate real component data
    for (i, component_name) in component_names.iter().enumerate() {
        // Bundle analysis - vary sizes
        let bundle_size = 1024 * (i + 1) as u64; // 1KB, 2KB, 3KB, etc.
        let component_analysis = bundle_analysis::ComponentBundleAnalysis::new(
            component_name.to_string(), 
            bundle_size
        );
        bundle_results.add_component(component_analysis);
        
        // Performance monitoring - vary performance
        let render_time = Duration::from_millis(5 + (i * 2) as u64); // 5ms, 7ms, 9ms, etc.
        let mut metrics = performance_monitoring::ComponentPerformanceMetrics::new(
            component_name.to_string()
        );
        metrics.update_render_time(render_time);
        metrics.update_memory_usage(512 * 1024 * (i + 1) as u64); // 512KB, 1MB, 1.5MB, etc.
        performance_results.add_component_metrics(metrics);
    }
    
    // Verify real component analysis
    assert_eq!(bundle_results.component_analyses.len(), 10);
    assert_eq!(performance_results.component_metrics.len(), 10);
    
    // Test optimization roadmap with real data
    let roadmap = optimization_roadmap::OptimizationRoadmapGenerator::generate_roadmap(
        &bundle_results,
        &performance_results,
    );
    
    assert!(!roadmap.recommendations.is_empty());
    assert!(roadmap.total_estimated_effort_hours > 0.0);
    
    // Test implementation plan
    let plan = roadmap.generate_implementation_plan();
    assert!(!plan.phases.is_empty());
    
    // Verify critical and high priority items exist
    let _critical_items = roadmap.get_recommendations_by_priority(
        optimization_roadmap::OptimizationPriority::Critical
    );
    let high_priority_items = roadmap.get_recommendations_by_priority(
        optimization_roadmap::OptimizationPriority::High
    );
    
    // Should have some high priority items based on our test data
    assert!(!high_priority_items.is_empty());
}

/// Test performance audit edge cases
#[tokio::test]
async fn test_performance_audit_edge_cases() {
    // This test will fail initially - we need to implement the functionality
    
    // Test with empty data
    let empty_bundle_results = bundle_analysis::BundleAnalysisResults::default();
    let empty_performance_results = performance_monitoring::PerformanceMonitoringResults::default();
    
    let empty_roadmap = optimization_roadmap::OptimizationRoadmapGenerator::generate_roadmap(
        &empty_bundle_results,
        &empty_performance_results,
    );
    
    // Should handle empty data gracefully
    assert!(empty_roadmap.recommendations.is_empty());
    assert_eq!(empty_roadmap.total_estimated_effort_hours, 0.0);
    
    // Test with extreme values
    let mut extreme_bundle_results = bundle_analysis::BundleAnalysisResults::default();
    let huge_component = bundle_analysis::ComponentBundleAnalysis::new(
        "huge-component".to_string(), 
        10 * 1024 * 1024 // 10MB
    );
    extreme_bundle_results.add_component(huge_component);
    
    let mut extreme_performance_results = performance_monitoring::PerformanceMonitoringResults::default();
    let mut extreme_metrics = performance_monitoring::ComponentPerformanceMetrics::new(
        "extreme-component".to_string()
    );
    extreme_metrics.update_render_time(Duration::from_secs(1)); // 1 second
    extreme_metrics.update_memory_usage(100 * 1024 * 1024); // 100MB
    extreme_performance_results.add_component_metrics(extreme_metrics);
    
    let extreme_roadmap = optimization_roadmap::OptimizationRoadmapGenerator::generate_roadmap(
        &extreme_bundle_results,
        &extreme_performance_results,
    );
    
    // Should handle extreme values and generate appropriate recommendations
    assert!(!extreme_roadmap.recommendations.is_empty());
    
    let high_priority = extreme_roadmap.get_high_priority_recommendations();
    assert!(!high_priority.is_empty()); // Should have high priority items for extreme cases
}

/// Test performance audit configuration
#[tokio::test]
async fn test_performance_audit_configuration() {
    // This test will fail initially - we need to implement the functionality
    
    // Test default configuration
    let default_config = PerformanceConfig::default();
    assert_eq!(default_config.max_component_size_kb, 5.0);
    assert_eq!(default_config.max_render_time_ms, 16.0);
    assert_eq!(default_config.max_memory_usage_mb, 1.0);
    assert!(default_config.monitoring_enabled);
    
    // Test custom configuration
    let custom_config = PerformanceConfig {
        max_component_size_kb: 10.0,
        max_render_time_ms: 32.0,
        max_memory_usage_mb: 2.0,
        monitoring_enabled: false,
    };
    
    assert_eq!(custom_config.max_component_size_kb, 10.0);
    assert_eq!(custom_config.max_render_time_ms, 32.0);
    assert_eq!(custom_config.max_memory_usage_mb, 2.0);
    assert!(!custom_config.monitoring_enabled);
    
    // Test benchmark configuration
    let benchmark_config = benchmarks::BenchmarkConfig::default();
    assert_eq!(benchmark_config.warmup_iterations, 10);
    assert_eq!(benchmark_config.benchmark_iterations, 100);
    assert_eq!(benchmark_config.target_time, Duration::from_millis(16));
    assert!(benchmark_config.enable_memory_profiling);
    assert!(benchmark_config.enable_statistical_analysis);
    
    // Test performance monitoring configuration
    let monitoring_config = performance_monitoring::PerformanceConfig::default();
    assert_eq!(monitoring_config.max_render_time_ms, 16.0);
    assert_eq!(monitoring_config.max_memory_usage_bytes, 1024 * 1024);
    assert_eq!(monitoring_config.monitoring_duration, Duration::from_secs(60));
    assert_eq!(monitoring_config.sample_rate, Duration::from_millis(100));
}
