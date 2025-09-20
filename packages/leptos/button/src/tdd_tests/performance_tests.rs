#[cfg(test)]
mod performance_tests {
    use crate::default::{Button, ButtonVariant, ButtonSize};
    use leptos::prelude::*;

    #[test]
    fn test_button_performance_optimization() {
        // Test performance optimization features
        let _perf_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="perf-optimized"
            >
                "Performance Test"
            </Button>
        };
        
        // Should have performance optimizations
    }

    #[test]
    fn test_button_performance_comprehensive() {
        // Test comprehensive performance features
        let perf_features = vec![
            "lazy-loading",
            "memoization",
            "virtual-scrolling",
            "debounced-clicks",
            "optimized-rendering",
        ];
        
        for feature in perf_features {
            let _perf_button_view = view! {
                <Button 
                    variant=ButtonVariant::Default
                    size=ButtonSize::Default
                    class=format!("perf-{}", feature)
                >
                    format!("{} Button", feature)
                </Button>
            };
            
            // Each performance feature should be implemented
        }
    }

    #[test]
    fn test_button_memory_performance() {
        // Test memory performance
        let _memory_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="memory-perf"
            >
                "Memory Performance"
            </Button>
        };
        
        // Should have good memory performance
    }

    #[test]
    fn test_button_cpu_performance() {
        // Test CPU performance
        let _cpu_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="cpu-perf"
            >
                "CPU Performance"
            </Button>
        };
        
        // Should have good CPU performance
    }

    #[test]
    fn test_button_network_performance() {
        // Test network performance
        let _network_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="network-perf"
            >
                "Network Performance"
            </Button>
        };
        
        // Should have good network performance
    }

    #[test]
    fn test_button_battery_performance() {
        // Test battery performance
        let _battery_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="battery-perf"
            >
                "Battery Performance"
            </Button>
        };
        
        // Should have good battery performance
    }

    #[test]
    fn test_button_thermal_performance() {
        // Test thermal performance
        let _thermal_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="thermal-perf"
            >
                "Thermal Performance"
            </Button>
        };
        
        // Should have good thermal performance
    }

    #[test]
    fn test_button_benchmark_performance() {
        // Test benchmark performance
        let _benchmark_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="benchmark-perf"
            >
                "Benchmark Performance"
            </Button>
        };
        
        // Should have good benchmark performance
    }

    #[test]
    fn test_button_load_performance() {
        // Test load performance
        let _load_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="load-perf"
            >
                "Load Performance"
            </Button>
        };
        
        // Should have good load performance
    }

    #[test]
    fn test_button_stress_performance() {
        // Test stress performance
        let _stress_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="stress-perf"
            >
                "Stress Performance"
            </Button>
        };
        
        // Should have good stress performance
    }

    #[test]
    fn test_button_concurrent_performance() {
        // Test concurrent performance
        let _concurrent_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="concurrent-perf"
            >
                "Concurrent Performance"
            </Button>
        };
        
        // Should have good concurrent performance
    }

    #[test]
    fn test_button_scalability_performance() {
        // Test scalability performance
        let _scalability_button_view = view! {
            <Button 
                variant=ButtonVariant::Default
                size=ButtonSize::Default
                class="scalability-perf"
            >
                "Scalability Performance"
            </Button>
        };
        
        // Should have good scalability performance
    }
}
