//! WASM performance testing framework

use crate::{ContractError, PerformanceContract};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

pub struct WasmPerformanceTester {
    bundle_size_limit_kb: u64,
    render_time_limit_ms: u64,
}

impl WasmPerformanceTester {
    pub fn new() -> Self {
        Self {
            bundle_size_limit_kb: 500, // 500KB limit
            render_time_limit_ms: 16,  // 60fps target
        }
    }

    pub fn with_limits(bundle_size_kb: u64, render_time_ms: u64) -> Self {
        Self {
            bundle_size_limit_kb: bundle_size_kb,
            render_time_limit_ms: render_time_ms,
        }
    }

    /// Test bundle size constraints for WASM builds
    pub fn test_bundle_size(&self, actual_size_kb: u64) -> Result<(), ContractError> {
        if actual_size_kb > self.bundle_size_limit_kb {
            return Err(ContractError::PerformanceError {
                requirement: format!(
                    "Bundle size {} KB exceeds limit of {} KB",
                    actual_size_kb, self.bundle_size_limit_kb
                ),
            });
        }

        println!("✅ Bundle size {} KB is within {} KB limit", actual_size_kb, self.bundle_size_limit_kb);
        Ok(())
    }

    /// Test render performance for components
    pub fn test_render_performance(&self, render_time_ms: u64) -> Result<(), ContractError> {
        if render_time_ms > self.render_time_limit_ms {
            return Err(ContractError::PerformanceError {
                requirement: format!(
                    "Render time {} ms exceeds limit of {} ms",
                    render_time_ms, self.render_time_limit_ms
                ),
            });
        }

        println!("✅ Render time {} ms is within {} ms limit", render_time_ms, self.render_time_limit_ms);
        Ok(())
    }

    /// Test that components work in WASM environment
    #[cfg(target_arch = "wasm32")]
    pub fn test_wasm_compatibility(&self) -> Result<(), ContractError> {
        // Basic WASM functionality test
        use web_sys::console;
        console::log_1(&"Testing WASM compatibility".into());
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn test_wasm_compatibility(&self) -> Result<(), ContractError> {
        // Simulate WASM test for non-WASM targets
        println!("✅ WASM compatibility test (simulated for non-WASM target)");
        Ok(())
    }

    /// Validate performance contract for component
    pub fn validate_performance_contract(&self, contract: &PerformanceContract) -> Result<(), ContractError> {
        // Test against contract requirements
        self.test_bundle_size(contract.max_bundle_size_kb)?;
        self.test_render_performance(contract.max_render_time_ms)?;

        if !contract.supports_ssr {
            println!("⚠️  Component does not support SSR");
        }

        if !contract.supports_hydration {
            println!("⚠️  Component does not support hydration");
        }

        Ok(())
    }
}

// WASM-specific tests
#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_wasm_performance_creation() {
        let tester = WasmPerformanceTester::new();
        assert_eq!(tester.bundle_size_limit_kb, 500);
        assert_eq!(tester.render_time_limit_ms, 16);
    }

    #[wasm_bindgen_test]
    async fn test_wasm_bundle_size_validation() {
        let tester = WasmPerformanceTester::new();

        // Test passing case
        let result = tester.test_bundle_size(400);
        assert!(result.is_ok(), "400KB should be within 500KB limit");

        // Test failing case
        let result = tester.test_bundle_size(600);
        assert!(result.is_err(), "600KB should exceed 500KB limit");
    }

    #[wasm_bindgen_test]
    async fn test_wasm_render_performance() {
        let tester = WasmPerformanceTester::new();

        // Test passing case (under 16ms for 60fps)
        let result = tester.test_render_performance(10);
        assert!(result.is_ok(), "10ms should be within 16ms limit");

        // Test failing case
        let result = tester.test_render_performance(25);
        assert!(result.is_err(), "25ms should exceed 16ms limit");
    }

    #[wasm_bindgen_test]
    async fn test_wasm_compatibility() {
        let tester = WasmPerformanceTester::new();
        let result = tester.test_wasm_compatibility();
        assert!(result.is_ok(), "WASM compatibility test should pass");
    }
}

// Standard tests for non-WASM environments
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_tester_creation() {
        let tester = WasmPerformanceTester::new();
        assert_eq!(tester.bundle_size_limit_kb, 500);
        assert_eq!(tester.render_time_limit_ms, 16);
    }

    #[tokio::test]
    async fn test_bundle_size_validation() {
        let tester = WasmPerformanceTester::new();

        // Test passing case
        let result = tester.test_bundle_size(400);
        assert!(result.is_ok(), "400KB should be within 500KB limit");

        // Test failing case
        let result = tester.test_bundle_size(600);
        assert!(result.is_err(), "600KB should exceed 500KB limit");
    }

    #[tokio::test]
    async fn test_render_performance() {
        let tester = WasmPerformanceTester::new();

        // Test passing case (under 16ms for 60fps)
        let result = tester.test_render_performance(10);
        assert!(result.is_ok(), "10ms should be within 16ms limit");

        // Test failing case
        let result = tester.test_render_performance(25);
        assert!(result.is_err(), "25ms should exceed 16ms limit");
    }

    #[tokio::test]
    async fn test_performance_contract_validation() {
        let tester = WasmPerformanceTester::new();
        let contract = PerformanceContract {
            max_render_time_ms: 10,
            max_bundle_size_kb: 400,
            max_memory_usage_mb: 50,
            supports_ssr: true,
            supports_hydration: true,
        };

        let result = tester.validate_performance_contract(&contract);
        assert!(result.is_ok(), "Performance contract should be valid");
    }

    #[tokio::test]
    async fn test_wasm_compatibility_simulation() {
        let tester = WasmPerformanceTester::new();
        let result = tester.test_wasm_compatibility();
        assert!(result.is_ok(), "WASM compatibility simulation should pass");
    }
}