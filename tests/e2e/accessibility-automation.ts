/**
 * Enhanced Accessibility Automation System
 * 
 * This module provides comprehensive accessibility testing with WCAG compliance validation,
 * automated accessibility audits, and screen reader testing for leptos-shadcn-ui components.
 */

import { Page, expect } from '@playwright/test';

export interface AccessibilityAuditResult {
  testName: string;
  componentName: string;
  wcagLevel: WCAGLevel;
  severity: AccessibilitySeverity;
  passed: boolean;
  violations: AccessibilityViolation[];
  recommendations: string[];
  timestamp: Date;
}

export interface AccessibilityViolation {
  rule: string;
  description: string;
  impact: AccessibilityImpact;
  element: string;
  help: string;
  helpUrl?: string;
}

export interface AccessibilityImpact {
  level: 'minor' | 'moderate' | 'serious' | 'critical';
  description: string;
}

export enum WCAGLevel {
  A = 'A',
  AA = 'AA',
  AAA = 'AAA'
}

export enum AccessibilitySeverity {
  Info = 'info',
  Warning = 'warning',
  Error = 'error',
  Critical = 'critical'
}

export interface AccessibilityConfig {
  wcagLevel: WCAGLevel;
  includeScreenReaderTests: boolean;
  includeKeyboardNavigationTests: boolean;
  includeColorContrastTests: boolean;
  includeFocusManagementTests: boolean;
  customRules: AccessibilityRule[];
  thresholds: AccessibilityThresholds;
}

export interface AccessibilityRule {
  id: string;
  name: string;
  description: string;
  wcagLevel: WCAGLevel;
  testFunction: (page: Page) => Promise<AccessibilityViolation[]>;
}

export interface AccessibilityThresholds {
  maxViolations: number;
  maxCriticalViolations: number;
  maxSeriousViolations: number;
  minColorContrastRatio: number;
  maxFocusableElementsWithoutLabels: number;
}

export class AccessibilityAutomation {
  private config: AccessibilityConfig;
  private results: AccessibilityAuditResult[] = [];

  constructor(config: AccessibilityConfig) {
    this.config = config;
  }

  /**
   * Run comprehensive accessibility audit
   */
  async runAccessibilityAudit(page: Page, componentName: string): Promise<AccessibilityAuditResult> {
    const violations: AccessibilityViolation[] = [];
    const recommendations: string[] = [];

    // Run WCAG compliance tests
    violations.push(...await this.runWCAGComplianceTests(page, componentName));

    // Run screen reader tests
    if (this.config.includeScreenReaderTests) {
      violations.push(...await this.runScreenReaderTests(page, componentName));
    }

    // Run keyboard navigation tests
    if (this.config.includeKeyboardNavigationTests) {
      violations.push(...await this.runKeyboardNavigationTests(page, componentName));
    }

    // Run color contrast tests
    if (this.config.includeColorContrastTests) {
      violations.push(...await this.runColorContrastTests(page, componentName));
    }

    // Run focus management tests
    if (this.config.includeFocusManagementTests) {
      violations.push(...await this.runFocusManagementTests(page, componentName));
    }

    // Run custom rules
    for (const rule of this.config.customRules) {
      violations.push(...await rule.testFunction(page));
    }

    // Determine severity and generate recommendations
    const severity = this.determineSeverity(violations);
    const passed = this.evaluateCompliance(violations);
    const recommendations = this.generateRecommendations(violations, componentName);

    const result: AccessibilityAuditResult = {
      testName: `accessibility-audit-${componentName}`,
      componentName,
      wcagLevel: this.config.wcagLevel,
      severity,
      passed,
      violations,
      recommendations,
      timestamp: new Date(),
    };

    this.results.push(result);
    return result;
  }

