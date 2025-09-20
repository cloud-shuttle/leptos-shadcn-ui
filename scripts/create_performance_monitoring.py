#!/usr/bin/env python3
"""
Create continuous performance monitoring system
Includes real-time metrics collection, performance regression detection, and automated alerts
"""

import os
import json
import time
from datetime import datetime

def create_performance_monitor():
    """Create the main performance monitoring system"""
    content = '''use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub component_name: String,
    pub metric_type: String,
    pub value: f64,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    pub component_name: String,
    pub metric_type: String,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: String,
    pub component_name: String,
    pub metric_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: u64,
    pub resolved: bool,
}

pub struct PerformanceMonitor {
    metrics: Arc<Mutex<Vec<PerformanceMetric>>>,
    thresholds: Arc<Mutex<Vec<PerformanceThreshold>>>,
    alerts: Arc<Mutex<Vec<PerformanceAlert>>>,
    is_monitoring: Arc<Mutex<bool>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(Vec::new())),
            thresholds: Arc::new(Mutex::new(Vec::new())),
            alerts: Arc::new(Mutex::new(Vec::new())),
            is_monitoring: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_monitoring(&self) {
        *self.is_monitoring.lock().unwrap() = true;
        self.collect_system_metrics();
    }

    pub fn stop_monitoring(&self) {
        *self.is_monitoring.lock().unwrap() = false;
    }

    pub fn record_metric(&self, metric: PerformanceMetric) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.push(metric.clone());
        
        // Keep only last 1000 metrics to prevent memory issues
        if metrics.len() > 1000 {
            metrics.drain(0..100);
        }
        
        self.check_thresholds(&metric);
    }

    pub fn record_render_time(&self, component_name: &str, render_time: Duration) {
        let metric = PerformanceMetric {
            component_name: component_name.to_string(),
            metric_type: "render_time".to_string(),
            value: render_time.as_millis() as f64,
            timestamp: current_timestamp(),
            metadata: HashMap::new(),
        };
        self.record_metric(metric);
    }

    pub fn record_memory_usage(&self, component_name: &str, memory_kb: f64) {
        let metric = PerformanceMetric {
            component_name: component_name.to_string(),
            metric_type: "memory_usage".to_string(),
            value: memory_kb,
            timestamp: current_timestamp(),
            metadata: HashMap::new(),
        };
        self.record_metric(metric);
    }

    pub fn record_interaction_time(&self, component_name: &str, interaction_type: &str, duration: Duration) {
        let mut metadata = HashMap::new();
        metadata.insert("interaction_type".to_string(), interaction_type.to_string());
        
        let metric = PerformanceMetric {
            component_name: component_name.to_string(),
            metric_type: "interaction_time".to_string(),
            value: duration.as_millis() as f64,
            timestamp: current_timestamp(),
            metadata,
        };
        self.record_metric(metric);
    }

    pub fn set_threshold(&self, threshold: PerformanceThreshold) {
        let mut thresholds = self.thresholds.lock().unwrap();
        if let Some(existing) = thresholds.iter_mut().find(|t| 
            t.component_name == threshold.component_name && 
            t.metric_type == threshold.metric_type
        ) {
            *existing = threshold;
        } else {
            thresholds.push(threshold);
        }
    }

    fn check_thresholds(&self, metric: &PerformanceMetric) {
        let thresholds = self.thresholds.lock().unwrap();
        let mut alerts = self.alerts.lock().unwrap();
        
        for threshold in thresholds.iter() {
            if threshold.component_name == metric.component_name 
                && threshold.metric_type == metric.metric_type 
                && threshold.enabled {
                
                let severity = if metric.value >= threshold.critical_threshold {
                    "critical"
                } else if metric.value >= threshold.warning_threshold {
                    "warning"
                } else {
                    continue;
                };
                
                let alert = PerformanceAlert {
                    id: format!("{}_{}_{}", metric.component_name, metric.metric_type, current_timestamp()),
                    component_name: metric.component_name.clone(),
                    metric_type: metric.metric_type.clone(),
                    severity: severity.to_string(),
                    message: format!(
                        "{} {} exceeded {} threshold: {:.2} (threshold: {:.2})",
                        metric.component_name,
                        metric.metric_type,
                        severity,
                        metric.value,
                        if severity == "critical" { threshold.critical_threshold } else { threshold.warning_threshold }
                    ),
                    timestamp: current_timestamp(),
                    resolved: false,
                };
                
                alerts.push(alert);
            }
        }
    }

    fn collect_system_metrics(&self) {
        // This would be implemented to collect system-wide metrics
        // For now, it's a placeholder
    }

    pub fn get_metrics(&self, component_name: Option<&str>, metric_type: Option<&str>) -> Vec<PerformanceMetric> {
        let metrics = self.metrics.lock().unwrap();
        metrics.iter()
            .filter(|m| {
                component_name.map_or(true, |name| m.component_name == name) &&
                metric_type.map_or(true, |type_| m.metric_type == type_)
            })
            .cloned()
            .collect()
    }

    pub fn get_alerts(&self, unresolved_only: bool) -> Vec<PerformanceAlert> {
        let alerts = self.alerts.lock().unwrap();
        alerts.iter()
            .filter(|a| !unresolved_only || !a.resolved)
            .cloned()
            .collect()
    }

    pub fn resolve_alert(&self, alert_id: &str) {
        let mut alerts = self.alerts.lock().unwrap();
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
        }
    }

    pub fn get_performance_summary(&self) -> HashMap<String, f64> {
        let metrics = self.metrics.lock().unwrap();
        let mut summary = HashMap::new();
        
        // Calculate averages for each component and metric type
        let mut grouped: HashMap<(String, String), Vec<f64>> = HashMap::new();
        
        for metric in metrics.iter() {
            let key = (metric.component_name.clone(), metric.metric_type.clone());
            grouped.entry(key).or_insert_with(Vec::new).push(metric.value);
        }
        
        for ((component, metric_type), values) in grouped {
            let avg = values.iter().sum::<f64>() / values.len() as f64;
            let key = format!("{}_{}_avg", component, metric_type);
            summary.insert(key, avg);
        }
        
        summary
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// Global performance monitor instance
lazy_static::lazy_static! {
    pub static ref PERFORMANCE_MONITOR: PerformanceMonitor = PerformanceMonitor::new();
}

// Convenience macros for performance monitoring
#[macro_export]
macro_rules! monitor_render_time {
    ($component_name:expr, $render_fn:expr) => {{
        let start = std::time::Instant::now();
        let result = $render_fn;
        let duration = start.elapsed();
        crate::performance_monitor::PERFORMANCE_MONITOR.record_render_time($component_name, duration);
        result
    }};
}

#[macro_export]
macro_rules! monitor_interaction {
    ($component_name:expr, $interaction_type:expr, $interaction_fn:expr) => {{
        let start = std::time::Instant::now();
        let result = $interaction_fn;
        let duration = start.elapsed();
        crate::performance_monitor::PERFORMANCE_MONITOR.record_interaction_time($component_name, $interaction_type, duration);
        result
    }};
}'''
    
    os.makedirs("packages/performance-monitoring/src", exist_ok=True)
    with open("packages/performance-monitoring/src/lib.rs", "w") as f:
        f.write(content)
    
    # Create Cargo.toml for the performance monitoring package
    cargo_content = '''[package]
name = "leptos-shadcn-performance-monitoring"
version = "0.8.1"
edition = "2021"
description = "Performance monitoring system for Leptos ShadCN UI components"

[dependencies]
leptos = "0.8.9"
serde = { version = "1.0", features = ["derive"] }
lazy_static = "1.4"
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"

[lib]
crate-type = ["cdylib", "rlib"]'''
    
    with open("packages/performance-monitoring/Cargo.toml", "w") as f:
        f.write(cargo_content)
    
    print("✅ Created performance monitoring system")

