/**
 * WASM Performance Monitoring Utility
 * 
 * This utility provides comprehensive monitoring and analysis of WASM performance
 * across different browsers and scenarios.
 */

export interface WASMPerformanceMetrics {
  initializationTime: number;
  memoryUsage: {
    initial: number;
    peak: number;
    current: number;
  };
  bundleSize: number;
  loadTime: number;
  firstPaint: number;
  firstContentfulPaint: number;
  interactionLatency: number[];
  errorCount: number;
  browserInfo: {
    name: string;
    version: string;
    userAgent: string;
    webAssemblySupport: boolean;
  };
}

export interface WASMTestResult {
  testName: string;
  browser: string;
  success: boolean;
  metrics: WASMPerformanceMetrics;
  errors: string[];
  timestamp: Date;
}

export class WASMPerformanceMonitor {
  private metrics: Partial<WASMPerformanceMetrics> = {};
  private startTime: number = 0;
  private errors: string[] = [];

  constructor() {
    this.startTime = performance.now();
    this.setupErrorHandling();
  }

  private setupErrorHandling(): void {
    // Capture WASM-related errors
    window.addEventListener('error', (event) => {
      if (event.message.includes('wasm') || event.message.includes('WebAssembly')) {
        this.errors.push(`WASM Error: ${event.message}`);
      }
    });

    window.addEventListener('unhandledrejection', (event) => {
      if (event.reason && event.reason.toString().includes('wasm')) {
        this.errors.push(`WASM Promise Rejection: ${event.reason}`);
      }
    });
  }

  /**
   * Start monitoring WASM initialization
   */
  async startInitializationMonitoring(): Promise<void> {
    this.startTime = performance.now();
    
    // Monitor memory usage
    if (performance.memory) {
      this.metrics.memoryUsage = {
        initial: performance.memory.usedJSHeapSize,
        peak: performance.memory.usedJSHeapSize,
        current: performance.memory.usedJSHeapSize
      };
    }

    // Monitor bundle size
    this.metrics.bundleSize = await this.measureBundleSize();
  }

  /**
   * Complete initialization monitoring and capture metrics
   */
  async completeInitializationMonitoring(): Promise<void> {
    const endTime = performance.now();
    this.metrics.initializationTime = endTime - this.startTime;

    // Capture final memory usage
    if (performance.memory && this.metrics.memoryUsage) {
      this.metrics.memoryUsage.current = performance.memory.usedJSHeapSize;
      this.metrics.memoryUsage.peak = Math.max(
        this.metrics.memoryUsage.peak,
        performance.memory.usedJSHeapSize
      );
    }

    // Capture paint timing
    const paintEntries = performance.getEntriesByType('paint');
    paintEntries.forEach(entry => {
      if (entry.name === 'first-paint') {
        this.metrics.firstPaint = entry.startTime;
      } else if (entry.name === 'first-contentful-paint') {
        this.metrics.firstContentfulPaint = entry.startTime;
      }
    });

    // Capture load timing
    const navEntries = performance.getEntriesByType('navigation');
    if (navEntries.length > 0) {
      const navEntry = navEntries[0] as PerformanceNavigationTiming;
      this.metrics.loadTime = navEntry.loadEventEnd - navEntry.loadEventStart;
    }

    // Capture browser info
    this.metrics.browserInfo = this.getBrowserInfo();
  }

  /**
   * Measure interaction latency
   */
  measureInteractionLatency(interaction: () => void): number {
    const start = performance.now();
    interaction();
    const end = performance.now();
    const latency = end - start;

    if (!this.metrics.interactionLatency) {
      this.metrics.interactionLatency = [];
    }
    this.metrics.interactionLatency.push(latency);

    return latency;
  }

  /**
   * Get comprehensive performance metrics
   */
  getMetrics(): WASMPerformanceMetrics {
    return {
      initializationTime: this.metrics.initializationTime || 0,
      memoryUsage: this.metrics.memoryUsage || {
        initial: 0,
        peak: 0,
        current: 0
      },
      bundleSize: this.metrics.bundleSize || 0,
      loadTime: this.metrics.loadTime || 0,
      firstPaint: this.metrics.firstPaint || 0,
      firstContentfulPaint: this.metrics.firstContentfulPaint || 0,
      interactionLatency: this.metrics.interactionLatency || [],
      errorCount: this.errors.length,
      browserInfo: this.metrics.browserInfo || this.getBrowserInfo()
    };
  }

  /**
   * Get errors encountered during monitoring
   */
  getErrors(): string[] {
    return [...this.errors];
  }

  /**
   * Check if performance meets benchmarks
   */
  meetsBenchmarks(): { passed: boolean; failures: string[] } {
    const failures: string[] = [];
    const metrics = this.getMetrics();

    // Performance benchmarks
    if (metrics.initializationTime > 5000) {
      failures.push(`WASM initialization too slow: ${metrics.initializationTime}ms (max: 5000ms)`);
    }

    if (metrics.firstPaint > 3000) {
      failures.push(`First paint too slow: ${metrics.firstPaint}ms (max: 3000ms)`);
    }

    if (metrics.firstContentfulPaint > 4000) {
      failures.push(`First contentful paint too slow: ${metrics.firstContentfulPaint}ms (max: 4000ms)`);
    }

    if (metrics.interactionLatency.length > 0) {
      const avgLatency = metrics.interactionLatency.reduce((a, b) => a + b, 0) / metrics.interactionLatency.length;
      if (avgLatency > 100) {
        failures.push(`Average interaction latency too high: ${avgLatency.toFixed(2)}ms (max: 100ms)`);
      }
    }

    // Memory benchmarks
    if (metrics.memoryUsage.peak > metrics.memoryUsage.initial * 2) {
      failures.push(`Memory usage doubled during initialization`);
    }

    return {
      passed: failures.length === 0,
      failures
    };
  }

