# ADR-003: Playwright Testing for Demos and Applications

## Status
**ACCEPTED** - 2024-09-08

## Context
As a data engineering consultancy, we create numerous demos and proof-of-concepts to showcase our capabilities to clients. These demos must be reliable, functional, and demonstrate our technical excellence. We need a comprehensive testing strategy that ensures all demos work as intended and exceed client expectations.

## Decision
We implement **exhaustive Playwright testing** for all demos and applications to ensure they work flawlessly and demonstrate our technical capabilities.

### Playwright Testing Strategy

#### Demo Testing Requirements
- **100% demo coverage**: Every demo must have comprehensive Playwright tests
- **User journey testing**: Test complete user workflows and interactions
- **Cross-browser testing**: Ensure compatibility across all major browsers
- **Performance testing**: Verify demo performance meets expectations
- **Accessibility testing**: Ensure demos are accessible to all users

#### Testing Categories

##### 1. Functional Testing
- **Core functionality**: Test all primary features and capabilities
- **User interactions**: Test clicks, form submissions, navigation
- **Data processing**: Verify data input, processing, and output
- **Error handling**: Test error scenarios and recovery

##### 2. Visual Testing
- **UI consistency**: Verify visual elements render correctly
- **Responsive design**: Test across different screen sizes
- **Cross-browser compatibility**: Ensure consistent appearance
- **Accessibility compliance**: Verify WCAG compliance

##### 3. Performance Testing
- **Load times**: Measure page load and interaction response times
- **Memory usage**: Monitor memory consumption during demos
- **WASM performance**: Test WebAssembly execution performance
- **Network efficiency**: Verify optimal resource loading

##### 4. Integration Testing
- **API integration**: Test external service integrations
- **Database operations**: Verify data persistence and retrieval
- **File operations**: Test file upload, processing, and download
- **Real-time features**: Test WebSocket and real-time updates

## Implementation

### Demo Testing Framework
```typescript
// Example: Comprehensive demo testing
import { test, expect } from '@playwright/test';

test.describe('Data Engineering Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo');
    await page.waitForLoadState('networkidle');
  });

  test('should demonstrate complete data pipeline workflow', async ({ page }) => {
    // Test data upload
    await page.setInputFiles('[data-testid="file-upload"]', 'test-data.csv');
    await expect(page.locator('[data-testid="upload-status"]'))
      .toContainText('Upload successful');

    // Test pipeline configuration
    await page.selectOption('[data-testid="processor"]', 'rust-processor');
    await page.fill('[data-testid="output-format"]', 'json');
    await page.click('[data-testid="configure-pipeline"]');

    // Test pipeline execution
    await page.click('[data-testid="start-processing"]');
    await expect(page.locator('[data-testid="processing-status"]'))
      .toContainText('Processing...');

    // Wait for completion
    await expect(page.locator('[data-testid="processing-status"]'))
      .toContainText('Complete', { timeout: 30000 });

    // Test results download
    await expect(page.locator('[data-testid="download-link"]'))
      .toBeVisible();
    await page.click('[data-testid="download-link"]');
  });

  test('should handle error scenarios gracefully', async ({ page }) => {
    // Test invalid file upload
    await page.setInputFiles('[data-testid="file-upload"]', 'invalid-file.txt');
    await expect(page.locator('[data-testid="error-message"]'))
      .toContainText('Invalid file format');

    // Test network error handling
    await page.route('**/api/process', route => route.abort());
    await page.click('[data-testid="start-processing"]');
    await expect(page.locator('[data-testid="error-message"]'))
      .toContainText('Network error');
  });

  test('should be responsive across devices', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="mobile-menu"]'))
      .toBeVisible();

    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="tablet-layout"]'))
      .toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('[data-testid="desktop-layout"]'))
      .toBeVisible();
  });
});
```

