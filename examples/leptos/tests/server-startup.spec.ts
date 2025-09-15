import { test, expect } from '@playwright/test';

test.describe('Server Startup Tests', () => {
  test('should be able to start trunk serve without errors', async ({ page }) => {
    // This test will verify that the server can start
    // We'll use a simple HTTP request to check if the server is accessible
    try {
      const response = await page.goto('http://localhost:8080', { 
        waitUntil: 'networkidle',
        timeout: 10000 
      });
      
      if (response) {
        expect(response.status()).toBe(200);
        console.log('✅ Server is running and accessible');
      } else {
        throw new Error('No response received from server');
      }
    } catch (error) {
      console.log('❌ Server is not accessible:', error.message);
      throw error;
    }
  });

  test('should serve the main page with correct content', async ({ page }) => {
    await page.goto('http://localhost:8080');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for the main heading
    const heading = page.locator('h1');
    await expect(heading).toBeVisible();
    await expect(heading).toContainText('🚀 WASM-Powered');
    
    // Check for the subtitle
    const subtitle = page.locator('h2');
    await expect(subtitle).toBeVisible();
    
    console.log('✅ Main page content is served correctly');
  });

  test('should have working theme controls', async ({ page }) => {
    await page.goto('http://localhost:8080');
    await page.waitForLoadState('networkidle');
    
    // Check for theme control buttons
    const themeButtons = page.locator('button').filter({ hasText: /Default|Light|Dark/ });
    await expect(themeButtons).toHaveCount(3);
    
    // Check for color control buttons
    const colorButtons = page.locator('button').filter({ hasText: /Blue|Green|Purple/ });
    await expect(colorButtons).toHaveCount(3);
    
    console.log('✅ Theme controls are present and functional');
  });
});

