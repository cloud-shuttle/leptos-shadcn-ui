#!/usr/bin/env rust-script
//! Performance Contract Monitoring and Alerting System
//! 
//! This binary provides real-time monitoring of performance contracts
//! and sends alerts when violations are detected.

use anyhow::Result;
use leptos_shadcn_contract_testing::{
    PerformanceContract,
    wasm_performance::WasmPerformanceTester,
    dependency_contracts::DependencyContractTester,
    ContractTestable,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub component: String,
    pub violation_type: ViolationType,
    pub current_value: f64,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    BundleSize,
    RenderTime,
    MemoryUsage,
    DependencyConflict,
    VersionMismatch,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationType::BundleSize => write!(f, "Bundle Size"),
            ViolationType::RenderTime => write!(f, "Render Time"),
            ViolationType::MemoryUsage => write!(f, "Memory Usage"),
            ViolationType::DependencyConflict => write!(f, "Dependency Conflict"),
            ViolationType::VersionMismatch => write!(f, "Version Mismatch"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct PerformanceMonitor {
    contracts: HashMap<String, PerformanceContract>,
    alerts: Vec<PerformanceAlert>,
    alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub bundle_size_warning: f64,    // KB
    pub bundle_size_critical: f64,   // KB
    pub render_time_warning: f64,    // ms
    pub render_time_critical: f64,   // ms
    pub memory_warning: f64,         // MB
    pub memory_critical: f64,        // MB
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            bundle_size_warning: 400.0,    // 400KB warning
            bundle_size_critical: 500.0,   // 500KB critical
            render_time_warning: 12.0,     // 12ms warning
            render_time_critical: 16.0,    // 16ms critical
            memory_warning: 50.0,          // 50MB warning
            memory_critical: 100.0,        // 100MB critical
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            alerts: Vec::new(),
            alert_thresholds: AlertThresholds::default(),
        }
    }

    pub fn add_contract(&mut self, component: String, contract: PerformanceContract) {
        self.contracts.insert(component, contract);
    }

    pub async fn start_monitoring(&mut self, interval_seconds: u64) -> Result<()> {
        let mut interval = interval(Duration::from_secs(interval_seconds));
        
        println!("🚀 Starting performance monitoring...");
        println!("📊 Monitoring {} components", self.contracts.len());
        println!("⏱️  Check interval: {} seconds", interval_seconds);
        
        loop {
            interval.tick().await;
            
            match self.check_performance_contracts().await {
                Ok(violations) => {
                    if !violations.is_empty() {
                        self.handle_violations(violations).await?;
                    } else {
                        println!("✅ All performance contracts satisfied at {}", 
                                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error checking performance contracts: {}", e);
                }
            }
        }
    }

    async fn check_performance_contracts(&self) -> Result<Vec<PerformanceAlert>> {
        let mut violations = Vec::new();
        let tester = WasmPerformanceTester::new();

        for (component, contract) in &self.contracts {
            // Check bundle size
            if let Err(_) = tester.test_bundle_size(contract.max_bundle_size_kb) {
                violations.push(PerformanceAlert {
                    component: component.clone(),
                    violation_type: ViolationType::BundleSize,
                    current_value: contract.max_bundle_size_kb as f64,
                    threshold: contract.max_bundle_size_kb as f64,
                    severity: AlertSeverity::Critical,
                    timestamp: Instant::now(),
                });
            }

            // Check render time
            if let Err(_) = tester.test_render_performance(contract.max_render_time_ms) {
                violations.push(PerformanceAlert {
                    component: component.clone(),
                    violation_type: ViolationType::RenderTime,
                    current_value: contract.max_render_time_ms as f64,
                    threshold: contract.max_render_time_ms as f64,
                    severity: AlertSeverity::Critical,
                    timestamp: Instant::now(),
                });
            }
        }

        // Check dependency contracts
        let dep_tester = DependencyContractTester::new(".");
        if let Err(e) = dep_tester.run_validation_tests() {
            violations.push(PerformanceAlert {
                component: "workspace".to_string(),
                violation_type: ViolationType::DependencyConflict,
                current_value: 0.0,
                threshold: 0.0,
                severity: AlertSeverity::High,
                timestamp: Instant::now(),
            });
        }

        Ok(violations)
    }

    async fn handle_violations(&mut self, violations: Vec<PerformanceAlert>) -> Result<()> {
        for violation in violations {
            self.send_alert(&violation).await?;
            self.alerts.push(violation);
        }
        Ok(())
    }

    async fn send_alert(&self, alert: &PerformanceAlert) -> Result<()> {
        let severity_emoji = match alert.severity {
            AlertSeverity::Low => "🟡",
            AlertSeverity::Medium => "🟠",
            AlertSeverity::High => "🔴",
            AlertSeverity::Critical => "🚨",
        };

        let violation_desc = match alert.violation_type {
            ViolationType::BundleSize => format!("Bundle size: {:.1}KB (limit: {:.1}KB)", 
                                                alert.current_value, alert.threshold),
            ViolationType::RenderTime => format!("Render time: {:.1}ms (limit: {:.1}ms)", 
                                                alert.current_value, alert.threshold),
            ViolationType::MemoryUsage => format!("Memory usage: {:.1}MB (limit: {:.1}MB)", 
                                                 alert.current_value, alert.threshold),
            ViolationType::DependencyConflict => "Dependency conflict detected".to_string(),
            ViolationType::VersionMismatch => "Version mismatch detected".to_string(),
        };

        println!("\n{} PERFORMANCE CONTRACT VIOLATION DETECTED", severity_emoji);
        println!("Component: {}", alert.component);
        println!("Violation: {}", violation_desc);
        println!("Severity: {:?}", alert.severity);
        println!("Timestamp: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!("{}", "=".repeat(50));

        // In a real implementation, you would send alerts to:
        // - Slack/Discord webhooks
        // - Email notifications
        // - PagerDuty/OpsGenie
        // - GitHub Issues
        // - Monitoring dashboards (Grafana, etc.)

        Ok(())
    }

    pub fn get_alert_summary(&self) -> String {
        let critical_count = self.alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Critical))
            .count();
        let high_count = self.alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::High))
            .count();
        let medium_count = self.alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Medium))
            .count();
        let low_count = self.alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Low))
            .count();

        format!(
            "Alert Summary: 🚨{} 🔴{} 🟠{} 🟡{} (Total: {})",
            critical_count, high_count, medium_count, low_count, self.alerts.len()
        )
    }

    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Performance Contract Monitoring Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", 
                                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        report.push_str(&self.get_alert_summary());
        report.push_str("\n\n");

        if self.alerts.is_empty() {
            report.push_str("✅ No performance contract violations detected.\n");
        } else {
            report.push_str("## Recent Violations\n\n");
            
            for alert in &self.alerts {
                let severity_emoji = match alert.severity {
                    AlertSeverity::Low => "🟡",
                    AlertSeverity::Medium => "🟠",
                    AlertSeverity::High => "🔴",
                    AlertSeverity::Critical => "🚨",
                };

                report.push_str(&format!(
                    "### {} {} - {}\n",
                    severity_emoji, alert.component, alert.violation_type
                ));
                
                report.push_str(&format!("- **Severity**: {:?}\n", alert.severity));
                report.push_str(&format!("- **Current Value**: {:.2}\n", alert.current_value));
                report.push_str(&format!("- **Threshold**: {:.2}\n", alert.threshold));
                report.push_str(&format!("- **Timestamp**: {}\n\n", 
                                        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
            }
        }

        report
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("monitor");

    match command {
        "monitor" => {
            let mut monitor = PerformanceMonitor::new();
            
            // Add contracts for all components
            let components = vec![
                "button", "input", "card", "dialog", "form", "table",
                "calendar", "date-picker", "pagination", "tooltip", "popover"
            ];

            for component in components {
                let contract = PerformanceContract {
                    max_bundle_size_kb: 500,
                    max_render_time_ms: 16,
                    max_memory_usage_mb: 100,
                    supports_ssr: true,
                    supports_hydration: true,
                };
                monitor.add_contract(component.to_string(), contract);
            }

            let interval = args.get(2)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(30);

            monitor.start_monitoring(interval).await?;
        }
        
        "check" => {
            let mut monitor = PerformanceMonitor::new();
            
            // Add contracts
            let components = vec![
                "button", "input", "card", "dialog", "form", "table"
            ];

            for component in components {
                let contract = PerformanceContract {
                    max_bundle_size_kb: 500,
                    max_render_time_ms: 16,
                    max_memory_usage_mb: 100,
                    supports_ssr: true,
                    supports_hydration: true,
                };
                monitor.add_contract(component.to_string(), contract);
            }

            let violations = monitor.check_performance_contracts().await?;
            
            if violations.is_empty() {
                println!("✅ All performance contracts satisfied");
                std::process::exit(0);
            } else {
                println!("❌ {} performance contract violations detected", violations.len());
                for violation in violations {
                    println!("- {}: {:?} (severity: {:?})", 
                            violation.component, violation.violation_type, violation.severity);
                }
                std::process::exit(1);
            }
        }
        
        "report" => {
            let mut monitor = PerformanceMonitor::new();
            
            // Simulate some alerts for demonstration
            monitor.alerts.push(PerformanceAlert {
                component: "button".to_string(),
                violation_type: ViolationType::BundleSize,
                current_value: 520.0,
                threshold: 500.0,
                severity: AlertSeverity::Critical,
                timestamp: Instant::now(),
            });

            let report = monitor.generate_performance_report();
            println!("{}", report);
        }
        
        _ => {
            eprintln!("Usage: {} [monitor|check|report] [interval_seconds]", args[0]);
            eprintln!("  monitor: Start continuous monitoring");
            eprintln!("  check:   Check contracts once and exit");
            eprintln!("  report:  Generate performance report");
            std::process::exit(1);
        }
    }

    Ok(())
}