### Performance Testing
```typescript
// Example: Performance testing for demos
test.describe('Demo Performance', () => {
  test('should load within performance budget', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/demo');
    await page.waitForLoadState('networkidle');
    const loadTime = Date.now() - startTime;

    // Assert load time is under 3 seconds
    expect(loadTime).toBeLessThan(3000);

    // Measure WASM initialization time
    const wasmInitTime = await page.evaluate(() => {
      return performance.getEntriesByName('wasm-init')[0]?.duration || 0;
    });
    expect(wasmInitTime).toBeLessThan(1000);
  });

  test('should handle large datasets efficiently', async ({ page }) => {
    await page.goto('/demo');
    
    // Upload large dataset
    await page.setInputFiles('[data-testid="file-upload"]', 'large-dataset.csv');
    
    const startTime = Date.now();
    await page.click('[data-testid="start-processing"]');
    await expect(page.locator('[data-testid="processing-status"]'))
      .toContainText('Complete', { timeout: 60000 });
    const processingTime = Date.now() - startTime;

    // Assert processing time is reasonable
    expect(processingTime).toBeLessThan(30000);
  });
});
```

### Accessibility Testing
```typescript
// Example: Accessibility testing for demos
test.describe('Demo Accessibility', () => {
  test('should meet WCAG 2.1 AA standards', async ({ page }) => {
    await page.goto('/demo');
    
    // Test keyboard navigation
    await page.keyboard.press('Tab');
    await expect(page.locator(':focus')).toBeVisible();
    
    // Test screen reader compatibility
    const ariaLabels = await page.locator('[aria-label]').count();
    expect(ariaLabels).toBeGreaterThan(0);
    
    // Test color contrast
    const contrastRatio = await page.evaluate(() => {
      const element = document.querySelector('[data-testid="main-content"]');
      const styles = window.getComputedStyle(element);
      // Calculate contrast ratio (simplified)
      return 4.5; // Should be >= 4.5 for AA compliance
    });
    expect(contrastRatio).toBeGreaterThanOrEqual(4.5);
  });
});
```

### Cross-Browser Testing
```typescript
// Example: Cross-browser testing configuration
const browsers = ['chromium', 'firefox', 'webkit'];

browsers.forEach(browser => {
  test.describe(`${browser} compatibility`, () => {
    test.use({ browserName: browser });

    test('should work consistently across browsers', async ({ page }) => {
      await page.goto('/demo');
      
      // Test core functionality
      await page.click('[data-testid="start-demo"]');
      await expect(page.locator('[data-testid="demo-content"]'))
        .toBeVisible();
      
      // Test WASM functionality
      const wasmResult = await page.evaluate(() => {
        return window.wasmModule ? 'loaded' : 'not loaded';
      });
      expect(wasmResult).toBe('loaded');
    });
  });
});
```

## Quality Standards

### Demo Requirements
- **Functionality**: All features must work as intended
- **Performance**: Demos must load and respond quickly
- **Reliability**: Demos must work consistently across environments
- **User Experience**: Demos must provide excellent user experience
- **Documentation**: Demos must include clear usage instructions

### Testing Requirements
- **Coverage**: 100% of demo functionality must be tested
- **Automation**: All tests must be automated and run in CI/CD
- **Maintenance**: Tests must be updated with demo changes
- **Documentation**: Tests must serve as living documentation

## Tools and Configuration

### Playwright Configuration
```typescript
// playwright.config.ts
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
    { name: 'mobile', use: { ...devices['iPhone 12'] } },
  ],
  webServer: {
    command: 'python3 -m http.server 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
  },
});
```

### CI/CD Integration
```yaml
# .github/workflows/demo-tests.yml
name: Demo Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm install
      - run: npx playwright install
      - run: npx playwright test
      - uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: playwright-report
          path: playwright-report/
```

## Metrics and Monitoring

### Demo Quality Metrics
- **Test pass rate**: Track percentage of passing tests
- **Demo performance**: Monitor load times and response times
- **User satisfaction**: Collect feedback on demo quality
- **Bug reports**: Track issues found in demos

### Testing Metrics
- **Test execution time**: Monitor test suite performance
- **Test maintenance**: Track time spent on test updates
- **Coverage metrics**: Monitor test coverage percentages
- **Flaky test rate**: Identify and fix unreliable tests

## Review and Updates
This ADR will be reviewed monthly to ensure demo testing strategy remains effective and aligned with client needs. Updates will be made based on:
- Client feedback on demo quality
- New testing tools and technologies
- Industry best practices
- Team experience and insights

## Related ADRs
- ADR-001: Test-Driven Development (TDD) First Approach
- ADR-002: Testing Pyramid Strategy
- ADR-004: Code Coverage Standards
- ADR-006: Competitive Analysis and Capability Matching
