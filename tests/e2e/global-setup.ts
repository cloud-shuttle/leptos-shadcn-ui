import { chromium, FullConfig } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Enhanced Global Setup for E2E Tests
 * 
 * This setup function handles environment preparation, dependency checks,
 * and initial test data setup for comprehensive E2E testing.
 */

async function globalSetup(config: FullConfig) {
  console.log('🎭 Setting up enhanced Playwright test environment...');
  
  const startTime = Date.now();
  const setupResults = {
    environment: 'unknown',
    dependencies: [] as string[],
    services: [] as string[],
    errors: [] as string[],
    warnings: [] as string[],
  };
  
  try {
    // 1. Environment Detection
    setupResults.environment = process.env.CI ? 'ci' : 'local';
    console.log(`📍 Environment: ${setupResults.environment}`);
    
    // 2. Dependency Checks
    console.log('🔍 Checking dependencies...');
    
    // Check if WASM target is installed
    try {
      const { execSync } = require('child_process');
      const rustTargets = execSync('rustup target list --installed', { encoding: 'utf8' });
      if (rustTargets.includes('wasm32-unknown-unknown')) {
        setupResults.dependencies.push('wasm32-unknown-unknown');
        console.log('✅ WASM target is installed');
      } else {
        setupResults.warnings.push('WASM target not installed - some tests may fail');
        console.log('⚠️ WASM target not installed');
      }
    } catch (error) {
      setupResults.errors.push('Failed to check Rust targets');
      console.error('❌ Failed to check Rust targets:', error);
    }
    
    // Check if Playwright browsers are installed
    try {
      const { execSync } = require('child_process');
      execSync('pnpm playwright --version', { encoding: 'utf8' });
      setupResults.dependencies.push('playwright');
      console.log('✅ Playwright is installed');
    } catch (error) {
      setupResults.errors.push('Playwright not installed');
      console.error('❌ Playwright not installed:', error);
    }
    
    // 3. Service Health Checks
    console.log('🏥 Checking service health...');
    
    // Check if test server is accessible
    try {
      const browser = await chromium.launch({ headless: true });
      const page = await browser.newPage();
      
      // Try to access the test server
      const baseURL = config.use?.baseURL || 'http://localhost:8082';
      await page.goto(baseURL, { timeout: 10000 });
      
      setupResults.services.push('test-server');
      console.log('✅ Test server is accessible');
      
      await browser.close();
    } catch (error) {
      setupResults.warnings.push('Test server not accessible - will be started by webServer');
      console.log('⚠️ Test server not accessible, will be started automatically');
    }
    
    // 4. Test Data Preparation
    console.log('📊 Preparing test data...');
    
    // Create test results directory
    const testResultsDir = path.join(process.cwd(), 'test-results');
    if (!fs.existsSync(testResultsDir)) {
      fs.mkdirSync(testResultsDir, { recursive: true });
      console.log('✅ Created test results directory');
    }
    
    // Create browser-specific directories
    const browsers = ['chromium', 'firefox', 'webkit', 'Mobile Chrome', 'Mobile Safari'];
    browsers.forEach(browser => {
      const browserDir = path.join(testResultsDir, browser);
      if (!fs.existsSync(browserDir)) {
        fs.mkdirSync(browserDir, { recursive: true });
      }
    });
    
    // 5. Performance Baseline Setup
    console.log('📈 Setting up performance baselines...');
    
    const performanceBaseline = {
      maxInitializationTime: parseInt(process.env.MAX_INIT_TIME || '5000'),
      maxFirstPaint: parseInt(process.env.MAX_FIRST_PAINT || '3000'),
      maxFirstContentfulPaint: parseInt(process.env.MAX_FCP || '4000'),
      maxInteractionLatency: parseInt(process.env.MAX_INTERACTION_LATENCY || '100'),
      environment: setupResults.environment,
      timestamp: new Date().toISOString(),
    };
    
    fs.writeFileSync(
      path.join(testResultsDir, 'performance-baseline.json'),
      JSON.stringify(performanceBaseline, null, 2)
    );
    
    // 6. Environment Variables Setup
    console.log('🔧 Setting up environment variables...');
    
    // Set test-specific environment variables
    process.env.TEST_ENVIRONMENT = setupResults.environment;
    process.env.TEST_START_TIME = startTime.toString();
    process.env.TEST_BASE_URL = config.use?.baseURL || 'http://localhost:8082';
    
    // 7. Browser Capability Detection
    console.log('🌐 Detecting browser capabilities...');
    
    try {
      const browser = await chromium.launch({ headless: true });
      const page = await browser.newPage();
      
      const capabilities = await page.evaluate(() => {
        return {
          webAssembly: typeof WebAssembly !== 'undefined',
          sharedArrayBuffer: typeof SharedArrayBuffer !== 'undefined',
          bigInt: typeof BigInt !== 'undefined',
          userAgent: navigator.userAgent,
          language: navigator.language,
          platform: navigator.platform,
        };
      });
      
      fs.writeFileSync(
        path.join(testResultsDir, 'browser-capabilities.json'),
        JSON.stringify(capabilities, null, 2)
      );
      
      await browser.close();
      console.log('✅ Browser capabilities detected');
    } catch (error) {
      setupResults.warnings.push('Failed to detect browser capabilities');
      console.log('⚠️ Failed to detect browser capabilities');
    }
    
    // 8. Setup Summary
    const setupDuration = Date.now() - startTime;
    console.log(`\n📋 Setup Summary (${setupDuration}ms):`);
    console.log(`   Environment: ${setupResults.environment}`);
    console.log(`   Dependencies: ${setupResults.dependencies.join(', ')}`);
    console.log(`   Services: ${setupResults.services.join(', ')}`);
    
    if (setupResults.warnings.length > 0) {
      console.log(`   Warnings: ${setupResults.warnings.join(', ')}`);
    }
    
    if (setupResults.errors.length > 0) {
      console.log(`   Errors: ${setupResults.errors.join(', ')}`);
    }
    
    // Save setup results
    fs.writeFileSync(
      path.join(testResultsDir, 'setup-results.json'),
      JSON.stringify({
        ...setupResults,
        duration: setupDuration,
        timestamp: new Date().toISOString(),
      }, null, 2)
    );
    
    console.log('✅ Enhanced global setup complete');
    
  } catch (error) {
    console.error('❌ Global setup failed:', error);
    setupResults.errors.push(`Setup failed: ${error}`);
    
    // Save error results
    const testResultsDir = path.join(process.cwd(), 'test-results');
    fs.writeFileSync(
      path.join(testResultsDir, 'setup-results.json'),
      JSON.stringify({
        ...setupResults,
        duration: Date.now() - startTime,
        timestamp: new Date().toISOString(),
      }, null, 2)
    );
    
    throw error;
  }
}

export default globalSetup;
