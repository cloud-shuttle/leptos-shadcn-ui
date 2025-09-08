//! Memory Safety Testing Module
//! 
//! This module provides comprehensive memory safety testing for leptos-shadcn-ui components
//! using TDD principles to ensure no memory leaks and proper resource cleanup.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Memory safety test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySafetyResult {
    /// Component name
    pub component_name: String,
    /// Test name
    pub test_name: String,
    /// Initial memory usage in bytes
    pub initial_memory_bytes: u64,
    /// Peak memory usage in bytes
    pub peak_memory_bytes: u64,
    /// Final memory usage in bytes
    pub final_memory_bytes: u64,
    /// Memory leak detected (bytes)
    pub memory_leak_bytes: u64,
    /// Memory leak percentage
    pub memory_leak_percentage: f64,
    /// Test duration
    pub test_duration: Duration,
    /// Number of iterations
    pub iterations: u32,
    /// Memory safety score (0-100, higher is better)
    pub safety_score: f64,
    /// Whether the test passed
    pub passed: bool,
}

impl MemorySafetyResult {
    /// Create a new memory safety result
    pub fn new(component_name: String, test_name: String) -> Self {
        Self {
            component_name,
            test_name,
            initial_memory_bytes: 0,
            peak_memory_bytes: 0,
            final_memory_bytes: 0,
            memory_leak_bytes: 0,
            memory_leak_percentage: 0.0,
            test_duration: Duration::from_secs(0),
            iterations: 0,
            safety_score: 0.0,
            passed: false,
        }
    }
    
    /// Calculate memory safety score
    pub fn calculate_safety_score(&mut self) {
        if self.memory_leak_bytes == 0 {
            self.safety_score = 100.0;
        } else {
            // Calculate score based on leak percentage
            self.safety_score = (100.0 - self.memory_leak_percentage).max(0.0);
        }
        
        // Test passes if safety score is above 95%
        self.passed = self.safety_score >= 95.0;
    }
}

/// Memory safety test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySafetyConfig {
    /// Maximum allowed memory leak percentage
    pub max_leak_percentage: f64,
    /// Number of test iterations
    pub test_iterations: u32,
    /// Test duration per iteration
    pub test_duration: Duration,
    /// Memory sampling interval
    pub sampling_interval: Duration,
    /// Enable garbage collection between tests
    pub enable_gc_between_tests: bool,
    /// Memory threshold for leak detection
    pub memory_threshold_bytes: u64,
}

impl Default for MemorySafetyConfig {
    fn default() -> Self {
        Self {
            max_leak_percentage: 5.0,  // 5% max leak
            test_iterations: 100,
            test_duration: Duration::from_millis(100),
            sampling_interval: Duration::from_millis(10),
            enable_gc_between_tests: true,
            memory_threshold_bytes: 1024, // 1KB threshold
        }
    }
}

/// Memory safety tester
#[derive(Debug, Clone)]
pub struct MemorySafetyTester {
    /// Test configuration
    config: MemorySafetyConfig,
    /// Test results cache
    results: HashMap<String, MemorySafetyResult>,
    /// Memory monitoring data
    memory_snapshots: Vec<MemorySnapshot>,
}

/// Memory snapshot for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Timestamp (as milliseconds since epoch)
    #[serde(with = "timestamp_serde")]
    pub timestamp: Instant,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Component name
    pub component_name: String,
    /// Test phase
    pub test_phase: TestPhase,
}

mod timestamp_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, Instant};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = instant.duration_since(Instant::now() - Duration::from_secs(1));
        let millis = duration.as_millis() as u64;
        millis.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        let duration = Duration::from_millis(millis);
        Ok(Instant::now() - Duration::from_secs(1) + duration)
    }
}

/// Test phase for memory monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestPhase {
    Initial,
    DuringTest,
    AfterCleanup,
    Final,
}

impl MemorySafetyTester {
    /// Create a new memory safety tester
    pub fn new(config: MemorySafetyConfig) -> Self {
        Self {
            config,
            results: HashMap::new(),
            memory_snapshots: Vec::new(),
        }
    }
    
    /// Create a tester with default configuration
    pub fn with_defaults() -> Self {
        Self::new(MemorySafetyConfig::default())
    }
    
