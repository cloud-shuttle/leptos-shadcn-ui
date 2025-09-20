#[cfg(test)]
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
}