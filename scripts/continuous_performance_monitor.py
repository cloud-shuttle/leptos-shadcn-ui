#!/usr/bin/env python3
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
        lines = output.split('\n')
        
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
                print("\n🛑 Monitoring stopped by user")
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
        print("\n🛑 Stopping monitoring...")
        monitor.stop_monitoring()
        print("✅ Monitoring stopped")

if __name__ == "__main__":
    main()