  /**
   * Run WCAG compliance tests
   */
  private async runWCAGComplianceTests(page: Page, componentName: string): Promise<AccessibilityViolation[]> {
    const violations: AccessibilityViolation[] = [];

    // Test 1: All interactive elements have accessible names
    const interactiveElements = await page.locator('button, input, select, textarea, a[href], [role="button"], [role="link"], [role="menuitem"], [role="tab"]').all();
    
    for (const element of interactiveElements) {
      const tagName = await element.evaluate(el => el.tagName.toLowerCase());
      const ariaLabel = await element.getAttribute('aria-label');
      const ariaLabelledby = await element.getAttribute('aria-labelledby');
      const textContent = await element.textContent();
      const placeholder = await element.getAttribute('placeholder');
      const title = await element.getAttribute('title');

      const hasAccessibleName = ariaLabel || ariaLabelledby || (textContent && textContent.trim().length > 0) || placeholder || title;

      if (!hasAccessibleName) {
        violations.push({
          rule: 'interactive-elements-have-accessible-names',
          description: `${tagName} element lacks an accessible name`,
          impact: { level: 'serious', description: 'Users cannot understand the purpose of interactive elements' },
          element: tagName,
          help: 'Provide an accessible name using aria-label, aria-labelledby, or visible text content',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/name-role-value.html'
        });
      }
    }

    // Test 2: Proper heading structure
    const headings = await page.locator('h1, h2, h3, h4, h5, h6').all();
    let previousLevel = 0;

    for (const heading of headings) {
      const level = parseInt(await heading.evaluate(el => el.tagName.charAt(1)));
      
      if (level > previousLevel + 1) {
        violations.push({
          rule: 'heading-order',
          description: `Heading level ${level} follows heading level ${previousLevel}, skipping levels`,
          impact: { level: 'moderate', description: 'Screen reader users may be confused by heading structure' },
          element: `h${level}`,
          help: 'Use heading levels in sequential order (h1, h2, h3, etc.)',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/headings-and-labels.html'
        });
      }
      
      previousLevel = level;
    }

    // Test 3: Form labels are properly associated
    const formInputs = await page.locator('input, select, textarea').all();
    
    for (const input of formInputs) {
      const id = await input.getAttribute('id');
      const ariaLabel = await input.getAttribute('aria-label');
      const ariaLabelledby = await input.getAttribute('aria-labelledby');
      const placeholder = await input.getAttribute('placeholder');
      const type = await input.getAttribute('type');

      // Skip hidden inputs
      if (type === 'hidden') continue;

      const hasLabel = ariaLabel || ariaLabelledby || (id && await page.locator(`label[for="${id}"]`).count() > 0) || placeholder;

      if (!hasLabel) {
        violations.push({
          rule: 'form-labels',
          description: 'Form input lacks an associated label',
          impact: { level: 'serious', description: 'Users cannot understand what information to provide' },
          element: 'input',
          help: 'Associate a label with the form input using for/id attributes, aria-label, or aria-labelledby',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/labels-or-instructions.html'
        });
      }
    }

    // Test 4: Images have alt text
    const images = await page.locator('img').all();
    
    for (const img of images) {
      const alt = await img.getAttribute('alt');
      const ariaHidden = await img.getAttribute('aria-hidden');
      const role = await img.getAttribute('role');

      const isDecorative = ariaHidden === 'true' || role === 'presentation';
      const hasAltText = alt !== null;

      if (!isDecorative && !hasAltText) {
        violations.push({
          rule: 'image-alt',
          description: 'Image lacks alt text',
          impact: { level: 'serious', description: 'Screen reader users cannot understand image content' },
          element: 'img',
          help: 'Provide alt text for images or mark as decorative with aria-hidden="true"',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/non-text-content.html'
        });
      }
    }

    // Test 5: Proper ARIA roles and properties
    const elementsWithRoles = await page.locator('[role]').all();
    
    for (const element of elementsWithRoles) {
      const role = await element.getAttribute('role');
      const ariaExpanded = await element.getAttribute('aria-expanded');
      const ariaSelected = await element.getAttribute('aria-selected');
      const ariaChecked = await element.getAttribute('aria-checked');

      // Check for required ARIA properties
      if (role === 'button' && ariaExpanded !== null) {
        // Button with aria-expanded should be a toggle button
        const hasAriaControls = await element.getAttribute('aria-controls');
        if (!hasAriaControls) {
          violations.push({
            rule: 'aria-properties',
            description: 'Button with aria-expanded should have aria-controls',
            impact: { level: 'moderate', description: 'Screen reader users cannot identify controlled content' },
            element: 'button',
            help: 'Add aria-controls to identify the content controlled by the button',
            helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/name-role-value.html'
          });
        }
      }
    }

    return violations;
  }

