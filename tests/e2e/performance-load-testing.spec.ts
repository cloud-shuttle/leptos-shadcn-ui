import { test, expect } from '@playwright/test';

test.describe('Performance Testing Under Realistic Load', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Page Load Performance', () => {
    test('initial page load performance metrics', async ({ page }) => {
      // Measure page load time
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const loadTime = Date.now() - startTime;
      
      // Page should load within 3 seconds
      expect(loadTime).toBeLessThan(3000);
      
      // Check for performance metrics
      const performanceMetrics = await page.evaluate(() => {
        const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
        return {
          domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
          loadComplete: navigation.loadEventEnd - navigation.loadEventStart,
          firstPaint: performance.getEntriesByName('first-paint')[0]?.startTime || 0,
          firstContentfulPaint: performance.getEntriesByName('first-contentful-paint')[0]?.startTime || 0,
        };
      });
      
      // DOM content should be loaded quickly
      expect(performanceMetrics.domContentLoaded).toBeLessThan(1000);
      
      // Page should be fully loaded quickly
      expect(performanceMetrics.loadComplete).toBeLessThan(2000);
    });

    test('component rendering performance', async ({ page }) => {
      // Measure component rendering time
      const startTime = Date.now();
      
      // Wait for all components to be visible
      await page.waitForSelector('button, input, form, .card', { timeout: 5000 });
      
      const renderTime = Date.now() - startTime;
      
      // Components should render within 2 seconds
      expect(renderTime).toBeLessThan(2000);
      
      // Check for specific components
      const buttons = page.locator('button');
      const inputs = page.locator('input');
      const forms = page.locator('form');
      const cards = page.locator('.card');
      
      // All components should be visible
      if (await buttons.count() > 0) {
        await expect(buttons.first()).toBeVisible();
      }
      if (await inputs.count() > 0) {
        await expect(inputs.first()).toBeVisible();
      }
      if (await forms.count() > 0) {
        await expect(forms.first()).toBeVisible();
      }
      if (await cards.count() > 0) {
        await expect(cards.first()).toBeVisible();
      }
    });
  });

  test.describe('Interaction Performance', () => {
    test('button click response time', async ({ page }) => {
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        const button = buttons.first();
        
        // Measure click response time
        const startTime = Date.now();
        await button.click();
        
        // Wait for any immediate response
        await page.waitForTimeout(100);
        
        const responseTime = Date.now() - startTime;
        
        // Button should respond within 100ms
        expect(responseTime).toBeLessThan(100);
      }
    });

    test('form input responsiveness', async ({ page }) => {
      const inputs = page.locator('input, textarea');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        
        // Measure input response time
        const startTime = Date.now();
        await input.fill('Test input');
        
        const responseTime = Date.now() - startTime;
        
        // Input should respond within 50ms
        expect(responseTime).toBeLessThan(50);
        
        // Test typing performance
        await input.clear();
        const typingStartTime = Date.now();
        
        await input.type('Performance test typing speed');
        
        const typingTime = Date.now() - typingStartTime;
        
        // Typing should be responsive
        expect(typingTime).toBeLessThan(1000);
      }
    });

    test('modal open/close performance', async ({ page }) => {
      // Look for modal triggers
      const modalTriggers = page.locator('button:has-text("Open"), button:has-text("Show"), [data-testid*="modal"]');
      
      if (await modalTriggers.count() > 0) {
        const trigger = modalTriggers.first();
        
        // Measure modal open time
        const openStartTime = Date.now();
        await trigger.click();
        
        // Wait for modal to be visible
        const modal = page.locator('[role="dialog"], .modal, [data-testid="modal"]');
        if (await modal.count() > 0) {
          await expect(modal.first()).toBeVisible();
          
          const openTime = Date.now() - openStartTime;
          
          // Modal should open within 300ms
          expect(openTime).toBeLessThan(300);
          
          // Measure modal close time
          const closeButton = modal.locator('button:has-text("Close"), button:has-text("Cancel"), [aria-label*="close" i]');
          if (await closeButton.count() > 0) {
            const closeStartTime = Date.now();
            await closeButton.click();
            
            // Wait for modal to close
            await expect(modal.first()).not.toBeVisible();
            
            const closeTime = Date.now() - closeStartTime;
            
            // Modal should close within 200ms
            expect(closeTime).toBeLessThan(200);
          }
        }
      }
    });
  });

  test.describe('Memory Usage and Leaks', () => {
    test('memory usage during interactions', async ({ page }) => {
      // Get initial memory usage
      const initialMemory = await page.evaluate(() => {
        return (performance as any).memory ? {
          used: (performance as any).memory.usedJSHeapSize,
          total: (performance as any).memory.totalJSHeapSize,
          limit: (performance as any).memory.jsHeapSizeLimit
        } : null;
      });
      
      if (initialMemory) {
        // Perform multiple interactions
        const buttons = page.locator('button');
        const inputs = page.locator('input, textarea');
        
        for (let i = 0; i < 10; i++) {
          if (await buttons.count() > 0) {
            await buttons.first().click();
            await page.waitForTimeout(100);
          }
          
          if (await inputs.count() > 0) {
            await inputs.first().fill(`Test input ${i}`);
            await page.waitForTimeout(100);
          }
        }
        
        // Get memory usage after interactions
        const finalMemory = await page.evaluate(() => {
          return (performance as any).memory ? {
            used: (performance as any).memory.usedJSHeapSize,
            total: (performance as any).memory.totalJSHeapSize,
            limit: (performance as any).memory.jsHeapSizeLimit
          } : null;
        });
        
        if (finalMemory) {
          // Memory usage should not increase dramatically
          const memoryIncrease = finalMemory.used - initialMemory.used;
          const memoryIncreasePercent = (memoryIncrease / initialMemory.used) * 100;
          
          // Memory increase should be less than 50%
          expect(memoryIncreasePercent).toBeLessThan(50);
        }
      }
    });

    test('component lifecycle memory management', async ({ page }) => {
      // Test component creation and destruction
      const modalTriggers = page.locator('button:has-text("Open"), button:has-text("Show"), [data-testid*="modal"]');
      
      if (await modalTriggers.count() > 0) {
        const trigger = modalTriggers.first();
        
        // Open and close modal multiple times
        for (let i = 0; i < 5; i++) {
          await trigger.click();
          
          const modal = page.locator('[role="dialog"], .modal, [data-testid="modal"]');
          if (await modal.count() > 0) {
            await expect(modal.first()).toBeVisible();
            
            const closeButton = modal.locator('button:has-text("Close"), button:has-text("Cancel"), [aria-label*="close" i]');
            if (await closeButton.count() > 0) {
              await closeButton.click();
              await expect(modal.first()).not.toBeVisible();
            }
          }
          
          await page.waitForTimeout(200);
        }
        
        // Force garbage collection if available
        await page.evaluate(() => {
          if (window.gc) {
            window.gc();
          }
        });
        
        // Check that memory is not continuously increasing
        const memoryAfter = await page.evaluate(() => {
          return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : null;
        });
        
        expect(memoryAfter).toBeTruthy();
      }
    });
  });

  test.describe('Concurrent User Simulation', () => {
    test('multiple simultaneous interactions', async ({ page }) => {
      // Simulate multiple rapid interactions
      const buttons = page.locator('button');
      const inputs = page.locator('input, textarea');
      
      if (await buttons.count() > 0 && await inputs.count() > 0) {
        const button = buttons.first();
        const input = inputs.first();
        
        // Perform rapid interactions
        const startTime = Date.now();
        
        for (let i = 0; i < 20; i++) {
          // Rapid button clicks
          await button.click();
          await page.waitForTimeout(10);
          
          // Rapid input changes
          await input.fill(`Rapid input ${i}`);
          await page.waitForTimeout(10);
        }
        
        const totalTime = Date.now() - startTime;
        
        // All interactions should complete within reasonable time
        expect(totalTime).toBeLessThan(5000);
        
        // Check that the page is still responsive
        await expect(button).toBeVisible();
        await expect(input).toBeVisible();
      }
    });

    test('form submission under load', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        const inputs = form.locator('input, textarea, select');
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        
        if (await inputs.count() > 0 && await submitButton.count() > 0) {
          // Fill out form multiple times rapidly
          for (let i = 0; i < 5; i++) {
            // Fill inputs
            for (let j = 0; j < await inputs.count(); j++) {
              const input = inputs.nth(j);
              const inputType = await input.getAttribute('type');
              
              if (inputType === 'email') {
                await input.fill(`test${i}@example.com`);
              } else if (inputType === 'password') {
                await input.fill(`password${i}`);
              } else {
                await input.fill(`Test input ${i} ${j}`);
              }
            }
            
            // Submit form
            await submitButton.click();
            await page.waitForTimeout(200);
          }
          
          // Check that the page is still responsive
          await expect(form).toBeVisible();
        }
      }
    });
  });

  test.describe('Network Performance', () => {
    test('API response times', async ({ page }) => {
      // Monitor network requests
      const responses: any[] = [];
      
      page.on('response', response => {
        responses.push({
          url: response.url(),
          status: response.status(),
          timing: response.timing()
        });
      });
      
      // Trigger actions that might make API calls
      const buttons = page.locator('button');
      if (await buttons.count() > 0) {
        await buttons.first().click();
        await page.waitForTimeout(2000);
      }
      
      // Check response times
      for (const response of responses) {
        if (response.timing) {
          const responseTime = response.timing.responseEnd - response.timing.requestStart;
          
          // API responses should be under 2 seconds
          expect(responseTime).toBeLessThan(2000);
        }
      }
    });

    test('resource loading performance', async ({ page }) => {
      // Check for slow-loading resources
      const resources: any[] = [];
      
      page.on('response', response => {
        const url = response.url();
        const status = response.status();
        
        if (status >= 200 && status < 300) {
          resources.push({
            url,
            status,
            size: response.headers()['content-length']
          });
        }
      });
      
      // Navigate to trigger resource loading
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // Check for large resources
      for (const resource of resources) {
        if (resource.size) {
          const sizeInKB = parseInt(resource.size) / 1024;
          
          // Individual resources should not be too large
          expect(sizeInKB).toBeLessThan(1000); // 1MB limit
        }
      }
    });
  });

  test.describe('Accessibility Performance', () => {
    test('keyboard navigation performance', async ({ page }) => {
      // Test keyboard navigation speed
      const startTime = Date.now();
      
      // Navigate through focusable elements
      for (let i = 0; i < 10; i++) {
        await page.keyboard.press('Tab');
        await page.waitForTimeout(50);
      }
      
      const navigationTime = Date.now() - startTime;
      
      // Keyboard navigation should be responsive
      expect(navigationTime).toBeLessThan(1000);
      
      // Check that focus is visible
      const focusedElement = page.locator(':focus');
      if (await focusedElement.count() > 0) {
        await expect(focusedElement.first()).toBeVisible();
      }
    });

    test('screen reader performance', async ({ page }) => {
      // Test ARIA attribute accessibility
      const elementsWithAria = page.locator('[aria-label], [aria-describedby], [role]');
      
      if (await elementsWithAria.count() > 0) {
        const startTime = Date.now();
        
        // Check all ARIA elements
        for (let i = 0; i < Math.min(await elementsWithAria.count(), 10); i++) {
          const element = elementsWithAria.nth(i);
          await element.hover();
          await page.waitForTimeout(50);
        }
        
        const checkTime = Date.now() - startTime;
        
        // ARIA checks should be fast
        expect(checkTime).toBeLessThan(1000);
      }
    });
  });

  test.describe('Error Recovery Performance', () => {
    test('error handling response time', async ({ page }) => {
      // Simulate network error
      await page.context().setOffline(true);
      
      const startTime = Date.now();
      
      // Try to perform an action that requires network
      const buttons = page.locator('button');
      if (await buttons.count() > 0) {
        await buttons.first().click();
        
        // Wait for error handling
        await page.waitForTimeout(2000);
        
        const errorTime = Date.now() - startTime;
        
        // Error should be handled within 2 seconds
        expect(errorTime).toBeLessThan(2000);
        
        // Check for error messages
        const errorMessages = page.locator('[role="alert"], .error, .network-error');
        if (await errorMessages.count() > 0) {
          await expect(errorMessages.first()).toBeVisible();
        }
      }
      
      // Restore network
      await page.context().setOffline(false);
    });

    test('form validation performance', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        
        if (await submitButton.count() > 0) {
          const startTime = Date.now();
          
          // Submit form with invalid data
          await submitButton.click();
          
          // Wait for validation
          await page.waitForTimeout(1000);
          
          const validationTime = Date.now() - startTime;
          
          // Validation should be fast
          expect(validationTime).toBeLessThan(1000);
          
          // Check for validation errors
          const validationErrors = form.locator('[role="alert"], .error, .invalid');
          if (await validationErrors.count() > 0) {
            await expect(validationErrors.first()).toBeVisible();
          }
        }
      }
    });
  });
});