def create_performance_dashboard():
    """Create a performance monitoring dashboard component"""
    content = '''#[cfg(test)]
mod performance_dashboard_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::performance_monitor::{PerformanceMonitor, PerformanceMetric, PerformanceThreshold, PerformanceAlert};
    use std::collections::HashMap;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_performance_monitoring_dashboard() {
        let monitor = PerformanceMonitor::new();
        let metrics = RwSignal::new(Vec::<PerformanceMetric>::new());
        let alerts = RwSignal::new(Vec::<PerformanceAlert>::new());
        let is_monitoring = RwSignal::new(false);

        // Set up some test thresholds
        monitor.set_threshold(PerformanceThreshold {
            component_name: "Button".to_string(),
            metric_type: "render_time".to_string(),
            warning_threshold: 10.0,
            critical_threshold: 50.0,
            enabled: true,
        });

        monitor.set_threshold(PerformanceThreshold {
            component_name: "Input".to_string(),
            metric_type: "memory_usage".to_string(),
            warning_threshold: 100.0,
            critical_threshold: 500.0,
            enabled: true,
        });

        mount_to_body(move || {
            view! {
                <div class="performance-dashboard">
                    <div class="dashboard-header">
                        <h1>"Performance Monitoring Dashboard"</h1>
                        <div class="controls">
                            <Button 
                                class=if is_monitoring.get() { "monitoring" } else { "" }
                                on_click=Callback::new(move || {
                                    if is_monitoring.get() {
                                        monitor.stop_monitoring();
                                        is_monitoring.set(false);
                                    } else {
                                        monitor.start_monitoring();
                                        is_monitoring.set(true);
                                    }
                                })
                            >
                                {if is_monitoring.get() { "Stop Monitoring" } else { "Start Monitoring" }}
                            </Button>
                            <Button 
                                on_click=Callback::new(move || {
                                    metrics.set(monitor.get_metrics(None, None));
                                    alerts.set(monitor.get_alerts(true));
                                })
                            >
                                "Refresh Data"
                            </Button>
                        </div>
                    </div>

                    <div class="dashboard-content">
                        <div class="metrics-section">
                            <h2>"Performance Metrics"</h2>
                            <div class="metrics-grid">
                                {for metrics.get().iter().map(|metric| {
                                    let metric = metric.clone();
                                    view! {
                                        <div class="metric-card">
                                            <div class="metric-header">
                                                <h3>{metric.component_name.clone()}</h3>
                                                <span class="metric-type">{metric.metric_type.clone()}</span>
                                            </div>
                                            <div class="metric-value">{format!("{:.2}", metric.value)}</div>
                                            <div class="metric-timestamp">
                                                {format!("{}", metric.timestamp)}
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>

                        <div class="alerts-section">
                            <h2>"Performance Alerts"</h2>
                            <div class="alerts-list">
                                {for alerts.get().iter().map(|alert| {
                                    let alert = alert.clone();
                                    view! {
                                        <div class="alert-item" class:critical=alert.severity == "critical" class:warning=alert.severity == "warning">
                                            <div class="alert-header">
                                                <span class="alert-severity">{alert.severity.clone()}</span>
                                                <span class="alert-component">{alert.component_name.clone()}</span>
                                            </div>
                                            <div class="alert-message">{alert.message.clone()}</div>
                                            <div class="alert-timestamp">
                                                {format!("{}", alert.timestamp)}
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>

                        <div class="summary-section">
                            <h2>"Performance Summary"</h2>
                            <div class="summary-stats">
                                {let summary = monitor.get_performance_summary();
                                for (key, value) in summary.iter() {
                                    view! {
                                        <div class="summary-item">
                                            <span class="summary-key">{key.clone()}</span>
                                            <span class="summary-value">{format!("{:.2}", value)}</span>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test monitoring controls
        let start_button = document.query_selector("button").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        if start_button.text_content().unwrap().contains("Start Monitoring") {
            start_button.click();
        }

        // Verify monitoring state
        let monitoring_button = document.query_selector(".monitoring").unwrap();
        assert!(monitoring_button.is_some(), "Monitoring button should show active state");

        // Test data refresh
        let refresh_button = document.query_selector_all("button").unwrap();
        for i in 0..refresh_button.length() {
            let button = refresh_button.item(i).unwrap().unchecked_into::<web_sys::HtmlButtonElement>();
            if button.text_content().unwrap().contains("Refresh Data") {
                button.click();
                break;
            }
        }

        // Verify dashboard sections
        let metrics_section = document.query_selector(".metrics-section").unwrap();
        assert!(metrics_section.is_some(), "Metrics section should be displayed");

        let alerts_section = document.query_selector(".alerts-section").unwrap();
        assert!(alerts_section.is_some(), "Alerts section should be displayed");

        let summary_section = document.query_selector(".summary-section").unwrap();
        assert!(summary_section.is_some(), "Summary section should be displayed");
    }

    #[wasm_bindgen_test]
    fn test_performance_metric_collection() {
        let monitor = PerformanceMonitor::new();
        
        // Record some test metrics
        monitor.record_render_time("Button", std::time::Duration::from_millis(15));
        monitor.record_memory_usage("Input", 150.0);
        monitor.record_interaction_time("Button", "click", std::time::Duration::from_millis(5));
        
        // Test metric retrieval
        let button_metrics = monitor.get_metrics(Some("Button"), None);
        assert!(button_metrics.len() >= 2, "Should have recorded Button metrics");
        
        let render_metrics = monitor.get_metrics(None, Some("render_time"));
        assert!(render_metrics.len() >= 1, "Should have recorded render time metrics");
        
        // Test performance summary
        let summary = monitor.get_performance_summary();
        assert!(!summary.is_empty(), "Performance summary should not be empty");
    }

    #[wasm_bindgen_test]
    fn test_performance_alerting() {
        let monitor = PerformanceMonitor::new();
        
        // Set up thresholds
        monitor.set_threshold(PerformanceThreshold {
            component_name: "TestComponent".to_string(),
            metric_type: "render_time".to_string(),
            warning_threshold: 10.0,
            critical_threshold: 50.0,
            enabled: true,
        });
        
        // Record metrics that should trigger alerts
        monitor.record_render_time("TestComponent", std::time::Duration::from_millis(15)); // Warning
        monitor.record_render_time("TestComponent", std::time::Duration::from_millis(60)); // Critical
        
        // Check alerts
        let alerts = monitor.get_alerts(false);
        assert!(alerts.len() >= 2, "Should have generated alerts");
        
        let critical_alerts = alerts.iter().filter(|a| a.severity == "critical").count();
        assert!(critical_alerts >= 1, "Should have critical alerts");
        
        let warning_alerts = alerts.iter().filter(|a| a.severity == "warning").count();
        assert!(warning_alerts >= 1, "Should have warning alerts");
        
        // Test alert resolution
        if let Some(alert) = alerts.first() {
            monitor.resolve_alert(&alert.id);
            let unresolved_alerts = monitor.get_alerts(true);
            assert!(unresolved_alerts.len() < alerts.len(), "Should have fewer unresolved alerts after resolution");
        }
    }
}'''
    
    with open("tests/performance/performance_dashboard_tests.rs", "w") as f:
        f.write(content)
    
    print("✅ Created performance monitoring dashboard")