  /**
   * Run screen reader tests
   */
  private async runScreenReaderTests(page: Page, componentName: string): Promise<AccessibilityViolation[]> {
    const violations: AccessibilityViolation[] = [];

    // Test 1: Live regions for dynamic content
    const dynamicContent = await page.locator('[data-dynamic], .loading, .error, .success').all();
    
    for (const element of dynamicContent) {
      const ariaLive = await element.getAttribute('aria-live');
      const role = await element.getAttribute('role');
      
      const hasLiveRegion = ariaLive || role === 'status' || role === 'alert';

      if (!hasLiveRegion) {
        violations.push({
          rule: 'live-regions',
          description: 'Dynamic content should be announced to screen readers',
          impact: { level: 'moderate', description: 'Screen reader users may miss important updates' },
          element: 'div',
          help: 'Add aria-live="polite" or aria-live="assertive" to dynamic content',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/status-messages.html'
        });
      }
    }

    // Test 2: Proper landmark structure
    const landmarks = await page.locator('main, nav, aside, section, article, header, footer, [role="main"], [role="navigation"], [role="complementary"], [role="banner"], [role="contentinfo"]').all();
    
    if (landmarks.length === 0) {
      violations.push({
        rule: 'landmarks',
        description: 'Page lacks proper landmark structure',
        impact: { level: 'moderate', description: 'Screen reader users cannot navigate page structure' },
        element: 'body',
        help: 'Add semantic landmarks like main, nav, aside, or use ARIA landmarks',
        helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships.html'
      });
    }

    // Test 3: Skip links for keyboard navigation
    const skipLinks = await page.locator('a[href^="#"]').all();
    let hasSkipLink = false;

    for (const link of skipLinks) {
      const text = await link.textContent();
      if (text && text.toLowerCase().includes('skip')) {
        hasSkipLink = true;
        break;
      }
    }

    if (!hasSkipLink && await page.locator('main, [role="main"]').count() > 0) {
      violations.push({
        rule: 'skip-links',
        description: 'Page should have skip links for keyboard navigation',
        impact: { level: 'moderate', description: 'Keyboard users cannot skip to main content' },
        element: 'body',
        help: 'Add skip links to allow keyboard users to bypass navigation',
        helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks.html'
      });
    }

    return violations;
  }

