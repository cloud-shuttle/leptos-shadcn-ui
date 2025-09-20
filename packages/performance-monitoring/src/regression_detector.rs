use leptos::prelude::*;
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
}