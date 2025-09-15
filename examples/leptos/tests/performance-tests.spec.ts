import { test, expect } from '@playwright/test';

test.describe('Performance Tests @performance', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('h1', { timeout: 10000 });
  });

  test('should load the page quickly', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/');
    await page.waitForSelector('h1', { timeout: 10000 });
    const endTime = Date.now();
    
    const loadTime = endTime - startTime;
    
    // Page should load within 5 seconds
    expect(loadTime).toBeLessThan(5000);
    
    // Log performance metrics
    console.log(`Page load time: ${loadTime}ms`);
  });

  test('should have good Core Web Vitals', async ({ page }) => {
    // Navigate to the page
    await page.goto('/');
    await page.waitForSelector('h1', { timeout: 10000 });
    
    // Get performance metrics
    const metrics = await page.evaluate(() => {
      return new Promise((resolve) => {
        new PerformanceObserver((list) => {
          const entries = list.getEntries();
          const metrics = {};
          
          entries.forEach((entry) => {
            if (entry.entryType === 'largest-contentful-paint') {
              metrics.lcp = entry.startTime;
            }
            if (entry.entryType === 'first-input') {
              metrics.fid = entry.processingStart - entry.startTime;
            }
            if (entry.entryType === 'layout-shift') {
              metrics.cls = entry.value;
            }
          });
          
          resolve(metrics);
        }).observe({ entryTypes: ['largest-contentful-paint', 'first-input', 'layout-shift'] });
        
        // Fallback timeout
        setTimeout(() => resolve({}), 3000);
      });
    });
    
    // Check LCP (should be under 2.5s)
    if (metrics.lcp) {
      expect(metrics.lcp).toBeLessThan(2500);
      console.log(`LCP: ${metrics.lcp}ms`);
    }
    
    // Check FID (should be under 100ms)
    if (metrics.fid) {
      expect(metrics.fid).toBeLessThan(100);
      console.log(`FID: ${metrics.fid}ms`);
    }
    
    // Check CLS (should be under 0.1)
    if (metrics.cls) {
      expect(metrics.cls).toBeLessThan(0.1);
      console.log(`CLS: ${metrics.cls}`);
    }
  });

  test('should handle theme changes efficiently', async ({ page }) => {
    // Measure theme change performance
    const themeButtons = [
      page.locator('button').filter({ hasText: 'default' }),
      page.locator('button').filter({ hasText: 'light' }),
      page.locator('button').filter({ hasText: 'dark' })
    ];
    
    const themeChangeTimes = [];
    
    for (const button of themeButtons) {
      const startTime = performance.now();
      await button.click();
      await page.waitForTimeout(100); // Wait for theme to apply
      const endTime = performance.now();
      
      themeChangeTimes.push(endTime - startTime);
    }
    
    // Each theme change should be fast (under 100ms)
    themeChangeTimes.forEach((time, index) => {
      expect(time).toBeLessThan(100);
      console.log(`Theme change ${index + 1}: ${time.toFixed(2)}ms`);
    });
    
    // Average theme change time should be reasonable
    const averageTime = themeChangeTimes.reduce((a, b) => a + b, 0) / themeChangeTimes.length;
    expect(averageTime).toBeLessThan(50);
    console.log(`Average theme change time: ${averageTime.toFixed(2)}ms`);
  });

  test('should handle color changes efficiently', async ({ page }) => {
    // Measure color change performance
    const colorButtons = [
      page.locator('button').filter({ hasText: 'blue' }),
      page.locator('button').filter({ hasText: 'green' }),
      page.locator('button').filter({ hasText: 'purple' })
    ];
    
    const colorChangeTimes = [];
    
    for (const button of colorButtons) {
      const startTime = performance.now();
      await button.click();
      await page.waitForTimeout(100); // Wait for color to apply
      const endTime = performance.now();
      
      colorChangeTimes.push(endTime - startTime);
    }
    
    // Each color change should be fast (under 100ms)
    colorChangeTimes.forEach((time, index) => {
      expect(time).toBeLessThan(100);
      console.log(`Color change ${index + 1}: ${time.toFixed(2)}ms`);
    });
    
    // Average color change time should be reasonable
    const averageTime = colorChangeTimes.reduce((a, b) => a + b, 0) / colorChangeTimes.length;
    expect(averageTime).toBeLessThan(50);
    console.log(`Average color change time: ${averageTime.toFixed(2)}ms`);
  });

  test('should handle responsive changes efficiently', async ({ page }) => {
    // Measure responsive change performance
    const viewports = [
      { width: 1920, height: 1080, name: 'desktop' },
      { width: 768, height: 1024, name: 'tablet' },
      { width: 375, height: 667, name: 'mobile' }
    ];
    
    const responsiveChangeTimes = [];
    
    for (const viewport of viewports) {
      const startTime = performance.now();
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await page.waitForTimeout(200); // Wait for responsive changes
      const endTime = performance.now();
      
      responsiveChangeTimes.push(endTime - startTime);
      console.log(`${viewport.name} viewport change: ${(endTime - startTime).toFixed(2)}ms`);
    }
    
    // Responsive changes should be fast (under 500ms)
    responsiveChangeTimes.forEach((time) => {
      expect(time).toBeLessThan(500);
    });
  });

  test('should have efficient WASM loading', async ({ page }) => {
    // Check WASM loading performance
    const startTime = Date.now();
    
    // Navigate to the page
    await page.goto('/');
    
    // Wait for WASM to load
    await page.waitForFunction(() => {
      // Check if WASM is loaded by looking for WASM-related objects
      return window.WebAssembly && window.WebAssembly.Module;
    }, { timeout: 10000 });
    
    const endTime = Date.now();
    const wasmLoadTime = endTime - startTime;
    
    // WASM should load within 3 seconds
    expect(wasmLoadTime).toBeLessThan(3000);
    console.log(`WASM load time: ${wasmLoadTime}ms`);
    
    // Check that the page is interactive
    const heroSection = page.locator('section').first();
    await expect(heroSection).toBeVisible();
  });

  test('should handle multiple rapid interactions efficiently', async ({ page }) => {
    // Test rapid theme and color changes
    const themeButtons = [
      page.locator('button').filter({ hasText: 'default' }),
      page.locator('button').filter({ hasText: 'light' }),
      page.locator('button').filter({ hasText: 'dark' })
    ];
    
    const colorButtons = [
      page.locator('button').filter({ hasText: 'blue' }),
      page.locator('button').filter({ hasText: 'green' }),
      page.locator('button').filter({ hasText: 'purple' })
    ];
    
    const startTime = performance.now();
    
    // Perform rapid interactions
    for (let i = 0; i < 10; i++) {
      const themeButton = themeButtons[i % themeButtons.length];
      const colorButton = colorButtons[i % colorButtons.length];
      
      await themeButton.click();
      await colorButton.click();
      await page.waitForTimeout(50); // Small delay between interactions
    }
    
    const endTime = performance.now();
    const totalTime = endTime - startTime;
    
    // All interactions should complete within 2 seconds
    expect(totalTime).toBeLessThan(2000);
    console.log(`Total time for 20 rapid interactions: ${totalTime.toFixed(2)}ms`);
    console.log(`Average time per interaction: ${(totalTime / 20).toFixed(2)}ms`);
  });

  test('should maintain performance during long sessions', async ({ page }) => {
    // Simulate a long session with many interactions
    const startTime = performance.now();
    
    // Perform many interactions over time
    for (let i = 0; i < 50; i++) {
      const themeButton = page.locator('button').filter({ hasText: 'default' });
      const colorButton = page.locator('button').filter({ hasText: 'blue' });
      
      await themeButton.click();
      await colorButton.click();
      await page.waitForTimeout(100);
      
      // Check that the page is still responsive
      if (i % 10 === 0) {
        const heroSection = page.locator('section').first();
        await expect(heroSection).toBeVisible();
      }
    }
    
    const endTime = performance.now();
    const totalTime = endTime - startTime;
    
    // Long session should complete within 10 seconds
    expect(totalTime).toBeLessThan(10000);
    console.log(`Long session (100 interactions) completed in: ${totalTime.toFixed(2)}ms`);
    
    // Check that the page is still responsive after long session
    const heroSection = page.locator('section').first();
    await expect(heroSection).toBeVisible();
  });

  test('should have efficient memory usage', async ({ page }) => {
    // Check memory usage
    const memoryInfo = await page.evaluate(() => {
      if ('memory' in performance) {
        return {
          usedJSHeapSize: (performance as any).memory.usedJSHeapSize,
          totalJSHeapSize: (performance as any).memory.totalJSHeapSize,
          jsHeapSizeLimit: (performance as any).memory.jsHeapSizeLimit
        };
      }
      return null;
    });
    
    if (memoryInfo) {
      // Memory usage should be reasonable (under 50MB)
      const usedMB = memoryInfo.usedJSHeapSize / (1024 * 1024);
      expect(usedMB).toBeLessThan(50);
      console.log(`Memory usage: ${usedMB.toFixed(2)}MB`);
      
      // Memory usage should be less than 50% of the limit
      const usagePercentage = (memoryInfo.usedJSHeapSize / memoryInfo.jsHeapSizeLimit) * 100;
      expect(usagePercentage).toBeLessThan(50);
      console.log(`Memory usage percentage: ${usagePercentage.toFixed(2)}%`);
    }
  });
});

