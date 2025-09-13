import { test, expect, Page } from '@playwright/test';

/**
 * Demo Page E2E Tests
 * 
 * Comprehensive tests for the leptos-shadcn-ui demo page showcasing
 * performance advantages and component capabilities.
 */

test.describe('Demo Page E2E Tests', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    await page.goto('/demo/index.html');
    await page.waitForLoadState('networkidle');
  });

  // ===== PAGE LOAD AND STRUCTURE TESTS =====
  
  test('should load demo page successfully', async () => {
    await expect(page).toHaveTitle(/leptos-shadcn-ui Demo - Performance Champion/);
    
    // Check main navigation
    const nav = page.locator('nav');
    await expect(nav).toBeVisible();
    
    // Check hero section
    const hero = page.locator('section.gradient-bg');
    await expect(hero).toBeVisible();
    
    // Check main heading
    const mainHeading = page.locator('h1').first();
    await expect(mainHeading).toHaveText(/Performance Champion/);
  });

  test('should have proper navigation links', async () => {
    const navLinks = page.locator('nav a');
    
    // Check navigation links exist
    await expect(navLinks.filter({ hasText: 'Performance' })).toBeVisible();
    await expect(navLinks.filter({ hasText: 'Components' })).toBeVisible();
    await expect(navLinks.filter({ hasText: 'Comparison' })).toBeVisible();
    await expect(navLinks.filter({ hasText: 'Live Demo' })).toBeVisible();
    await expect(navLinks.filter({ hasText: 'GitHub' })).toBeVisible();
  });

  test('should have smooth scrolling navigation', async () => {
    // Test smooth scrolling to sections
    await page.click('a[href="#performance"]');
    await page.waitForTimeout(500); // Wait for scroll animation
    
    const performanceSection = page.locator('#performance');
    await expect(performanceSection).toBeInViewport();
    
    await page.click('a[href="#components"]');
    await page.waitForTimeout(500);
    
    const componentsSection = page.locator('#components');
    await expect(componentsSection).toBeInViewport();
  });

  // ===== PERFORMANCE METRICS TESTS =====
  
  test('should display performance metrics correctly', async () => {
    const performanceSection = page.locator('#performance');
    await expect(performanceSection).toBeVisible();
    
    // Check performance grid
    const performanceGrid = performanceSection.locator('.performance-grid');
    await expect(performanceGrid).toBeVisible();
    
    // Check individual metrics
    const metrics = [
      '3-5x', '5x', '3-8x', '0', '60 FPS', '100%'
    ];
    
    for (const metric of metrics) {
      const metricElement = performanceGrid.locator(`text=${metric}`).first();
      await expect(metricElement).toBeVisible();
    }
  });

  test('should show performance comparison cards', async () => {
    const comparisonCards = page.locator('.metric-card');
    await expect(comparisonCards).toHaveCount(3);
    
    // Check card titles
    await expect(comparisonCards.nth(0)).toContainText('Initial Load Time');
    await expect(comparisonCards.nth(1)).toContainText('Memory Usage');
    await expect(comparisonCards.nth(2)).toContainText('Bundle Size');
    
    // Check leptos-shadcn-ui metrics (should be green)
    const leptosMetrics = comparisonCards.locator('.bg-green-100');
    await expect(leptosMetrics).toHaveCount(3);
  });

  // ===== COMPONENT SHOWCASE TESTS =====
  
  test('should display component showcase', async () => {
    const componentsSection = page.locator('#components');
    await expect(componentsSection).toBeVisible();
    
    // Check component grid
    const componentGrid = componentsSection.locator('.component-grid');
    await expect(componentGrid).toBeVisible();
    
    // Check component cards
    const componentCards = componentGrid.locator('.component-showcase');
    await expect(componentCards).toHaveCount(6);
    
    // Check component titles
    const componentTitles = [
      'Button', 'Input', 'Card', 'Modal', 'Data Table', 'Form'
    ];
    
    for (const title of componentTitles) {
      const titleElement = componentCards.locator(`h3:has-text("${title}")`);
      await expect(titleElement).toBeVisible();
    }
  });

  test('should show component performance metrics', async () => {
    const componentCards = page.locator('.component-showcase');
    
    // Check that each component card has performance metrics
    for (let i = 0; i < 6; i++) {
      const card = componentCards.nth(i);
      const metrics = card.locator('.text-sm.text-gray-600');
      await expect(metrics).toBeVisible();
      
      // Should have render time and memory metrics
      await expect(metrics).toContainText('Render Time:');
      await expect(metrics).toContainText('Memory:');
    }
  });

  // ===== INTERACTIVE DEMO TESTS =====
  
  test('should have working performance test', async () => {
    const perfTestButton = page.locator('#perfTest');
    await expect(perfTestButton).toBeVisible();
    await expect(perfTestButton).toHaveText('Run Performance Test');
    
    // Click performance test button
    await perfTestButton.click();
    
    // Wait for results to appear
    await page.waitForTimeout(100);
    
    const results = page.locator('#perfResults');
    await expect(results).toBeVisible();
    
    // Check that results show performance metrics
    await expect(results).toContainText('Click Response:');
    await expect(results).toContainText('Render Time:');
    await expect(results).toContainText('Memory Usage:');
  });

  test('should have working memory test', async () => {
    const memoryTestButton = page.locator('#memoryTest');
    await expect(memoryTestButton).toBeVisible();
    await expect(memoryTestButton).toHaveText('Start Memory Test');
    
    // Click memory test button
    await memoryTestButton.click();
    
    // Wait for memory test to run
    await page.waitForTimeout(1000);
    
    const memoryUsage = page.locator('#memoryUsage');
    await expect(memoryUsage).toBeVisible();
    
    const memoryBar = page.locator('#memoryBar');
    await expect(memoryBar).toBeVisible();
  });

  test('should have working speed test', async () => {
    const speedTestButton = page.locator('#speedTest');
    await expect(speedTestButton).toBeVisible();
    await expect(speedTestButton).toHaveText('Run Speed Test');
    
    // Click speed test button
    await speedTestButton.click();
    
    // Wait for results to appear
    await page.waitForTimeout(100);
    
    const results = page.locator('#speedResults');
    await expect(results).toBeVisible();
    
    // Check that results show speed metrics
    await expect(results).toContainText('Button Render:');
    await expect(results).toContainText('Input Render:');
    await expect(results).toContainText('Card Render:');
  });

  // ===== COMPARISON TABLE TESTS =====
  
  test('should display detailed comparison table', async () => {
    const comparisonSection = page.locator('#comparison');
    await expect(comparisonSection).toBeVisible();
    
    const comparisonTable = comparisonSection.locator('.comparison-table table');
    await expect(comparisonTable).toBeVisible();
    
    // Check table headers
    const headers = comparisonTable.locator('th');
    await expect(headers).toHaveCount(7);
    
    const expectedHeaders = [
      'Framework', 'Language', 'Initial Load', 'Memory Usage', 
      'Bundle Size', 'Type Safety', 'Memory Safety'
    ];
    
    for (const header of expectedHeaders) {
      await expect(headers.filter({ hasText: header })).toBeVisible();
    }
  });

  test('should highlight leptos-shadcn-ui advantages', async () => {
    const comparisonTable = page.locator('.comparison-table table');
    
    // Check that leptos-shadcn-ui row has winner styling
    const leptosRow = comparisonTable.locator('tr').filter({ hasText: 'leptos-shadcn-ui' });
    await expect(leptosRow).toBeVisible();
    
    // Check that leptos metrics are highlighted
    const leptosCells = leptosRow.locator('td.leptos-winner');
    await expect(leptosCells).toHaveCount(6); // All metric cells should be highlighted
  });

  // ===== RESPONSIVE DESIGN TESTS =====
  
  test('should be responsive on mobile', async () => {
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check that navigation is still accessible
    const nav = page.locator('nav');
    await expect(nav).toBeVisible();
    
    // Check that hero section adapts
    const hero = page.locator('section.gradient-bg');
    await expect(hero).toBeVisible();
    
    // Check that performance grid adapts
    const performanceGrid = page.locator('.performance-grid');
    await expect(performanceGrid).toBeVisible();
  });

  test('should be responsive on tablet', async () => {
    await page.setViewportSize({ width: 768, height: 1024 });
    
    // Check that all sections are visible
    const sections = page.locator('section');
    await expect(sections).toHaveCount(6); // Should have 6 main sections
    
    // Check that component grid adapts
    const componentGrid = page.locator('.component-grid');
    await expect(componentGrid).toBeVisible();
  });

  // ===== ACCESSIBILITY TESTS =====
  
  test('should be keyboard accessible', async () => {
    // Test keyboard navigation
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    
    // Check that focus is visible
    const focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
  });

  test('should have proper ARIA labels', async () => {
    // Check that interactive elements have proper labels
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const text = await button.textContent();
      const ariaLabel = await button.getAttribute('aria-label');
      
      // Should have either text content or aria-label
      expect(text || ariaLabel).toBeTruthy();
    }
  });

  test('should have proper heading hierarchy', async () => {
    // Check heading structure
    const h1 = page.locator('h1');
    await expect(h1).toHaveCount(1);
    
    const h2 = page.locator('h2');
    await expect(h2).toHaveCount(4); // Should have 4 main sections
    
    const h3 = page.locator('h3');
    await expect(h3).toHaveCount(6); // Should have 6 component cards
  });

  // ===== PERFORMANCE TESTS =====
  
  test('should load within performance budget', async () => {
    const startTime = Date.now();
    
    await page.goto('/demo/index.html');
    await page.waitForLoadState('networkidle');
    
    const loadTime = Date.now() - startTime;
    
    // Demo page should load within 2 seconds
    expect(loadTime).toBeLessThan(2000);
  });

  test('should have optimal bundle size', async () => {
    // Check that external resources load efficiently
    const responses = await page.evaluate(() => {
      return performance.getEntriesByType('resource')
        .filter((entry: any) => entry.name.includes('tailwindcss.com'))
        .map((entry: any) => ({
          name: entry.name,
          size: entry.transferSize || 0,
          duration: entry.duration
        }));
    });
    
    // Tailwind CSS should load efficiently
    if (responses.length > 0) {
      expect(responses[0].duration).toBeLessThan(1000);
    }
  });

  // ===== INTERACTION TESTS =====
  
  test('should handle button interactions smoothly', async () => {
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    // Test clicking all buttons
    for (let i = 0; i < Math.min(buttonCount, 5); i++) {
      const button = buttons.nth(i);
      await button.click();
      await page.waitForTimeout(100);
    }
    
    // Page should still be responsive
    const hero = page.locator('section.gradient-bg');
    await expect(hero).toBeVisible();
  });

  test('should handle hover effects', async () => {
    const hoverElements = page.locator('.card-hover');
    const elementCount = await hoverElements.count();
    
    // Test hover effects
    for (let i = 0; i < Math.min(elementCount, 3); i++) {
      const element = hoverElements.nth(i);
      await element.hover();
      await page.waitForTimeout(200);
    }
    
    // Elements should still be visible after hover
    await expect(hoverElements.first()).toBeVisible();
  });

  // ===== CONTENT VALIDATION TESTS =====
  
  test('should have correct performance messaging', async () => {
    // Check hero section messaging
    const heroText = page.locator('section.gradient-bg p');
    await expect(heroText).toContainText('3-5x Faster than React/Next.js');
    
    // Check performance section messaging
    const performanceText = page.locator('#performance p');
    await expect(performanceText).toContainText('Measurable performance advantages');
  });

  test('should have correct component information', async () => {
    const componentsText = page.locator('#components p');
    await expect(componentsText).toContainText('38 production-ready components');
  });

  test('should have correct comparison information', async () => {
    const comparisonText = page.locator('#comparison p');
    await expect(comparisonText).toContainText('Comprehensive performance comparison');
  });

  // ===== EXTERNAL LINKS TESTS =====
  
  test('should have working external links', async () => {
    const githubLink = page.locator('a[href*="github.com"]');
    await expect(githubLink).toBeVisible();
    
    // Check that link opens in new tab
    const target = await githubLink.getAttribute('target');
    expect(target).toBe('_blank');
  });

  // ===== ERROR HANDLING TESTS =====
  
  test('should handle missing resources gracefully', async () => {
    // Simulate network failure for external resources
    await page.route('**/tailwindcss.com/**', (route) => {
      route.abort('failed');
    });
    
    // Page should still load and function
    await page.reload();
    await page.waitForLoadState('networkidle');
    
    const hero = page.locator('section.gradient-bg');
    await expect(hero).toBeVisible();
  });

  // ===== CROSS-BROWSER COMPATIBILITY TESTS =====
  
  test('should work consistently across browsers', async () => {
    // Test basic functionality
    const hero = page.locator('section.gradient-bg');
    await expect(hero).toBeVisible();
    
    // Test navigation
    await page.click('a[href="#performance"]');
    await page.waitForTimeout(500);
    
    const performanceSection = page.locator('#performance');
    await expect(performanceSection).toBeInViewport();
    
    // Test interactions
    const perfTestButton = page.locator('#perfTest');
    await perfTestButton.click();
    await page.waitForTimeout(100);
    
    const results = page.locator('#perfResults');
    await expect(results).toBeVisible();
  });
});

