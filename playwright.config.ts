import { defineConfig, devices } from '@playwright/test';

/**
 * Enhanced Playwright Configuration for leptos-shadcn-ui
 * 
 * This configuration provides comprehensive E2E testing with CI/CD integration,
 * performance monitoring, and cross-browser compatibility testing.
 */

// Environment-based configuration
const isCI = !!process.env.CI;
const isDebug = !!process.env.DEBUG;
const isHeadless = process.env.HEADLESS !== 'false';

// Performance thresholds
const PERFORMANCE_THRESHOLDS = {
  maxInitializationTime: parseInt(process.env.MAX_INIT_TIME || '5000'),
  maxFirstPaint: parseInt(process.env.MAX_FIRST_PAINT || '3000'),
  maxFirstContentfulPaint: parseInt(process.env.MAX_FCP || '4000'),
  maxInteractionLatency: parseInt(process.env.MAX_INTERACTION_LATENCY || '100'),
};

// Browser-specific configurations
const browserConfigs = {
  chromium: {
    timeout: 30000,
    retries: isCI ? 2 : 0,
    headless: isHeadless,
  },
  firefox: {
    timeout: 35000,
    retries: isCI ? 2 : 0,
    headless: isHeadless,
  },
  webkit: {
    timeout: 40000,
    retries: isCI ? 3 : 0,
    headless: isHeadless,
  },
  'Mobile Chrome': {
    timeout: 45000,
    retries: isCI ? 2 : 0,
    headless: isHeadless,
  },
  'Mobile Safari': {
    timeout: 50000,
    retries: isCI ? 3 : 0,
    headless: isHeadless,
  },
};

export default defineConfig({
  testDir: './tests/e2e',
  
  /* Run tests in files in parallel */
  fullyParallel: !isDebug,
  
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: isCI,
  
  /* Retry on CI only */
  retries: isCI ? 2 : 0,
  
  /* Opt out of parallel tests on CI or debug mode. */
  workers: isCI ? 1 : isDebug ? 1 : undefined,
  
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [
    // HTML reporter for local development
    ['html', { 
      open: isDebug ? 'always' : 'never',
      outputFolder: 'test-results/html-report'
    }],
    
    // JSON reporter for CI/CD integration
    ['json', { 
      outputFile: 'test-results/results.json' 
    }],
    
    // JUnit reporter for CI/CD systems
    ['junit', { 
      outputFile: 'test-results/results.xml' 
    }],
    
    // Line reporter for CI
    ...(isCI ? [['line']] : []),
    
    // List reporter for debug mode
    ...(isDebug ? [['list']] : []),
  ],
  
  /* Shared settings for all the projects below. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: process.env.BASE_URL || 'http://localhost:8082',
    
    /* Collect trace when retrying the failed test. */
    trace: isCI ? 'on-first-retry' : 'retain-on-failure',
    
    /* Take screenshot on failure */
    screenshot: 'only-on-failure',
    
    /* Record video on failure */
    video: isCI ? 'retain-on-failure' : 'retain-on-failure',
    
    /* Global test timeout */
    actionTimeout: 10000,
    navigationTimeout: 30000,
    
    /* Ignore HTTPS errors */
    ignoreHTTPSErrors: true,
    
    /* Extra HTTP headers */
    extraHTTPHeaders: {
      'Accept-Language': 'en-US,en;q=0.9',
    },
    
    /* Viewport size */
    viewport: { width: 1280, height: 720 },
    
    /* User agent */
    userAgent: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
  },
  
  /* Configure projects for major browsers */
  projects: [
    // Desktop browsers - Only Chromium for faster CI
    {
      name: 'chromium',
      use: { 
        ...devices['Desktop Chrome'],
        ...browserConfigs.chromium,
      },
    },
    // Disabled for faster CI - only Chromium installed
    // {
    //   name: 'firefox',
    //   use: { 
    //     ...devices['Desktop Firefox'],
    //     ...browserConfigs.firefox,
    //   },
    // },
    // {
    //   name: 'webkit',
    //   use: { 
    //     ...devices['Desktop Safari'],
    //     ...browserConfigs.webkit,
    //   },
    // },
    
    // Mobile browsers - Disabled for faster CI
    // {
    //   name: 'Mobile Chrome',
    //   use: { 
    //     ...devices['Pixel 5'],
    //     ...browserConfigs['Mobile Chrome'],
    //   },
    // },
    // {
    //   name: 'Mobile Safari',
    //   use: { 
    //     ...devices['iPhone 12'],
    //     ...browserConfigs['Mobile Safari'],
    //   },
    // },
    
    // Test-specific projects
    {
      name: 'accessibility-tests',
      testMatch: '**/accessibility*.spec.ts',
      use: { 
        ...devices['Desktop Chrome'],
        ...browserConfigs.chromium,
      },
    },
    {
      name: 'performance-tests',
      testMatch: '**/performance*.spec.ts',
      use: { 
        ...devices['Desktop Chrome'],
        ...browserConfigs.chromium,
      },
    },
    {
      name: 'wasm-tests',
      testMatch: '**/wasm*.spec.ts',
      use: { 
        ...devices['Desktop Chrome'],
        ...browserConfigs.chromium,
      },
    },
  ],
  
  /* Run your local dev server before starting the tests */
  webServer: [
    {
      command: 'cd examples/comprehensive-demo && python3 -m http.server 8001',
      port: 8001,
      reuseExistingServer: true, // Always reuse existing server to avoid port conflicts
      timeout: 30 * 1000,
      stdout: 'pipe',
      stderr: 'pipe',
    },
  ],
  
  /* Global setup and teardown */
  globalSetup: require.resolve('./tests/e2e/global-setup.ts'),
  globalTeardown: require.resolve('./tests/e2e/global-teardown.ts'),
  
  /* Test timeout */
  timeout: 30 * 1000,
  expect: {
    timeout: 5 * 1000,
  },
  
  /* Output directory for test artifacts */
  outputDir: 'test-results/',
  
  /* Global test timeout */
  globalTimeout: isCI ? 60 * 60 * 1000 : 30 * 60 * 1000, // 1 hour in CI, 30 minutes locally
  
  /* Maximum number of test failures */
  maxFailures: isCI ? 10 : undefined,
  
  /* Update snapshots */
  updateSnapshots: process.env.UPDATE_SNAPSHOTS === 'true' ? 'all' : 'none',
  
  /* Ignore test files */
  testIgnore: [
    '**/node_modules/**',
    '**/dist/**',
    '**/build/**',
    '**/.git/**',
  ],
  
  /* Test match patterns */
  testMatch: [
    '**/*.spec.ts',
    '**/*.test.ts',
  ],
  
  /* Metadata for test results */
  metadata: {
    testEnvironment: isCI ? 'ci' : 'local',
    browserVersions: {
      chromium: process.env.CHROMIUM_VERSION || 'latest',
      firefox: process.env.FIREFOX_VERSION || 'latest',
      webkit: process.env.WEBKIT_VERSION || 'latest',
    },
    performanceThresholds: PERFORMANCE_THRESHOLDS,
  },
});

// Export configuration for use in other files
export { PERFORMANCE_THRESHOLDS, browserConfigs };