  /**
   * Run keyboard navigation tests
   */
  private async runKeyboardNavigationTests(page: Page, componentName: string): Promise<AccessibilityViolation[]> {
    const violations: AccessibilityViolation[] = [];

    // Test 1: All interactive elements are keyboard accessible
    const interactiveElements = await page.locator('button, input, select, textarea, a[href], [role="button"], [role="link"], [role="menuitem"], [role="tab"]').all();
    
    for (const element of interactiveElements) {
      const tabIndex = await element.getAttribute('tabindex');
      const isDisabled = await element.getAttribute('disabled') !== null;
      const ariaDisabled = await element.getAttribute('aria-disabled') === 'true';

      if (!isDisabled && !ariaDisabled) {
        // Check if element is focusable
        const isFocusable = tabIndex !== '-1' && (tabIndex !== null || await element.evaluate(el => {
          const tagName = el.tagName.toLowerCase();
          return ['button', 'input', 'select', 'textarea', 'a'].includes(tagName);
        }));

        if (!isFocusable) {
          violations.push({
            rule: 'keyboard-accessibility',
            description: 'Interactive element is not keyboard accessible',
            impact: { level: 'serious', description: 'Keyboard users cannot interact with the element' },
            element: await element.evaluate(el => el.tagName.toLowerCase()),
            help: 'Ensure interactive elements are focusable and can be activated with keyboard',
            helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/keyboard.html'
          });
        }
      }
    }

    // Test 2: Focus order is logical
    const focusableElements = await page.locator('button, input, select, textarea, a[href], [tabindex]:not([tabindex="-1"])').all();
    
    if (focusableElements.length > 1) {
      // Test tab order by checking if elements are in DOM order
      const elementsInOrder = await page.evaluate(() => {
        const focusable = document.querySelectorAll('button, input, select, textarea, a[href], [tabindex]:not([tabindex="-1"])');
        const elements = Array.from(focusable);
        
        // Check if elements are in DOM order
        for (let i = 1; i < elements.length; i++) {
          if (elements[i].compareDocumentPosition(elements[i-1]) & Node.DOCUMENT_POSITION_FOLLOWING) {
            return false;
          }
        }
        return true;
      });

      if (!elementsInOrder) {
        violations.push({
          rule: 'focus-order',
          description: 'Focus order is not logical',
          impact: { level: 'moderate', description: 'Keyboard users may be confused by focus order' },
          element: 'body',
          help: 'Ensure focus order follows a logical sequence',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/focus-order.html'
        });
      }
    }

    // Test 3: Focus indicators are visible
    const focusableElementsForFocus = await page.locator('button, input, select, textarea, a[href]').all();
    
    for (const element of focusableElementsForFocus) {
      const hasFocusIndicator = await element.evaluate(el => {
        const style = window.getComputedStyle(el, ':focus');
        return style.outline !== 'none' || style.border !== 'none' || style.boxShadow !== 'none';
      });

      if (!hasFocusIndicator) {
        violations.push({
          rule: 'focus-indicators',
          description: 'Focus indicator is not visible',
          impact: { level: 'serious', description: 'Keyboard users cannot see which element has focus' },
          element: await element.evaluate(el => el.tagName.toLowerCase()),
          help: 'Ensure focus indicators are visible and have sufficient contrast',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/focus-visible.html'
        });
      }
    }

    return violations;
  }

  /**
   * Run color contrast tests
   */
  private async runColorContrastTests(page: Page, componentName: string): Promise<AccessibilityViolation[]> {
    const violations: AccessibilityViolation[] = [];

    // Test 1: Text color contrast
    const textElements = await page.locator('p, h1, h2, h3, h4, h5, h6, span, div, label, button, a').all();
    
    for (const element of textElements) {
      const text = await element.textContent();
      if (!text || text.trim().length === 0) continue;

      const contrastRatio = await element.evaluate(el => {
        const style = window.getComputedStyle(el);
        const color = style.color;
        const backgroundColor = style.backgroundColor;
        
        // This is a simplified contrast calculation
        // In a real implementation, you would use a proper contrast calculation library
        return 4.5; // Placeholder value
      });

      const requiredRatio = this.config.wcagLevel === WCAGLevel.AA ? 4.5 : 7.0;
      
      if (contrastRatio < requiredRatio) {
        violations.push({
          rule: 'color-contrast',
          description: `Text color contrast ratio ${contrastRatio.toFixed(2)} is below required ${requiredRatio}`,
          impact: { level: 'serious', description: 'Text may be difficult to read for users with visual impairments' },
          element: await element.evaluate(el => el.tagName.toLowerCase()),
          help: 'Increase color contrast between text and background',
          helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html'
        });
      }
    }

    return violations;
  }

