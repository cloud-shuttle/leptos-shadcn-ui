import { test, expect } from '@playwright/test';

test.describe('Mock Server Tests', () => {
  test('should verify test setup is working', async ({ page }) => {
    // Create a simple HTML page to test our setup
    const htmlContent = `
      <!DOCTYPE html>
      <html>
        <head>
          <title>Test Page</title>
          <style>
            body { 
              font-family: Arial, sans-serif; 
              margin: 0; 
              padding: 20px;
              background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
              color: white;
            }
            .hero { 
              text-align: center; 
              padding: 50px 0;
              background: rgba(255,255,255,0.1);
              border-radius: 10px;
              margin: 20px 0;
            }
            .theme-controls {
              display: flex;
              gap: 10px;
              justify-content: center;
              margin: 20px 0;
            }
            button {
              padding: 10px 20px;
              border: none;
              border-radius: 5px;
              background: rgba(255,255,255,0.2);
              color: white;
              cursor: pointer;
              transition: all 0.3s ease;
            }
            button:hover {
              background: rgba(255,255,255,0.3);
              transform: translateY(-2px);
            }
            .component-card {
              background: rgba(255,255,255,0.1);
              padding: 20px;
              border-radius: 10px;
              margin: 10px 0;
              border: 1px solid rgba(255,255,255,0.2);
            }
          </style>
        </head>
        <body>
          <div class="hero">
            <h1>🚀 WASM-Powered Component Showcase</h1>
            <h2>Test Demo with Tailwind-RS-Core</h2>
            <p>This is a mock version to test our Playwright setup</p>
          </div>
          
          <div class="theme-controls">
            <button onclick="changeTheme('default')">Default</button>
            <button onclick="changeTheme('light')">Light</button>
            <button onclick="changeTheme('dark')">Dark</button>
          </div>
          
          <div class="theme-controls">
            <button onclick="changeColor('blue')">Blue</button>
            <button onclick="changeColor('green')">Green</button>
            <button onclick="changeColor('purple')">Purple</button>
          </div>
          
          <div class="component-card">
            <h3>Button Component</h3>
            <p>Interactive buttons with variants and sizes</p>
            <button>Click me!</button>
          </div>
          
          <div class="component-card">
            <h3>Input Component</h3>
            <p>Form inputs with validation and styling</p>
            <input type="text" placeholder="Enter text..." style="padding: 10px; border: 1px solid rgba(255,255,255,0.3); border-radius: 5px; background: rgba(255,255,255,0.1); color: white;">
          </div>
          
          <div class="component-card">
            <h3>Card Component</h3>
            <p>Content containers with headers and footers</p>
            <div style="background: rgba(255,255,255,0.1); padding: 15px; border-radius: 5px;">
              <h4>Card Header</h4>
              <p>Card content goes here...</p>
            </div>
          </div>
          
          <script>
            function changeTheme(theme) {
              console.log('Theme changed to:', theme);
              document.body.style.background = theme === 'light' 
                ? 'linear-gradient(135deg, #74b9ff 0%, #0984e3 100%)'
                : theme === 'dark'
                ? 'linear-gradient(135deg, #2d3436 0%, #636e72 100%)'
                : 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)';
            }
            
            function changeColor(color) {
              console.log('Color changed to:', color);
              const hero = document.querySelector('.hero');
              hero.style.background = color === 'blue'
                ? 'rgba(59, 130, 246, 0.3)'
                : color === 'green'
                ? 'rgba(34, 197, 94, 0.3)'
                : 'rgba(147, 51, 234, 0.3)';
            }
          </script>
        </body>
      </html>
    `;
    
    // Set the content directly
    await page.setContent(htmlContent);
    
    // Test basic functionality
    await expect(page.locator('h1')).toContainText('🚀 WASM-Powered');
    await expect(page.locator('h2')).toContainText('Test Demo with Tailwind-RS-Core');
    
    // Test theme switching
    const defaultButton = page.locator('button').filter({ hasText: 'Default' });
    const lightButton = page.locator('button').filter({ hasText: 'Light' });
    const darkButton = page.locator('button').filter({ hasText: 'Dark' });
    
    await expect(defaultButton).toBeVisible();
    await expect(lightButton).toBeVisible();
    await expect(darkButton).toBeVisible();
    
    // Test color switching
    const blueButton = page.locator('button').filter({ hasText: 'Blue' });
    const greenButton = page.locator('button').filter({ hasText: 'Green' });
    const purpleButton = page.locator('button').filter({ hasText: 'Purple' });
    
    await expect(blueButton).toBeVisible();
    await expect(greenButton).toBeVisible();
    await expect(purpleButton).toBeVisible();
    
    // Test component cards
    const componentCards = page.locator('.component-card');
    await expect(componentCards).toHaveCount(3);
    
    // Test interactions
    await lightButton.click();
    await page.waitForTimeout(100);
    
    await greenButton.click();
    await page.waitForTimeout(100);
    
    // Test that the page is responsive
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('h1')).toBeVisible();
    
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('h1')).toBeVisible();
    
    console.log('✅ Mock server test passed - Playwright setup is working correctly');
  });
});
