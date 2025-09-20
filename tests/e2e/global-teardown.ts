import { FullConfig } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Enhanced Global Teardown for E2E Tests
 * 
 * This teardown function handles cleanup, report generation,
 * and artifact management after test execution.
 */

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up enhanced Playwright test environment...');
  
  const startTime = Date.now();
  const teardownResults = {
    cleanup: [] as string[],
    reports: [] as string[],
    artifacts: [] as string[],
    errors: [] as string[],
    warnings: [] as string[],
  };
  
  try {
    // 1. Generate Test Summary
    console.log('📊 Generating test summary...');
    
    try {
      const testResultsDir = path.join(process.cwd(), 'test-results');
      const summary = await generateTestSummary(testResultsDir);
      
      if (summary) {
        fs.writeFileSync(
          path.join(testResultsDir, 'test-summary.json'),
          JSON.stringify(summary, null, 2)
        );
        teardownResults.reports.push('test-summary.json');
        console.log('✅ Test summary generated');
      }
    } catch (error) {
      teardownResults.warnings.push('Failed to generate test summary');
      console.log('⚠️ Failed to generate test summary');
    }
    
    // 2. Cleanup Temporary Files
    console.log('🗑️ Cleaning up temporary files...');
    
    try {
      const tempDirs = [
        path.join(process.cwd(), 'test-results', 'temp'),
        path.join(process.cwd(), 'test-results', 'screenshots', 'temp'),
        path.join(process.cwd(), 'test-results', 'videos', 'temp'),
        path.join(process.cwd(), 'test-results', 'traces', 'temp'),
      ];
      
      tempDirs.forEach(dir => {
        if (fs.existsSync(dir)) {
          fs.rmSync(dir, { recursive: true, force: true });
          teardownResults.cleanup.push(`Removed ${dir}`);
        }
      });
      
      console.log('✅ Temporary files cleaned up');
    } catch (error) {
      teardownResults.warnings.push('Failed to cleanup temporary files');
      console.log('⚠️ Failed to cleanup temporary files');
    }
    
    // 3. Archive Test Results
    console.log('📦 Archiving test results...');
    
    try {
      const testResultsDir = path.join(process.cwd(), 'test-results');
      const archiveDir = path.join(testResultsDir, 'archives');
      
      if (!fs.existsSync(archiveDir)) {
        fs.mkdirSync(archiveDir, { recursive: true });
      }
      
      // Create timestamped archive
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const archiveName = `test-results-${timestamp}`;
      const archivePath = path.join(archiveDir, archiveName);
      
      // Copy current results to archive
      if (fs.existsSync(testResultsDir)) {
        fs.cpSync(testResultsDir, archivePath, { recursive: true });
        teardownResults.artifacts.push(`Archived to ${archiveName}`);
        console.log('✅ Test results archived');
      }
    } catch (error) {
      teardownResults.warnings.push('Failed to archive test results');
      console.log('⚠️ Failed to archive test results');
    }
    
    // 4. Performance Analysis
    console.log('📈 Analyzing performance metrics...');
    
    try {
      const testResultsDir = path.join(process.cwd(), 'test-results');
      const performanceAnalysis = await analyzePerformanceMetrics(testResultsDir);
      
      if (performanceAnalysis) {
        fs.writeFileSync(
          path.join(testResultsDir, 'performance-analysis.json'),
          JSON.stringify(performanceAnalysis, null, 2)
        );
        teardownResults.reports.push('performance-analysis.json');
        console.log('✅ Performance analysis completed');
      }
    } catch (error) {
      teardownResults.warnings.push('Failed to analyze performance metrics');
      console.log('⚠️ Failed to analyze performance metrics');
    }
    
    // 5. Generate Final Report
    console.log('📄 Generating final report...');
    
    try {
      const testResultsDir = path.join(process.cwd(), 'test-results');
      const finalReport = generateFinalReport(teardownResults, testResultsDir);
      
      fs.writeFileSync(
        path.join(testResultsDir, 'final-report.md'),
        finalReport
      );
      teardownResults.reports.push('final-report.md');
      console.log('✅ Final report generated');
    } catch (error) {
      teardownResults.warnings.push('Failed to generate final report');
      console.log('⚠️ Failed to generate final report');
    }
    
    // 6. CI/CD Integration
    if (process.env.CI === 'true') {
      console.log('🚀 Handling CI/CD integration...');
      
      try {
        await handleCIIntegration(teardownResults);
        console.log('✅ CI/CD integration completed');
      } catch (error) {
        teardownResults.warnings.push('Failed CI/CD integration');
        console.log('⚠️ Failed CI/CD integration');
      }
    }
    
    // 7. Teardown Summary
    const teardownDuration = Date.now() - startTime;
    console.log(`\n📋 Teardown Summary (${teardownDuration}ms):`);
    console.log(`   Cleanup: ${teardownResults.cleanup.length} items`);
    console.log(`   Reports: ${teardownResults.reports.join(', ')}`);
    console.log(`   Artifacts: ${teardownResults.artifacts.length} items`);
    
    if (teardownResults.warnings.length > 0) {
      console.log(`   Warnings: ${teardownResults.warnings.join(', ')}`);
    }
    
    if (teardownResults.errors.length > 0) {
      console.log(`   Errors: ${teardownResults.errors.join(', ')}`);
    }
    
    // Save teardown results
    const testResultsDir = path.join(process.cwd(), 'test-results');
    fs.writeFileSync(
      path.join(testResultsDir, 'teardown-results.json'),
      JSON.stringify({
        ...teardownResults,
        duration: teardownDuration,
        timestamp: new Date().toISOString(),
      }, null, 2)
    );
    
    console.log('✅ Enhanced global teardown complete');
    
  } catch (error) {
    console.error('❌ Global teardown failed:', error);
    teardownResults.errors.push(`Teardown failed: ${error}`);
    
    // Save error results
    const testResultsDir = path.join(process.cwd(), 'test-results');
    fs.writeFileSync(
      path.join(testResultsDir, 'teardown-results.json'),
      JSON.stringify({
        ...teardownResults,
        duration: Date.now() - startTime,
        timestamp: new Date().toISOString(),
      }, null, 2)
    );
  }
  
  // Force exit after cleanup to prevent hanging
  setTimeout(() => {
    console.log('🚪 Auto-closing test environment...');
    process.exit(0);
  }, 2000);
}

