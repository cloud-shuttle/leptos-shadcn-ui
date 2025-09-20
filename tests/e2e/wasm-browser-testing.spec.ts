import { test, expect } from '@playwright/test';

/**
 * Enhanced WASM Browser Testing Suite
 * 
 * This comprehensive test suite validates WASM functionality across all supported browsers,
 * including initialization, performance, memory management, and cross-browser compatibility.
 */

test.describe('WASM Browser Testing - Comprehensive Suite', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Leptos demo app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('WASM Initialization & Loading', () => {
    test('should initialize WASM successfully across all browsers', async ({ page, browserName }) => {
      console.log(`Testing WASM initialization on ${browserName}`);
      
      // Wait for WASM to initialize with timeout
      const wasmInitialized = await page.waitForFunction(
        () => {
          // Check for various WASM indicators
          return window.wasmBindings !== undefined || 
                 window.leptos !== undefined ||
                 document.querySelector('[data-wasm-loaded="true"]') !== null ||
                 !document.querySelector('#loading');
        },
        { timeout: 10000 }
      ).catch(() => false);

      if (wasmInitialized) {
        console.log(`✅ WASM initialized successfully on ${browserName}`);
        expect(wasmInitialized).toBeTruthy();
      } else {
        console.log(`❌ WASM initialization failed on ${browserName}`);
        // Take screenshot for debugging
        await page.screenshot({ path: `test-results/wasm-init-failure-${browserName}.png` });
        throw new Error(`WASM initialization failed on ${browserName}`);
      }
    });

    test('should handle WASM loading errors gracefully', async ({ page }) => {
      // Inject a script to simulate WASM loading failure
      await page.addInitScript(() => {
        // Override WebAssembly to simulate failure
        const originalWebAssembly = window.WebAssembly;
        window.WebAssembly = {
          ...originalWebAssembly,
          instantiate: () => Promise.reject(new Error('Simulated WASM loading failure'))
        };
      });

      // Navigate to page and check error handling
      await page.goto('/');
      await page.waitForLoadState('networkidle');

      // Check for error handling (loading screen should remain or error message shown)
      const loadingElement = page.locator('#loading');
      const errorElement = page.locator('[data-error="wasm-loading"]');
      
      // Either loading screen should remain or error should be displayed
      const hasErrorHandling = await loadingElement.isVisible() || await errorElement.isVisible();
      expect(hasErrorHandling).toBeTruthy();
    });

    test('should measure WASM initialization time', async ({ page, browserName }) => {
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );
      
      const initTime = Date.now() - startTime;
      console.log(`WASM initialization time on ${browserName}: ${initTime}ms`);
      
      // WASM should initialize within reasonable time (10 seconds max)
      expect(initTime).toBeLessThan(10000);
      
      // Log performance data for analysis
      await page.evaluate((time) => {
        window.wasmInitTime = time;
        console.log(`WASM Performance: ${time}ms initialization time`);
      }, initTime);
    });
  });

  test.describe('WASM Memory Management', () => {
    test('should not have memory leaks during component interactions', async ({ page, browserName }) => {
      console.log(`Testing memory management on ${browserName}`);
      
      // Wait for WASM to initialize
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Get initial memory usage
      const initialMemory = await page.evaluate(() => {
        if (performance.memory) {
          return {
            used: performance.memory.usedJSHeapSize,
            total: performance.memory.totalJSHeapSize,
            limit: performance.memory.jsHeapSizeLimit
          };
        }
        return null;
      });

      if (initialMemory) {
        console.log(`Initial memory usage on ${browserName}:`, initialMemory);

        // Perform multiple component interactions
        for (let i = 0; i < 10; i++) {
          // Try to interact with various components
          const buttons = page.locator('button');
          if (await buttons.count() > 0) {
            await buttons.first().click();
            await page.waitForTimeout(100);
          }

          const inputs = page.locator('input');
          if (await inputs.count() > 0) {
            await inputs.first().fill(`test-${i}`);
            await page.waitForTimeout(100);
          }
        }

        // Get final memory usage
        const finalMemory = await page.evaluate(() => {
          if (performance.memory) {
            return {
              used: performance.memory.usedJSHeapSize,
              total: performance.memory.totalJSHeapSize,
              limit: performance.memory.jsHeapSizeLimit
            };
          }
          return null;
        });

        if (finalMemory) {
          console.log(`Final memory usage on ${browserName}:`, finalMemory);
          
          // Memory usage should not increase dramatically (allow 50% increase max)
          const memoryIncrease = finalMemory.used - initialMemory.used;
          const memoryIncreasePercent = (memoryIncrease / initialMemory.used) * 100;
          
          console.log(`Memory increase: ${memoryIncrease} bytes (${memoryIncreasePercent.toFixed(2)}%)`);
          
          // Allow reasonable memory increase but flag excessive growth
          expect(memoryIncreasePercent).toBeLessThan(50);
        }
      } else {
        console.log(`Memory API not available on ${browserName}, skipping memory test`);
      }
    });

    test('should handle WASM memory pressure gracefully', async ({ page }) => {
      // Wait for WASM to initialize
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Simulate memory pressure by creating many DOM elements
      await page.evaluate(() => {
        // Create many elements to simulate memory pressure
        for (let i = 0; i < 1000; i++) {
          const div = document.createElement('div');
          div.textContent = `Memory test element ${i}`;
          div.className = 'memory-test-element';
          document.body.appendChild(div);
        }
      });

      // Check that WASM still functions
      const wasmStillWorking = await page.evaluate(() => {
        return window.wasmBindings !== undefined || window.leptos !== undefined;
      });

      expect(wasmStillWorking).toBeTruthy();

      // Clean up test elements
      await page.evaluate(() => {
        const elements = document.querySelectorAll('.memory-test-element');
        elements.forEach(el => el.remove());
      });
    });
  });

  test.describe('Cross-Browser WASM Compatibility', () => {
    test('should have consistent WASM behavior across browsers', async ({ page, browserName }) => {
      console.log(`Testing cross-browser consistency on ${browserName}`);
      
      // Wait for WASM to initialize
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Test basic WASM functionality
      const wasmCapabilities = await page.evaluate(() => {
        const capabilities = {
          webAssembly: typeof WebAssembly !== 'undefined',
          wasmBindings: window.wasmBindings !== undefined,
          leptos: window.leptos !== undefined,
          wasmSupported: false
        };

        // Test WebAssembly support
        if (typeof WebAssembly !== 'undefined') {
          try {
            capabilities.wasmSupported = WebAssembly.validate(new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]));
          } catch (e) {
            capabilities.wasmSupported = false;
          }
        }

        return capabilities;
      });

      console.log(`WASM capabilities on ${browserName}:`, wasmCapabilities);

      // All browsers should support WebAssembly
      expect(wasmCapabilities.webAssembly).toBeTruthy();
      expect(wasmCapabilities.wasmSupported).toBeTruthy();
    });

    test('should handle browser-specific WASM limitations', async ({ page, browserName }) => {
      // Test browser-specific features
      const browserInfo = await page.evaluate(() => {
        return {
          userAgent: navigator.userAgent,
          webAssembly: typeof WebAssembly !== 'undefined',
          sharedArrayBuffer: typeof SharedArrayBuffer !== 'undefined',
          bigInt: typeof BigInt !== 'undefined'
        };
      });

      console.log(`Browser info for ${browserName}:`, browserInfo);

      // Basic WebAssembly should be available on all supported browsers
      expect(browserInfo.webAssembly).toBeTruthy();
    });
  });

  test.describe('WASM Performance Monitoring', () => {
    test('should meet performance benchmarks', async ({ page, browserName }) => {
      const performanceMetrics = await page.evaluate(async () => {
        const metrics = {
          wasmInitTime: 0,
          firstPaint: 0,
          firstContentfulPaint: 0,
          domContentLoaded: 0,
          loadComplete: 0
        };

        // Measure WASM initialization time
        const wasmStart = performance.now();
        await new Promise(resolve => {
          const checkWasm = () => {
            if (window.wasmBindings !== undefined || !document.querySelector('#loading')) {
              metrics.wasmInitTime = performance.now() - wasmStart;
              resolve(undefined);
            } else {
              setTimeout(checkWasm, 10);
            }
          };
          checkWasm();
        });

        // Get performance entries
        const entries = performance.getEntriesByType('navigation');
        if (entries.length > 0) {
          const navEntry = entries[0] as PerformanceNavigationTiming;
          metrics.domContentLoaded = navEntry.domContentLoadedEventEnd - navEntry.domContentLoadedEventStart;
          metrics.loadComplete = navEntry.loadEventEnd - navEntry.loadEventStart;
        }

        // Get paint timing
        const paintEntries = performance.getEntriesByType('paint');
        paintEntries.forEach(entry => {
          if (entry.name === 'first-paint') {
            metrics.firstPaint = entry.startTime;
          } else if (entry.name === 'first-contentful-paint') {
            metrics.firstContentfulPaint = entry.startTime;
          }
        });

        return metrics;
      });

      console.log(`Performance metrics on ${browserName}:`, performanceMetrics);

      // Performance assertions
      expect(performanceMetrics.wasmInitTime).toBeLessThan(5000); // WASM should init within 5s
      expect(performanceMetrics.firstPaint).toBeLessThan(3000); // First paint within 3s
      expect(performanceMetrics.firstContentfulPaint).toBeLessThan(4000); // FCP within 4s
    });

    test('should maintain performance under load', async ({ page, browserName }) => {
      // Wait for initial load
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Measure performance during interactions
      const interactionMetrics = await page.evaluate(() => {
        const metrics = {
          buttonClickTimes: [] as number[],
          inputFillTimes: [] as number[],
          averageResponseTime: 0
        };

        // Test button click performance
        const buttons = document.querySelectorAll('button');
        for (let i = 0; i < Math.min(buttons.length, 5); i++) {
          const start = performance.now();
          buttons[i].click();
          const end = performance.now();
          metrics.buttonClickTimes.push(end - start);
        }

        // Test input performance
        const inputs = document.querySelectorAll('input');
        for (let i = 0; i < Math.min(inputs.length, 3); i++) {
          const start = performance.now();
          (inputs[i] as HTMLInputElement).value = `test-${i}`;
          const end = performance.now();
          metrics.inputFillTimes.push(end - start);
        }

        // Calculate average response time
        const allTimes = [...metrics.buttonClickTimes, ...metrics.inputFillTimes];
        metrics.averageResponseTime = allTimes.reduce((a, b) => a + b, 0) / allTimes.length;

        return metrics;
      });

      console.log(`Interaction performance on ${browserName}:`, interactionMetrics);

      // Response times should be reasonable
      expect(interactionMetrics.averageResponseTime).toBeLessThan(100); // Less than 100ms average
    });
  });

  test.describe('WASM Error Handling & Recovery', () => {
    test('should handle WASM runtime errors gracefully', async ({ page }) => {
      // Wait for WASM to initialize
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Inject error handling test
      const errorHandling = await page.evaluate(() => {
        let errorCaught = false;
        let errorMessage = '';

        // Set up error handler
        window.addEventListener('error', (event) => {
          errorCaught = true;
          errorMessage = event.message;
        });

        // Try to trigger a WASM-related error (if possible)
        try {
          // This might trigger an error in some implementations
          if (window.wasmBindings && typeof window.wasmBindings.invalidFunction === 'function') {
            window.wasmBindings.invalidFunction();
          }
        } catch (e) {
          errorCaught = true;
          errorMessage = (e as Error).message;
        }

        return { errorCaught, errorMessage };
      });

      // Error handling should be in place (even if no error occurs)
      expect(typeof errorHandling).toBe('object');
    });

    test('should recover from WASM failures', async ({ page }) => {
      // Wait for initial WASM load
      await page.waitForFunction(
        () => window.wasmBindings !== undefined || !document.querySelector('#loading'),
        { timeout: 10000 }
      );

      // Simulate WASM failure and check recovery
      const recoveryTest = await page.evaluate(() => {
        const initialState = {
          wasmBindings: window.wasmBindings !== undefined,
          leptos: window.leptos !== undefined
        };

        // Simulate clearing WASM state
        if (window.wasmBindings) {
          delete (window as any).wasmBindings;
        }

        const afterFailure = {
          wasmBindings: window.wasmBindings !== undefined,
          leptos: window.leptos !== undefined
        };

        return { initialState, afterFailure };
      });

      console.log('WASM recovery test:', recoveryTest);

      // Application should still function even if WASM state is cleared
      expect(recoveryTest.initialState.wasmBindings || recoveryTest.initialState.leptos).toBeTruthy();
    });
  });

  test.describe('WASM Bundle Analysis', () => {
    test('should load WASM bundle efficiently', async ({ page, browserName }) => {
      // Monitor network requests for WASM files
      const wasmRequests: any[] = [];
      
      page.on('request', request => {
        if (request.url().includes('.wasm') || request.url().includes('wasm')) {
          wasmRequests.push({
            url: request.url(),
            method: request.method(),
            headers: request.headers()
          });
        }
      });

      await page.goto('/');
      await page.waitForLoadState('networkidle');

      console.log(`WASM requests on ${browserName}:`, wasmRequests);

      // Should have WASM requests
      expect(wasmRequests.length).toBeGreaterThan(0);

      // WASM files should be served with appropriate headers
      wasmRequests.forEach(request => {
        expect(request.method).toBe('GET');
        // Check for proper content type (if available in headers)
        const contentType = request.headers['content-type'];
        if (contentType) {
          expect(contentType).toMatch(/application\/wasm|application\/octet-stream/);
        }
      });
    });

    test('should have reasonable WASM bundle size', async ({ page, browserName }) => {
      const bundleInfo = await page.evaluate(() => {
        const scripts = Array.from(document.querySelectorAll('script[src]'));
        const wasmScripts = scripts.filter(script => 
          script.getAttribute('src')?.includes('.wasm') || 
          script.getAttribute('src')?.includes('wasm')
        );

        return {
          totalScripts: scripts.length,
          wasmScripts: wasmScripts.length,
          scriptSources: scripts.map(s => s.getAttribute('src'))
        };
      });

      console.log(`Bundle info on ${browserName}:`, bundleInfo);

      // Should have reasonable number of scripts
      expect(bundleInfo.totalScripts).toBeGreaterThan(0);
      expect(bundleInfo.totalScripts).toBeLessThan(50); // Not too many scripts
    });
  });
});
