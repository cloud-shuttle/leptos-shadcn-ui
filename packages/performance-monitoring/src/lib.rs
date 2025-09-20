use leptos::prelude::*;
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
}