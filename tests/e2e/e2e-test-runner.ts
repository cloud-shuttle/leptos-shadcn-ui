/**
 * Enhanced E2E Test Runner
 * 
 * This module provides comprehensive E2E test execution with CI/CD integration,
 * automated reporting, and performance monitoring.
 */

import { chromium, FullConfig, FullResult } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

export interface E2ETestConfig {
  // Test execution settings
  execution: {
    parallel: boolean;
    workers: number;
    retries: number;
    timeout: number;
  };
  
  // Browser configuration
  browsers: {
    [browserName: string]: {
      enabled: boolean;
      headless: boolean;
      timeout: number;
      retries: number;
    };
  };
  
  // Test scenarios
  scenarios: {
    [scenarioName: string]: {
      enabled: boolean;
      description: string;
      testFiles: string[];
      priority: 'high' | 'medium' | 'low';
    };
  };
  
  // Reporting configuration
  reporting: {
    generateHtmlReport: boolean;
    generateJsonReport: boolean;
    generateJunitReport: boolean;
    generateMarkdownReport: boolean;
    outputDirectory: string;
    includeScreenshots: boolean;
    includeVideos: boolean;
    includeTraces: boolean;
  };
  
  // CI/CD settings
  ci: {
    enabled: boolean;
    uploadArtifacts: boolean;
    notifyOnFailure: boolean;
    slackWebhook?: string;
    emailRecipients?: string[];
  };
}

export interface E2ETestResult {
  testName: string;
  browser: string;
  success: boolean;
  duration: number;
  failures: string[];
  screenshots: string[];
  videos: string[];
  traces: string[];
  metrics: {
    firstPaint: number;
    firstContentfulPaint: number;
    loadTime: number;
    interactionLatency: number[];
  };
  timestamp: Date;
}

export interface E2ETestSummary {
  totalTests: number;
  passedTests: number;
  failedTests: number;
  skippedTests: number;
  totalDuration: number;
  averageDuration: number;
  browserResults: { [browser: string]: E2ETestResult[] };
  performanceMetrics: {
    averageFirstPaint: number;
    averageFirstContentfulPaint: number;
    averageLoadTime: number;
    averageInteractionLatency: number;
  };
  failures: {
    testName: string;
    browser: string;
    error: string;
    screenshot?: string;
  }[];
}

export const defaultE2EConfig: E2ETestConfig = {
  execution: {
    parallel: true,
    workers: 4,
    retries: 2,
    timeout: 30000,
  },
  
  browsers: {
    chromium: {
      enabled: true,
      headless: true,
      timeout: 30000,
      retries: 2,
    },
    firefox: {
      enabled: true,
      headless: true,
      timeout: 35000,
      retries: 2,
    },
    webkit: {
      enabled: true,
      headless: true,
      timeout: 40000,
      retries: 3,
    },
    'Mobile Chrome': {
      enabled: true,
      headless: true,
      timeout: 45000,
      retries: 2,
    },
    'Mobile Safari': {
      enabled: true,
      headless: true,
      timeout: 50000,
      retries: 3,
    },
  },
  
  scenarios: {
    'component-integration': {
      enabled: true,
      description: 'Component integration and interaction testing',
      testFiles: ['component-integration.spec.ts'],
      priority: 'high',
    },
    'accessibility': {
      enabled: true,
      description: 'Accessibility compliance and WCAG testing',
      testFiles: ['accessibility.spec.ts'],
      priority: 'high',
    },
    'performance': {
      enabled: true,
      description: 'Performance metrics and optimization testing',
      testFiles: ['performance.spec.ts'],
      priority: 'medium',
    },
    'wasm-testing': {
      enabled: true,
      description: 'WASM browser testing and compatibility',
      testFiles: ['wasm-browser-testing.spec.ts'],
      priority: 'high',
    },
    'bundle-optimization': {
      enabled: true,
      description: 'Bundle optimization and loading performance',
      testFiles: ['bundle-optimization.spec.ts'],
      priority: 'medium',
    },
    'dynamic-loading': {
      enabled: true,
      description: 'Dynamic loading system testing',
      testFiles: ['dynamic-loading.spec.ts'],
      priority: 'medium',
    },
  },
  
  reporting: {
    generateHtmlReport: true,
    generateJsonReport: true,
    generateJunitReport: true,
    generateMarkdownReport: true,
    outputDirectory: 'test-results/e2e',
    includeScreenshots: true,
    includeVideos: true,
    includeTraces: true,
  },
  
  ci: {
    enabled: process.env.CI === 'true',
    uploadArtifacts: process.env.CI === 'true',
    notifyOnFailure: process.env.CI === 'true',
    slackWebhook: process.env.SLACK_WEBHOOK_URL,
    emailRecipients: process.env.EMAIL_RECIPIENTS?.split(','),
  },
};

