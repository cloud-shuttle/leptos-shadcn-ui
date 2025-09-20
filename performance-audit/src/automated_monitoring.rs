//! Automated Performance Monitoring Module
//! 
//! This module provides automated performance monitoring with real-time metrics collection,
//! alerting, and optimization recommendations for leptos-shadcn-ui components.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::{interval, sleep};
use serde::{Deserialize, Serialize};
use crate::PerformanceAuditError;

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Data retention period
    pub retention_period: Duration,
    /// Enable real-time alerts
    pub enable_alerts: bool,
    /// Alert channels
    pub alert_channels: Vec<AlertChannel>,
}

/// Alert thresholds
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Performance degradation threshold (percentage)
    pub performance_degradation_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_usage_threshold: f64,
    /// Bundle size increase threshold (percentage)
    pub bundle_size_threshold: f64,
    /// Error rate threshold (percentage)
    pub error_rate_threshold: f64,
}

/// Alert channel types
#[derive(Debug, Clone)]
pub enum AlertChannel {
    /// Console output
    Console,
    /// File output
    File { path: String },
    /// Webhook notification
    Webhook { url: String },
    /// Email notification
    Email { recipients: Vec<String> },
}

/// Performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: u64,
    /// Component metrics
    pub component_metrics: HashMap<String, ComponentMetrics>,
    /// Overall system metrics
    pub system_metrics: SystemMetrics,
    /// Performance score
    pub overall_score: f64,
}

/// Component-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    /// Component name
    pub component_name: String,
    /// Render time in milliseconds
    pub render_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Bundle size in bytes
    pub bundle_size_bytes: u64,
    /// Error count
    pub error_count: u32,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Performance score (0.0 to 100.0)
    pub performance_score: f64,
}

/// System-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total memory usage in bytes
    pub total_memory_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Active connections
    pub active_connections: u32,
    /// Request rate (requests per second)
    pub request_rate: f64,
    /// Error rate (errors per second)
    pub error_rate: f64,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub alert_type: AlertType,
    /// Severity level
    pub severity: AlertSeverity,
    /// Component name (if applicable)
    pub component_name: Option<String>,
    /// Alert message
    pub message: String,
    /// Metrics that triggered the alert
    pub triggered_metrics: HashMap<String, f64>,
    /// Timestamp
    pub timestamp: u64,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// Performance degradation
    PerformanceDegradation,
    /// Memory usage spike
    MemoryUsageSpike,
    /// Bundle size increase
    BundleSizeIncrease,
    /// Error rate spike
    ErrorRateSpike,
    /// System resource exhaustion
    ResourceExhaustion,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    /// Component name
    pub component_name: String,
    /// Trend direction
    pub trend_direction: TrendDirection,
    /// Trend strength (0.0 to 1.0)
    pub trend_strength: f64,
    /// Predicted future value
    pub predicted_value: f64,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Time period analyzed
    pub time_period: Duration,
}

/// Trend directions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Improving performance
    Improving,
    /// Degrading performance
    Degrading,
    /// Stable performance
    Stable,
}

/// Automated performance monitor
pub struct AutomatedMonitor {
    config: MonitoringConfig,
    metrics_history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    alerts_history: Arc<RwLock<Vec<PerformanceAlert>>>,
    is_monitoring: Arc<RwLock<bool>>,
    baseline_metrics: Arc<RwLock<Option<PerformanceSnapshot>>>,
}

