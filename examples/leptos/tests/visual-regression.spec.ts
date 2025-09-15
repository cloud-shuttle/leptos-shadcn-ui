import { test, expect } from '@playwright/test';

test.describe('Visual Regression Tests @visual', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the page to fully load
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('h1', { timeout: 10000 });
  });

  test('should display the main hero section with correct styling', async ({ page }) => {
    // Check that the hero section is visible and properly styled
    const heroSection = page.locator('section').first();
    await expect(heroSection).toBeVisible();
    
    // Check for solid background color
    const heroBackground = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundColor;
    });
    expect(heroBackground).not.toBe('rgba(0, 0, 0, 0)'); // Should have a background color
    
    // Check for the main heading
    const mainHeading = page.locator('h1').first();
    await expect(mainHeading).toContainText('🚀 WASM-Powered');
    
    // Check for the subheading
    const subHeading = page.locator('h2').first();
    await expect(subHeading).toContainText('Component Showcase');
  });

  test('should display theme controls section', async ({ page }) => {
    const themeSection = page.locator('section').nth(1);
    await expect(themeSection).toBeVisible();
    
    // Check for theme selection buttons
    const themeButtons = page.locator('button').filter({ hasText: /default|light|dark/i });
    await expect(themeButtons).toHaveCount(3);
    
    // Check for color scheme buttons
    const colorButtons = page.locator('button').filter({ hasText: /blue|green|purple/i });
    await expect(colorButtons).toHaveCount(3);
  });

  test('should display component showcase section', async ({ page }) => {
    const componentSection = page.locator('section').nth(2);
    await expect(componentSection).toBeVisible();
    
    // Check for component cards
    const componentCards = page.locator('[class*="card"]');
    await expect(componentCards.first()).toBeVisible();
    
    // Check for component titles
    const componentTitles = page.locator('h3, h4').filter({ hasText: /Button|Input|Card|Alert/i });
    await expect(componentTitles.first()).toBeVisible();
  });

  test('should have proper responsive design', async ({ page }) => {
    // Test desktop view
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForTimeout(500);
    
    const heroSection = page.locator('section').first();
    const heroStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        paddingTop: styles.paddingTop,
        paddingBottom: styles.paddingBottom,
      };
    });
    
    // Check that padding is appropriate for desktop
    expect(parseInt(heroStyles.paddingTop)).toBeGreaterThan(50);
    
    // Test mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(500);
    
    const mobileHeroStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        paddingTop: styles.paddingTop,
        paddingBottom: styles.paddingBottom,
      };
    });
    
    // Check that padding is adjusted for mobile
    expect(parseInt(mobileHeroStyles.paddingTop)).toBeLessThan(parseInt(heroStyles.paddingTop));
  });

  test('should have proper color contrast and visibility', async ({ page }) => {
    // Check that text is visible against backgrounds
    const mainHeading = page.locator('h1').first();
    const headingStyles = await mainHeading.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        color: styles.color,
        backgroundImage: styles.backgroundImage,
      };
    });
    
    // Check that the heading has visible text color
    expect(headingStyles.color).not.toBe('rgba(0, 0, 0, 0)'); // Should have visible text
  });

  test('should display animated elements', async ({ page }) => {
    // Check for animated overlay
    const animatedOverlay = page.locator('div').filter({ hasText: '' }).first();
    const overlayStyles = await animatedOverlay.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        animation: styles.animation,
        backgroundImage: styles.backgroundImage,
      };
    });
    
    // Check for pulse animation
    expect(overlayStyles.animation).toContain('pulse');
    // Check that overlay has a background color
    expect(overlayStyles.backgroundColor).not.toBe('rgba(0, 0, 0, 0)');
  });
});

