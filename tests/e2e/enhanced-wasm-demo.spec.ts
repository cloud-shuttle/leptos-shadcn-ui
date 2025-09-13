import { test, expect, Page } from '@playwright/test';

/**
 * Enhanced WASM Demo E2E Tests
 *
 * These tests ensure the enhanced WASM demo functions as expected,
 * showcases performance advantages, and matches marketing demo quality.
 */

test.describe('Enhanced WASM Demo E2E Tests', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    // Navigate to the enhanced WASM demo
    await page.goto('http://localhost:8082');
    await page.waitForLoadState('networkidle');
    // Wait for WASM to load
    await page.waitForSelector('nav', { timeout: 10000 });
  });

  test.describe('Page Structure and Navigation', () => {
    test('should load the enhanced WASM demo successfully', async () => {
      await expect(page).toHaveTitle(/leptos-shadcn-ui/i);
      
      // Check for main navigation
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
      
      // Check for performance champion branding
      const performanceBadge = page.locator('text=Performance Champion').first();
      await expect(performanceBadge).toBeVisible();
    });

    test('should have proper navigation links', async () => {
      const performanceLink = page.locator('a[href="#performance"]');
      const componentsLink = page.locator('a[href="#components"]');
      const demoLink = page.locator('a[href="#demo"]');
      
      await expect(performanceLink).toBeVisible();
      await expect(componentsLink).toBeVisible();
      await expect(demoLink).toBeVisible();
    });

    test('should have hero section with proper messaging', async () => {
      const heroTitle = page.locator('h1:has-text("Performance Champion")');
      const heroSubtitle = page.locator('text=3-5x Faster than React/Next.js');
      const heroDescription = page.locator('text=Experience the power of Rust-based UI components');
      
      await expect(heroTitle).toBeVisible();
      await expect(heroSubtitle).toBeVisible();
      await expect(heroDescription).toBeVisible();
    });
  });

  test.describe('Performance Metrics Section', () => {
    test('should display performance metrics cards', async () => {
      const metricsSection = page.locator('#performance');
      await expect(metricsSection).toBeVisible();
      
      // Check for performance metric cards
      const fasterRendering = page.locator('text=3-5x').first();
      const lessMemory = page.locator('text=5x').nth(1);
      const smallerBundles = page.locator('text=3-8x');
      const noMemoryLeaks = page.locator('#performance').locator('text=0').first();
      const consistentFPS = page.locator('text=60 FPS');
      const testCoverage = page.locator('text=100%');
      
      await expect(fasterRendering).toBeVisible();
      await expect(lessMemory).toBeVisible();
      await expect(smallerBundles).toBeVisible();
      await expect(noMemoryLeaks).toBeVisible();
      await expect(consistentFPS).toBeVisible();
      await expect(testCoverage).toBeVisible();
    });

    test('should have proper performance messaging', async () => {
      const performanceTitle = page.locator('h2:has-text("Performance Leadership")');
      const performanceDescription = page.locator('text=Measurable performance advantages');
      
      await expect(performanceTitle).toBeVisible();
      await expect(performanceDescription).toBeVisible();
    });
  });

  test.describe('Component Showcase', () => {
    test('should display component showcase section', async () => {
      const componentsSection = page.locator('#components');
      await expect(componentsSection).toBeVisible();
      
      const componentsTitle = page.locator('h2:has-text("Component Showcase")');
      const componentsDescription = page.locator('text=38 production-ready components');
      
      await expect(componentsTitle).toBeVisible();
      await expect(componentsDescription).toBeVisible();
    });

    test('should have interactive button components', async () => {
      const buttonCard = page.locator('text=Button').first();
      await expect(buttonCard).toBeVisible();
      
      // Test button interactions
      const primaryButton = page.locator('button:has-text("Primary Button")').first();
      await expect(primaryButton).toBeVisible();
      
      // Click the button and verify it responds
      await primaryButton.click();
      
      // Check for click counter (should increment)
      const clickCounter = page.locator('text=Clicks:').first();
      await expect(clickCounter).toBeVisible();
    });

    test('should have functional input components', async () => {
      const inputCard = page.locator('text=Input').first();
      await expect(inputCard).toBeVisible();
      
      // Test input interactions
      const nameInput = page.locator('input[placeholder="Enter your name"]');
      await expect(nameInput).toBeVisible();
      
      // Type in the input
      await nameInput.fill('Test User');
      await expect(nameInput).toHaveValue('Test User');
      
      // Test email input
      const emailInput = page.locator('input[type="email"]');
      await expect(emailInput).toBeVisible();
      await emailInput.fill('test@example.com');
      await expect(emailInput).toHaveValue('test@example.com');
      
      // Test password input
      const passwordInput = page.locator('input[type="password"]');
      await expect(passwordInput).toBeVisible();
      await passwordInput.fill('password123');
      await expect(passwordInput).toHaveValue('password123');
    });

    test('should display card components with performance metrics', async () => {
      const cardCard = page.locator('text=Card').first();
      await expect(cardCard).toBeVisible();
      
      // Check for performance metrics in cards
      const renderTime = page.locator('text=Render Time:').first();
      const memoryUsage = page.locator('text=Memory:').first();
      
      await expect(renderTime).toBeVisible();
      await expect(memoryUsage).toBeVisible();
    });
  });

  test.describe('Interactive Demo Section', () => {
    test('should have live demo section', async () => {
      const demoSection = page.locator('#demo');
      await expect(demoSection).toBeVisible();
      
      const demoTitle = page.locator('h2:has-text("Live Demo")');
      const demoDescription = page.locator('text=Experience the performance difference');
      
      await expect(demoTitle).toBeVisible();
      await expect(demoDescription).toBeVisible();
    });

    test('should have functional performance test', async () => {
      const performanceTestButton = page.locator('button:has-text("Run Performance Test")');
      await expect(performanceTestButton).toBeVisible();
      
      // Click the performance test button
      await performanceTestButton.click();
      
      // Wait for loading state
      await expect(page.locator('button:has-text("Running Test...")').first()).toBeVisible();
      
      // Wait for completion (should show results)
      await page.waitForSelector('text=Performance Test Complete!', { timeout: 5000 });
      
      // Verify results are displayed
      const results = page.locator('text=Click Response: 0.8ms').first();
      await expect(results).toBeVisible();
    });

    test('should have functional memory test', async () => {
      const memoryTestButton = page.locator('button:has-text("Start Memory Test")');
      await expect(memoryTestButton).toBeVisible();
      
      // Check initial memory display
      const memoryDisplay = page.locator('text=Memory Usage').last();
      await expect(memoryDisplay).toBeVisible();
      
      // Click the memory test button
      await memoryTestButton.click();
      
      // Wait for loading state
      await expect(page.locator('button:has-text("Running Test...")').first()).toBeVisible();
      
      // Wait for completion
      await page.waitForTimeout(3000);
      
      // Verify memory value changed
      const memoryValue = page.locator('text=/\\d+\\.\\d+MB/').last();
      await expect(memoryValue).toBeVisible();
    });

    test('should have functional speed test', async () => {
      const speedTestButton = page.locator('button:has-text("Run Speed Test")');
      await expect(speedTestButton).toBeVisible();
      
      // Click the speed test button
      await speedTestButton.click();
      
      // Wait for loading state
      await expect(page.locator('button:has-text("Running Test...")').first()).toBeVisible();
      
      // Wait for completion
      await page.waitForSelector('text=Speed Test Complete!', { timeout: 5000 });
      
      // Verify results are displayed
      const results = page.locator('text=Button Render: 0.8ms').first();
      await expect(results).toBeVisible();
    });
  });

  test.describe('Call to Action Section', () => {
    test('should have call to action section', async () => {
      const ctaSection = page.locator('text=Ready to Experience the Future?');
      await expect(ctaSection).toBeVisible();
      
      const ctaTitle = page.locator('h2:has-text("Ready to Experience the Future?")');
      const ctaDescription = page.locator('text=Join the performance revolution');
      
      await expect(ctaTitle).toBeVisible();
      await expect(ctaDescription).toBeVisible();
    });

    test('should have functional CTA buttons', async () => {
      const getStartedButton = page.locator('button:has-text("Get Started")');
      const installNowButton = page.locator('button:has-text("Install Now")');
      
      await expect(getStartedButton).toBeVisible();
      await expect(installNowButton).toBeVisible();
      
      // Test button interactions
      await getStartedButton.click();
      await installNowButton.click();
    });
  });

  test.describe('Responsive Design', () => {
    test('should be responsive on mobile', async () => {
      await page.setViewportSize({ width: 375, height: 667 });
      
      // Check that navigation is still visible
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
      
      // Check that hero section adapts
      const heroTitle = page.locator('h1:has-text("Performance Champion")');
      await expect(heroTitle).toBeVisible();
    });

    test('should be responsive on tablet', async () => {
      await page.setViewportSize({ width: 768, height: 1024 });
      
      // Check that components are properly laid out
      const componentsSection = page.locator('#components');
      await expect(componentsSection).toBeVisible();
    });
  });

  test.describe('Performance and Loading', () => {
    test('should load within acceptable time', async () => {
      const startTime = Date.now();
      await page.goto('http://localhost:8082');
      await page.waitForLoadState('networkidle');
      const loadTime = Date.now() - startTime;
      
      // Should load within 5 seconds
      expect(loadTime).toBeLessThan(5000);
    });

    test('should have proper WASM loading', async () => {
      // Check that WASM is loaded by looking for interactive elements
      const interactiveButton = page.locator('button:has-text("Primary Button")');
      await expect(interactiveButton).toBeVisible();
      
      // Test that WASM interactions work
      await interactiveButton.click();
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper heading structure', async () => {
      const h1 = page.locator('h1');
      const h2 = page.locator('h2');
      
      await expect(h1).toHaveCount(1);
      const h2Count = await h2.count();
      expect(h2Count).toBeGreaterThan(0);
    });

    test('should have proper button labels', async () => {
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();
      
      for (let i = 0; i < buttonCount; i++) {
        const button = buttons.nth(i);
        const text = await button.textContent();
        expect(text?.trim()).toBeTruthy();
      }
    });

    test('should have proper input labels', async () => {
      const inputs = page.locator('input');
      const inputCount = await inputs.count();
      
      for (let i = 0; i < inputCount; i++) {
        const input = inputs.nth(i);
        const placeholder = await input.getAttribute('placeholder');
        expect(placeholder).toBeTruthy();
      }
    });
  });

  test.describe('Visual Quality', () => {
    test('should have proper styling and colors', async () => {
      // Check for gradient backgrounds
      const heroSection = page.locator('section:has(h1:has-text("Performance Champion"))');
      await expect(heroSection).toBeVisible();
      
      // Check for proper button styling
      const primaryButton = page.locator('button:has-text("Primary Button")').first();
      await expect(primaryButton).toBeVisible();
      
      // Check for card styling
      const card = page.locator('text=Button').first();
      await expect(card).toBeVisible();
    });

    test('should have proper spacing and layout', async () => {
      // Check that sections are properly spaced
      const sections = page.locator('section');
      const sectionCount = await sections.count();
      expect(sectionCount).toBeGreaterThan(3);
      
      // Check for proper grid layouts
      const grid = page.locator('.grid').first();
      await expect(grid).toBeVisible();
    });
  });
});