impl AutomatedMonitor {
    /// Create new automated monitor
    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            config,
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            alerts_history: Arc::new(RwLock::new(Vec::new())),
            is_monitoring: Arc::new(RwLock::new(false)),
            baseline_metrics: Arc::new(RwLock::new(None)),
        }
    }

    /// Start automated monitoring
    pub async fn start_monitoring(&self) -> Result<(), PerformanceAuditError> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if *is_monitoring {
            return Err(PerformanceAuditError::ConfigurationError(
                "Monitoring is already running".to_string()
            ));
        }
        *is_monitoring = true;
        drop(is_monitoring);

        println!("🚀 Starting automated performance monitoring...");
        println!("📊 Monitoring interval: {:?}", self.config.monitoring_interval);
        println!("🔔 Alerts enabled: {}", self.config.enable_alerts);

        // Start monitoring task
        let monitor_task = self.clone().monitoring_loop();
        tokio::spawn(monitor_task);

        Ok(())
    }

    /// Stop automated monitoring
    pub async fn stop_monitoring(&self) -> Result<(), PerformanceAuditError> {
        let mut is_monitoring = self.is_monitoring.write().await;
        *is_monitoring = false;
        drop(is_monitoring);

        println!("🛑 Stopping automated performance monitoring...");
        Ok(())
    }

    /// Monitoring loop
    async fn monitoring_loop(self) {
        let mut interval = interval(self.config.monitoring_interval);
        
        loop {
            interval.tick().await;
            
            let is_monitoring = *self.is_monitoring.read().await;
            if !is_monitoring {
                break;
            }

            // Collect performance metrics
            match self.collect_metrics().await {
                Ok(snapshot) => {
                    // Store metrics
                    {
                        let mut history = self.metrics_history.write().await;
                        history.push(snapshot.clone());
                        
                        // Cleanup old metrics
                        self.cleanup_old_metrics(&mut history).await;
                    }

                    // Analyze trends
                    if let Ok(trends) = self.analyze_trends().await {
                        for trend in trends {
                            println!("📈 Trend detected for {}: {:?} (strength: {:.2})", 
                                trend.component_name, trend.trend_direction, trend.trend_strength);
                        }
                    }

                    // Check for alerts
                    if self.config.enable_alerts {
                        if let Ok(alerts) = self.check_alerts(&snapshot).await {
                            for alert in alerts {
                                self.send_alert(&alert).await;
                            }
                        }
                    }

                    // Update baseline if needed
                    self.update_baseline(&snapshot).await;
                }
                Err(e) => {
                    eprintln!("❌ Failed to collect metrics: {}", e);
                }
            }
        }
    }

    /// Collect current performance metrics
    async fn collect_metrics(&self) -> Result<PerformanceSnapshot, PerformanceAuditError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_secs();

        // Collect component metrics
        let mut component_metrics = HashMap::new();
        
        // This would integrate with actual component monitoring
        // For now, we'll simulate metrics collection
        let components = vec!["button", "input", "select", "card", "badge"];
        
        for component in components {
            let metrics = ComponentMetrics {
                component_name: component.to_string(),
                render_time_ms: self.simulate_render_time(component).await,
                memory_usage_bytes: self.simulate_memory_usage(component).await,
                bundle_size_bytes: self.simulate_bundle_size(component).await,
                error_count: self.simulate_error_count(component).await,
                success_rate: self.simulate_success_rate(component).await,
                performance_score: 0.0, // Will be calculated
            };
            
            component_metrics.insert(component.to_string(), metrics);
        }

        // Calculate performance scores
        for (_, metrics) in component_metrics.iter_mut() {
            metrics.performance_score = self.calculate_performance_score(metrics);
        }

        // Collect system metrics
        let system_metrics = SystemMetrics {
            total_memory_bytes: self.get_system_memory_usage().await,
            cpu_usage_percent: self.get_cpu_usage().await,
            active_connections: self.get_active_connections().await,
            request_rate: self.get_request_rate().await,
            error_rate: self.get_error_rate().await,
        };

        // Calculate overall score
        let overall_score = self.calculate_overall_score(&component_metrics);

        Ok(PerformanceSnapshot {
            timestamp,
            component_metrics,
            system_metrics,
            overall_score,
        })
    }

    /// Analyze performance trends
    async fn analyze_trends(&self) -> Result<Vec<PerformanceTrend>, PerformanceAuditError> {
        let history = self.metrics_history.read().await;
        
        if history.len() < 10 {
            return Ok(Vec::new()); // Need at least 10 data points
        }

        let mut trends = Vec::new();
        let recent_snapshots = history.iter().rev().take(10).collect::<Vec<_>>();

        // Analyze trends for each component
        for component_name in ["button", "input", "select", "card", "badge"] {
            if let Some(trend) = self.analyze_component_trend(component_name, &recent_snapshots) {
                trends.push(trend);
            }
        }

        Ok(trends)
    }

    /// Analyze trend for a specific component
    fn analyze_component_trend(
        &self,
        component_name: &str,
        snapshots: &[&PerformanceSnapshot],
    ) -> Option<PerformanceTrend> {
        let mut render_times = Vec::new();
        
        for snapshot in snapshots {
            if let Some(metrics) = snapshot.component_metrics.get(component_name) {
                render_times.push(metrics.render_time_ms);
            }
        }

        if render_times.len() < 5 {
            return None;
        }

        // Simple linear regression to determine trend
        let n = render_times.len() as f64;
        let sum_x: f64 = (0..render_times.len()).map(|i| i as f64).sum();
        let sum_y: f64 = render_times.iter().sum();
        let sum_xy: f64 = render_times.iter().enumerate()
            .map(|(i, &y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..render_times.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let trend_strength = slope.abs().min(1.0);
        
        let trend_direction = if slope > 0.1 {
            TrendDirection::Degrading
        } else if slope < -0.1 {
            TrendDirection::Improving
        } else {
            TrendDirection::Stable
        };

        let predicted_value = if let Some(last_value) = render_times.last() {
            last_value + slope * 5.0 // Predict 5 steps ahead
        } else {
            0.0
        };

        Some(PerformanceTrend {
            component_name: component_name.to_string(),
            trend_direction,
            trend_strength,
            predicted_value,
            confidence: trend_strength,
            time_period: Duration::from_secs(10 * self.config.monitoring_interval.as_secs()),
        })
    }

    /// Check for performance alerts
    async fn check_alerts(
        &self,
        snapshot: &PerformanceSnapshot,
    ) -> Result<Vec<PerformanceAlert>, PerformanceAuditError> {
        let mut alerts = Vec::new();
        
        // Check component-level alerts
        for (component_name, metrics) in &snapshot.component_metrics {
            // Performance degradation alert
            if let Some(baseline) = self.baseline_metrics.read().await.as_ref() {
                if let Some(baseline_metrics) = baseline.component_metrics.get(component_name) {
                    let performance_change = (metrics.render_time_ms - baseline_metrics.render_time_ms) 
                        / baseline_metrics.render_time_ms * 100.0;
                    
                    if performance_change > self.config.alert_thresholds.performance_degradation_threshold {
                        alerts.push(PerformanceAlert {
                            alert_id: format!("perf-deg-{}-{}", component_name, snapshot.timestamp),
                            alert_type: AlertType::PerformanceDegradation,
                            severity: self.determine_alert_severity(performance_change),
                            component_name: Some(component_name.clone()),
                            message: format!(
                                "Performance degradation detected for {}: {:.1}% slower",
                                component_name, performance_change
                            ),
                            triggered_metrics: {
                                let mut map = HashMap::new();
                                map.insert("render_time_ms".to_string(), metrics.render_time_ms);
                                map.insert("performance_change_percent".to_string(), performance_change);
                                map
                            },
                            timestamp: snapshot.timestamp,
                            recommendations: self.generate_performance_recommendations(component_name, performance_change),
                        });
                    }
                }
            }

            // Memory usage alert
            if let Some(baseline) = self.baseline_metrics.read().await.as_ref() {
                if let Some(baseline_metrics) = baseline.component_metrics.get(component_name) {
                    let memory_change = (metrics.memory_usage_bytes as f64 - baseline_metrics.memory_usage_bytes as f64) 
                        / baseline_metrics.memory_usage_bytes as f64 * 100.0;
                    
                    if memory_change > self.config.alert_thresholds.memory_usage_threshold {
                        alerts.push(PerformanceAlert {
                            alert_id: format!("memory-{}-{}", component_name, snapshot.timestamp),
                            alert_type: AlertType::MemoryUsageSpike,
                            severity: self.determine_alert_severity(memory_change),
                            component_name: Some(component_name.clone()),
                            message: format!(
                                "Memory usage spike detected for {}: {:.1}% increase",
                                component_name, memory_change
                            ),
                            triggered_metrics: {
                                let mut map = HashMap::new();
                                map.insert("memory_usage_bytes".to_string(), metrics.memory_usage_bytes as f64);
                                map.insert("memory_change_percent".to_string(), memory_change);
                                map
                            },
                            timestamp: snapshot.timestamp,
                            recommendations: self.generate_memory_recommendations(component_name, memory_change),
                        });
                    }
                }
            }
        }

        // Store alerts
        if !alerts.is_empty() {
            let mut alerts_history = self.alerts_history.write().await;
            alerts_history.extend(alerts.clone());
        }

        Ok(alerts)
    }

    /// Send alert through configured channels
    async fn send_alert(&self, alert: &PerformanceAlert) {
        println!("🚨 ALERT [{}]: {}", alert.severity, alert.message);
        
        for channel in &self.config.alert_channels {
            match channel {
                AlertChannel::Console => {
                    println!("📢 Console Alert: {}", alert.message);
                }
                AlertChannel::File { path } => {
                    if let Err(e) = self.write_alert_to_file(alert, path).await {
                        eprintln!("❌ Failed to write alert to file: {}", e);
                    }
                }
                AlertChannel::Webhook { url } => {
                    if let Err(e) = self.send_webhook_alert(alert, url).await {
                        eprintln!("❌ Failed to send webhook alert: {}", e);
                    }
                }
                AlertChannel::Email { recipients } => {
                    if let Err(e) = self.send_email_alert(alert, recipients).await {
                        eprintln!("❌ Failed to send email alert: {}", e);
                    }
                }
            }
        }
    }

    /// Determine alert severity based on threshold
    fn determine_alert_severity(&self, change_percent: f64) -> AlertSeverity {
        if change_percent > 50.0 {
            AlertSeverity::Critical
        } else if change_percent > 25.0 {
            AlertSeverity::High
        } else if change_percent > 10.0 {
            AlertSeverity::Medium
        } else {
            AlertSeverity::Low
        }
    }

    /// Generate performance recommendations
    fn generate_performance_recommendations(&self, component_name: &str, degradation_percent: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        recommendations.push(format!(
            "Performance degradation of {:.1}% detected for {} component",
            degradation_percent, component_name
        ));
        
        if degradation_percent > 20.0 {
            recommendations.push("Consider optimizing component rendering logic".to_string());
            recommendations.push("Review component lifecycle and state management".to_string());
            recommendations.push("Check for unnecessary re-renders".to_string());
        }
        
        if degradation_percent > 10.0 {
            recommendations.push("Profile component performance with browser dev tools".to_string());
            recommendations.push("Consider implementing memoization".to_string());
        }
        
        recommendations
    }

    /// Generate memory recommendations
    fn generate_memory_recommendations(&self, component_name: &str, memory_increase_percent: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        recommendations.push(format!(
            "Memory usage increased by {:.1}% for {} component",
            memory_increase_percent, component_name
        ));
        
        if memory_increase_percent > 30.0 {
            recommendations.push("Check for memory leaks in component cleanup".to_string());
            recommendations.push("Review component state and signal management".to_string());
            recommendations.push("Consider implementing proper resource disposal".to_string());
        }
        
        if memory_increase_percent > 15.0 {
            recommendations.push("Profile memory usage with browser dev tools".to_string());
            recommendations.push("Review component lifecycle hooks".to_string());
        }
        
        recommendations
    }

    /// Update baseline metrics
    async fn update_baseline(&self, snapshot: &PerformanceSnapshot) {
        let mut baseline = self.baseline_metrics.write().await;
        *baseline = Some(snapshot.clone());
    }

    /// Cleanup old metrics based on retention period
    async fn cleanup_old_metrics(&self, history: &mut Vec<PerformanceSnapshot>) {
        let cutoff_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default() - self.config.retention_period;
        
        history.retain(|snapshot| snapshot.timestamp > cutoff_time.as_secs());
    }

    /// Calculate performance score for a component
    fn calculate_performance_score(&self, metrics: &ComponentMetrics) -> f64 {
        let render_score = if metrics.render_time_ms < 16.0 {
            100.0 // 60fps
        } else if metrics.render_time_ms < 33.0 {
            80.0 // 30fps
        } else if metrics.render_time_ms < 100.0 {
            60.0
        } else {
            40.0
        };
        
        let memory_score = if metrics.memory_usage_bytes < 1024 * 1024 {
            100.0 // < 1MB
        } else if metrics.memory_usage_bytes < 5 * 1024 * 1024 {
            80.0 // < 5MB
        } else if metrics.memory_usage_bytes < 10 * 1024 * 1024 {
            60.0 // < 10MB
        } else {
            40.0
        };
        
        let success_score = metrics.success_rate * 100.0;
        
        (render_score + memory_score + success_score) / 3.0
    }

    /// Calculate overall performance score
    fn calculate_overall_score(&self, component_metrics: &HashMap<String, ComponentMetrics>) -> f64 {
        if component_metrics.is_empty() {
            return 0.0;
        }
        
        let total_score: f64 = component_metrics.values()
            .map(|metrics| metrics.performance_score)
            .sum();
        
        total_score / component_metrics.len() as f64
    }

    // Simulation methods (would be replaced with actual monitoring in production)
    
    async fn simulate_render_time(&self, _component: &str) -> f64 {
        // Simulate render time with some variation
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(10.0..50.0)
    }
    
    async fn simulate_memory_usage(&self, _component: &str) -> u64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(1024..10240) // 1KB to 10KB
    }
    
    async fn simulate_bundle_size(&self, _component: &str) -> u64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(1024..5120) // 1KB to 5KB
    }
    
    async fn simulate_error_count(&self, _component: &str) -> u32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0..5)
    }
    
    async fn simulate_success_rate(&self, _component: &str) -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.95..1.0)
    }
    
    async fn get_system_memory_usage(&self) -> u64 {
        // Would use actual system monitoring
        1024 * 1024 * 100 // 100MB
    }
    
    async fn get_cpu_usage(&self) -> f64 {
        // Would use actual system monitoring
        25.0 // 25%
    }
    
    async fn get_active_connections(&self) -> u32 {
        // Would use actual system monitoring
        10
    }
    
    async fn get_request_rate(&self) -> f64 {
        // Would use actual system monitoring
        5.0 // 5 requests/second
    }
    
    async fn get_error_rate(&self) -> f64 {
        // Would use actual system monitoring
        0.1 // 0.1 errors/second
    }
    
    async fn write_alert_to_file(&self, alert: &PerformanceAlert, path: &str) -> Result<(), std::io::Error> {
        let alert_json = serde_json::to_string_pretty(alert)?;
        std::fs::write(path, alert_json)?;
        Ok(())
    }
    
    async fn send_webhook_alert(&self, alert: &PerformanceAlert, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(alert)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("Webhook request failed with status: {}", response.status()).into());
        }
        
        Ok(())
    }
    
    async fn send_email_alert(&self, _alert: &PerformanceAlert, _recipients: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        // Would integrate with email service
        println!("📧 Email alert would be sent to: {:?}", _recipients);
        Ok(())
    }
}

