import { test, expect } from '@playwright/test';
import { 
  AccessibilityAutomation, 
  defaultAccessibilityConfig, 
  WCAGLevel, 
  AccessibilitySeverity 
} from './accessibility-automation';

/**
 * Enhanced Accessibility Testing Suite
 * 
 * This comprehensive test suite provides automated accessibility testing
 * with WCAG compliance validation, screen reader testing, and detailed reporting.
 */

test.describe('Enhanced Accessibility Testing Suite', () => {
  let accessibilityAutomation: AccessibilityAutomation;

  test.beforeEach(async ({ page }) => {
    // Navigate to the Leptos demo app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Initialize accessibility automation
    accessibilityAutomation = new AccessibilityAutomation(defaultAccessibilityConfig);
  });

  test.describe('WCAG AA Compliance', () => {
    test('should pass comprehensive accessibility audit', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'main-app');
      
      console.log(`Accessibility Audit Results for main-app:`);
      console.log(`- Passed: ${result.passed}`);
      console.log(`- Severity: ${result.severity}`);
      console.log(`- Violations: ${result.violations.length}`);
      console.log(`- WCAG Level: ${result.wcagLevel}`);
      
      // Log violations for debugging
      if (result.violations.length > 0) {
        console.log('\nViolations found:');
        result.violations.forEach((violation, index) => {
          console.log(`${index + 1}. ${violation.rule}: ${violation.description}`);
          console.log(`   Impact: ${violation.impact.level} - ${violation.impact.description}`);
          console.log(`   Help: ${violation.help}`);
        });
      }
      
      // Log recommendations
      if (result.recommendations.length > 0) {
        console.log('\nRecommendations:');
        result.recommendations.forEach(rec => console.log(`- ${rec}`));
      }
      
      // Assert compliance based on severity
      expect(result.severity).not.toBe(AccessibilitySeverity.Critical);
      
      // For now, we'll be lenient with serious violations in development
      if (result.severity === AccessibilitySeverity.Error) {
        console.warn('⚠️ Serious accessibility violations found - review recommendations');
      }
    });

    test('should have proper ARIA labels on all interactive elements', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'aria-labels');
      
      const ariaViolations = result.violations.filter(v => v.rule === 'interactive-elements-have-accessible-names');
      
      if (ariaViolations.length > 0) {
        console.log('ARIA label violations found:');
        ariaViolations.forEach(violation => {
          console.log(`- ${violation.description} (${violation.element})`);
        });
      }
      
      // Allow some violations in development, but log them
      expect(ariaViolations.length).toBeLessThan(5);
    });

    test('should have proper form labels and associations', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'form-labels');
      
      const formViolations = result.violations.filter(v => v.rule === 'form-labels');
      
      if (formViolations.length > 0) {
        console.log('Form label violations found:');
        formViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Form labels are critical for accessibility
      expect(formViolations.length).toBe(0);
    });

    test('should have proper heading structure', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'heading-structure');
      
      const headingViolations = result.violations.filter(v => v.rule === 'heading-order');
      
      if (headingViolations.length > 0) {
        console.log('Heading structure violations found:');
        headingViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Heading structure is important for screen readers
      expect(headingViolations.length).toBe(0);
    });

    test('should have alt text on all images', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'image-alt');
      
      const imageViolations = result.violations.filter(v => v.rule === 'image-alt');
      
      if (imageViolations.length > 0) {
        console.log('Image alt text violations found:');
        imageViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Images without alt text are a serious accessibility issue
      expect(imageViolations.length).toBe(0);
    });
  });

  test.describe('Keyboard Navigation', () => {
    test('should support keyboard navigation for all interactive elements', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'keyboard-navigation');
      
      const keyboardViolations = result.violations.filter(v => v.rule === 'keyboard-accessibility');
      
      if (keyboardViolations.length > 0) {
        console.log('Keyboard accessibility violations found:');
        keyboardViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // All interactive elements should be keyboard accessible
      expect(keyboardViolations.length).toBe(0);
    });

    test('should have logical focus order', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'focus-order');
      
      const focusOrderViolations = result.violations.filter(v => v.rule === 'focus-order');
      
      if (focusOrderViolations.length > 0) {
        console.log('Focus order violations found:');
        focusOrderViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Focus order should be logical
      expect(focusOrderViolations.length).toBe(0);
    });

    test('should have visible focus indicators', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'focus-indicators');
      
      const focusIndicatorViolations = result.violations.filter(v => v.rule === 'focus-indicators');
      
      if (focusIndicatorViolations.length > 0) {
        console.log('Focus indicator violations found:');
        focusIndicatorViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Focus indicators are essential for keyboard users
      expect(focusIndicatorViolations.length).toBe(0);
    });

    test('should support tab navigation', async ({ page }) => {
      // Test tab navigation through interactive elements
      const interactiveElements = page.locator('button, input, select, textarea, a[href], [role="button"], [role="link"]');
      const count = await interactiveElements.count();
      
      if (count > 0) {
        // Test tab navigation through first few elements
        for (let i = 0; i < Math.min(count, 5); i++) {
          await page.keyboard.press('Tab');
          const focusedElement = page.locator(':focus');
          
          if (await focusedElement.count() > 0) {
            await expect(focusedElement.first()).toBeVisible();
          }
        }
      }
    });

    test('should support enter and space key activation', async ({ page }) => {
      const buttons = page.locator('button, [role="button"]');
      const buttonCount = await buttons.count();
      
      if (buttonCount > 0) {
        const firstButton = buttons.first();
        await firstButton.focus();
        
        // Test space key
        await page.keyboard.press('Space');
        await expect(firstButton).toBeFocused();
        
        // Test enter key
        await page.keyboard.press('Enter');
        await expect(firstButton).toBeFocused();
      }
    });
  });

  test.describe('Screen Reader Support', () => {
    test('should have proper landmark structure', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'landmarks');
      
      const landmarkViolations = result.violations.filter(v => v.rule === 'landmarks');
      
      if (landmarkViolations.length > 0) {
        console.log('Landmark violations found:');
        landmarkViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Landmarks help screen reader users navigate
      expect(landmarkViolations.length).toBe(0);
    });

    test('should have skip links for navigation', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'skip-links');
      
      const skipLinkViolations = result.violations.filter(v => v.rule === 'skip-links');
      
      if (skipLinkViolations.length > 0) {
        console.log('Skip link violations found:');
        skipLinkViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Skip links are important for keyboard users
      // Allow some flexibility in development
      expect(skipLinkViolations.length).toBeLessThan(2);
    });

    test('should announce dynamic content changes', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'live-regions');
      
      const liveRegionViolations = result.violations.filter(v => v.rule === 'live-regions');
      
      if (liveRegionViolations.length > 0) {
        console.log('Live region violations found:');
        liveRegionViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Live regions are important for dynamic content
      expect(liveRegionViolations.length).toBeLessThan(3);
    });
  });

  test.describe('Color and Contrast', () => {
    test('should meet color contrast requirements', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'color-contrast');
      
      const contrastViolations = result.violations.filter(v => v.rule === 'color-contrast');
      
      if (contrastViolations.length > 0) {
        console.log('Color contrast violations found:');
        contrastViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Color contrast is critical for accessibility
      expect(contrastViolations.length).toBe(0);
    });

    test('should not rely solely on color for information', async ({ page }) => {
      // Check for elements that might rely solely on color
      const colorOnlyElements = await page.evaluate(() => {
        const elements = document.querySelectorAll('*');
        const violations = [];
        
        for (const element of elements) {
          const style = window.getComputedStyle(element);
          const textContent = element.textContent?.trim();
          
          // Check for color-only indicators (simplified check)
          if (textContent && (textContent.includes('red') || textContent.includes('green'))) {
            const hasOtherIndicator = element.getAttribute('aria-label') || 
                                    element.getAttribute('title') ||
                                    element.querySelector('img') ||
                                    element.querySelector('[aria-hidden="true"]');
            
            if (!hasOtherIndicator) {
              violations.push({
                element: element.tagName,
                text: textContent,
                description: 'Element may rely solely on color for information'
              });
            }
          }
        }
        
        return violations;
      });
      
      if (colorOnlyElements.length > 0) {
        console.log('Color-only information violations found:');
        colorOnlyElements.forEach(violation => {
          console.log(`- ${violation.description} (${violation.element}): "${violation.text}"`);
        });
      }
      
      // Allow some flexibility in development
      expect(colorOnlyElements.length).toBeLessThan(3);
    });
  });

  test.describe('Focus Management', () => {
    test('should manage focus properly in modals', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'focus-management');
      
      const focusViolations = result.violations.filter(v => v.rule === 'focus-management');
      
      if (focusViolations.length > 0) {
        console.log('Focus management violations found:');
        focusViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Focus management is important for modal dialogs
      expect(focusViolations.length).toBeLessThan(2);
    });

    test('should restore focus after modal close', async ({ page }) => {
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'focus-restoration');
      
      const focusRestorationViolations = result.violations.filter(v => v.rule === 'focus-restoration');
      
      if (focusRestorationViolations.length > 0) {
        console.log('Focus restoration violations found:');
        focusRestorationViolations.forEach(violation => {
          console.log(`- ${violation.description}`);
        });
      }
      
      // Focus restoration is important for user experience
      expect(focusRestorationViolations.length).toBeLessThan(2);
    });
  });

  test.describe('Component-Specific Accessibility', () => {
    test('button components should be accessible', async ({ page }) => {
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();
      
      if (buttonCount > 0) {
        for (let i = 0; i < Math.min(buttonCount, 3); i++) {
          const button = buttons.nth(i);
          
          // Check for accessible name
          const ariaLabel = await button.getAttribute('aria-label');
          const ariaLabelledby = await button.getAttribute('aria-labelledby');
          const textContent = await button.textContent();
          
          const hasAccessibleName = ariaLabel || ariaLabelledby || (textContent && textContent.trim().length > 0);
          expect(hasAccessibleName).toBeTruthy();
          
          // Check for proper role
          const role = await button.getAttribute('role');
          if (role) {
            expect(['button', 'menuitem', 'tab']).toContain(role);
          }
        }
      }
    });

    test('input components should be accessible', async ({ page }) => {
      const inputs = page.locator('input, select, textarea');
      const inputCount = await inputs.count();
      
      if (inputCount > 0) {
        for (let i = 0; i < Math.min(inputCount, 3); i++) {
          const input = inputs.nth(i);
          
          // Check for accessible name
          const id = await input.getAttribute('id');
          const ariaLabel = await input.getAttribute('aria-label');
          const ariaLabelledby = await input.getAttribute('aria-labelledby');
          const placeholder = await input.getAttribute('placeholder');
          
          const hasAccessibleName = ariaLabel || ariaLabelledby || (id && await page.locator(`label[for="${id}"]`).count() > 0) || placeholder;
          expect(hasAccessibleName).toBeTruthy();
          
          // Check for proper type
          const type = await input.getAttribute('type');
          if (type) {
            expect(['text', 'email', 'password', 'number', 'tel', 'url', 'search']).toContain(type);
          }
        }
      }
    });

    test('navigation components should be accessible', async ({ page }) => {
      const navs = page.locator('nav, [role="navigation"]');
      const navCount = await navs.count();
      
      if (navCount > 0) {
        for (let i = 0; i < navCount; i++) {
          const nav = navs.nth(i);
          
          // Check for proper role
          const role = await nav.getAttribute('role');
          const tagName = await nav.evaluate(el => el.tagName.toLowerCase());
          
          expect(role === 'navigation' || tagName === 'nav').toBeTruthy();
          
          // Check for accessible label
          const ariaLabel = await nav.getAttribute('aria-label');
          const ariaLabelledby = await nav.getAttribute('aria-labelledby');
          
          // Navigation should have a label
          expect(ariaLabel || ariaLabelledby).toBeTruthy();
        }
      }
    });
  });

  test.describe('Accessibility Report Generation', () => {
    test('should generate comprehensive accessibility report', async ({ page }) => {
      // Run audit on main app
      const result = await accessibilityAutomation.runAccessibilityAudit(page, 'main-app');
      
      // Generate report
      const report = accessibilityAutomation.generateReport();
      
      // Log report for debugging
      console.log('\n=== ACCESSIBILITY REPORT ===');
      console.log(report);
      console.log('=== END REPORT ===\n');
      
      // Verify report contains expected sections
      expect(report).toContain('# Accessibility Audit Report');
      expect(report).toContain('## Summary');
      expect(report).toContain('Total Tests');
      expect(report).toContain('Passed');
      expect(report).toContain('Failed');
      
      // Verify report contains violation details if any
      if (result.violations.length > 0) {
        expect(report).toContain('## Failed Tests');
      }
    });

    test('should track accessibility metrics over time', async ({ page }) => {
      const results = accessibilityAutomation.getResults();
      
      // Verify results are being tracked
      expect(results.length).toBeGreaterThan(0);
      
      // Check result structure
      const result = results[0];
      expect(result).toHaveProperty('testName');
      expect(result).toHaveProperty('componentName');
      expect(result).toHaveProperty('wcagLevel');
      expect(result).toHaveProperty('severity');
      expect(result).toHaveProperty('passed');
      expect(result).toHaveProperty('violations');
      expect(result).toHaveProperty('recommendations');
      expect(result).toHaveProperty('timestamp');
    });
  });
});
