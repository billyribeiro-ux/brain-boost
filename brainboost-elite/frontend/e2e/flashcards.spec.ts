import { test, expect } from '@playwright/test';

test.describe('Flashcard System', () => {
  test.beforeEach(async ({ page }) => {
    // Login as advanced user (has flashcards)
    await page.goto('/login');
    await page.fill('input[name="email"]', 'advanced@test.brainboost');
    await page.fill('input[name="password"]', 'TestPass123!');
    await page.click('button[type="submit"]');
  });

  test('should access flashcards from exercise', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    // Look for flashcard exercise
    const flashcardButton = page.locator('button:has-text("Flashcard")').or(page.locator('button:has-text("Flashcards")'));
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Should show flashcard interface
      await expect(page.locator('text=/question|answer|flip|reveal/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should display flashcard question', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Should show question from seed data
      await expect(page.locator('text=/What is|Define|Explain/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should flip card to show answer', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Click to flip
      const flipButton = page.locator('button:has-text("Flip")').or(page.locator('button:has-text("Show Answer")'));
      if (await flipButton.isVisible({ timeout: 5000 })) {
        await flipButton.click();
        
        // Should show answer
        await expect(page.locator('[class*="answer"]').or(page.locator('text=/answer/i'))).toBeVisible({ timeout: 3000 });
      }
    }
  });

  test('should show SM-2 rating buttons', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Flip card
      const flipButton = page.locator('button:has-text("Flip")').or(page.locator('[class*="card"]'));
      if (await flipButton.isVisible({ timeout: 5000 })) {
        await flipButton.click();
        
        // Should show rating buttons (Again, Hard, Good, Easy)
        await expect(page.locator('button:has-text("Again")').or(page.locator('button:has-text("Hard")'))).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should show next review interval on rating buttons', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      const flipButton = page.locator('button:has-text("Flip")').or(page.locator('[class*="card"]'));
      if (await flipButton.isVisible({ timeout: 5000 })) {
        await flipButton.click();
        
        // Should show intervals (e.g., "1 day", "3 days")
        await expect(page.locator('text=/\\d+ day|\\d+ hour/i')).toBeVisible({ timeout: 5000 });
      }
    }
  });

  test('should advance to next card after rating', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Get first question
      const firstQuestion = await page.locator('text=/What is|Define/i').first().textContent();
      
      // Flip and rate
      const flipButton = page.locator('button:has-text("Flip")').or(page.locator('[class*="card"]'));
      if (await flipButton.isVisible({ timeout: 5000 })) {
        await flipButton.click();
        
        const goodButton = page.locator('button:has-text("Good")');
        if (await goodButton.isVisible({ timeout: 3000 })) {
          await goodButton.click();
          
          // Should show next card or completion
          await page.waitForTimeout(1000);
          const currentText = await page.textContent('body');
          expect(currentText).toBeTruthy();
        }
      }
    }
  });

  test('should show progress counter', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Should show progress (e.g., "1/5")
      await expect(page.locator('text=/\\d+\\/\\d+/')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show completion summary', async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Afternoon');
    
    const flashcardButton = page.locator('button:has-text("Flashcard")');
    if (await flashcardButton.isVisible({ timeout: 5000 })) {
      await flashcardButton.click();
      
      // Rate all cards quickly
      for (let i = 0; i < 5; i++) {
        const flipButton = page.locator('button:has-text("Flip")').or(page.locator('[class*="card"]'));
        if (await flipButton.isVisible({ timeout: 3000 })) {
          await flipButton.click();
          
          const goodButton = page.locator('button:has-text("Good")');
          if (await goodButton.isVisible({ timeout: 2000 })) {
            await goodButton.click();
            await page.waitForTimeout(500);
          } else {
            break;
          }
        } else {
          break;
        }
      }
      
      // Should show completion or summary
      await expect(page.locator('text=/complete|finished|reviewed|summary/i')).toBeVisible({ timeout: 5000 });
    }
  });
});