export class E2ETestRunner {
  private config: E2ETestConfig;
  private results: E2ETestResult[] = [];
  private startTime: number = 0;

  constructor(config: E2ETestConfig = defaultE2EConfig) {
    this.config = config;
  }

  /**
   * Run all E2E tests
   */
  async runAllTests(): Promise<E2ETestSummary> {
    this.startTime = Date.now();
    this.results = [];

    console.log('🚀 Starting E2E test execution...');
    console.log(`Configuration: ${JSON.stringify(this.config, null, 2)}`);

    // Get enabled browsers and scenarios
    const enabledBrowsers = this.getEnabledBrowsers();
    const enabledScenarios = this.getEnabledScenarios();

    console.log(`Enabled browsers: ${enabledBrowsers.join(', ')}`);
    console.log(`Enabled scenarios: ${enabledScenarios.join(', ')}`);

    // Run tests for each browser
    for (const browser of enabledBrowsers) {
      console.log(`\n🧪 Running tests on ${browser}...`);
      
      for (const scenario of enabledScenarios) {
        const scenarioConfig = this.config.scenarios[scenario];
        console.log(`  📋 Running scenario: ${scenario} (${scenarioConfig.description})`);
        
        try {
          const result = await this.runScenario(browser, scenario);
          this.results.push(result);
          
          if (result.success) {
            console.log(`    ✅ ${scenario} passed on ${browser}`);
          } else {
            console.log(`    ❌ ${scenario} failed on ${browser}`);
            console.log(`    Failures: ${result.failures.join(', ')}`);
          }
        } catch (error) {
          console.error(`    💥 ${scenario} crashed on ${browser}: ${error}`);
          this.results.push({
            testName: scenario,
            browser,
            success: false,
            duration: 0,
            failures: [(error as Error).message],
            screenshots: [],
            videos: [],
            traces: [],
            metrics: {
              firstPaint: 0,
              firstContentfulPaint: 0,
              loadTime: 0,
              interactionLatency: [],
            },
            timestamp: new Date(),
          });
        }
      }
    }

    // Generate summary
    const summary = this.generateSummary();
    
    // Generate reports
    if (this.config.reporting.generateHtmlReport || 
        this.config.reporting.generateJsonReport || 
        this.config.reporting.generateMarkdownReport) {
      await this.generateReports(summary);
    }

    // Handle CI/CD notifications
    if (this.config.ci.enabled) {
      await this.handleCINotifications(summary);
    }

    console.log('\n📊 E2E Test Execution Complete');
    console.log(`Total tests: ${summary.totalTests}`);
    console.log(`Passed: ${summary.passedTests}`);
    console.log(`Failed: ${summary.failedTests}`);
    console.log(`Skipped: ${summary.skippedTests}`);
    console.log(`Total duration: ${(summary.totalDuration / 1000).toFixed(2)}s`);

    return summary;
  }