def create_performance_regression_detector():
    """Create performance regression detection system"""
    content = '''use leptos::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub component_name: String,
    pub metric_type: String,
    pub baseline_value: f64,
    pub standard_deviation: f64,
    pub sample_size: usize,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAlert {
    pub id: String,
    pub component_name: String,
    pub metric_type: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub regression_percentage: f64,
    pub severity: String,
    pub timestamp: u64,
}

pub struct PerformanceRegressionDetector {
    baselines: HashMap<(String, String), PerformanceBaseline>,
    regression_threshold: f64, // Percentage threshold for regression detection
}

impl PerformanceRegressionDetector {
    pub fn new(regression_threshold: f64) -> Self {
        Self {
            baselines: HashMap::new(),
            regression_threshold,
        }
    }

    pub fn update_baseline(&mut self, component_name: &str, metric_type: &str, values: &[f64]) {
        if values.is_empty() {
            return;
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        let standard_deviation = variance.sqrt();

        let baseline = PerformanceBaseline {
            component_name: component_name.to_string(),
            metric_type: metric_type.to_string(),
            baseline_value: mean,
            standard_deviation,
            sample_size: values.len(),
            last_updated: current_timestamp(),
        };

        self.baselines.insert((component_name.to_string(), metric_type.to_string()), baseline);
    }

    pub fn check_for_regression(&self, component_name: &str, metric_type: &str, current_value: f64) -> Option<RegressionAlert> {
        let key = (component_name.to_string(), metric_type.to_string());
        
        if let Some(baseline) = self.baselines.get(&key) {
            let regression_percentage = ((current_value - baseline.baseline_value) / baseline.baseline_value) * 100.0;
            
            if regression_percentage > self.regression_threshold {
                let severity = if regression_percentage > self.regression_threshold * 2.0 {
                    "critical"
                } else {
                    "warning"
                };

                return Some(RegressionAlert {
                    id: format!("regression_{}_{}_{}", component_name, metric_type, current_timestamp()),
                    component_name: component_name.to_string(),
                    metric_type: metric_type.to_string(),
                    current_value,
                    baseline_value: baseline.baseline_value,
                    regression_percentage,
                    severity: severity.to_string(),
                    timestamp: current_timestamp(),
                });
            }
        }
        
        None
    }

    pub fn get_baseline(&self, component_name: &str, metric_type: &str) -> Option<&PerformanceBaseline> {
        let key = (component_name.to_string(), metric_type.to_string());
        self.baselines.get(&key)
    }

    pub fn get_all_baselines(&self) -> Vec<&PerformanceBaseline> {
        self.baselines.values().collect()
    }

    pub fn export_baselines(&self) -> String {
        serde_json::to_string_pretty(&self.baselines).unwrap_or_default()
    }

    pub fn import_baselines(&mut self, json_data: &str) -> Result<(), serde_json::Error> {
        let baselines: HashMap<(String, String), PerformanceBaseline> = serde_json::from_str(json_data)?;
        self.baselines.extend(baselines);
        Ok(())
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// Global regression detector instance
lazy_static::lazy_static! {
    pub static ref REGRESSION_DETECTOR: std::sync::Mutex<PerformanceRegressionDetector> = 
        std::sync::Mutex::new(PerformanceRegressionDetector::new(20.0)); // 20% regression threshold
}'''
    
    with open("packages/performance-monitoring/src/regression_detector.rs", "w") as f:
        f.write(content)
    
    print("✅ Created performance regression detector")

