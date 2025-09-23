import { test, expect } from '@playwright/test';

test.describe('Leptos ShadCN UI Comprehensive Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the WASM to load
    await page.waitForLoadState('networkidle');
  });

  test('should load the demo page successfully', async ({ page }) => {
    await expect(page).toHaveTitle(/Leptos ShadCN UI Comprehensive Demo/);
    await expect(page.locator('h1')).toContainText('Leptos ShadCN UI Comprehensive Demo');
  });

  test('should display all refactored components', async ({ page }) => {
    // Check for Drawer component
    await expect(page.locator('text=🎯 Drawer Component')).toBeVisible();
    await expect(page.locator('text=Refactored from 15k to 12k bytes')).toBeVisible();

    // Check for Context Menu component
    await expect(page.locator('text=📋 Context Menu Component')).toBeVisible();
    await expect(page.locator('text=Refactored from 13k to 14.8k bytes')).toBeVisible();

    // Check for Alert Dialog component
    await expect(page.locator('text=⚠️ Alert Dialog Component')).toBeVisible();
    await expect(page.locator('text=Refactored from 12k to 9.5k bytes')).toBeVisible();

    // Check for achievements section
    await expect(page.locator('text=🚀 Refactoring Achievements')).toBeVisible();
    await expect(page.locator('text=✅ 5 Major Components Refactored')).toBeVisible();
  });

  test('should have working theme toggle', async ({ page }) => {
    const themeButton = page.locator('button:has-text("🌙 Dark Mode"), button:has-text("🌞 Light Mode")');
    await expect(themeButton).toBeVisible();
    
    // Click theme toggle
    await themeButton.click();
    
    // Verify theme changed (button text should change)
    await expect(themeButton).toContainText('🌞 Light Mode');
  });

  test('should have working drawer component', async ({ page }) => {
    const drawerButton = page.locator('button:has-text("Open Drawer")').first();
    await expect(drawerButton).toBeVisible();
    
    // Click to open drawer
    await drawerButton.click();
    
    // Check if drawer content is visible (use more specific selector)
    await expect(page.locator('h2:has-text("Drawer Component")')).toBeVisible();
    await expect(page.locator('text=This drawer was refactored from 15k to 12k bytes')).toBeVisible();
    
    // Close drawer
    const closeButton = page.locator('button:has-text("Close")');
    await closeButton.click();
  });

  test('should have working alert dialog', async ({ page }) => {
    const alertButton = page.locator('button:has-text("Show Alert Dialog")').first();
    await expect(alertButton).toBeVisible();
    
    // Click to open alert dialog
    await alertButton.click();
    
    // Check if alert dialog content is visible (use more specific selector)
    await expect(page.locator('h2:has-text("Alert Dialog Component")')).toBeVisible();
    await expect(page.locator('text=This alert dialog was refactored from 12k to 9.5k bytes')).toBeVisible();
    
    // Close alert dialog
    const continueButton = page.locator('button:has-text("Continue")');
    await continueButton.click();
  });

  test('should have working context menu', async ({ page }) => {
    // Use a more specific selector that finds the context menu trigger area
    const contextMenuTrigger = page.locator('[class*="border-dashed"]').first();
    await expect(contextMenuTrigger).toBeVisible();
    
    // Right-click to open context menu
    await contextMenuTrigger.click({ button: 'right' });
    
    // Check if context menu items are visible - update to match new item names
    await expect(page.locator('text=Edit Item')).toBeVisible();
    await expect(page.locator('text=Copy to Clipboard')).toBeVisible();
    await expect(page.locator('text=Delete')).toBeVisible();
  });

  test('should have working counter', async ({ page }) => {
    const counterDisplay = page.locator('text=0').first();
    await expect(counterDisplay).toBeVisible();
    
    // Test increment
    const incrementButton = page.locator('button:has-text("+")');
    await incrementButton.click();
    await expect(counterDisplay).toContainText('1');
    
    // Test decrement
    const decrementButton = page.locator('button:has-text("-")');
    await decrementButton.click();
    await expect(counterDisplay).toContainText('0');
    
    // Test reset
    const resetButton = page.locator('button:has-text("Reset")');
    await resetButton.click();
    await expect(counterDisplay).toContainText('0');
  });

  test('should have working input component', async ({ page }) => {
    const input = page.locator('input[placeholder="Type something..."]');
    await expect(input).toBeVisible();
    
    // Type in input
    await input.fill('Hello, World!');
    await expect(input).toHaveValue('Hello, World!');
    
    // Check if value is displayed
    await expect(page.locator('text=Current value: Hello, World!')).toBeVisible();
  });

  test('should display technical achievements', async ({ page }) => {
    // Check for all achievement cards
    await expect(page.locator('text=✅ 5 Major Components Refactored')).toBeVisible();
    await expect(page.locator('text=✅ 40 Components Reviewed')).toBeVisible();
    await expect(page.locator('text=✅ Zero Regressions')).toBeVisible();
    
    // Check for component details
    await expect(page.locator('text=Drawer, Context-Menu, Alert-Dialog, Select, Command')).toBeVisible();
    await expect(page.locator('text=87% already well-organized, no refactoring needed')).toBeVisible();
    await expect(page.locator('text=All components compile and work perfectly')).toBeVisible();
  });

  test('should be responsive on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Check if main content is still visible
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('text=🎯 Drawer Component')).toBeVisible();
    
    // Check if buttons are still clickable
    const drawerButton = page.locator('button:has-text("Open Drawer")').first();
    await expect(drawerButton).toBeVisible();
    await drawerButton.click();
    
    // Verify drawer opened on mobile
    await expect(page.locator('text=Drawer Component')).toBeVisible();
  });

  test('should load WASM components successfully', async ({ page }) => {
    // Wait for WASM to load by checking for interactive elements
    await expect(page.locator('button:has-text("🌙 Dark Mode"), button:has-text("🌞 Light Mode")')).toBeVisible();
    
    // Test that WASM components are functional
    const counterDisplay = page.locator('text=0').first();
    await expect(counterDisplay).toBeVisible();
    
    // Test WASM reactivity
    const incrementButton = page.locator('button:has-text("+")');
    await incrementButton.click();
    await expect(counterDisplay).toContainText('1');
  });
});