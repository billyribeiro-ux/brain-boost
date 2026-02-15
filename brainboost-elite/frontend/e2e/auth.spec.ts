import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should register a new user successfully', async ({ page }) => {
    // Navigate to register page
    await page.click('text=Create Account');
    await expect(page).toHaveURL('/register');

    // Fill registration form
    await page.fill('input[name="email"]', `test${Date.now()}@brainboost.test`);
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.fill('input[name="confirmPassword"]', 'TestPass123!');
    await page.fill('input[name="displayName"]', 'E2E Test User');

    // Submit form
    await page.click('button[type="submit"]');

    // Should redirect to onboarding
    await expect(page).toHaveURL('/onboarding', { timeout: 10000 });
    await expect(page.locator('h1')).toContainText('Unlock Your Super Brain');
  });

  test('should login existing user successfully', async ({ page }) => {
    // Navigate to login page
    await page.click('text=Sign In');
    await expect(page).toHaveURL('/login');

    // Fill login form with seeded test user
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');

    // Submit form
    await page.click('button[type="submit"]');

    // Should redirect to dashboard
    await expect(page).toHaveURL('/dashboard', { timeout: 10000 });
    await expect(page.locator('text=Brain Score')).toBeVisible();
  });

  test('should show validation errors for invalid credentials', async ({ page }) => {
    await page.goto('/login');

    // Try to login with wrong password
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'WrongPassword123!');
    await page.click('button[type="submit"]');

    // Should show error message
    await expect(page.locator('text=Invalid credentials')).toBeVisible({ timeout: 5000 });
  });

  test('should logout user successfully', async ({ page }) => {
    // Login first
    await page.goto('/login');
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/dashboard');

    // Click logout
    await page.click('[aria-label="Profile"]');
    await page.click('text=Logout');

    // Should redirect to login
    await expect(page).toHaveURL('/login', { timeout: 5000 });
  });

  test('should protect authenticated routes', async ({ page }) => {
    // Try to access dashboard without auth
    await page.goto('/dashboard');

    // Should redirect to login
    await expect(page).toHaveURL('/login', { timeout: 5000 });
  });
});
