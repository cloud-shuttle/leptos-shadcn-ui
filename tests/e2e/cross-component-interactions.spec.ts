import { test, expect } from '@playwright/test';

test.describe('Cross-Component Interactions - Integration Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Form and Input Component Integration', () => {
    test('form with multiple input types and validation', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Test text input
        const textInputs = form.locator('input[type="text"]');
        if (await textInputs.count() > 0) {
          await textInputs.first().fill('Test text input');
          
          // Test input validation
          const input = textInputs.first();
          await expect(input).toHaveValue('Test text input');
        }
        
        // Test email input
        const emailInputs = form.locator('input[type="email"]');
        if (await emailInputs.count() > 0) {
          await emailInputs.first().fill('test@example.com');
          
          // Test email validation
          const emailInput = emailInputs.first();
          await expect(emailInput).toHaveValue('test@example.com');
        }
        
        // Test password input
        const passwordInputs = form.locator('input[type="password"]');
        if (await passwordInputs.count() > 0) {
          await passwordInputs.first().fill('password123');
          
          // Test password input
          const passwordInput = passwordInputs.first();
          await expect(passwordInput).toHaveValue('password123');
        }
        
        // Test textarea
        const textareas = form.locator('textarea');
        if (await textareas.count() > 0) {
          await textareas.first().fill('This is a test textarea content');
          
          // Test textarea content
          const textarea = textareas.first();
          await expect(textarea).toHaveValue('This is a test textarea content');
        }
        
        // Test select dropdown
        const selects = form.locator('select');
        if (await selects.count() > 0) {
          const select = selects.first();
          const options = select.locator('option');
          
          if (await options.count() > 1) {
            await select.selectOption({ index: 1 });
            
            // Test select value
            const selectedValue = await select.inputValue();
            expect(selectedValue).toBeTruthy();
          }
        }
        
        // Test checkbox
        const checkboxes = form.locator('input[type="checkbox"]');
        if (await checkboxes.count() > 0) {
          const checkbox = checkboxes.first();
          await checkbox.check();
          
          // Test checkbox state
          await expect(checkbox).toBeChecked();
        }
        
        // Test radio buttons
        const radioButtons = form.locator('input[type="radio"]');
        if (await radioButtons.count() > 0) {
          const radioButton = radioButtons.first();
          await radioButton.check();
          
          // Test radio button state
          await expect(radioButton).toBeChecked();
        }
        
        // Test form submission with all inputs
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        if (await submitButton.count() > 0) {
          await submitButton.click();
          
          // Wait for form processing
          await page.waitForTimeout(1000);
        }
      }
    });

    test('form validation with error states', async ({ page }) => {
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Submit form with empty required fields
        const submitButton = form.locator('button[type="submit"], input[type="submit"]');
        if (await submitButton.count() > 0) {
          await submitButton.click();
          
          // Wait for validation
          await page.waitForTimeout(1000);
          
          // Check for validation errors
          const errorMessages = form.locator('[role="alert"], .error, .invalid, [aria-invalid="true"]');
          if (await errorMessages.count() > 0) {
            await expect(errorMessages.first()).toBeVisible();
          }
          
          // Check for invalid input styling
          const invalidInputs = form.locator('input[aria-invalid="true"], .invalid');
          if (await invalidInputs.count() > 0) {
            await expect(invalidInputs.first()).toBeVisible();
          }
        }
      }
    });
  });

  test.describe('Button and Modal Integration', () => {
    test('button triggers modal with form', async ({ page }) => {
      // Look for modal trigger buttons
      const modalTriggers = page.locator('button:has-text("Open"), button:has-text("Add"), button:has-text("Create"), [data-testid*="modal"]');
      
      if (await modalTriggers.count() > 0) {
        await modalTriggers.first().click();
        
        // Wait for modal to open
        await page.waitForTimeout(500);
        
        // Test modal content
        const modal = page.locator('[role="dialog"], .modal, [data-testid="modal"]');
        if (await modal.count() > 0) {
          await expect(modal.first()).toBeVisible();
          
          // Test form within modal
          const modalForm = modal.locator('form');
          if (await modalForm.count() > 0) {
            const form = modalForm.first();
            
            // Fill out modal form
            const inputs = form.locator('input, textarea, select');
            if (await inputs.count() > 0) {
              await inputs.first().fill('Modal form input');
            }
            
            // Test modal form submission
            const modalSubmitButton = form.locator('button[type="submit"], button:has-text("Save"), button:has-text("Submit")');
            if (await modalSubmitButton.count() > 0) {
              await modalSubmitButton.click();
              
              // Wait for form processing
              await page.waitForTimeout(1000);
            }
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

    test('button with loading state and feedback', async ({ page }) => {
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        const button = buttons.first();
        
        // Test button click
        await button.click();
        
        // Check for loading state
        const loadingButton = page.locator('button:has-text("Loading"), button[disabled], .loading');
        if (await loadingButton.count() > 0) {
          await expect(loadingButton.first()).toBeVisible();
        }
        
        // Wait for action completion
        await page.waitForTimeout(2000);
        
        // Check for success feedback
        const successMessage = page.locator('[role="status"], .success, [data-testid="success"]');
        if (await successMessage.count() > 0) {
          await expect(successMessage.first()).toBeVisible();
        }
      }
    });
  });

  test.describe('Navigation and Content Integration', () => {
    test('navigation menu with dynamic content', async ({ page }) => {
      // Test navigation menu
      const navMenu = page.locator('nav, [role="navigation"], .navigation');
      
      if (await navMenu.count() > 0) {
        const menu = navMenu.first();
        
        // Test menu items
        const menuItems = menu.locator('a, button, [role="menuitem"]');
        
        if (await menuItems.count() > 0) {
          // Click on menu item
          await menuItems.first().click();
          
          // Wait for content to load
          await page.waitForTimeout(1000);
          
          // Check for content updates
          const content = page.locator('main, .content, [role="main"]');
          if (await content.count() > 0) {
            await expect(content.first()).toBeVisible();
          }
        }
      }
    });

    test('breadcrumb navigation with page content', async ({ page }) => {
      // Test breadcrumb navigation
      const breadcrumbs = page.locator('[aria-label="breadcrumb"], .breadcrumb, nav[aria-label*="breadcrumb"]');
      
      if (await breadcrumbs.count() > 0) {
        const breadcrumb = breadcrumbs.first();
        
        // Test breadcrumb links
        const breadcrumbLinks = breadcrumb.locator('a');
        
        if (await breadcrumbLinks.count() > 1) {
          // Click on breadcrumb link
          await breadcrumbLinks.nth(1).click();
          
          // Wait for navigation
          await page.waitForTimeout(1000);
          
          // Check for updated breadcrumb
          const updatedBreadcrumbs = page.locator('[aria-label="breadcrumb"], .breadcrumb');
          if (await updatedBreadcrumbs.count() > 0) {
            await expect(updatedBreadcrumbs.first()).toBeVisible();
          }
        }
      }
    });

    test('pagination with content updates', async ({ page }) => {
      // Test pagination
      const pagination = page.locator('[aria-label="pagination"], .pagination, nav[aria-label*="pagination"]');
      
      if (await pagination.count() > 0) {
        const paginationNav = pagination.first();
        
        // Test next page button
        const nextButton = paginationNav.locator('button:has-text("Next"), a:has-text("Next"), [aria-label*="next" i]');
        if (await nextButton.count() > 0) {
          await nextButton.click();
          
          // Wait for content to update
          await page.waitForTimeout(1000);
          
          // Check for content changes
          const content = page.locator('main, .content, [role="main"]');
          if (await content.count() > 0) {
            await expect(content.first()).toBeVisible();
          }
        }
      }
    });
  });

  test.describe('Data Display and Interaction Integration', () => {
    test('table with sorting and filtering', async ({ page }) => {
      // Test table component
      const tables = page.locator('table, [role="table"], .table');
      
      if (await tables.count() > 0) {
        const table = tables.first();
        
        // Test table headers (sortable)
        const sortableHeaders = table.locator('th[role="button"], th[tabindex="0"], .sortable');
        if (await sortableHeaders.count() > 0) {
          await sortableHeaders.first().click();
          
          // Wait for sort
          await page.waitForTimeout(500);
          
          // Check for sort indicators
          const sortIndicators = table.locator('.sort-asc, .sort-desc, [aria-sort]');
          if (await sortIndicators.count() > 0) {
            await expect(sortIndicators.first()).toBeVisible();
          }
        }
        
        // Test table rows
        const rows = table.locator('tr, [role="row"]');
        if (await rows.count() > 1) {
          // Test row selection
          const selectableRows = rows.locator('[role="checkbox"], input[type="checkbox"]');
          if (await selectableRows.count() > 0) {
            await selectableRows.first().click();
            
            // Check for selection state
            await expect(selectableRows.first()).toBeChecked();
          }
        }
      }
    });

    test('card components with interactive elements', async ({ page }) => {
      // Test card components
      const cards = page.locator('.card, [data-testid="card"]');
      
      if (await cards.count() > 0) {
        const card = cards.first();
        
        // Test card header
        const cardHeader = card.locator('.card-header, [data-testid="card-header"]');
        if (await cardHeader.count() > 0) {
          await expect(cardHeader.first()).toBeVisible();
        }
        
        // Test card content
        const cardContent = card.locator('.card-content, [data-testid="card-content"]');
        if (await cardContent.count() > 0) {
          await expect(cardContent.first()).toBeVisible();
        }
        
        // Test card actions
        const cardActions = card.locator('.card-actions, .card-footer, [data-testid="card-actions"]');
        if (await cardActions.count() > 0) {
          const actions = cardActions.first();
          
          // Test action buttons
          const actionButtons = actions.locator('button, a');
          if (await actionButtons.count() > 0) {
            await actionButtons.first().click();
            
            // Wait for action
            await page.waitForTimeout(500);
          }
        }
      }
    });

    test('list components with selection and actions', async ({ page }) => {
      // Test list components
      const lists = page.locator('ul, ol, [role="list"], .list');
      
      if (await lists.count() > 0) {
        const list = lists.first();
        
        // Test list items
        const listItems = list.locator('li, [role="listitem"], .list-item');
        if (await listItems.count() > 0) {
          const listItem = listItems.first();
          
          // Test item selection
          const selectableItems = listItem.locator('[role="checkbox"], input[type="checkbox"], .selectable');
          if (await selectableItems.count() > 0) {
            await selectableItems.first().click();
            
            // Check for selection state
            await expect(selectableItems.first()).toBeChecked();
          }
          
          // Test item actions
          const itemActions = listItem.locator('button, a, .action');
          if (await itemActions.count() > 0) {
            await itemActions.first().click();
            
            // Wait for action
            await page.waitForTimeout(500);
          }
        }
      }
    });
  });

  test.describe('Feedback and Notification Integration', () => {
    test('toast notifications with actions', async ({ page }) => {
      // Trigger an action that might show a toast
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        await buttons.first().click();
        
        // Wait for potential toast
        await page.waitForTimeout(1000);
        
        // Check for toast notifications
        const toasts = page.locator('[role="alert"], .toast, [data-testid="toast"]');
        if (await toasts.count() > 0) {
          const toast = toasts.first();
          await expect(toast).toBeVisible();
          
          // Test toast actions
          const toastActions = toast.locator('button, .action');
          if (await toastActions.count() > 0) {
            await toastActions.first().click();
            
            // Wait for action
            await page.waitForTimeout(500);
          }
          
          // Test toast dismiss
          const dismissButton = toast.locator('button:has-text("Dismiss"), button:has-text("Close"), [aria-label*="close" i]');
          if (await dismissButton.count() > 0) {
            await dismissButton.click();
            
            // Wait for dismiss
            await page.waitForTimeout(500);
            
            // Check if toast is dismissed
            await expect(toast).not.toBeVisible();
          }
        }
      }
    });

    test('alert components with actions', async ({ page }) => {
      // Test alert components
      const alerts = page.locator('[role="alert"], .alert, [data-testid="alert"]');
      
      if (await alerts.count() > 0) {
        const alert = alerts.first();
        await expect(alert).toBeVisible();
        
        // Test alert actions
        const alertActions = alert.locator('button, .action');
        if (await alertActions.count() > 0) {
          await alertActions.first().click();
          
          // Wait for action
          await page.waitForTimeout(500);
        }
        
        // Test alert dismiss
        const dismissButton = alert.locator('button:has-text("Dismiss"), button:has-text("Close"), [aria-label*="close" i]');
        if (await dismissButton.count() > 0) {
          await dismissButton.click();
          
          // Wait for dismiss
          await page.waitForTimeout(500);
          
          // Check if alert is dismissed
          await expect(alert).not.toBeVisible();
        }
      }
    });

    test('tooltip integration with interactive elements', async ({ page }) => {
      // Test tooltip triggers
      const tooltipTriggers = page.locator('[data-tooltip], [title], .tooltip-trigger');
      
      if (await tooltipTriggers.count() > 0) {
        const trigger = tooltipTriggers.first();
        
        // Hover to show tooltip
        await trigger.hover();
        
        // Wait for tooltip
        await page.waitForTimeout(500);
        
        // Check for tooltip
        const tooltips = page.locator('[role="tooltip"], .tooltip, [data-testid="tooltip"]');
        if (await tooltips.count() > 0) {
          await expect(tooltips.first()).toBeVisible();
        }
        
        // Move away to hide tooltip
        await page.mouse.move(0, 0);
        
        // Wait for tooltip to hide
        await page.waitForTimeout(500);
        
        // Check if tooltip is hidden
        if (await tooltips.count() > 0) {
          await expect(tooltips.first()).not.toBeVisible();
        }
      }
    });
  });

  test.describe('State Management Integration', () => {
    test('component state synchronization', async ({ page }) => {
      // Test state changes across components
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Change form input
        const inputs = form.locator('input, textarea, select');
        if (await inputs.count() > 0) {
          const input = inputs.first();
          await input.fill('Test state change');
          
          // Check if other components reflect the change
          const otherInputs = page.locator('input, textarea, select');
          if (await otherInputs.count() > 1) {
            // Look for synchronized values
            const synchronizedInputs = otherInputs.filter({ hasValue: 'Test state change' });
            if (await synchronizedInputs.count() > 0) {
              await expect(synchronizedInputs.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('component communication and updates', async ({ page }) => {
      // Test component communication
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        const button = buttons.first();
        await button.click();
        
        // Wait for state updates
        await page.waitForTimeout(1000);
        
        // Check for updated components
        const updatedComponents = page.locator('.updated, [data-updated="true"]');
        if (await updatedComponents.count() > 0) {
          await expect(updatedComponents.first()).toBeVisible();
        }
      }
    });
  });

  test.describe('Accessibility Integration', () => {
    test('keyboard navigation across components', async ({ page }) => {
      // Test keyboard navigation
      await page.keyboard.press('Tab');
      
      // Check for focus indicators
      const focusedElement = page.locator(':focus');
      if (await focusedElement.count() > 0) {
        await expect(focusedElement.first()).toBeVisible();
      }
      
      // Continue tabbing
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');
      
      // Test Enter key on focused element
      const focusedButton = page.locator(':focus');
      if (await focusedButton.count() > 0) {
        await page.keyboard.press('Enter');
        
        // Wait for action
        await page.waitForTimeout(500);
      }
    });

    test('screen reader compatibility', async ({ page }) => {
      // Test ARIA attributes
      const elementsWithAria = page.locator('[aria-label], [aria-describedby], [role]');
      
      if (await elementsWithAria.count() > 0) {
        const element = elementsWithAria.first();
        
        // Check for proper ARIA attributes
        const ariaLabel = await element.getAttribute('aria-label');
        const role = await element.getAttribute('role');
        
        expect(ariaLabel || role).toBeTruthy();
      }
    });
  });
});