  /**
   * Generate performance report
   */
  generateReport(): string {
    const metrics = this.getMetrics();
    const benchmarks = this.meetsBenchmarks();
    
    let report = `# WASM Performance Report\n\n`;
    report += `**Browser**: ${metrics.browserInfo.name} ${metrics.browserInfo.version}\n`;
    report += `**Test Time**: ${new Date().toISOString()}\n\n`;
    
    report += `## Performance Metrics\n\n`;
    report += `- **Initialization Time**: ${metrics.initializationTime.toFixed(2)}ms\n`;
    report += `- **First Paint**: ${metrics.firstPaint.toFixed(2)}ms\n`;
    report += `- **First Contentful Paint**: ${metrics.firstContentfulPaint.toFixed(2)}ms\n`;
    report += `- **Load Time**: ${metrics.loadTime.toFixed(2)}ms\n`;
    report += `- **Bundle Size**: ${(metrics.bundleSize / 1024).toFixed(2)}KB\n\n`;
    
    report += `## Memory Usage\n\n`;
    report += `- **Initial**: ${(metrics.memoryUsage.initial / 1024 / 1024).toFixed(2)}MB\n`;
    report += `- **Peak**: ${(metrics.memoryUsage.peak / 1024 / 1024).toFixed(2)}MB\n`;
    report += `- **Current**: ${(metrics.memoryUsage.current / 1024 / 1024).toFixed(2)}MB\n\n`;
    
    if (metrics.interactionLatency.length > 0) {
      const avgLatency = metrics.interactionLatency.reduce((a, b) => a + b, 0) / metrics.interactionLatency.length;
      report += `## Interaction Performance\n\n`;
      report += `- **Average Latency**: ${avgLatency.toFixed(2)}ms\n`;
      report += `- **Max Latency**: ${Math.max(...metrics.interactionLatency).toFixed(2)}ms\n`;
      report += `- **Min Latency**: ${Math.min(...metrics.interactionLatency).toFixed(2)}ms\n\n`;
    }
    
    report += `## Benchmark Results\n\n`;
    if (benchmarks.passed) {
      report += `✅ **All benchmarks passed**\n\n`;
    } else {
      report += `❌ **Benchmark failures**:\n`;
      benchmarks.failures.forEach(failure => {
        report += `- ${failure}\n`;
      });
      report += `\n`;
    }
    
    if (this.errors.length > 0) {
      report += `## Errors Encountered\n\n`;
      this.errors.forEach(error => {
        report += `- ${error}\n`;
      });
      report += `\n`;
    }
    
    return report;
  }

  private async measureBundleSize(): Promise<number> {
    try {
      // Try to measure bundle size from network requests
      const scripts = Array.from(document.querySelectorAll('script[src]'));
      let totalSize = 0;

      for (const script of scripts) {
        const src = script.getAttribute('src');
        if (src && (src.includes('.wasm') || src.includes('wasm'))) {
          try {
            const response = await fetch(src, { method: 'HEAD' });
            const contentLength = response.headers.get('content-length');
            if (contentLength) {
              totalSize += parseInt(contentLength);
            }
          } catch (e) {
            // Ignore fetch errors
          }
        }
      }

      return totalSize;
    } catch (e) {
      return 0;
    }
  }

  private getBrowserInfo(): WASMPerformanceMetrics['browserInfo'] {
    const userAgent = navigator.userAgent;
    let name = 'Unknown';
    let version = 'Unknown';

    if (userAgent.includes('Chrome')) {
      name = 'Chrome';
      const match = userAgent.match(/Chrome\/(\d+)/);
      if (match) version = match[1];
    } else if (userAgent.includes('Firefox')) {
      name = 'Firefox';
      const match = userAgent.match(/Firefox\/(\d+)/);
      if (match) version = match[1];
    } else if (userAgent.includes('Safari') && !userAgent.includes('Chrome')) {
      name = 'Safari';
      const match = userAgent.match(/Version\/(\d+)/);
      if (match) version = match[1];
    } else if (userAgent.includes('Edge')) {
      name = 'Edge';
      const match = userAgent.match(/Edge\/(\d+)/);
      if (match) version = match[1];
    }

    return {
      name,
      version,
      userAgent,
      webAssemblySupport: typeof WebAssembly !== 'undefined'
    };
  }
}

/**
 * Utility function to run WASM performance tests
 */
export async function runWASMPerformanceTest(
  testName: string,
  browserName: string,
  testFunction: (monitor: WASMPerformanceMonitor) => Promise<void>
): Promise<WASMTestResult> {
  const monitor = new WASMPerformanceMonitor();
  
  try {
    await monitor.startInitializationMonitoring();
    await testFunction(monitor);
    await monitor.completeInitializationMonitoring();
    
    return {
      testName,
      browser: browserName,
      success: true,
      metrics: monitor.getMetrics(),
      errors: monitor.getErrors(),
      timestamp: new Date()
    };
  } catch (error) {
    return {
      testName,
      browser: browserName,
      success: false,
      metrics: monitor.getMetrics(),
      errors: [...monitor.getErrors(), `Test Error: ${(error as Error).message}`],
      timestamp: new Date()
    };
  }
}