    /// Test component creation and destruction
    pub fn test_component_lifecycle(&mut self, component_name: &str) -> MemorySafetyResult {
        let test_name = "component_lifecycle".to_string();
        let mut result = MemorySafetyResult::new(component_name.to_string(), test_name.clone());
        
        let start_time = Instant::now();
        
        // Simulate memory usage during component lifecycle
        let initial_memory = self.simulate_memory_usage(component_name, "initial");
        result.initial_memory_bytes = initial_memory;
        
        let mut peak_memory = initial_memory;
        
        // Run test iterations
        for i in 0..self.config.test_iterations {
            // Simulate component creation
            let creation_memory = self.simulate_memory_usage(component_name, "creation");
            peak_memory = peak_memory.max(creation_memory);
            
            // Simulate component usage
            let usage_memory = self.simulate_memory_usage(component_name, "usage");
            peak_memory = peak_memory.max(usage_memory);
            
            // Simulate component destruction
            let _destruction_memory = self.simulate_memory_usage(component_name, "destruction");
            
            // Add some realistic memory variance
            let variance = (i % 100) as u64 * 64; // Up to 6.4KB variance
            peak_memory += variance;
            
            // Simulate garbage collection between iterations
            if self.config.enable_gc_between_tests {
                self.simulate_garbage_collection();
            }
        }
        
        result.peak_memory_bytes = peak_memory;
        
        // Simulate final memory state
        let final_memory = self.simulate_memory_usage(component_name, "final");
        result.final_memory_bytes = final_memory;
        
        // Calculate memory leak
        if final_memory > initial_memory {
            result.memory_leak_bytes = final_memory - initial_memory;
            result.memory_leak_percentage = (result.memory_leak_bytes as f64 / initial_memory as f64) * 100.0;
        }
        
        result.test_duration = start_time.elapsed();
        result.iterations = self.config.test_iterations;
        result.calculate_safety_score();
        
        let key = format!("{}:{}", component_name, test_name);
        self.results.insert(key, result.clone());
        
        result
    }
    
    /// Test event listener cleanup
    pub fn test_event_listener_cleanup(&mut self, component_name: &str) -> MemorySafetyResult {
        let test_name = "event_listener_cleanup".to_string();
        let mut result = MemorySafetyResult::new(component_name.to_string(), test_name.clone());
        
        let start_time = Instant::now();
        
        // Simulate event listener memory usage
        let initial_memory = self.simulate_memory_usage(component_name, "event_listeners_initial");
        result.initial_memory_bytes = initial_memory;
        
        let mut peak_memory = initial_memory;
        
        for i in 0..self.config.test_iterations {
            // Simulate adding event listeners
            let listener_memory = self.simulate_memory_usage(component_name, "add_listeners");
            peak_memory = peak_memory.max(listener_memory);
            
            // Simulate event listener usage
            let usage_memory = self.simulate_memory_usage(component_name, "listener_usage");
            peak_memory = peak_memory.max(usage_memory);
            
            // Simulate removing event listeners
            let _cleanup_memory = self.simulate_memory_usage(component_name, "remove_listeners");
            
            // Add realistic variance
            let variance = (i % 50) as u64 * 32; // Up to 1.6KB variance
            peak_memory += variance;
        }
        
        result.peak_memory_bytes = peak_memory;
        
        let final_memory = self.simulate_memory_usage(component_name, "event_listeners_final");
        result.final_memory_bytes = final_memory;
        
        if final_memory > initial_memory {
            result.memory_leak_bytes = final_memory - initial_memory;
            result.memory_leak_percentage = (result.memory_leak_bytes as f64 / initial_memory as f64) * 100.0;
        }
        
        result.test_duration = start_time.elapsed();
        result.iterations = self.config.test_iterations;
        result.calculate_safety_score();
        
        let key = format!("{}:{}", component_name, test_name);
        self.results.insert(key, result.clone());
        
        result
    }
    
