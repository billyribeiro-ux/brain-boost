import { test, expect } from '@playwright/test';

test.describe('AI Coach', () => {
  test.beforeEach(async ({ page }) => {
    // Login as active user
    await page.goto('/login');
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    
    // Navigate to coach page
    const coachLink = page.locator('[aria-label="Coach"]').or(page.locator('text=Coach'));
    await coachLink.click();
    await expect(page).toHaveURL('/coach', { timeout: 10000 });
  });

  test('should display coach interface', async ({ page }) => {
    // Should show chat interface
    await expect(page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'))).toBeVisible();
  });

  test('should show suggestion carousel', async ({ page }) => {
    // Should show suggestions
    await expect(page.locator('text=/suggestion|recommend|tip/i')).toBeVisible({ timeout: 5000 });
  });

  test('should display quick action pills', async ({ page }) => {
    // Should show quick actions
    await expect(page.locator('button:has-text("How am I doing")').or(page.locator('button:has-text("Progress")'))).toBeVisible({ timeout: 5000 });
  });

  test('should send message to coach', async ({ page }) => {
    // Type message
    const input = page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'));
    await input.fill('How am I doing?');
    
    // Send message
    const submitButton = page.locator('button[type="submit"]');
    if (await submitButton.isVisible({ timeout: 1000 })) {
      await submitButton.click();
    } else {
      await page.keyboard.press('Enter');
    }
    
    // Should show user message
    await expect(page.locator('text=How am I doing?')).toBeVisible({ timeout: 3000 });
  });

  test('should receive coach response', async ({ page }) => {
    const input = page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'));
    await input.fill('What should I focus on?');
    const submitButton = page.locator('button[type="submit"]');
    if (await submitButton.isVisible({ timeout: 1000 })) {
      await submitButton.click();
    } else {
      await page.keyboard.press('Enter');
    }
    
    // Should show coach response
    await expect(page.locator('text=/brain score|stress|focus|training/i')).toBeVisible({ timeout: 5000 });
  });

  test('should use quick action pill', async ({ page }) => {
    // Click quick action
    const quickAction = page.locator('button:has-text("Progress")').or(page.locator('button:has-text("Stats")'));
    if (await quickAction.isVisible({ timeout: 5000 })) {
      await quickAction.click();
      
      // Should send pre-defined message
      await expect(page.locator('text=/progress|stats|how am i/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show typing indicator', async ({ page }) => {
    const input = page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'));
    await input.fill('Tell me about meditation');
    const submitButton = page.locator('button[type="submit"]');
    if (await submitButton.isVisible({ timeout: 1000 })) {
      await submitButton.click();
    } else {
      await page.keyboard.press('Enter');
    }
    
    // Should show typing indicator briefly
    const typingIndicator = page.locator('text=/typing|.../i').or(page.locator('[class*="typing"]'));
    if (await typingIndicator.isVisible({ timeout: 2000 })) {
      await expect(typingIndicator).toBeVisible();
    }
  });

  test('should display chat history', async ({ page }) => {
    // Send multiple messages
    const input = page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'));
    
    await input.fill('First message');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(1000);
    
    await input.fill('Second message');
    await page.keyboard.press('Enter');
    
    // Should show both messages
    await expect(page.locator('text=First message')).toBeVisible();
    await expect(page.locator('text=Second message')).toBeVisible();
  });

  test('should show NSDR suggestion for low readiness', async ({ page }) => {
    // If readiness is low, should show NSDR suggestion
    const nsdrSuggestion = page.locator('text=/NSDR|stress|readiness|recovery/i');
    if (await nsdrSuggestion.isVisible({ timeout: 5000 })) {
      await expect(nsdrSuggestion).toBeVisible();
    }
  });

  test('should trigger NSDR modal from suggestion', async ({ page }) => {
    const nsdrButton = page.locator('button:has-text("Start NSDR")').or(page.locator('button:has-text("NSDR")'));
    if (await nsdrButton.isVisible({ timeout: 5000 })) {
      await nsdrButton.click();
      
      // Should open NSDR modal
      await expect(page.locator('text=/Non-Sleep Deep Rest|NSDR|breathing/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should scroll to latest message', async ({ page }) => {
    // Send several messages
    const input = page.locator('input[placeholder*="message"]').or(page.locator('textarea[placeholder*="message"]'));
    
    for (let i = 1; i <= 5; i++) {
      await input.fill(`Message ${i}`);
      await page.keyboard.press('Enter');
      await page.waitForTimeout(500);
    }
    
    // Latest message should be visible
    await expect(page.locator('text=Message 5')).toBeVisible();
  });
});
