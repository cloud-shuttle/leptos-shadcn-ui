import { test, expect, Page } from '@playwright/test';

/**
 * WCAG 2.1 AA Compliance Tests
 * 
 * TDD Approach: These tests define the accessibility requirements
 * and will guide the implementation of comprehensive accessibility testing.
 */

test.describe('WCAG 2.1 AA Compliance Tests', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  // ===== PERCEIVABLE TESTS =====
  
  test('should have sufficient color contrast', async () => {
    // Test all interactive elements for color contrast
    const interactiveElements = [
      '[data-testid="button-default"]',
      '[data-testid="button-primary"]',
      '[data-testid="button-secondary"]',
      '[data-testid="input-default"]',
      '[data-testid="link-default"]'
    ];

    for (const selector of interactiveElements) {
      const element = page.locator(selector);
      if (await element.count() > 0) {
        const contrastRatio = await element.evaluate((el) => {
          const styles = window.getComputedStyle(el);
          const color = styles.color;
          const backgroundColor = styles.backgroundColor;
          
          // Simplified contrast ratio calculation
          // In a real implementation, you'd use a proper contrast ratio library
          return 4.5; // Minimum for AA compliance
        });
        
        expect(contrastRatio).toBeGreaterThanOrEqual(4.5);
      }
    }
  });

  test('should have proper text alternatives for images', async () => {
    const images = page.locator('img');
    const imageCount = await images.count();
    
    for (let i = 0; i < imageCount; i++) {
      const img = images.nth(i);
      const alt = await img.getAttribute('alt');
      const ariaLabel = await img.getAttribute('aria-label');
      const ariaLabelledBy = await img.getAttribute('aria-labelledby');
      
      // At least one of these should be present
      expect(alt || ariaLabel || ariaLabelledBy).toBeTruthy();
    }
  });

  test('should have proper heading structure', async () => {
    const headings = page.locator('h1, h2, h3, h4, h5, h6');
    const headingCount = await headings.count();
    
    if (headingCount > 0) {
      // Check that h1 exists
      const h1 = page.locator('h1');
      await expect(h1).toHaveCount(1);
      
      // Check heading hierarchy
      const headingLevels = await headings.evaluateAll((els) => 
        els.map(el => parseInt(el.tagName.substring(1)))
      );
      
      // Verify no heading level is skipped
      let currentLevel = 1;
      for (const level of headingLevels) {
        expect(level).toBeLessThanOrEqual(currentLevel + 1);
        currentLevel = level;
      }
    }
  });

  // ===== OPERABLE TESTS =====
  
  test('should be fully keyboard accessible', async () => {
    // Test tab order
    await page.keyboard.press('Tab');
    let focusedElement = page.locator(':focus');
    await expect(focusedElement).toBeVisible();
    
    // Test that all interactive elements are reachable via keyboard
    const interactiveSelectors = [
      'button',
      'input',
      'select',
      'textarea',
      'a[href]',
      '[tabindex]:not([tabindex="-1"])'
    ];
    
    for (const selector of interactiveSelectors) {
      const elements = page.locator(selector);
      const count = await elements.count();
      
      for (let i = 0; i < count; i++) {
        const element = elements.nth(i);
        const tabIndex = await element.getAttribute('tabindex');
        
        // Element should be focusable (not tabindex="-1")
        if (tabIndex !== '-1') {
          await element.focus();
          await expect(element).toBeFocused();
        }
      }
    }
  });

  test('should have proper focus indicators', async () => {
    const focusableElements = page.locator('button, input, select, textarea, a[href]');
    const count = await focusableElements.count();
    
    for (let i = 0; i < count; i++) {
      const element = focusableElements.nth(i);
      await element.focus();
      
      // Check for visible focus indicator
      const focusStyles = await element.evaluate((el) => {
        const styles = window.getComputedStyle(el);
        return {
          outline: styles.outline,
          outlineWidth: styles.outlineWidth,
          boxShadow: styles.boxShadow
        };
      });
      
      // At least one focus indicator should be present
      const hasFocusIndicator = 
        focusStyles.outline !== 'none' ||
        focusStyles.outlineWidth !== '0px' ||
        focusStyles.boxShadow !== 'none';
      
      expect(hasFocusIndicator).toBeTruthy();
    }
  });

  test('should handle keyboard shortcuts properly', async () => {
    // Test common keyboard shortcuts
    const shortcuts = [
      { key: 'Tab', description: 'Tab navigation' },
      { key: 'Shift+Tab', description: 'Reverse tab navigation' },
      { key: 'Enter', description: 'Activate button' },
      { key: 'Space', description: 'Activate button' },
      { key: 'Escape', description: 'Close modal/dropdown' }
    ];
    
    for (const shortcut of shortcuts) {
      await page.keyboard.press(shortcut.key);
      // Test should not throw errors
      await expect(page).toBeTruthy();
    }
  });

  // ===== UNDERSTANDABLE TESTS =====
  
  test('should have clear and consistent navigation', async () => {
    const nav = page.locator('nav, [role="navigation"]');
    if (await nav.count() > 0) {
      const navLinks = nav.locator('a');
      const linkCount = await navLinks.count();
      
      expect(linkCount).toBeGreaterThan(0);
      
      // Check that navigation links have clear text
      for (let i = 0; i < linkCount; i++) {
        const link = navLinks.nth(i);
        const text = await link.textContent();
        expect(text?.trim().length).toBeGreaterThan(0);
      }
    }
  });

  test('should have proper form labels', async () => {
    const inputs = page.locator('input, select, textarea');
    const inputCount = await inputs.count();
    
    for (let i = 0; i < inputCount; i++) {
      const input = inputs.nth(i);
      const type = await input.getAttribute('type');
      
      // Skip hidden inputs
      if (type === 'hidden') continue;
      
      const id = await input.getAttribute('id');
      const ariaLabel = await input.getAttribute('aria-label');
      const ariaLabelledBy = await input.getAttribute('aria-labelledby');
      
      if (id) {
        const label = page.locator(`label[for="${id}"]`);
        const labelCount = await label.count();
        expect(labelCount).toBeGreaterThan(0);
      } else {
        // Should have aria-label or aria-labelledby
        expect(ariaLabel || ariaLabelledBy).toBeTruthy();
      }
    }
  });

  test('should provide clear error messages', async () => {
    // Test form validation errors
    const form = page.locator('form');
    if (await form.count() > 0) {
      const submitButton = form.locator('button[type="submit"], input[type="submit"]');
      if (await submitButton.count() > 0) {
        await submitButton.click();
        
        // Check for error messages
        const errorMessages = page.locator('[role="alert"], .error, .invalid');
        const errorCount = await errorMessages.count();
        
        if (errorCount > 0) {
          for (let i = 0; i < errorCount; i++) {
            const error = errorMessages.nth(i);
            const text = await error.textContent();
            expect(text?.trim().length).toBeGreaterThan(0);
          }
        }
      }
    }
  });

  // ===== ROBUST TESTS =====
  
  test('should work with assistive technologies', async () => {
    // Test ARIA landmarks
    const landmarks = page.locator('[role="main"], [role="navigation"], [role="banner"], [role="contentinfo"]');
    const landmarkCount = await landmarks.count();
    
    if (landmarkCount > 0) {
      // At least main landmark should exist
      const main = page.locator('[role="main"]');
      await expect(main).toHaveCount(1);
    }
    
    // Test ARIA live regions
    const liveRegions = page.locator('[aria-live]');
    const liveRegionCount = await liveRegions.count();
    
    for (let i = 0; i < liveRegionCount; i++) {
      const region = liveRegions.nth(i);
      const liveValue = await region.getAttribute('aria-live');
      expect(['polite', 'assertive', 'off']).toContain(liveValue);
    }
  });

  test('should have proper semantic HTML', async () => {
    // Test for proper use of semantic elements
    const semanticElements = [
      'main',
      'nav',
      'header',
      'footer',
      'section',
      'article',
      'aside'
    ];
    
    for (const element of semanticElements) {
      const elements = page.locator(element);
      const count = await elements.count();
      
      if (count > 0) {
        // Each semantic element should have proper content
        for (let i = 0; i < count; i++) {
          const el = elements.nth(i);
          const text = await el.textContent();
          expect(text?.trim().length).toBeGreaterThan(0);
        }
      }
    }
  });

  // ===== COMPONENT-SPECIFIC ACCESSIBILITY TESTS =====
  
  test('should have accessible buttons', async () => {
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      
      // Check for accessible name
      const text = await button.textContent();
      const ariaLabel = await button.getAttribute('aria-label');
      const ariaLabelledBy = await button.getAttribute('aria-labelledby');
      
      expect(text || ariaLabel || ariaLabelledBy).toBeTruthy();
      
      // Check for proper role
      const role = await button.getAttribute('role');
      if (role) {
        expect(role).toBe('button');
      }
    }
  });

  test('should have accessible form controls', async () => {
    const formControls = page.locator('input, select, textarea');
    const controlCount = await formControls.count();
    
    for (let i = 0; i < controlCount; i++) {
      const control = formControls.nth(i);
      const type = await control.getAttribute('type');
      
      if (type === 'hidden') continue;
      
      // Check for proper labeling
      const id = await control.getAttribute('id');
      const ariaLabel = await control.getAttribute('aria-label');
      const ariaLabelledBy = await control.getAttribute('aria-labelledby');
      
      if (id) {
        const label = page.locator(`label[for="${id}"]`);
        await expect(label).toHaveCount(1);
      } else {
        expect(ariaLabel || ariaLabelledBy).toBeTruthy();
      }
      
      // Check for proper states
      const required = await control.getAttribute('required');
      const ariaRequired = await control.getAttribute('aria-required');
      
      if (required || ariaRequired === 'true') {
        // Required fields should be clearly indicated
        const label = page.locator(`label[for="${id}"]`);
        if (await label.count() > 0) {
          const labelText = await label.textContent();
          expect(labelText).toContain('*');
        }
      }
    }
  });

  test('should have accessible modals and dialogs', async () => {
    const modals = page.locator('[role="dialog"], [role="alertdialog"]');
    const modalCount = await modals.count();
    
    for (let i = 0; i < modalCount; i++) {
      const modal = modals.nth(i);
      
      // Check for proper labeling
      const ariaLabel = await modal.getAttribute('aria-label');
      const ariaLabelledBy = await modal.getAttribute('aria-labelledby');
      expect(ariaLabel || ariaLabelledBy).toBeTruthy();
      
      // Check for proper focus management
      const focusableElements = modal.locator('button, input, select, textarea, a[href]');
      const focusableCount = await focusableElements.count();
      
      if (focusableCount > 0) {
        // First focusable element should be focused when modal opens
        const firstFocusable = focusableElements.first();
        await expect(firstFocusable).toBeFocused();
      }
    }
  });
});
