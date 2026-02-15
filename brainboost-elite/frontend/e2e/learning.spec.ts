import { test, expect } from '@playwright/test';

test.describe('Hyper-Learning Module', () => {
  test.beforeEach(async ({ page }) => {
    // Login as advanced user (has learning topics)
    await page.goto('/login');
    await page.fill('input[name="email"]', 'advanced@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    
    // Navigate to learning page
    await page.click('[aria-label="Learning"]').or(page.click('text=Learning'));
    await expect(page).toHaveURL('/learning', { timeout: 10000 });
  });

  test('should display active learning topics', async ({ page }) => {
    // Should show topics from seed data
    await expect(page.locator('text=options_trading').or(page.locator('text=Options Trading'))).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=python_basics').or(page.locator('text=Python'))).toBeVisible({ timeout: 5000 });
  });

  test('should show mastery percentage for topics', async ({ page }) => {
    // Should show progress
    await expect(page.locator('text=/%|%/')).toBeVisible({ timeout: 5000 });
  });

  test('should create new learning topic', async ({ page }) => {
    // Click add topic button
    const addButton = page.locator('button:has-text("Add Topic")').or(page.locator('button:has-text("Learn Something New")'));
    if (await addButton.isVisible({ timeout: 5000 })) {
      await addButton.click();
      
      // Fill topic form
      await page.fill('input[placeholder*="topic"]', 'Machine Learning');
      await page.click('button[type="submit"]').or(page.click('button:has-text("Create")'));
      
      // Should show new topic
      await expect(page.locator('text=Machine Learning')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should display next due lesson', async ({ page }) => {
    // Should show next lesson banner
    await expect(page.locator('text=/next lesson|due now|lesson \\d+/i')).toBeVisible({ timeout: 5000 });
  });

  test('should open topic detail view', async ({ page }) => {
    // Click on a topic
    await page.click('text=Options Trading').or(page.click('text=options_trading'));
    
    // Should show lesson list
    await expect(page.locator('text=/lesson|What are Options|Calls vs Puts/i')).toBeVisible({ timeout: 5000 });
  });

  test('should show lesson status (completed/current/locked)', async ({ page }) => {
    await page.click('text=Options Trading');
    
    // Should show different lesson states
    const lessons = page.locator('[class*="lesson"]').or(page.locator('text=/lesson \\d+/i'));
    await expect(lessons.first()).toBeVisible({ timeout: 5000 });
  });

  test('should start a lesson', async ({ page }) => {
    await page.click('text=Options Trading');
    
    // Click on an available lesson
    const lessonButton = page.locator('button:has-text("Calls vs Puts")').or(page.locator('button').filter({ hasText: /lesson/i }).first());
    if (await lessonButton.isVisible({ timeout: 5000 })) {
      await lessonButton.click();
      
      // Should show lesson content
      await expect(page.locator('text=/call option|put option|strike price/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should display lesson content', async ({ page }) => {
    await page.click('text=Options Trading');
    const lessonButton = page.locator('button').filter({ hasText: /lesson|Calls/i }).first();
    if (await lessonButton.isVisible({ timeout: 5000 })) {
      await lessonButton.click();
      
      // Should show reading content
      const content = page.locator('[class*="content"]').or(page.locator('p'));
      await expect(content).toHaveCount({ min: 1 }, { timeout: 5000 });
    }
  });

  test('should show quiz after lesson', async ({ page }) => {
    await page.click('text=Options Trading');
    const lessonButton = page.locator('button').filter({ hasText: /lesson/i }).first();
    if (await lessonButton.isVisible({ timeout: 5000 })) {
      await lessonButton.click();
      
      // Scroll to end or click continue
      const continueButton = page.locator('button:has-text("Continue")').or(page.locator('button:has-text("Take Quiz")'));
      if (await continueButton.isVisible({ timeout: 5000 })) {
        await continueButton.click();
        
        // Should show quiz questions
        await expect(page.locator('text=/question|answer|select/i')).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should complete lesson and show score', async ({ page }) => {
    await page.click('text=Options Trading');
    const lessonButton = page.locator('button').filter({ hasText: /lesson/i }).first();
    if (await lessonButton.isVisible({ timeout: 5000 })) {
      await lessonButton.click();
      
      // Complete quiz (if present)
      const submitButton = page.locator('button:has-text("Submit")').or(page.locator('button:has-text("Complete")'));
      if (await submitButton.isVisible({ timeout: 5000 })) {
        // Select answers
        const answerButtons = page.locator('button[role="radio"]').or(page.locator('input[type="radio"]'));
        if (await answerButtons.count() > 0) {
          await answerButtons.first().click();
        }
        
        await submitButton.click();
        
        // Should show score
        await expect(page.locator('text=/score|%|correct/i')).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should show mastery prediction', async ({ page }) => {
    await page.click('text=Options Trading');
    
    // Should show prediction
    await expect(page.locator('text=/days remaining|predicted|on track|ahead|behind/i')).toBeVisible({ timeout: 5000 });
  });
});