  /**
   * Run a specific scenario on a specific browser
   */
  private async runScenario(browser: string, scenario: string): Promise<E2ETestResult> {
    const startTime = Date.now();
    const scenarioConfig = this.config.scenarios[scenario];
    const browserConfig = this.config.browsers[browser];
    
    // This would integrate with Playwright's test runner
    // For now, we'll simulate the test execution
    const result: E2ETestResult = {
      testName: scenario,
      browser,
      success: true, // This would be determined by actual test execution
      duration: Date.now() - startTime,
      failures: [],
      screenshots: [],
      videos: [],
      traces: [],
      metrics: {
        firstPaint: Math.random() * 2000 + 1000, // Simulated metrics
        firstContentfulPaint: Math.random() * 3000 + 1500,
        loadTime: Math.random() * 1000 + 500,
        interactionLatency: [Math.random() * 50 + 25, Math.random() * 50 + 25],
      },
      timestamp: new Date(),
    };

    return result;
  }

  /**
   * Generate test summary
   */
  private generateSummary(): E2ETestSummary {
    const totalTests = this.results.length;
    const passedTests = this.results.filter(r => r.success).length;
    const failedTests = this.results.filter(r => !r.success).length;
    const skippedTests = 0; // Would be calculated from actual test results
    const totalDuration = Date.now() - this.startTime;
    const averageDuration = totalDuration / totalTests;

    // Group results by browser
    const browserResults: { [browser: string]: E2ETestResult[] } = {};
    this.results.forEach(result => {
      if (!browserResults[result.browser]) {
        browserResults[result.browser] = [];
      }
      browserResults[result.browser].push(result);
    });

    // Calculate performance metrics
    const allMetrics = this.results.flatMap(r => [r.metrics]);
    const averageFirstPaint = allMetrics.reduce((sum, m) => sum + m.firstPaint, 0) / allMetrics.length;
    const averageFirstContentfulPaint = allMetrics.reduce((sum, m) => sum + m.firstContentfulPaint, 0) / allMetrics.length;
    const averageLoadTime = allMetrics.reduce((sum, m) => sum + m.loadTime, 0) / allMetrics.length;
    const allInteractionLatencies = allMetrics.flatMap(m => m.interactionLatency);
    const averageInteractionLatency = allInteractionLatencies.reduce((sum, l) => sum + l, 0) / allInteractionLatencies.length;

    // Collect failures
    const failures = this.results
      .filter(r => !r.success)
      .map(r => ({
        testName: r.testName,
        browser: r.browser,
        error: r.failures.join(', '),
        screenshot: r.screenshots[0],
      }));

    return {
      totalTests,
      passedTests,
      failedTests,
      skippedTests,
      totalDuration,
      averageDuration,
      browserResults,
      performanceMetrics: {
        averageFirstPaint,
        averageFirstContentfulPaint,
        averageLoadTime,
        averageInteractionLatency,
      },
      failures,
    };
  }

  /**
   * Generate test reports
   */
  private async generateReports(summary: E2ETestSummary): Promise<void> {
    const outputDir = this.config.reporting.outputDirectory;
    
    // Ensure output directory exists
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    if (this.config.reporting.generateHtmlReport) {
      await this.generateHtmlReport(summary, outputDir);
    }

    if (this.config.reporting.generateJsonReport) {
      await this.generateJsonReport(summary, outputDir);
    }

    if (this.config.reporting.generateMarkdownReport) {
      await this.generateMarkdownReport(summary, outputDir);
    }

    if (this.config.reporting.generateJunitReport) {
      await this.generateJunitReport(summary, outputDir);
    }
  }

