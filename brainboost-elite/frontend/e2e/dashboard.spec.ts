import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    // Login as active user
    await page.goto('/login');
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/dashboard');
  });

  test('should display brain score gauge', async ({ page }) => {
    // Wait for metrics to load
    await expect(page.locator('text=Brain Score')).toBeVisible();
    
    // Should show score value
    const scoreElement = page.locator('text=/\\d+/').first();
    await expect(scoreElement).toBeVisible();
    
    // Score should be between 0-100
    const scoreText = await scoreElement.textContent();
    const score = parseInt(scoreText || '0');
    expect(score).toBeGreaterThanOrEqual(0);
    expect(score).toBeLessThanOrEqual(100);
  });

  test('should display readiness gauge', async ({ page }) => {
    await expect(page.locator('text=Readiness')).toBeVisible();
    
    // Should show percentage
    await expect(page.locator('text=/%/')).toBeVisible();
  });

  test('should display streak flame', async ({ page }) => {
    // Should show streak count
    await expect(page.locator('[class*="streak"]')).toBeVisible();
  });

  test('should display daily checklist with 3 sessions', async ({ page }) => {
    // Wait for checklist to load
    await page.waitForSelector('text=Morning', { timeout: 10000 });
    
    // Should show all 3 slots
    await expect(page.locator('text=Morning')).toBeVisible();
    await expect(page.locator('text=Afternoon')).toBeVisible();
    await expect(page.locator('text=Evening')).toBeVisible();
  });

  test('should expand session to show exercises', async ({ page }) => {
    await page.waitForSelector('text=Morning');
    
    // Click to expand morning session
    await page.click('text=Morning');
    
    // Should show exercises
    await expect(page.locator('text=Meditation').or(page.locator('text=Flashcards'))).toBeVisible({ timeout: 5000 });
  });

  test('should show completed status for finished sessions', async ({ page }) => {
    // Morning session should be completed (from seed data)
    const morningSession = page.locator('text=Morning').locator('..');
    await expect(morningSession.locator('text=Completed')).toBeVisible({ timeout: 10000 });
  });

  test('should display metrics overview cards', async ({ page }) => {
    // Should show metric cards
    await expect(page.locator('text=Brain Score')).toBeVisible();
    await expect(page.locator('text=Streak')).toBeVisible();
    await expect(page.locator('text=Stress')).toBeVisible();
    await expect(page.locator('text=Longevity')).toBeVisible();
  });

  test('should show quick start button', async ({ page }) => {
    // Should show floating action button
    const fab = page.locator('button[class*="fixed"][class*="bottom"]');
    await expect(fab).toBeVisible();
  });

  test('should navigate to exercise on quick start click', async ({ page }) => {
    // Click quick start button
    const fab = page.locator('button[class*="fixed"][class*="bottom"]');
    await fab.click();
    
    // Should navigate to next pending exercise
    await expect(page).toHaveURL(/\/(exercise|dashboard)/, { timeout: 5000 });
  });

  test('should display neural background animation', async ({ page }) => {
    // Should have animated background
    const background = page.locator('svg').first();
    await expect(background).toBeVisible();
  });

  test('should be responsive on mobile', async ({ page, isMobile }) => {
    test.skip(!isMobile, 'Mobile-only test');
    
    // All key elements should be visible on mobile
    await expect(page.locator('text=Brain Score')).toBeVisible();
    await expect(page.locator('text=Morning')).toBeVisible();
    
    // Bottom nav should be visible
    await expect(page.locator('[aria-label="Dashboard"]')).toBeVisible();
  });
});
