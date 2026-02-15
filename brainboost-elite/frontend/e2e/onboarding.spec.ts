import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Login as new user (no chronotype/goal set)
    await page.goto('/login');
    await page.fill('input[name="email"]', 'newuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    
    // Should redirect to onboarding
    await expect(page).toHaveURL('/onboarding', { timeout: 10000 });
  });

  test('should complete full onboarding flow', async ({ page }) => {
    // Screen 1: Welcome
    await expect(page.locator('h1')).toContainText('Unlock Your Super Brain');
    await page.click('button:has-text("Get Started")');

    // Screen 2: Chronotype selection
    await expect(page.locator('h2')).toContainText('When do you feel most energized');
    await page.click('button:has-text("Flexible")');
    
    // Adjust times
    await page.click('button:has-text("Next")');

    // Screen 3: Goal selection
    await expect(page.locator('h2')).toContainText('primary super-brain goal');
    await page.click('button:has-text("Rapid Learning")');
    await page.click('button:has-text("Next")');

    // Screen 4: Experience level
    await expect(page.locator('h2')).toContainText('How experienced are you');
    await page.click('button:has-text("Intermediate")');
    await page.click('button:has-text("Next")');

    // Screen 5: Summary and complete
    await expect(page.locator('h2')).toContainText('Your Personalized Plan');
    await expect(page.locator('text=Chronotype')).toBeVisible();
    await expect(page.locator('text=Flexible')).toBeVisible();
    
    await page.click('button:has-text("Begin Your Journey")');

    // Should redirect to dashboard
    await expect(page).toHaveURL('/dashboard', { timeout: 10000 });
  });

  test('should allow skipping to summary', async ({ page }) => {
    // Click skip button
    await page.click('button:has-text("Get Started")');
    await page.click('button:has-text("Skip")');

    // Should jump to summary
    await expect(page.locator('h2')).toContainText('Your Personalized Plan');
  });

  test('should show progress dots', async ({ page }) => {
    await page.click('button:has-text("Get Started")');

    // Should show progress indicators
    const dots = page.locator('[class*="rounded-full"][class*="bg-"]');
    await expect(dots).toHaveCount(4); // 4 dots for 4 middle screens
  });

  test('should support swipe navigation on mobile', async ({ page, isMobile }) => {
    test.skip(!isMobile, 'Mobile-only test');

    await page.click('button:has-text("Get Started")');
    
    // Swipe left (next screen)
    await page.touchscreen.swipe({ x: 300, y: 400 }, { x: 100, y: 400 });
    
    // Should advance to next screen
    await expect(page.locator('h2')).toContainText('primary super-brain goal');
  });
});