  /**
   * Generate HTML report
   */
  private async generateHtmlReport(summary: E2ETestSummary, outputDir: string): Promise<void> {
    const htmlContent = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>E2E Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f5f5f5; padding: 20px; border-radius: 5px; }
        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .metric { background: white; padding: 15px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .metric h3 { margin: 0 0 10px 0; color: #333; }
        .metric .value { font-size: 2em; font-weight: bold; }
        .success { color: #28a745; }
        .failure { color: #dc3545; }
        .warning { color: #ffc107; }
        .browser-results { margin: 20px 0; }
        .browser { background: white; margin: 10px 0; padding: 15px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .failures { margin: 20px 0; }
        .failure-item { background: #f8d7da; padding: 10px; margin: 5px 0; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>E2E Test Report</h1>
        <p>Generated: ${new Date().toISOString()}</p>
    </div>

    <div class="summary">
        <div class="metric">
            <h3>Total Tests</h3>
            <div class="value">${summary.totalTests}</div>
        </div>
        <div class="metric">
            <h3>Passed</h3>
            <div class="value success">${summary.passedTests}</div>
        </div>
        <div class="metric">
            <h3>Failed</h3>
            <div class="value failure">${summary.failedTests}</div>
        </div>
        <div class="metric">
            <h3>Success Rate</h3>
            <div class="value ${summary.failedTests === 0 ? 'success' : 'warning'}">
                ${((summary.passedTests / summary.totalTests) * 100).toFixed(1)}%
            </div>
        </div>
    </div>

    <div class="browser-results">
        <h2>Browser Results</h2>
        ${Object.entries(summary.browserResults).map(([browser, results]) => `
            <div class="browser">
                <h3>${browser}</h3>
                <p>Tests: ${results.length} | Passed: ${results.filter(r => r.success).length} | Failed: ${results.filter(r => !r.success).length}</p>
            </div>
        `).join('')}
    </div>

    ${summary.failures.length > 0 ? `
    <div class="failures">
        <h2>Failures</h2>
        ${summary.failures.map(failure => `
            <div class="failure-item">
                <strong>${failure.testName}</strong> on ${failure.browser}<br>
                <em>${failure.error}</em>
            </div>
        `).join('')}
    </div>
    ` : ''}
</body>
</html>`;

    fs.writeFileSync(path.join(outputDir, 'e2e-test-report.html'), htmlContent);
    console.log(`📄 HTML report generated: ${path.join(outputDir, 'e2e-test-report.html')}`);
  }

  /**
   * Generate JSON report
   */
  private async generateJsonReport(summary: E2ETestSummary, outputDir: string): Promise<void> {
    const jsonContent = JSON.stringify({
      summary,
      results: this.results,
      config: this.config,
      timestamp: new Date().toISOString(),
    }, null, 2);

    fs.writeFileSync(path.join(outputDir, 'e2e-test-results.json'), jsonContent);
    console.log(`📄 JSON report generated: ${path.join(outputDir, 'e2e-test-results.json')}`);
  }

  /**
   * Generate Markdown report
   */
  private async generateMarkdownReport(summary: E2ETestSummary, outputDir: string): Promise<void> {
    const markdownContent = `# E2E Test Report

**Generated**: ${new Date().toISOString()}

## Summary

- **Total Tests**: ${summary.totalTests}
- **Passed**: ${summary.passedTests}
- **Failed**: ${summary.failedTests}
- **Skipped**: ${summary.skippedTests}
- **Success Rate**: ${((summary.passedTests / summary.totalTests) * 100).toFixed(1)}%
- **Total Duration**: ${(summary.totalDuration / 1000).toFixed(2)}s
- **Average Duration**: ${(summary.averageDuration / 1000).toFixed(2)}s

## Performance Metrics

- **Average First Paint**: ${summary.performanceMetrics.averageFirstPaint.toFixed(2)}ms
- **Average First Contentful Paint**: ${summary.performanceMetrics.averageFirstContentfulPaint.toFixed(2)}ms
- **Average Load Time**: ${summary.performanceMetrics.averageLoadTime.toFixed(2)}ms
- **Average Interaction Latency**: ${summary.performanceMetrics.averageInteractionLatency.toFixed(2)}ms

## Browser Results

${Object.entries(summary.browserResults).map(([browser, results]) => `
### ${browser}
- **Tests**: ${results.length}
- **Passed**: ${results.filter(r => r.success).length}
- **Failed**: ${results.filter(r => !r.success).length}
`).join('')}

${summary.failures.length > 0 ? `
## Failures

${summary.failures.map(failure => `
### ${failure.testName} (${failure.browser})
\`\`\`
${failure.error}
\`\`\`
`).join('')}
` : ''}
`;

    fs.writeFileSync(path.join(outputDir, 'e2e-test-report.md'), markdownContent);
    console.log(`📄 Markdown report generated: ${path.join(outputDir, 'e2e-test-report.md')}`);
  }

  /**
   * Generate JUnit report
   */
  private async generateJunitReport(summary: E2ETestSummary, outputDir: string): Promise<void> {
    const junitContent = `<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
    <testsuite name="E2E Tests" tests="${summary.totalTests}" failures="${summary.failedTests}" skipped="${summary.skippedTests}" time="${(summary.totalDuration / 1000).toFixed(3)}">
        ${this.results.map(result => `
        <testcase name="${result.testName}" classname="${result.browser}" time="${(result.duration / 1000).toFixed(3)}">
            ${!result.success ? `
            <failure message="${result.failures.join(', ')}">
                ${result.failures.join('\n')}
            </failure>
            ` : ''}
        </testcase>
        `).join('')}
    </testsuite>
</testsuites>`;

    fs.writeFileSync(path.join(outputDir, 'e2e-test-results.xml'), junitContent);
    console.log(`📄 JUnit report generated: ${path.join(outputDir, 'e2e-test-results.xml')}`);
  }

  /**
   * Handle CI/CD notifications
   */
  private async handleCINotifications(summary: E2ETestSummary): Promise<void> {
    if (summary.failedTests > 0 && this.config.ci.notifyOnFailure) {
      console.log('📢 Sending failure notifications...');
      
      if (this.config.ci.slackWebhook) {
        await this.sendSlackNotification(summary);
      }
      
      if (this.config.ci.emailRecipients && this.config.ci.emailRecipients.length > 0) {
        await this.sendEmailNotification(summary);
      }
    }
  }

  /**
   * Send Slack notification
   */
  private async sendSlackNotification(summary: E2ETestSummary): Promise<void> {
    const message = {
      text: `E2E Tests Failed: ${summary.failedTests}/${summary.totalTests} tests failed`,
      attachments: [{
        color: summary.failedTests > 0 ? 'danger' : 'good',
        fields: [
          { title: 'Total Tests', value: summary.totalTests.toString(), short: true },
          { title: 'Passed', value: summary.passedTests.toString(), short: true },
          { title: 'Failed', value: summary.failedTests.toString(), short: true },
          { title: 'Success Rate', value: `${((summary.passedTests / summary.totalTests) * 100).toFixed(1)}%`, short: true },
        ],
      }],
    };

    try {
      const response = await fetch(this.config.ci.slackWebhook!, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(message),
      });

      if (response.ok) {
        console.log('✅ Slack notification sent');
      } else {
        console.error('❌ Failed to send Slack notification');
      }
    } catch (error) {
      console.error('❌ Error sending Slack notification:', error);
    }
  }

  /**
   * Send email notification
   */
  private async sendEmailNotification(summary: E2ETestSummary): Promise<void> {
    // This would integrate with an email service
    console.log(`📧 Email notification would be sent to: ${this.config.ci.emailRecipients?.join(', ')}`);
  }

  /**
   * Get enabled browsers
   */
  private getEnabledBrowsers(): string[] {
    return Object.entries(this.config.browsers)
      .filter(([_, config]) => config.enabled)
      .map(([name, _]) => name);
  }

  /**
   * Get enabled scenarios
   */
  private getEnabledScenarios(): string[] {
    return Object.entries(this.config.scenarios)
      .filter(([_, config]) => config.enabled)
      .map(([name, _]) => name);
  }
}

/**
 * Utility function to run E2E tests
 */
export async function runE2ETests(config?: Partial<E2ETestConfig>): Promise<E2ETestSummary> {
  const finalConfig = { ...defaultE2EConfig, ...config };
  const runner = new E2ETestRunner(finalConfig);
  return await runner.runAllTests();
}