impl Clone for AutomatedMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            metrics_history: Arc::clone(&self.metrics_history),
            alerts_history: Arc::clone(&self.alerts_history),
            is_monitoring: Arc::clone(&self.is_monitoring),
            baseline_metrics: Arc::clone(&self.baseline_metrics),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(30),
            alert_thresholds: AlertThresholds {
                performance_degradation_threshold: 10.0,
                memory_usage_threshold: 20.0,
                bundle_size_threshold: 15.0,
                error_rate_threshold: 5.0,
            },
            retention_period: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_alerts: true,
            alert_channels: vec![AlertChannel::Console],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_automated_monitor_creation() {
        let config = MonitoringConfig::default();
        let monitor = AutomatedMonitor::new(config);
        
        // Test that monitor is created successfully
        assert!(monitor.is_monitoring.read().await);
    }

    #[tokio::test]
    async fn test_alert_severity_determination() {
        let config = MonitoringConfig::default();
        let monitor = AutomatedMonitor::new(config);
        
        assert_eq!(monitor.determine_alert_severity(5.0), AlertSeverity::Low);
        assert_eq!(monitor.determine_alert_severity(15.0), AlertSeverity::Medium);
        assert_eq!(monitor.determine_alert_severity(30.0), AlertSeverity::High);
        assert_eq!(monitor.determine_alert_severity(60.0), AlertSeverity::Critical);
    }

    #[test]
    fn test_performance_score_calculation() {
        let config = MonitoringConfig::default();
        let monitor = AutomatedMonitor::new(config);
        
        let metrics = ComponentMetrics {
            component_name: "test".to_string(),
            render_time_ms: 10.0,
            memory_usage_bytes: 1024,
            bundle_size_bytes: 2048,
            error_count: 0,
            success_rate: 1.0,
            performance_score: 0.0,
        };
        
        let score = monitor.calculate_performance_score(&metrics);
        assert!(score > 90.0); // Should be high score for good metrics
    }
}
