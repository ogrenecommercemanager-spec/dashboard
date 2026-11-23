import { test, expect } from '@playwright/test'

test('creates and displays a demo business', async ({ page }) => {
    await page.goto('/');

    await expect(
        page.getByRole('heading', { name: 'Platform — TB-01.1' })
    ).toBeVisible();

    await page.getByRole('button', {
        name: 'Create and load demo business',
    }).click();

    await expect(
        page.getByText(
            /Business ID: [0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/i 
        )
    ).toBeVisible();

    await expect(page.getByRole('alert')).toHaveCount(0);
})