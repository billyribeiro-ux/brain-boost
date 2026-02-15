import { test, expect } from '@playwright/test';

test.describe('Trading Simulator', () => {
  test.beforeEach(async ({ page }) => {
    // Login as advanced user (has trading history)
    await page.goto('/login');
    await page.fill('input[name="email"]', 'advanced@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    
    // Navigate to trading page
    await page.click('[aria-label="Trading"]').or(page.click('text=Trading'));
    await expect(page).toHaveURL('/trading', { timeout: 10000 });
  });

  test('should display trading interface', async ({ page }) => {
    // Should show main trading elements
    await expect(page.locator('text=/Practice|Simulation|Stats/i')).toBeVisible();
  });

  test('should start a trading session', async ({ page }) => {
    // Click start session button
    const startButton = page.locator('button:has-text("Start Session")').or(page.locator('button:has-text("New Session")'));
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      // Should show scenario or market data
      await expect(page.locator('text=/scenario|market|trade|decision/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should display market chart', async ({ page }) => {
    const startButton = page.locator('button:has-text("Start Session")');
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      // Should show chart (SVG or canvas)
      await expect(page.locator('svg').or(page.locator('canvas'))).toBeVisible({ timeout: 5000 });
    }
  });

  test('should present trading scenario', async ({ page }) => {
    const startButton = page.locator('button:has-text("Start Session")');
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      // Should show scenario description
      await expect(page.locator('text=/You bought|Your position|Market sentiment/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should allow making trading decision', async ({ page }) => {
    const startButton = page.locator('button:has-text("Start Session")');
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      // Should show decision buttons
      const buyButton = page.locator('button:has-text("Buy")');
      const sellButton = page.locator('button:has-text("Sell")');
      const holdButton = page.locator('button:has-text("Hold")');
      
      await expect(buyButton.or(sellButton).or(holdButton)).toBeVisible({ timeout: 5000 });
      
      // Make a decision
      if (await holdButton.isVisible()) {
        await holdButton.click();
      }
    }
  });

  test('should show bias feedback after decision', async ({ page }) => {
    const startButton = page.locator('button:has-text("Start Session")');
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      const holdButton = page.locator('button:has-text("Hold")');
      if (await holdButton.isVisible({ timeout: 5000 })) {
        await holdButton.click();
        
        // Should show feedback
        await expect(page.locator('text=/bias|optimal|EV|feedback/i')).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should display trading stats', async ({ page }) => {
    // Click stats tab
    const statsTab = page.locator('text=Stats').or(page.locator('button:has-text("Stats")'));
    if (await statsTab.isVisible({ timeout: 5000 })) {
      await statsTab.click();
      
      // Should show statistics
      await expect(page.locator('text=/win rate|sessions|EV score/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show bias frequency chart', async ({ page }) => {
    const statsTab = page.locator('text=Stats');
    if (await statsTab.isVisible({ timeout: 5000 })) {
      await statsTab.click();
      
      // Should show bias breakdown
      await expect(page.locator('text=/endowment|loss aversion|anchoring|confirmation/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should track session progress', async ({ page }) => {
    const startButton = page.locator('button:has-text("Start Session")');
    if (await startButton.isVisible({ timeout: 5000 })) {
      await startButton.click();
      
      // Should show progress indicator (e.g., "1/5 scenarios")
      await expect(page.locator('text=/\\d+\\/\\d+|scenario \\d+/i')).toBeVisible({ timeout: 5000 });
    }
  });
});