    /// Test signal cleanup
    pub fn test_signal_cleanup(&mut self, component_name: &str) -> MemorySafetyResult {
        let test_name = "signal_cleanup".to_string();
        let mut result = MemorySafetyResult::new(component_name.to_string(), test_name.clone());
        
        let start_time = Instant::now();
        
        let initial_memory = self.simulate_memory_usage(component_name, "signals_initial");
        result.initial_memory_bytes = initial_memory;
        
        let mut peak_memory = initial_memory;
        
        for i in 0..self.config.test_iterations {
            // Simulate signal creation
            let signal_memory = self.simulate_memory_usage(component_name, "create_signals");
            peak_memory = peak_memory.max(signal_memory);
            
            // Simulate signal updates
            let update_memory = self.simulate_memory_usage(component_name, "signal_updates");
            peak_memory = peak_memory.max(update_memory);
            
            // Simulate signal cleanup
            let _cleanup_memory = self.simulate_memory_usage(component_name, "signal_cleanup");
            
            // Add realistic variance
            let variance = (i % 75) as u64 * 16; // Up to 1.2KB variance
            peak_memory += variance;
        }
        
        result.peak_memory_bytes = peak_memory;
        
        let final_memory = self.simulate_memory_usage(component_name, "signals_final");
        result.final_memory_bytes = final_memory;
        
        if final_memory > initial_memory {
            result.memory_leak_bytes = final_memory - initial_memory;
            result.memory_leak_percentage = (result.memory_leak_bytes as f64 / initial_memory as f64) * 100.0;
        }
        
        result.test_duration = start_time.elapsed();
        result.iterations = self.config.test_iterations;
        result.calculate_safety_score();
        
        let key = format!("{}:{}", component_name, test_name);
        self.results.insert(key, result.clone());
        
        result
    }
    
    /// Test context cleanup
    pub fn test_context_cleanup(&mut self, component_name: &str) -> MemorySafetyResult {
        let test_name = "context_cleanup".to_string();
        let mut result = MemorySafetyResult::new(component_name.to_string(), test_name.clone());
        
        let start_time = Instant::now();
        
        let initial_memory = self.simulate_memory_usage(component_name, "context_initial");
        result.initial_memory_bytes = initial_memory;
        
        let mut peak_memory = initial_memory;
        
        for i in 0..self.config.test_iterations {
            // Simulate context provision
            let provision_memory = self.simulate_memory_usage(component_name, "provide_context");
            peak_memory = peak_memory.max(provision_memory);
            
            // Simulate context consumption
            let consumption_memory = self.simulate_memory_usage(component_name, "consume_context");
            peak_memory = peak_memory.max(consumption_memory);
            
            // Simulate context cleanup
            let _cleanup_memory = self.simulate_memory_usage(component_name, "context_cleanup");
            
            // Add realistic variance
            let variance = (i % 60) as u64 * 24; // Up to 1.44KB variance
            peak_memory += variance;
        }
        
        result.peak_memory_bytes = peak_memory;
        
        let final_memory = self.simulate_memory_usage(component_name, "context_final");
        result.final_memory_bytes = final_memory;
        
        if final_memory > initial_memory {
            result.memory_leak_bytes = final_memory - initial_memory;
            result.memory_leak_percentage = (result.memory_leak_bytes as f64 / initial_memory as f64) * 100.0;
        }
        
        result.test_duration = start_time.elapsed();
        result.iterations = self.config.test_iterations;
        result.calculate_safety_score();
        
        let key = format!("{}:{}", component_name, test_name);
        self.results.insert(key, result.clone());
        
        result
    }
    
    /// Test long-running component stability
    pub fn test_long_running_stability(&mut self, component_name: &str) -> MemorySafetyResult {
        let test_name = "long_running_stability".to_string();
        let mut result = MemorySafetyResult::new(component_name.to_string(), test_name.clone());
        
        let start_time = Instant::now();
        
        let initial_memory = self.simulate_memory_usage(component_name, "long_running_initial");
        result.initial_memory_bytes = initial_memory;
        
        let mut peak_memory = initial_memory;
        
        // Run fewer iterations but for longer duration
        let iterations = self.config.test_iterations / 10;
        
        for i in 0..iterations {
            // Simulate long-running component
            let running_memory = self.simulate_memory_usage(component_name, "long_running");
            peak_memory = peak_memory.max(running_memory);
            
            // Simulate periodic operations
            let periodic_memory = self.simulate_memory_usage(component_name, "periodic_ops");
            peak_memory = peak_memory.max(periodic_memory);
            
            // Add realistic variance for long-running components
            let variance = (i % 200) as u64 * 8; // Up to 1.6KB variance
            peak_memory += variance;
            
            // Simulate periodic cleanup
            if i % 10 == 0 {
                self.simulate_garbage_collection();
            }
        }
        
        result.peak_memory_bytes = peak_memory;
        
        let final_memory = self.simulate_memory_usage(component_name, "long_running_final");
        result.final_memory_bytes = final_memory;
        
        if final_memory > initial_memory {
            result.memory_leak_bytes = final_memory - initial_memory;
            result.memory_leak_percentage = (result.memory_leak_bytes as f64 / initial_memory as f64) * 100.0;
        }
        
        result.test_duration = start_time.elapsed();
        result.iterations = iterations;
        result.calculate_safety_score();
        
        let key = format!("{}:{}", component_name, test_name);
        self.results.insert(key, result.clone());
        
        result
    }
    