def create_continuous_monitoring_runner():
    """Create a continuous monitoring runner script"""
    content = '''#!/usr/bin/env python3
"""
Continuous Performance Monitoring Runner
Runs performance tests continuously and monitors for regressions
"""

import subprocess
import time
import json
import os
from datetime import datetime
import threading
import queue

class PerformanceMonitor:
    def __init__(self):
        self.monitoring = False
        self.results_queue = queue.Queue()
        self.baseline_file = "performance_baselines.json"
        self.results_file = "performance_results.json"
        self.regression_threshold = 20.0  # 20% regression threshold
        
    def load_baselines(self):
        """Load performance baselines from file"""
        if os.path.exists(self.baseline_file):
            with open(self.baseline_file, 'r') as f:
                return json.load(f)
        return {}
    
    def save_baselines(self, baselines):
        """Save performance baselines to file"""
        with open(self.baseline_file, 'w') as f:
            json.dump(baselines, f, indent=2)
    
    def load_results(self):
        """Load performance results from file"""
        if os.path.exists(self.results_file):
            with open(self.results_file, 'r') as f:
                return json.load(f)
        return []
    
    def save_results(self, results):
        """Save performance results to file"""
        with open(self.results_file, 'w') as f:
            json.dump(results, f, indent=2)
    
    def run_performance_tests(self):
        """Run performance tests and collect metrics"""
        print(f"🧪 Running performance tests at {datetime.now()}")
        
        try:
            result = subprocess.run([
                "cargo", "test", 
                "--test", "performance_tests",
                "--", "--nocapture"
            ], capture_output=True, text=True, timeout=300)
            
            if result.returncode == 0:
                # Parse performance metrics from test output
                metrics = self.parse_performance_metrics(result.stdout)
                return metrics
            else:
                print(f"❌ Performance tests failed: {result.stderr}")
                return {}
                
        except subprocess.TimeoutExpired:
            print("⏰ Performance tests timed out")
            return {}
        except Exception as e:
            print(f"❌ Error running performance tests: {e}")
            return {}
    
    def parse_performance_metrics(self, output):
        """Parse performance metrics from test output"""
        metrics = {}
        lines = output.split('\\n')
        
        for line in lines:
            if "Render time:" in line:
                # Extract render time metrics
                parts = line.split("Render time:")
                if len(parts) > 1:
                    time_part = parts[1].strip().split()[0]
                    try:
                        render_time = float(time_part.replace("ms", ""))
                        metrics["render_time"] = render_time
                    except ValueError:
                        pass
            
            elif "Memory usage:" in line:
                # Extract memory usage metrics
                parts = line.split("Memory usage:")
                if len(parts) > 1:
                    memory_part = parts[1].strip().split()[0]
                    try:
                        memory_usage = float(memory_part.replace("KB", ""))
                        metrics["memory_usage"] = memory_usage
                    except ValueError:
                        pass
        
        return metrics
    
    def check_for_regressions(self, current_metrics, baselines):
        """Check for performance regressions"""
        regressions = []
        
        for metric_name, current_value in current_metrics.items():
            if metric_name in baselines:
                baseline_value = baselines[metric_name]
                regression_percentage = ((current_value - baseline_value) / baseline_value) * 100
                
                if regression_percentage > self.regression_threshold:
                    regressions.append({
                        "metric": metric_name,
                        "current_value": current_value,
                        "baseline_value": baseline_value,
                        "regression_percentage": regression_percentage,
                        "severity": "critical" if regression_percentage > self.regression_threshold * 2 else "warning",
                        "timestamp": datetime.now().isoformat()
                    })
        
        return regressions
    
    def update_baselines(self, current_metrics, baselines):
        """Update baselines with current metrics"""
        for metric_name, current_value in current_metrics.items():
            if metric_name in baselines:
                # Update with weighted average (80% old, 20% new)
                baselines[metric_name] = baselines[metric_name] * 0.8 + current_value * 0.2
            else:
                baselines[metric_name] = current_value
        
        return baselines
    
    def send_alert(self, regression):
        """Send alert for performance regression"""
        print(f"🚨 PERFORMANCE REGRESSION DETECTED!")
        print(f"   Metric: {regression['metric']}")
        print(f"   Current: {regression['current_value']:.2f}")
        print(f"   Baseline: {regression['baseline_value']:.2f}")
        print(f"   Regression: {regression['regression_percentage']:.1f}%")
        print(f"   Severity: {regression['severity']}")
        print(f"   Time: {regression['timestamp']}")
        print("-" * 50)
    
    def monitoring_loop(self):
        """Main monitoring loop"""
        baselines = self.load_baselines()
        results = self.load_results()
        
        while self.monitoring:
            try:
                # Run performance tests
                current_metrics = self.run_performance_tests()
                
                if current_metrics:
                    # Check for regressions
                    regressions = self.check_for_regressions(current_metrics, baselines)
                    
                    # Send alerts for regressions
                    for regression in regressions:
                        self.send_alert(regression)
                    
                    # Update baselines
                    baselines = self.update_baselines(current_metrics, baselines)
                    
                    # Save results
                    result_entry = {
                        "timestamp": datetime.now().isoformat(),
                        "metrics": current_metrics,
                        "regressions": regressions
                    }
                    results.append(result_entry)
                    
                    # Keep only last 100 results
                    if len(results) > 100:
                        results = results[-100:]
                    
                    self.save_results(results)
                    self.save_baselines(baselines)
                
                # Wait before next iteration
                time.sleep(300)  # 5 minutes
                
            except KeyboardInterrupt:
                print("\\n🛑 Monitoring stopped by user")
                break
            except Exception as e:
                print(f"❌ Error in monitoring loop: {e}")
                time.sleep(60)  # Wait 1 minute before retrying
    
    def start_monitoring(self):
        """Start continuous monitoring"""
        print("🚀 Starting continuous performance monitoring...")
        print(f"📊 Regression threshold: {self.regression_threshold}%")
        print("⏰ Monitoring interval: 5 minutes")
        print("🛑 Press Ctrl+C to stop")
        print("=" * 50)
        
        self.monitoring = True
        self.monitoring_loop()
    
    def stop_monitoring(self):
        """Stop continuous monitoring"""
        self.monitoring = False

def main():
    """Main function"""
    monitor = PerformanceMonitor()
    
    try:
        monitor.start_monitoring()
    except KeyboardInterrupt:
        print("\\n🛑 Stopping monitoring...")
        monitor.stop_monitoring()
        print("✅ Monitoring stopped")

if __name__ == "__main__":
    main()
'''
    
    with open("scripts/continuous_performance_monitor.py", "w") as f:
        f.write(content)
    
    # Make it executable
    os.chmod("scripts/continuous_performance_monitor.py", 0o755)
    
    print("✅ Created continuous performance monitoring runner")

def main():
    """Create the complete performance monitoring system"""
    print("🚀 Creating Continuous Performance Monitoring System")
    print("=" * 60)
    
    # Create the monitoring system
    create_performance_monitor()
    create_performance_dashboard()
    create_performance_regression_detector()
    create_continuous_monitoring_runner()
    
    print("\\n🎉 Continuous Performance Monitoring System Created!")
    print("\\n📁 Created Files:")
    print("   - packages/performance-monitoring/src/lib.rs")
    print("   - packages/performance-monitoring/src/regression_detector.rs")
    print("   - packages/performance-monitoring/Cargo.toml")
    print("   - tests/performance/performance_dashboard_tests.rs")
    print("   - scripts/continuous_performance_monitor.py")
    
    print("\\n🚀 To start continuous monitoring:")
    print("   python3 scripts/continuous_performance_monitor.py")
    
    print("\\n📊 Features:")
    print("   - Real-time performance metric collection")
    print("   - Performance regression detection")
    print("   - Automated alerting system")
    print("   - Performance baseline management")
    print("   - Continuous monitoring with configurable intervals")

if __name__ == "__main__":
    main()
