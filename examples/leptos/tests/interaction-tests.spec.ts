import { test, expect } from '@playwright/test';

test.describe('Interaction Tests @interaction', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('h1', { timeout: 10000 });
  });

  test('should switch themes dynamically', async ({ page }) => {
    // Get initial theme styles
    const body = page.locator('body');
    const initialBackground = await body.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    // Click on light theme button
    const lightThemeButton = page.locator('button').filter({ hasText: 'light' });
    await lightThemeButton.click();
    await page.waitForTimeout(500); // Wait for theme change
    
    // Check that the background has changed
    const lightBackground = await body.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    expect(lightBackground).not.toBe(initialBackground);
    
    // Click on dark theme button
    const darkThemeButton = page.locator('button').filter({ hasText: 'dark' });
    await darkThemeButton.click();
    await page.waitForTimeout(500);
    
    // Check that the background has changed again
    const darkBackground = await body.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    expect(darkBackground).not.toBe(lightBackground);
    expect(darkBackground).not.toBe(initialBackground);
  });

  test('should switch color schemes dynamically', async ({ page }) => {
    // Get initial color scheme
    const heroSection = page.locator('section').first();
    const initialStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        backgroundImage: styles.backgroundImage,
        boxShadow: styles.boxShadow,
      };
    });
    
    // Click on green color button
    const greenButton = page.locator('button').filter({ hasText: 'green' });
    await greenButton.click();
    await page.waitForTimeout(500);
    
    // Check that colors have changed
    const greenStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        backgroundImage: styles.backgroundImage,
        boxShadow: styles.boxShadow,
      };
    });
    
    expect(greenStyles.backgroundImage).not.toBe(initialStyles.backgroundImage);
    
    // Click on purple color button
    const purpleButton = page.locator('button').filter({ hasText: 'purple' });
    await purpleButton.click();
    await page.waitForTimeout(500);
    
    // Check that colors have changed again
    const purpleStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        backgroundImage: styles.backgroundImage,
        boxShadow: styles.boxShadow,
      };
    });
    
    expect(purpleStyles.backgroundImage).not.toBe(greenStyles.backgroundImage);
  });

  test('should update text gradients when color changes', async ({ page }) => {
    // Get initial text gradient
    const mainHeading = page.locator('h1').first();
    const initialGradient = await mainHeading.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    // Click on blue color button
    const blueButton = page.locator('button').filter({ hasText: 'blue' });
    await blueButton.click();
    await page.waitForTimeout(500);
    
    // Check that text gradient has changed
    const blueGradient = await mainHeading.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    expect(blueGradient).not.toBe(initialGradient);
    expect(blueGradient).toContain('blue');
  });

  test('should update animated overlays when color changes', async ({ page }) => {
    // Find the animated overlay
    const animatedOverlay = page.locator('div').filter({ hasText: '' }).first();
    const initialOverlay = await animatedOverlay.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    // Click on green color button
    const greenButton = page.locator('button').filter({ hasText: 'green' });
    await greenButton.click();
    await page.waitForTimeout(500);
    
    // Check that overlay gradient has changed
    const greenOverlay = await animatedOverlay.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return styles.backgroundImage;
    });
    
    expect(greenOverlay).not.toBe(initialOverlay);
    expect(greenOverlay).toContain('green');
  });

  test('should update component card styling when theme changes', async ({ page }) => {
    // Get initial card styles
    const componentCard = page.locator('[class*="card"]').first();
    const initialCardStyles = await componentCard.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        borderColor: styles.borderColor,
      };
    });
    
    // Click on dark theme
    const darkThemeButton = page.locator('button').filter({ hasText: 'dark' });
    await darkThemeButton.click();
    await page.waitForTimeout(500);
    
    // Check that card styling has changed
    const darkCardStyles = await componentCard.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        backgroundColor: styles.backgroundColor,
        borderColor: styles.borderColor,
      };
    });
    
    expect(darkCardStyles.backgroundColor).not.toBe(initialCardStyles.backgroundColor);
  });

  test('should maintain responsive behavior during theme changes', async ({ page }) => {
    // Test desktop view
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForTimeout(500);
    
    const heroSection = page.locator('section').first();
    const desktopStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        paddingTop: styles.paddingTop,
        paddingBottom: styles.paddingBottom,
      };
    });
    
    // Change theme
    const lightThemeButton = page.locator('button').filter({ hasText: 'light' });
    await lightThemeButton.click();
    await page.waitForTimeout(500);
    
    // Check that responsive behavior is maintained
    const desktopStylesAfterTheme = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        paddingTop: styles.paddingTop,
        paddingBottom: styles.paddingBottom,
      };
    });
    
    expect(desktopStylesAfterTheme.paddingTop).toBe(desktopStyles.paddingTop);
    
    // Test mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(500);
    
    const mobileStyles = await heroSection.evaluate((el) => {
      const styles = window.getComputedStyle(el);
      return {
        paddingTop: styles.paddingTop,
        paddingBottom: styles.paddingBottom,
      };
    });
    
    // Mobile padding should be smaller than desktop
    expect(parseInt(mobileStyles.paddingTop)).toBeLessThan(parseInt(desktopStyles.paddingTop));
  });
});

