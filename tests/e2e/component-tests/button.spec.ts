import { test, expect, Page } from '@playwright/test';

/**
 * Button Component E2E Tests
 * 
 * TDD Approach: These tests define the expected behavior of the Button component
 * and will guide the implementation of comprehensive E2E testing.
 */

test.describe('Button Component E2E Tests', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    await page.goto('/components/button');
    await page.waitForLoadState('networkidle');
  });

  // ===== BASIC FUNCTIONALITY TESTS =====
  
  test('should render button with default variant', async () => {
    const button = page.locator('[data-testid="button-default"]');
    await expect(button).toBeVisible();
    await expect(button).toHaveClass(/btn/);
    await expect(button).toHaveText('Default Button');
  });

  test('should render button with different variants', async () => {
    const variants = ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link'];
    
    for (const variant of variants) {
      const button = page.locator(`[data-testid="button-${variant}"]`);
      await expect(button).toBeVisible();
      await expect(button).toHaveClass(new RegExp(`btn-${variant}`));
    }
  });

  test('should render button with different sizes', async () => {
    const sizes = ['sm', 'default', 'lg', 'icon'];
    
    for (const size of sizes) {
      const button = page.locator(`[data-testid="button-${size}"]`);
      await expect(button).toBeVisible();
      await expect(button).toHaveClass(new RegExp(`btn-${size}`));
    }
  });

  // ===== INTERACTION TESTS =====
  
  test('should handle click events', async () => {
    const button = page.locator('[data-testid="button-clickable"]');
    const clickCounter = page.locator('[data-testid="click-counter"]');
    
    await expect(clickCounter).toHaveText('0');
    
    await button.click();
    await expect(clickCounter).toHaveText('1');
    
    await button.click();
    await expect(clickCounter).toHaveText('2');
  });

  test('should be disabled when disabled prop is set', async () => {
    const disabledButton = page.locator('[data-testid="button-disabled"]');
    
    await expect(disabledButton).toBeDisabled();
    await expect(disabledButton).toHaveClass(/disabled/);
    
    // Click should not work
    await disabledButton.click({ force: true });
    const clickCounter = page.locator('[data-testid="click-counter"]');
    await expect(clickCounter).toHaveText('0'); // Should remain unchanged
  });

  test('should show loading state', async () => {
    const loadingButton = page.locator('[data-testid="button-loading"]');
    
    await expect(loadingButton).toBeVisible();
    await expect(loadingButton).toHaveClass(/loading/);
    await expect(loadingButton).toBeDisabled();
    
    // Should show loading spinner or text
    const loadingIndicator = loadingButton.locator('[data-testid="loading-indicator"]');
    await expect(loadingIndicator).toBeVisible();
  });

  // ===== ACCESSIBILITY TESTS =====
  
  test('should be keyboard accessible', async () => {
    const button = page.locator('[data-testid="button-keyboard"]');
    
    // Focus the button
    await button.focus();
    await expect(button).toBeFocused();
    
    // Press Enter to activate
    await button.press('Enter');
    const clickCounter = page.locator('[data-testid="click-counter"]');
    await expect(clickCounter).toHaveText('1');
    
    // Press Space to activate
    await button.press(' ');
    await expect(clickCounter).toHaveText('2');
  });

  test('should have proper ARIA attributes', async () => {
    const button = page.locator('[data-testid="button-aria"]');
    
    await expect(button).toHaveAttribute('role', 'button');
    await expect(button).toHaveAttribute('type', 'button');
    
    // Check for aria-label if present
    const ariaLabel = await button.getAttribute('aria-label');
    if (ariaLabel) {
      expect(ariaLabel).toBeTruthy();
    }
  });

  test('should support screen readers', async () => {
    const button = page.locator('[data-testid="button-screen-reader"]');
    
    // Check for accessible name
    const accessibleName = await button.evaluate((el) => {
      return el.getAttribute('aria-label') || el.textContent?.trim();
    });
    
    expect(accessibleName).toBeTruthy();
    expect(accessibleName?.length).toBeGreaterThan(0);
  });

  // ===== PERFORMANCE TESTS =====
  
  test('should render within performance budget', async () => {
    const startTime = Date.now();
    
    await page.goto('/components/button');
    await page.waitForLoadState('networkidle');
    
    const renderTime = Date.now() - startTime;
    
    // Should render within 1 second
    expect(renderTime).toBeLessThan(1000);
  });

  test('should handle rapid clicks without performance degradation', async () => {
    const button = page.locator('[data-testid="button-performance"]');
    const startTime = Date.now();
    
    // Perform 10 rapid clicks
    for (let i = 0; i < 10; i++) {
      await button.click();
    }
    
    const totalTime = Date.now() - startTime;
    
    // Should handle 10 clicks within 2 seconds
    expect(totalTime).toBeLessThan(2000);
    
    const clickCounter = page.locator('[data-testid="click-counter"]');
    await expect(clickCounter).toHaveText('10');
  });

  // ===== CROSS-BROWSER COMPATIBILITY TESTS =====
  
  test('should work consistently across browsers', async () => {
    const button = page.locator('[data-testid="button-cross-browser"]');
    
    // Basic functionality should work
    await expect(button).toBeVisible();
    await expect(button).toHaveClass(/btn/);
    
    // Click should work
    await button.click();
    const clickCounter = page.locator('[data-testid="click-counter"]');
    await expect(clickCounter).toHaveText('1');
    
    // Keyboard navigation should work
    await button.focus();
    await expect(button).toBeFocused();
  });

  // ===== ERROR HANDLING TESTS =====
  
  test('should handle missing props gracefully', async () => {
    const button = page.locator('[data-testid="button-minimal"]');
    
    // Should still render even with minimal props
    await expect(button).toBeVisible();
    await expect(button).toHaveClass(/btn/);
  });

  test('should handle invalid variant gracefully', async () => {
    const button = page.locator('[data-testid="button-invalid-variant"]');
    
    // Should fallback to default variant
    await expect(button).toBeVisible();
    await expect(button).toHaveClass(/btn/);
  });

  // ===== INTEGRATION TESTS =====
  
  test('should work within forms', async () => {
    const form = page.locator('[data-testid="form-with-button"]');
    const submitButton = form.locator('[data-testid="submit-button"]');
    const input = form.locator('[data-testid="form-input"]');
    
    // Fill form
    await input.fill('test value');
    
    // Submit form
    await submitButton.click();
    
    // Check form submission
    const result = page.locator('[data-testid="form-result"]');
    await expect(result).toBeVisible();
    await expect(result).toHaveText('Form submitted');
  });

  test('should work with other components', async () => {
    const button = page.locator('[data-testid="button-with-tooltip"]');
    const tooltip = page.locator('[data-testid="tooltip"]');
    
    // Hover to show tooltip
    await button.hover();
    await expect(tooltip).toBeVisible();
    
    // Click button
    await button.click();
    
    // Tooltip should still work
    await expect(tooltip).toBeVisible();
  });
});
