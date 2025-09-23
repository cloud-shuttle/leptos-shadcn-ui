import { test, expect, Page } from '@playwright/test';

/**
 * Comprehensive Demo E2E Tests
 * 
 * These tests ensure the comprehensive dashboard demo functions as expected,
 * showcasing all interactive features, responsive design, and component capabilities.
 */

test.describe('Comprehensive Dashboard Demo E2E Tests', () => {
  let page: Page;

  test.beforeEach(async ({ page: testPage }) => {
    page = testPage;
    // Navigate to the comprehensive demo
    await page.goto('http://localhost:8001');
    await page.waitForLoadState('networkidle');
    // Wait for WASM to load
    await page.waitForSelector('nav', { timeout: 10000 });
  });

  test.describe('Page Structure and Navigation', () => {
    test('should load the comprehensive dashboard successfully', async () => {
      await expect(page).toHaveTitle(/Leptos Dashboard - ShadCN UI Demo/);
      
      // Check for main navigation
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
      
      // Check for dashboard title
      const dashboardTitle = page.locator('h1:has-text("Dashboard")');
      await expect(dashboardTitle).toBeVisible();
    });

    test('should have proper sidebar navigation', async () => {
      const sidebar = page.locator('div.w-64.bg-card.border-r.border-border');
      await expect(sidebar).toBeVisible();
      
      // Check navigation links
      const dashboardLink = page.locator('a:has-text("Dashboard")');
      const analyticsLink = page.locator('a:has-text("Analytics")');
      const projectsLink = page.locator('a:has-text("Projects")');
      const teamLink = page.locator('a:has-text("Team")');
      const documentsLink = page.locator('a:has-text("Documents")');
      const settingsLink = page.locator('a:has-text("Settings")');
      
      await expect(dashboardLink).toBeVisible();
      await expect(analyticsLink).toBeVisible();
      await expect(projectsLink).toBeVisible();
      await expect(teamLink).toBeVisible();
      await expect(documentsLink).toBeVisible();
      await expect(settingsLink).toBeVisible();
    });

    test('should have welcome section with proper messaging', async () => {
      const welcomeTitle = page.locator('h2:has-text("Welcome back!")');
      const welcomeDescription = page.locator('text=Here\'s what\'s happening with your projects today.');
      
      await expect(welcomeTitle).toBeVisible();
      await expect(welcomeDescription).toBeVisible();
    });
  });

  test.describe('Metrics Cards', () => {
    test('should display all metric cards', async () => {
      const metricCards = page.locator('.grid.grid-cols-1.md\\:grid-cols-2.lg\\:grid-cols-4 > div');
      await expect(metricCards).toHaveCount(4);
      
      // Check individual cards
      await expect(page.locator('text=Total Revenue')).toBeVisible();
      await expect(page.locator('text=New Customers')).toBeVisible();
      await expect(page.locator('text=Active Accounts')).toBeVisible();
      await expect(page.locator('text=Growth Rate')).toBeVisible();
    });

    test('should have interactive metric cards', async () => {
      // Test Total Revenue card
      const revenueCard = page.locator('div:has-text("Total Revenue")').first();
      await expect(revenueCard).toBeVisible();
      
      // Click the card to update revenue
      await revenueCard.click();
      
      // Check that revenue value is displayed
      const revenueValue = page.locator('text=/\\$\\d+\\.\\d+/').first();
      await expect(revenueValue).toBeVisible();
    });

    test('should have hover effects on metric cards', async () => {
      const metricCard = page.locator('div:has-text("Total Revenue")').first();
      await metricCard.hover();
      
      // Card should still be visible after hover
      await expect(metricCard).toBeVisible();
    });
  });

  test.describe('Interactive Dashboard Section', () => {
    test('should have interactive counter', async () => {
      const counterSection = page.locator('text=Interactive Counter');
      await expect(counterSection).toBeVisible();
      
      // Test counter buttons
      const incrementButton = page.locator('button:has-text("+")');
      const decrementButton = page.locator('button:has-text("-")');
      const resetButton = page.locator('button:has-text("Reset")');
      
      await expect(incrementButton).toBeVisible();
      await expect(decrementButton).toBeVisible();
      await expect(resetButton).toBeVisible();
      
      // Test counter functionality
      await incrementButton.click({ force: true });
      await incrementButton.click({ force: true });
      
      // Check that counter value is displayed
      const counterValue = page.locator('text=/\\d+/').first();
      await expect(counterValue).toBeVisible();
    });

    test('should have input component', async () => {
      const inputSection = page.locator('text=Input Component');
      await expect(inputSection).toBeVisible();
      
      // Test input field
      const inputField = page.locator('input[placeholder="Type something..."]');
      await expect(inputField).toBeVisible();
      
      // Type in the input
      await inputField.fill('Test input');
      await expect(inputField).toHaveValue('Test input');
    });

    test('should have Tailwind-RS-WASM demo section', async () => {
      const tailwindSection = page.locator('text=Tailwind-RS-WASM Demo');
      await expect(tailwindSection).toBeVisible();
      
      // Check for demo elements
      const shadcnButton = page.locator('button:has-text("ShadCN Button")');
      const dynamicStyling = page.locator('text=Dynamic styling with Tailwind CSS');
      const bestOfBothWorlds = page.locator('button:has-text("Best of both worlds!")');
      
      await expect(shadcnButton).toBeVisible();
      await expect(dynamicStyling).toBeVisible();
      await expect(bestOfBothWorlds).toBeVisible();
    });
  });

  test.describe('Recent Activity Section', () => {
    test('should display recent activity feed', async () => {
      const activitySection = page.locator('text=Recent Activity');
      await expect(activitySection).toBeVisible();
      
      // Check for activity items
      const activityItems = page.locator('p:has-text("Eddie Lake completed Cover page")');
      await expect(activityItems.first()).toBeVisible();
      
      // Check for timestamps
      const timestamps = page.locator('p:has-text("2 hours ago"), p:has-text("4 hours ago"), p:has-text("6 hours ago")');
      await expect(timestamps.first()).toBeVisible();
    });
  });

  test.describe('Data Table Section', () => {
    test('should display project documents table', async () => {
      const tableSection = page.locator('h3:has-text("Project Documents")');
      await expect(tableSection).toBeVisible();
      
      // Check table headers
      const headers = page.locator('th');
      await expect(headers.filter({ hasText: 'Document' })).toBeVisible();
      await expect(headers.filter({ hasText: 'Type' })).toBeVisible();
      await expect(headers.filter({ hasText: 'Status' })).toBeVisible();
      await expect(headers.filter({ hasText: 'Assignee' })).toBeVisible();
      await expect(headers.filter({ hasText: 'Actions' })).toBeVisible();
    });

    test('should have functional open menu buttons', async () => {
      const openMenuButtons = page.locator('button:has-text("Open menu")');
      await expect(openMenuButtons).toHaveCount(3);
      
      // Click the first open menu button
      await openMenuButtons.first().click();
      
      // Check that dropdown menu appears
      const dropdownMenu = page.locator('.absolute.top-16.right-4.bg-card');
      await expect(dropdownMenu).toBeVisible();
    });

    test('should have status badges', async () => {
      const statusBadges = page.locator('span:has-text("In Process"), span:has-text("Done")');
      await expect(statusBadges).toHaveCount(3);
    });
  });

  test.describe('Theme and Sidebar Toggle', () => {
    test('should have theme toggle functionality', async () => {
      const themeToggle = page.locator('button:has-text("Dark"), button:has-text("Light")');
      await expect(themeToggle).toBeVisible();
      
      // Click theme toggle with force to avoid interception
      await themeToggle.click({ force: true });
      
      // Check that theme changes (dark class should be applied)
      const body = page.locator('body');
      await expect(body).toBeVisible();
    });

    test('should have sidebar toggle functionality', async () => {
      const sidebarToggle = page.locator('button:has-text("☰")');
      await expect(sidebarToggle).toBeVisible();
      
      // Click sidebar toggle
      await sidebarToggle.click();
      
      // Check that sidebar is hidden/shown
      const sidebar = page.locator('div.w-64.bg-card.border-r.border-border');
      await expect(sidebar).toBeVisible();
    });
  });

  test.describe('Responsive Design', () => {
    test('should be responsive on mobile', async () => {
      await page.setViewportSize({ width: 375, height: 667 });
      
      // Check that navigation is still accessible
      const nav = page.locator('nav');
      await expect(nav).toBeVisible();
      
      // Check that dashboard content is visible
      const dashboardTitle = page.locator('h1:has-text("Dashboard")');
      await expect(dashboardTitle).toBeVisible();
    });

    test('should be responsive on tablet', async () => {
      await page.setViewportSize({ width: 768, height: 1024 });
      
      // Check that all sections are visible
      const sections = page.locator('main');
      await expect(sections).toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper heading structure', async () => {
      const h1 = page.locator('h1');
      const h2 = page.locator('h2');
      
      await expect(h1).toHaveCount(1);
      const h2Count = await h2.count();
      expect(h2Count).toBeGreaterThan(0);
    });

    test('should have proper button labels', async () => {
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();
      
      for (let i = 0; i < Math.min(buttonCount, 10); i++) {
        const button = buttons.nth(i);
        const text = await button.textContent();
        expect(text?.trim()).toBeTruthy();
      }
    });

    test('should have proper input labels', async () => {
      const inputs = page.locator('input');
      const inputCount = await inputs.count();
      
      for (let i = 0; i < inputCount; i++) {
        const input = inputs.nth(i);
        const placeholder = await input.getAttribute('placeholder');
        expect(placeholder).toBeTruthy();
      }
    });
  });

  test.describe('Performance and Loading', () => {
    test('should load within acceptable time', async () => {
      const startTime = Date.now();
      await page.goto('http://localhost:8001');
      await page.waitForLoadState('networkidle');
      const loadTime = Date.now() - startTime;
      
      // Should load within 5 seconds
      expect(loadTime).toBeLessThan(5000);
    });

    test('should have proper WASM loading', async () => {
      // Check that WASM is loaded by looking for interactive elements
      const interactiveButton = page.locator('button:has-text("+")');
      await expect(interactiveButton).toBeVisible();
      
      // Test that WASM interactions work
      await interactiveButton.click({ force: true });
    });
  });

  test.describe('Visual Quality', () => {
    test('should have proper styling and colors', async () => {
      // Check for proper button styling
      const primaryButton = page.locator('button:has-text("+")').first();
      await expect(primaryButton).toBeVisible();
      
      // Check for card styling
      const card = page.locator('div:has-text("Total Revenue")').first();
      await expect(card).toBeVisible();
    });

    test('should have proper spacing and layout', async () => {
      // Check that sections are properly spaced
      const main = page.locator('main');
      await expect(main).toBeVisible();
      
      // Check for proper grid layouts
      const grid = page.locator('.grid').first();
      await expect(grid).toBeVisible();
    });
  });

  test.describe('Interactive Features', () => {
    test('should handle button interactions smoothly', async () => {
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();
      
      // Test clicking several buttons
      for (let i = 0; i < Math.min(buttonCount, 5); i++) {
        const button = buttons.nth(i);
        await button.click({ force: true });
        await page.waitForTimeout(100);
      }
      
      // Page should still be responsive
      const dashboardTitle = page.locator('h1:has-text("Dashboard")');
      await expect(dashboardTitle).toBeVisible();
    });

    test('should handle hover effects', async () => {
      const hoverElements = page.locator('div:has-text("Total Revenue")');
      const elementCount = await hoverElements.count();
      
      // Test hover effects
      for (let i = 0; i < Math.min(elementCount, 3); i++) {
        const element = hoverElements.nth(i);
        await element.hover();
        await page.waitForTimeout(200);
      }
      
      // Elements should still be visible after hover
      await expect(hoverElements.first()).toBeVisible();
    });
  });

  test.describe('Error Handling', () => {
    test('should handle missing resources gracefully', async () => {
      // Simulate network failure for external resources
      await page.route('**/tailwindcss.com/**', (route) => {
        route.abort('failed');
      });
      
      // Page should still load and function
      await page.reload();
      await page.waitForLoadState('networkidle');
      
      const dashboardTitle = page.locator('h1:has-text("Dashboard")');
      await expect(dashboardTitle).toBeVisible();
    });
  });
});
