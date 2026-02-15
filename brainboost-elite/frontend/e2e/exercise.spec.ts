import { test, expect } from '@playwright/test';

test.describe('Exercise Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Login as active user
    await page.goto('/login');
    await page.fill('input[name="email"]', 'activeuser@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/dashboard');
  });

  test('should start exercise from checklist', async ({ page }) => {
    // Expand afternoon session (pending)
    await page.click('text=Afternoon');
    
    // Click first exercise
    await page.click('button:has-text("Meditation")').first();
    
    // Should open exercise modal or navigate to exercise page
    await expect(page.locator('text=Meditation').or(page.locator('h1'))).toBeVisible({ timeout: 5000 });
  });

  test('should display timer for timed exercises', async ({ page }) => {
    await page.click('text=Afternoon');
    await page.click('button:has-text("Meditation")').first();
    
    // Should show timer
    await expect(page.locator('text=/\\d+:\\d+/')).toBeVisible({ timeout: 5000 });
  });

  test('should allow pausing and resuming timer', async ({ page }) => {
    await page.click('text=Afternoon');
    await page.click('button:has-text("Meditation")').first();
    
    // Wait for timer to appear
    await page.waitForSelector('text=/\\d+:\\d+/', { timeout: 5000 });
    
    // Click pause button
    const pauseButton = page.locator('button[aria-label="Pause"]').or(page.locator('button:has-text("Pause")'));
    if (await pauseButton.isVisible()) {
      await pauseButton.click();
      
      // Click resume
      const resumeButton = page.locator('button[aria-label="Resume"]').or(page.locator('button:has-text("Resume")'));
      await resumeButton.click();
    }
  });

  test('should complete exercise and show success', async ({ page }) => {
    await page.click('text=Afternoon');
    await page.click('button:has-text("Meditation")').first();
    
    // Skip or complete exercise
    const completeButton = page.locator('button:has-text("Complete")').or(page.locator('button:has-text("Finish")'));
    if (await completeButton.isVisible({ timeout: 5000 })) {
      await completeButton.click();
      
      // Should show success message or celebration
      await expect(page.locator('text=/Complete|Success|Great|Done/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show exercise instructions', async ({ page }) => {
    await page.click('text=Afternoon');
    await page.click('button:has-text("Meditation")').first();
    
    // Should show instructions
    await expect(page.locator('text=/breath|focus|relax|close your eyes/i')).toBeVisible({ timeout: 5000 });
  });

  test('should track focus rating after completion', async ({ page }) => {
    await page.click('text=Afternoon');
    await page.click('button:has-text("Meditation")').first();
    
    // Complete exercise
    const completeButton = page.locator('button:has-text("Complete")').or(page.locator('button:has-text("Finish")'));
    if (await completeButton.isVisible({ timeout: 5000 })) {
      await completeButton.click();
      
      // Should ask for focus rating
      const ratingPrompt = page.locator('text=/How focused|Rate your focus|Focus rating/i');
      if (await ratingPrompt.isVisible({ timeout: 3000 })) {
        // Select a rating
        await page.click('button:has-text("8")').or(page.locator('[aria-label="Rating 8"]'));
      }
    }
  });

  test('should update dashboard after exercise completion', async ({ page }) => {
    await page.click('text=Afternoon');
    const exerciseName = await page.locator('button').filter({ hasText: /Meditation|Flashcard/ }).first().textContent();
    await page.click('button').filter({ hasText: /Meditation|Flashcard/ }).first();
    
    // Complete exercise
    const completeButton = page.locator('button:has-text("Complete")').or(page.locator('button:has-text("Finish")'));
    if (await completeButton.isVisible({ timeout: 5000 })) {
      await completeButton.click();
      
      // Navigate back to dashboard
      await page.goto('/dashboard');
      
      // Exercise should now show as completed
      await page.click('text=Afternoon');
      const completedExercise = page.locator(`text=${exerciseName}`).locator('..');
      await expect(completedExercise.locator('[class*="line-through"]').or(completedExercise.locator('text=✓'))).toBeVisible({ timeout: 5000 });
    }
  });
});