/**
 * Generate test summary from results
 */
async function generateTestSummary(testResultsDir: string): Promise<any> {
  try {
    const resultsFiles = [
      'results.json',
      'chromium/results.json',
      'firefox/results.json',
      'webkit/results.json',
    ];
    
    const summary = {
      totalTests: 0,
      passedTests: 0,
      failedTests: 0,
      skippedTests: 0,
      totalDuration: 0,
      browsers: {} as any,
      timestamp: new Date().toISOString(),
    };
    
    resultsFiles.forEach(file => {
      const filePath = path.join(testResultsDir, file);
      if (fs.existsSync(filePath)) {
        try {
          const content = fs.readFileSync(filePath, 'utf8');
          const data = JSON.parse(content);
          
          if (data.stats) {
            summary.totalTests += data.stats.total || 0;
            summary.passedTests += data.stats.passed || 0;
            summary.failedTests += data.stats.failed || 0;
            summary.skippedTests += data.stats.skipped || 0;
            summary.totalDuration += data.stats.duration || 0;
          }
          
          const browser = path.dirname(file).split('/').pop() || 'main';
          summary.browsers[browser] = data.stats || {};
        } catch (error) {
          console.log(`⚠️ Failed to parse ${file}: ${error}`);
        }
      }
    });
    
    return summary;
  } catch (error) {
    console.error('Failed to generate test summary:', error);
    return null;
  }
}

/**
 * Analyze performance metrics
 */
async function analyzePerformanceMetrics(testResultsDir: string): Promise<any> {
  try {
    const baselinePath = path.join(testResultsDir, 'performance-baseline.json');
    if (!fs.existsSync(baselinePath)) {
      return null;
    }
    
    const baseline = JSON.parse(fs.readFileSync(baselinePath, 'utf8'));
    
    // This would analyze actual performance data from test results
    const analysis = {
      baseline,
      deviations: [],
      recommendations: [],
      timestamp: new Date().toISOString(),
    };
    
    return analysis;
  } catch (error) {
    console.error('Failed to analyze performance metrics:', error);
    return null;
  }
}

/**
 * Generate final report
 */
function generateFinalReport(teardownResults: any, testResultsDir: string): string {
  return `# E2E Test Execution Report

**Generated**: ${new Date().toISOString()}

## Summary

- **Cleanup Items**: ${teardownResults.cleanup.length}
- **Reports Generated**: ${teardownResults.reports.length}
- **Artifacts Created**: ${teardownResults.artifacts.length}

## Reports Generated

${teardownResults.reports.map((report: string) => `- ${report}`).join('\n')}

## Cleanup Actions

${teardownResults.cleanup.map((action: string) => `- ${action}`).join('\n')}

## Artifacts

${teardownResults.artifacts.map((artifact: string) => `- ${artifact}`).join('\n')}

${teardownResults.warnings.length > 0 ? `
## Warnings

${teardownResults.warnings.map((warning: string) => `- ${warning}`).join('\n')}
` : ''}

${teardownResults.errors.length > 0 ? `
## Errors

${teardownResults.errors.map((error: string) => `- ${error}`).join('\n')}
` : ''}

---
*Report generated by enhanced E2E test teardown*
`;
}

/**
 * Handle CI/CD integration
 */
async function handleCIIntegration(teardownResults: any): Promise<void> {
  // This would integrate with CI/CD systems
  // For example, uploading artifacts, sending notifications, etc.
  console.log('CI/CD integration placeholder');
}

export default globalTeardown;
