import { test, expect } from '@playwright/test';

test.describe('Server Startup Tests', () => {
  test('should be able to start trunk serve without errors', async ({ page }) => {
    // This test will verify that the server can start
    // We'll use a simple HTTP request to check if the server is accessible
    try {
      const response = await page.goto('/', { 
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
    await page.goto('/');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for the main dashboard heading (now h2)
    const heading = page.locator('h2').filter({ hasText: 'Dashboard' }).first();
    await expect(heading).toBeVisible();
    await expect(heading).toContainText('Dashboard');
    
    // Check for the company name in sidebar (only visible on desktop)
    const companyName = page.locator('h1').filter({ hasText: 'Acme Inc.' }).first();
    // On mobile, sidebar is hidden, so we just check it exists
    await expect(companyName).toHaveCount(1);
    
    // Check for key metrics cards
    const revenueCard = page.locator('text=Total Revenue').first();
    await expect(revenueCard).toBeVisible();
    
    const customersCard = page.locator('text=New Customers').first();
    await expect(customersCard).toBeVisible();
    
    // Check for the documents table
    const documentsTable = page.locator('text=Documents').first();
    await expect(documentsTable).toBeVisible();
    
    console.log('✅ Sophisticated dashboard content is served correctly');
  });

  test('should have working dashboard features', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Check for sidebar navigation
    const sidebarNav = page.locator('nav').filter({ hasText: /Dashboard|Lifecycle|Analytics|Projects|Team/ }).first();
    await expect(sidebarNav).toBeVisible();
    
    // Check for action buttons
    const viewButton = page.locator('button').filter({ hasText: 'View' });
    const quickCreateButton = page.locator('button').filter({ hasText: 'Quick Create' });
    await expect(viewButton).toBeVisible();
    await expect(quickCreateButton).toBeVisible();
    
    // Check for dashboard metrics cards
    const metrics = page.locator('text=Total Revenue').or(page.locator('text=New Customers')).or(page.locator('text=Active Accounts')).or(page.locator('text=Growth Rate'));
    await expect(metrics).toHaveCount(4);
    
    // Check for data table
    const table = page.locator('table');
    await expect(table).toBeVisible();
    
    // Check for table headers
    const tableHeaders = page.locator('th').filter({ hasText: /Header|Section Type|Status|Target|Limit|Reviewer/ });
    await expect(tableHeaders).toHaveCount(7); // Updated count for new table structure
    
    // Check for search functionality
    const searchInput = page.locator('input[placeholder*="Search"]');
    await expect(searchInput).toBeVisible();
    
    // Test search input
    await searchInput.fill('Cover page');
    await expect(searchInput).toHaveValue('Cover page');
    
    // Check for filter buttons
    const filterButtons = page.locator('button').filter({ hasText: /Past Performance|Key Personnel|Focus Documents/ });
    await expect(filterButtons).toHaveCount(3);
    
    // Check for pagination
    const pagination = page.locator('text=Showing').or(page.locator('text=Previous')).or(page.locator('text=Next'));
    await expect(pagination).toBeVisible();
    
    // Test button interactions
    await quickCreateButton.click();
    await viewButton.click();
    
    console.log('✅ Sophisticated dashboard features are present and functional');
  });
});