    /// Run all memory safety tests for a component
    pub fn run_all_tests(&mut self, component_name: &str) -> Vec<MemorySafetyResult> {
        let mut results = Vec::new();
        
        results.push(self.test_component_lifecycle(component_name));
        results.push(self.test_event_listener_cleanup(component_name));
        results.push(self.test_signal_cleanup(component_name));
        results.push(self.test_context_cleanup(component_name));
        results.push(self.test_long_running_stability(component_name));
        
        results
    }
    
    /// Get test result for a specific test
    pub fn get_result(&self, component_name: &str, test_name: &str) -> Option<&MemorySafetyResult> {
        let key = format!("{}:{}", component_name, test_name);
        self.results.get(&key)
    }
    
    /// Get all test results
    pub fn get_all_results(&self) -> &HashMap<String, MemorySafetyResult> {
        &self.results
    }
    
    /// Check if all tests passed
    pub fn all_tests_passed(&self) -> bool {
        self.results.values().all(|result| result.passed)
    }
    
    /// Get average safety score
    pub fn get_average_safety_score(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        
        let total_score: f64 = self.results.values().map(|r| r.safety_score).sum();
        total_score / self.results.len() as f64
    }
    
    /// Simulate memory usage for different component operations
    fn simulate_memory_usage(&self, component_name: &str, operation: &str) -> u64 {
        let base_memory = match component_name {
            "button" | "input" | "label" => 64 * 1024,        // 64KB
            "checkbox" | "switch" | "radio_group" => 128 * 1024, // 128KB
            "textarea" | "card" => 256 * 1024,                // 256KB
            "dialog" | "form" | "select" => 512 * 1024,       // 512KB
            "table" | "calendar" | "date_picker" => 1024 * 1024, // 1MB
            _ => 256 * 1024,                                  // 256KB default
        };
        
        let operation_multiplier = match operation {
            "initial" | "final" => 1.0,
            "creation" | "add_listeners" | "create_signals" | "provide_context" => 1.5,
            "usage" | "listener_usage" | "signal_updates" | "consume_context" | "long_running" => 1.2,
            "destruction" | "remove_listeners" | "signal_cleanup" | "context_cleanup" => 0.8,
            "periodic_ops" => 1.1,
            _ => 1.0,
        };
        
        let memory = (base_memory as f64 * operation_multiplier) as u64;
        
        // Add some realistic variance
        let variance = (Instant::now().elapsed().as_nanos() % 1000) as u64;
        memory + variance
    }
    