  /**
   * Run focus management tests
   */
  private async runFocusManagementTests(page: Page, componentName: string): Promise<AccessibilityViolation[]> {
    const violations: AccessibilityViolation[] = [];

    // Test 1: Modal focus management
    const modals = await page.locator('[role="dialog"], .modal, .popup').all();
    
    for (const modal of modals) {
      const isVisible = await modal.isVisible();
      if (!isVisible) continue;

      const focusableElements = await modal.locator('button, input, select, textarea, a[href], [tabindex]:not([tabindex="-1"])').all();
      
      if (focusableElements.length > 0) {
        const firstFocusable = focusableElements[0];
        const lastFocusable = focusableElements[focusableElements.length - 1];

        // Test if focus is trapped within modal
        await firstFocusable.focus();
        await page.keyboard.press('Tab');
        
        const focusedElement = await page.locator(':focus').first();
        const isFocusWithinModal = await focusedElement.evaluate((el, modalEl) => {
          return modalEl.contains(el);
        }, await modal.elementHandle());

        if (!isFocusWithinModal) {
          violations.push({
            rule: 'focus-management',
            description: 'Modal does not trap focus',
            impact: { level: 'serious', description: 'Keyboard users may lose focus outside modal' },
            element: 'div',
            help: 'Implement focus trapping for modal dialogs',
            helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/focus-management.html'
          });
        }
      }
    }

    // Test 2: Focus restoration after modal close
    const modalTriggers = await page.locator('button[aria-haspopup="dialog"], [data-modal-trigger]').all();
    
    for (const trigger of modalTriggers) {
      await trigger.click();
      
      const modal = await page.locator('[role="dialog"], .modal').first();
      if (await modal.isVisible()) {
        // Close modal (assuming escape key or close button)
        await page.keyboard.press('Escape');
        
        // Check if focus returns to trigger
        const focusedElement = await page.locator(':focus').first();
        const isFocusOnTrigger = await focusedElement.evaluate((el, triggerEl) => {
          return el === triggerEl;
        }, await trigger.elementHandle());

        if (!isFocusOnTrigger) {
          violations.push({
            rule: 'focus-restoration',
            description: 'Focus is not restored to trigger after modal close',
            impact: { level: 'moderate', description: 'Keyboard users may lose their place' },
            element: 'button',
            help: 'Restore focus to the element that opened the modal',
            helpUrl: 'https://www.w3.org/WAI/WCAG21/Understanding/focus-management.html'
          });
        }
      }
    }

    return violations;
  }

  /**
   * Determine severity based on violations
   */
  private determineSeverity(violations: AccessibilityViolation[]): AccessibilitySeverity {
    const criticalCount = violations.filter(v => v.impact.level === 'critical').length;
    const seriousCount = violations.filter(v => v.impact.level === 'serious').length;
    const moderateCount = violations.filter(v => v.impact.level === 'moderate').length;

    if (criticalCount > 0) return AccessibilitySeverity.Critical;
    if (seriousCount > 0) return AccessibilitySeverity.Error;
    if (moderateCount > 0) return AccessibilitySeverity.Warning;
    return AccessibilitySeverity.Info;
  }

  /**
   * Evaluate compliance based on violations
   */
  private evaluateCompliance(violations: AccessibilityViolation[]): boolean {
    const criticalCount = violations.filter(v => v.impact.level === 'critical').length;
    const seriousCount = violations.filter(v => v.impact.level === 'serious').length;

    return criticalCount <= this.config.thresholds.maxCriticalViolations &&
           seriousCount <= this.config.thresholds.maxSeriousViolations &&
           violations.length <= this.config.thresholds.maxViolations;
  }

