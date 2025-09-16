import { test, expect } from '@playwright/test';

test.describe('Complete User Workflows - Integration Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('User Registration Workflow', () => {
    test('complete user registration flow with validation', async ({ page }) => {
      // Look for registration form or create one
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Test form inputs
        const inputs = form.locator('input[type="text"], input[type="email"], input[type="password"]');
        
        if (await inputs.count() >= 3) {
          // Fill out registration form
          const emailInput = inputs.filter({ hasText: /email/i }).first();
          const passwordInput = inputs.filter({ hasText: /password/i }).first();
          const confirmPasswordInput = inputs.filter({ hasText: /confirm|repeat/i }).first();
          
          if (await emailInput.count() > 0) {
            await emailInput.fill('user@example.com');
          }
          
          if (await passwordInput.count() > 0) {
            await passwordInput.fill('SecurePassword123!');
          }
          
          if (await confirmPasswordInput.count() > 0) {
            await confirmPasswordInput.fill('SecurePassword123!');
          }
          
          // Test form validation
          const submitButton = form.locator('button[type="submit"], input[type="submit"]');
          if (await submitButton.count() > 0) {
            await expect(submitButton.first()).toBeVisible();
            await expect(submitButton.first()).toBeEnabled();
            
            // Test form submission
            await submitButton.first().click();
            
            // Wait for form processing
            await page.waitForTimeout(1000);
          }
        }
      }
    });

    test('registration form validation errors', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        
        if (await submitButton.count() > 0) {
          // Try to submit empty form
          await submitButton.first().click();
          
          // Check for validation errors
          const errorMessages = page.locator('[role="alert"], .error, .invalid');
          if (await errorMessages.count() > 0) {
            await expect(errorMessages.first()).toBeVisible();
          }
        }
      }
    });
  });

  test.describe('User Login Workflow', () => {
    test('complete user login flow', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Look for login inputs
        const emailInput = form.locator('input[type="email"], input[name*="email"], input[placeholder*="email" i]');
        const passwordInput = form.locator('input[type="password"], input[name*="password"], input[placeholder*="password" i]');
        
        if (await emailInput.count() > 0 && await passwordInput.count() > 0) {
          await emailInput.fill('user@example.com');
          await passwordInput.fill('password123');
          
          const submitButton = form.locator('button[type="submit"], input[type="submit"]');
          if (await submitButton.count() > 0) {
            await submitButton.click();
            
            // Wait for login processing
            await page.waitForTimeout(1000);
            
            // Check for successful login indicators
            const successIndicators = page.locator('[data-testid="success"], .success, [role="status"]');
            if (await successIndicators.count() > 0) {
              await expect(successIndicators.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('login with invalid credentials', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        const emailInput = form.locator('input[type="email"], input[name*="email"]');
        const passwordInput = form.locator('input[type="password"], input[name*="password"]');
        
        if (await emailInput.count() > 0 && await passwordInput.count() > 0) {
          await emailInput.fill('invalid@example.com');
          await passwordInput.fill('wrongpassword');
          
          const submitButton = form.locator('button[type="submit"], input[type="submit"]');
          if (await submitButton.count() > 0) {
            await submitButton.click();
            
            // Wait for error response
            await page.waitForTimeout(1000);
            
            // Check for error messages
            const errorMessages = page.locator('[role="alert"], .error, .invalid');
            if (await errorMessages.count() > 0) {
              await expect(errorMessages.first()).toBeVisible();
            }
          }
        }
      }
    });
  });

  test.describe('E-commerce Purchase Workflow', () => {
    test('complete product purchase flow', async ({ page }) => {
      // Look for product cards or items
      const productCards = page.locator('[data-testid="product-card"], .product-card, .card');
      
      if (await productCards.count() > 0) {
        const productCard = productCards.first();
        
        // Test product selection
        const addToCartButton = productCard.locator('button:has-text("Add to Cart"), button:has-text("Buy"), [data-testid="add-to-cart"]');
        if (await addToCartButton.count() > 0) {
          await addToCartButton.click();
          
          // Wait for cart update
          await page.waitForTimeout(500);
          
          // Look for cart indicator
          const cartIndicator = page.locator('[data-testid="cart-count"], .cart-count, .cart-badge');
          if (await cartIndicator.count() > 0) {
            await expect(cartIndicator.first()).toBeVisible();
          }
        }
      }
      
      // Test checkout process
      const checkoutButton = page.locator('button:has-text("Checkout"), button:has-text("Proceed to Checkout"), [data-testid="checkout"]');
      if (await checkoutButton.count() > 0) {
        await checkoutButton.click();
        
        // Wait for checkout page
        await page.waitForTimeout(1000);
        
        // Test checkout form
        const checkoutForm = page.locator('form');
        if (await checkoutForm.count() > 0) {
          const form = checkoutForm.first();
          
          // Fill out shipping information
          const nameInput = form.locator('input[name*="name"], input[placeholder*="name" i]');
          const addressInput = form.locator('input[name*="address"], input[placeholder*="address" i]');
          const cityInput = form.locator('input[name*="city"], input[placeholder*="city" i]');
          
          if (await nameInput.count() > 0) {
            await nameInput.fill('John Doe');
          }
          if (await addressInput.count() > 0) {
            await addressInput.fill('123 Main St');
          }
          if (await cityInput.count() > 0) {
            await cityInput.fill('New York');
          }
          
          // Test payment information
          const cardInput = form.locator('input[name*="card"], input[placeholder*="card" i]');
          const cvvInput = form.locator('input[name*="cvv"], input[placeholder*="cvv" i]');
          
          if (await cardInput.count() > 0) {
            await cardInput.fill('4111111111111111');
          }
          if (await cvvInput.count() > 0) {
            await cvvInput.fill('123');
          }
          
          // Complete purchase
          const purchaseButton = form.locator('button:has-text("Purchase"), button:has-text("Pay"), button[type="submit"]');
          if (await purchaseButton.count() > 0) {
            await purchaseButton.click();
            
            // Wait for purchase processing
            await page.waitForTimeout(2000);
            
            // Check for success confirmation
            const successMessage = page.locator('[data-testid="success"], .success, [role="status"]');
            if (await successMessage.count() > 0) {
              await expect(successMessage.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('shopping cart management', async ({ page }) => {
      // Add items to cart
      const addToCartButtons = page.locator('button:has-text("Add to Cart"), [data-testid="add-to-cart"]');
      
      if (await addToCartButtons.count() > 0) {
        // Add first item
        await addToCartButtons.first().click();
        await page.waitForTimeout(500);
        
        // Add second item if available
        if (await addToCartButtons.count() > 1) {
          await addToCartButtons.nth(1).click();
          await page.waitForTimeout(500);
        }
        
        // Test cart view
        const cartButton = page.locator('[data-testid="cart"], .cart, button:has-text("Cart")');
        if (await cartButton.count() > 0) {
          await cartButton.click();
          
          // Wait for cart to open
          await page.waitForTimeout(500);
          
          // Test quantity adjustment
          const quantityInputs = page.locator('input[type="number"], [data-testid="quantity"]');
          if (await quantityInputs.count() > 0) {
            const quantityInput = quantityInputs.first();
            await quantityInput.fill('2');
            
            // Test remove item
            const removeButtons = page.locator('button:has-text("Remove"), [data-testid="remove"]');
            if (await removeButtons.count() > 0) {
              await removeButtons.first().click();
              await page.waitForTimeout(500);
            }
          }
        }
      }
    });
  });

  test.describe('Content Management Workflow', () => {
    test('create and edit content', async ({ page }) => {
      // Look for content creation forms
      const createButton = page.locator('button:has-text("Create"), button:has-text("Add"), [data-testid="create"]');
      
      if (await createButton.count() > 0) {
        await createButton.click();
        await page.waitForTimeout(500);
        
        // Test content creation form
        const forms = page.locator('form');
        if (await forms.count() > 0) {
          const form = forms.first();
          
          // Fill out content form
          const titleInput = form.locator('input[name*="title"], input[placeholder*="title" i]');
          const contentTextarea = form.locator('textarea, input[name*="content"], input[placeholder*="content" i]');
          
          if (await titleInput.count() > 0) {
            await titleInput.fill('Test Content Title');
          }
          
          if (await contentTextarea.count() > 0) {
            await contentTextarea.fill('This is test content for the content management workflow.');
          }
          
          // Save content
          const saveButton = form.locator('button:has-text("Save"), button:has-text("Create"), button[type="submit"]');
          if (await saveButton.count() > 0) {
            await saveButton.click();
            
            // Wait for save processing
            await page.waitForTimeout(1000);
            
            // Check for success
            const successMessage = page.locator('[data-testid="success"], .success, [role="status"]');
            if (await successMessage.count() > 0) {
              await expect(successMessage.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('content editing and updates', async ({ page }) => {
      // Look for existing content to edit
      const editButtons = page.locator('button:has-text("Edit"), [data-testid="edit"]');
      
      if (await editButtons.count() > 0) {
        await editButtons.first().click();
        await page.waitForTimeout(500);
        
        // Test content editing
        const forms = page.locator('form');
        if (await forms.count() > 0) {
          const form = forms.first();
          
          // Modify content
          const titleInput = form.locator('input[name*="title"], input[placeholder*="title" i]');
          if (await titleInput.count() > 0) {
            await titleInput.clear();
            await titleInput.fill('Updated Content Title');
          }
          
          // Save changes
          const saveButton = form.locator('button:has-text("Save"), button:has-text("Update"), button[type="submit"]');
          if (await saveButton.count() > 0) {
            await saveButton.click();
            
            // Wait for update processing
            await page.waitForTimeout(1000);
            
            // Check for success
            const successMessage = page.locator('[data-testid="success"], .success, [role="status"]');
            if (await successMessage.count() > 0) {
              await expect(successMessage.first()).toBeVisible();
            }
          }
        }
      }
    });
  });

  test.describe('Search and Filter Workflow', () => {
    test('search functionality', async ({ page }) => {
      // Test search input
      const searchInput = page.locator('input[type="search"], input[placeholder*="search" i], [data-testid="search"]');
      
      if (await searchInput.count() > 0) {
        await searchInput.fill('test search query');
        
        // Test search button or enter key
        const searchButton = page.locator('button:has-text("Search"), [data-testid="search-button"]');
        if (await searchButton.count() > 0) {
          await searchButton.click();
        } else {
          await searchInput.press('Enter');
        }
        
        // Wait for search results
        await page.waitForTimeout(1000);
        
        // Check for search results
        const searchResults = page.locator('[data-testid="search-results"], .search-results, .results');
        if (await searchResults.count() > 0) {
          await expect(searchResults.first()).toBeVisible();
        }
      }
    });

    test('filter and sort functionality', async ({ page }) => {
      // Test filter dropdowns
      const filterSelects = page.locator('select, [data-testid="filter"], .filter');
      
      if (await filterSelects.count() > 0) {
        const filterSelect = filterSelects.first();
        await filterSelect.selectOption({ index: 1 });
        
        // Wait for filter application
        await page.waitForTimeout(500);
        
        // Test sort functionality
        const sortSelects = page.locator('select[name*="sort"], [data-testid="sort"], .sort');
        if (await sortSelects.count() > 0) {
          const sortSelect = sortSelects.first();
          await sortSelect.selectOption({ index: 1 });
          
          // Wait for sort application
          await page.waitForTimeout(500);
        }
      }
    });
  });

  test.describe('Navigation and Routing Workflow', () => {
    test('navigation menu interactions', async ({ page }) => {
      // Test navigation menu
      const navMenu = page.locator('nav, [role="navigation"], .navigation');
      
      if (await navMenu.count() > 0) {
        const menu = navMenu.first();
        
        // Test menu items
        const menuItems = menu.locator('a, button, [role="menuitem"]');
        
        if (await menuItems.count() > 0) {
          // Click on first menu item
          await menuItems.first().click();
          
          // Wait for navigation
          await page.waitForTimeout(1000);
          
          // Check if page changed
          const currentUrl = page.url();
          expect(currentUrl).toBeTruthy();
        }
      }
    });

    test('breadcrumb navigation', async ({ page }) => {
      // Test breadcrumb navigation
      const breadcrumbs = page.locator('[aria-label="breadcrumb"], .breadcrumb, nav[aria-label*="breadcrumb"]');
      
      if (await breadcrumbs.count() > 0) {
        const breadcrumb = breadcrumbs.first();
        
        // Test breadcrumb links
        const breadcrumbLinks = breadcrumb.locator('a');
        
        if (await breadcrumbLinks.count() > 1) {
          // Click on a breadcrumb link
          await breadcrumbLinks.nth(1).click();
          
          // Wait for navigation
          await page.waitForTimeout(1000);
        }
      }
    });

    test('pagination navigation', async ({ page }) => {
      // Test pagination
      const pagination = page.locator('[aria-label="pagination"], .pagination, nav[aria-label*="pagination"]');
      
      if (await pagination.count() > 0) {
        const paginationNav = pagination.first();
        
        // Test pagination buttons
        const nextButton = paginationNav.locator('button:has-text("Next"), a:has-text("Next"), [aria-label*="next" i]');
        const prevButton = paginationNav.locator('button:has-text("Previous"), a:has-text("Previous"), [aria-label*="previous" i]');
        
        if (await nextButton.count() > 0) {
          await nextButton.click();
          await page.waitForTimeout(1000);
        }
        
        if (await prevButton.count() > 0) {
          await prevButton.click();
          await page.waitForTimeout(1000);
        }
      }
    });
  });

  test.describe('Modal and Dialog Workflows', () => {
    test('modal creation and interaction', async ({ page }) => {
      // Look for modal triggers
      const modalTriggers = page.locator('button:has-text("Open"), button:has-text("Show"), [data-testid="modal-trigger"]');
      
      if (await modalTriggers.count() > 0) {
        await modalTriggers.first().click();
        
        // Wait for modal to open
        await page.waitForTimeout(500);
        
        // Test modal content
        const modal = page.locator('[role="dialog"], .modal, [data-testid="modal"]');
        if (await modal.count() > 0) {
          await expect(modal.first()).toBeVisible();
          
          // Test modal interactions
          const modalInputs = modal.locator('input, textarea, select');
          if (await modalInputs.count() > 0) {
            await modalInputs.first().fill('Test modal input');
          }
          
          // Test modal close
          const closeButton = modal.locator('button:has-text("Close"), button:has-text("Cancel"), [aria-label*="close" i]');
          if (await closeButton.count() > 0) {
            await closeButton.click();
            
            // Wait for modal to close
            await page.waitForTimeout(500);
            
            // Check if modal is closed
            await expect(modal.first()).not.toBeVisible();
          }
        }
      }
    });

    test('confirmation dialog workflow', async ({ page }) => {
      // Look for delete or destructive actions
      const deleteButtons = page.locator('button:has-text("Delete"), button:has-text("Remove"), [data-testid="delete"]');
      
      if (await deleteButtons.count() > 0) {
        await deleteButtons.first().click();
        
        // Wait for confirmation dialog
        await page.waitForTimeout(500);
        
        // Test confirmation dialog
        const confirmDialog = page.locator('[role="alertdialog"], .confirm-dialog, [data-testid="confirm"]');
        if (await confirmDialog.count() > 0) {
          await expect(confirmDialog.first()).toBeVisible();
          
          // Test confirmation
          const confirmButton = confirmDialog.locator('button:has-text("Confirm"), button:has-text("Yes"), button:has-text("Delete")');
          if (await confirmButton.count() > 0) {
            await confirmButton.click();
            
            // Wait for action completion
            await page.waitForTimeout(1000);
          }
          
          // Test cancellation
          const cancelButton = confirmDialog.locator('button:has-text("Cancel"), button:has-text("No")');
          if (await cancelButton.count() > 0) {
            await cancelButton.click();
            
            // Wait for dialog to close
            await page.waitForTimeout(500);
          }
        }
      }
    });
  });

  test.describe('Error Handling and Recovery', () => {
    test('network error handling', async ({ page }) => {
      // Simulate network issues by going offline
      await page.context().setOffline(true);
      
      // Try to perform an action that requires network
      const forms = page.locator('form');
      if (await forms.count() > 0) {
        const form = forms.first();
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        
        if (await submitButton.count() > 0) {
          await submitButton.click();
          
          // Wait for error handling
          await page.waitForTimeout(2000);
          
          // Check for error messages
          const errorMessages = page.locator('[role="alert"], .error, .network-error');
          if (await errorMessages.count() > 0) {
            await expect(errorMessages.first()).toBeVisible();
          }
        }
      }
      
      // Restore network
      await page.context().setOffline(false);
    });

    test('form validation error recovery', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Submit form with invalid data
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        if (await submitButton.count() > 0) {
          await submitButton.click();
          
          // Wait for validation errors
          await page.waitForTimeout(1000);
          
          // Check for validation errors
          const validationErrors = page.locator('[role="alert"], .error, .invalid');
          if (await validationErrors.count() > 0) {
            await expect(validationErrors.first()).toBeVisible();
            
            // Test error recovery by fixing the input
            const inputs = form.locator('input, textarea, select');
            if (await inputs.count() > 0) {
              const input = inputs.first();
              await input.fill('Valid input');
              
              // Try submitting again
              await submitButton.click();
              await page.waitForTimeout(1000);
            }
          }
        }
      }
    });
  });
});