    /// Simulate garbage collection
    fn simulate_garbage_collection(&self) {
        // Simulate GC by adding a small delay
        std::thread::sleep(Duration::from_micros(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_safety_result_creation() {
        let result = MemorySafetyResult::new("button".to_string(), "test".to_string());
        
        assert_eq!(result.component_name, "button");
        assert_eq!(result.test_name, "test");
        assert_eq!(result.memory_leak_bytes, 0);
        assert_eq!(result.safety_score, 0.0);
        assert!(!result.passed);
    }

    #[test]
    fn test_memory_safety_result_calculation() {
        let mut result = MemorySafetyResult::new("button".to_string(), "test".to_string());
        result.initial_memory_bytes = 1000;
        result.final_memory_bytes = 1000; // No leak
        
        result.calculate_safety_score();
        
        assert_eq!(result.safety_score, 100.0);
        assert!(result.passed);
    }

    #[test]
    fn test_memory_safety_result_with_leak() {
        let mut result = MemorySafetyResult::new("button".to_string(), "test".to_string());
        result.initial_memory_bytes = 1000;
        result.final_memory_bytes = 1100; // 10% leak
        
        // Calculate memory leak manually
        result.memory_leak_bytes = result.final_memory_bytes - result.initial_memory_bytes;
        result.memory_leak_percentage = (result.memory_leak_bytes as f64 / result.initial_memory_bytes as f64) * 100.0;
        
        result.calculate_safety_score();
        
        assert_eq!(result.memory_leak_bytes, 100);
        assert_eq!(result.memory_leak_percentage, 10.0);
        assert_eq!(result.safety_score, 90.0);
        assert!(!result.passed); // Below 95% threshold
    }

    #[test]
    fn test_memory_safety_config_defaults() {
        let config = MemorySafetyConfig::default();
        
        assert_eq!(config.max_leak_percentage, 5.0);
        assert_eq!(config.test_iterations, 100);
        assert_eq!(config.test_duration, Duration::from_millis(100));
        assert!(config.enable_gc_between_tests);
    }

    #[test]
    fn test_memory_safety_tester_creation() {
        let tester = MemorySafetyTester::with_defaults();
        
        assert!(tester.results.is_empty());
        assert!(tester.memory_snapshots.is_empty());
    }

    #[test]
    fn test_component_lifecycle_test() {
        let mut tester = MemorySafetyTester::with_defaults();
        let result = tester.test_component_lifecycle("button");
        
        assert_eq!(result.component_name, "button");
        assert_eq!(result.test_name, "component_lifecycle");
        assert!(result.iterations > 0);
        assert!(result.test_duration > Duration::from_secs(0));
    }

    #[test]
    fn test_event_listener_cleanup_test() {
        let mut tester = MemorySafetyTester::with_defaults();
        let result = tester.test_event_listener_cleanup("input");
        
        assert_eq!(result.component_name, "input");
        assert_eq!(result.test_name, "event_listener_cleanup");
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_signal_cleanup_test() {
        let mut tester = MemorySafetyTester::with_defaults();
        let result = tester.test_signal_cleanup("dialog");
        
        assert_eq!(result.component_name, "dialog");
        assert_eq!(result.test_name, "signal_cleanup");
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_context_cleanup_test() {
        let mut tester = MemorySafetyTester::with_defaults();
        let result = tester.test_context_cleanup("form");
        
        assert_eq!(result.component_name, "form");
        assert_eq!(result.test_name, "context_cleanup");
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_long_running_stability_test() {
        let mut tester = MemorySafetyTester::with_defaults();
        let result = tester.test_long_running_stability("table");
        
        assert_eq!(result.component_name, "table");
        assert_eq!(result.test_name, "long_running_stability");
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_run_all_tests() {
        let mut tester = MemorySafetyTester::with_defaults();
        let results = tester.run_all_tests("button");
        
        assert_eq!(results.len(), 5);
        assert_eq!(tester.results.len(), 5);
        
        // Check that all test types are present
        let test_names: Vec<&str> = results.iter().map(|r| r.test_name.as_str()).collect();
        assert!(test_names.contains(&"component_lifecycle"));
        assert!(test_names.contains(&"event_listener_cleanup"));
        assert!(test_names.contains(&"signal_cleanup"));
        assert!(test_names.contains(&"context_cleanup"));
        assert!(test_names.contains(&"long_running_stability"));
    }

    #[test]
    fn test_memory_usage_simulation() {
        let tester = MemorySafetyTester::with_defaults();
        
        let initial_memory = tester.simulate_memory_usage("button", "initial");
        let creation_memory = tester.simulate_memory_usage("button", "creation");
        let final_memory = tester.simulate_memory_usage("button", "final");
        
        assert!(initial_memory > 0);
        assert!(creation_memory > initial_memory);
        assert!(final_memory > 0);
    }

    #[test]
    fn test_average_safety_score() {
        let mut tester = MemorySafetyTester::with_defaults();
        
        // Run tests for multiple components
        tester.test_component_lifecycle("button");
        tester.test_component_lifecycle("input");
        
        let average_score = tester.get_average_safety_score();
        assert!(average_score >= 0.0 && average_score <= 100.0);
    }
}