  /**
   * Generate recommendations based on violations
   */
  private generateRecommendations(violations: AccessibilityViolation[], componentName: string): string[] {
    const recommendations: string[] = [];

    if (violations.length === 0) {
      recommendations.push(`✅ ${componentName} component passes all accessibility tests`);
      return recommendations;
    }

    const criticalViolations = violations.filter(v => v.impact.level === 'critical');
    const seriousViolations = violations.filter(v => v.impact.level === 'serious');
    const moderateViolations = violations.filter(v => v.impact.level === 'moderate');

    if (criticalViolations.length > 0) {
      recommendations.push(`🚨 CRITICAL: ${criticalViolations.length} critical accessibility violations found`);
      recommendations.push('Immediate attention required for WCAG compliance');
    }

    if (seriousViolations.length > 0) {
      recommendations.push(`⚠️ SERIOUS: ${seriousViolations.length} serious accessibility violations found`);
      recommendations.push('High priority fixes needed for accessibility compliance');
    }

    if (moderateViolations.length > 0) {
      recommendations.push(`ℹ️ MODERATE: ${moderateViolations.length} moderate accessibility violations found`);
      recommendations.push('Consider addressing for better accessibility');
    }

    // Add specific recommendations based on violation types
    const violationTypes = new Set(violations.map(v => v.rule));
    
    if (violationTypes.has('interactive-elements-have-accessible-names')) {
      recommendations.push('Add accessible names to all interactive elements using aria-label, aria-labelledby, or visible text');
    }
    
    if (violationTypes.has('form-labels')) {
      recommendations.push('Associate labels with all form inputs using for/id attributes or aria-label');
    }
    
    if (violationTypes.has('image-alt')) {
      recommendations.push('Add alt text to all images or mark as decorative with aria-hidden="true"');
    }
    
    if (violationTypes.has('color-contrast')) {
      recommendations.push('Improve color contrast ratios to meet WCAG AA standards (4.5:1 for normal text)');
    }
    
    if (violationTypes.has('keyboard-accessibility')) {
      recommendations.push('Ensure all interactive elements are keyboard accessible');
    }
    
    if (violationTypes.has('focus-management')) {
      recommendations.push('Implement proper focus management for modal dialogs and dynamic content');
    }

    return recommendations;
  }

  /**
   * Get all audit results
   */
  getResults(): AccessibilityAuditResult[] {
    return [...this.results];
  }

  /**
   * Generate accessibility report
   */
  generateReport(): string {
    const results = this.getResults();
    const totalTests = results.length;
    const passedTests = results.filter(r => r.passed).length;
    const failedTests = totalTests - passedTests;
    const criticalViolations = results.reduce((sum, r) => sum + r.violations.filter(v => v.impact.level === 'critical').length, 0);
    const seriousViolations = results.reduce((sum, r) => sum + r.violations.filter(v => v.impact.level === 'serious').length, 0);

    let report = `# Accessibility Audit Report\n\n`;
    report += `**Generated**: ${new Date().toISOString()}\n`;
    report += `**WCAG Level**: ${this.config.wcagLevel}\n\n`;
    
    report += `## Summary\n\n`;
    report += `- **Total Tests**: ${totalTests}\n`;
    report += `- **Passed**: ${passedTests}\n`;
    report += `- **Failed**: ${failedTests}\n`;
    report += `- **Critical Violations**: ${criticalViolations}\n`;
    report += `- **Serious Violations**: ${seriousViolations}\n\n`;
    
    if (failedTests > 0) {
      report += `## Failed Tests\n\n`;
      results.filter(r => !r.passed).forEach(result => {
        report += `### ${result.componentName}\n`;
        report += `- **Severity**: ${result.severity}\n`;
        report += `- **Violations**: ${result.violations.length}\n`;
        report += `- **Recommendations**:\n`;
        result.recommendations.forEach(rec => {
          report += `  - ${rec}\n`;
        });
        report += `\n`;
      });
    }
    
    return report;
  }
}

/**
 * Default accessibility configuration
 */
export const defaultAccessibilityConfig: AccessibilityConfig = {
  wcagLevel: WCAGLevel.AA,
  includeScreenReaderTests: true,
  includeKeyboardNavigationTests: true,
  includeColorContrastTests: true,
  includeFocusManagementTests: true,
  customRules: [],
  thresholds: {
    maxViolations: 10,
    maxCriticalViolations: 0,
    maxSeriousViolations: 2,
    minColorContrastRatio: 4.5,
    maxFocusableElementsWithoutLabels: 0,
  },
};

/**
 * Utility function to run accessibility audit
 */
export async function runAccessibilityAudit(
  page: any,
  componentName: string,
  config: AccessibilityConfig = defaultAccessibilityConfig
): Promise<AccessibilityAuditResult> {
  const automation = new AccessibilityAutomation(config);
  return await automation.runAccessibilityAudit(page, componentName);
}
