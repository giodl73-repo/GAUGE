import {test,expect} from '@playwright/test';
import fs from 'node:fs/promises';
test('historical baseline, keyboard scenario, share and JSON use actual WASM',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto('/GAUGE/');await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#below')).toHaveText('8 → 8');await expect(page.locator('#rows tr')).toHaveCount(12);
 await page.locator('#trips').focus();await page.keyboard.press('End');await expect(page.locator('#status')).toContainText('Ready');
 await expect(page.locator('#below')).toHaveText('8 → 7');await expect(page.locator('#score')).toHaveText('0.6 → 10.0');
 const promise=page.waitForEvent('download');await page.locator('#download').click();const download=await promise;const json=JSON.parse(await fs.readFile(await download.path(),'utf8'));expect(json.model).toBe('gauge-frequency-v1');expect(json.input.round_trips).toBe(32);expect(json.baseline.corridors[0].historical_source_id).toBe('wikipedia-amtrak-routes');
 await page.locator('#share').click();await expect(page).toHaveURL(/round_trips=32/);await page.reload();await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#below')).toHaveText('8 → 7');
 await page.getByRole('button',{name:'Reset',exact:true}).click();await expect(page.locator('#below')).toHaveText('8 → 8');expect(errors).toEqual([]);
});
test('corridor selection, changed bar and mobile retain evidence scope',async({page})=>{
 await page.setViewportSize({width:390,height:844});await page.goto('/GAUGE/');await expect(page.locator('#status')).toContainText('Ready');
 await page.locator('#corridor').selectOption({label:'Cascades'});await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#trips')).toHaveValue('4');
 await page.locator('#bar').fill('0');await expect(page.locator('#below')).toHaveText('0 → 0');
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);await expect(page.getByText(/Historical 2023\/2024 counts/)).toBeVisible();
 const license=await page.request.get('/GAUGE/LICENSE');expect(license.status()).toBe(200);expect(await license.text()).toContain('NC');
});
test('invalid URL recovers and failed WASM disables exports',async({page})=>{
 await page.goto('/GAUGE/?corridor=99&round_trips=999');await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#trips')).toHaveValue('1');
 await page.route('**/pkg/gauge_web_bg.wasm',route=>route.abort());await page.reload();await expect(page.locator('#status')).toContainText('could not load');await expect(page.locator('#download')).toBeDisabled();
});

test('rapid changes initialize each corridor from its own historical frequency',async({page})=>{
 await page.goto('/GAUGE/');await expect(page.locator('#status')).toContainText('Ready');
 await page.evaluate(()=>{const trips=document.getElementById('trips'),corridor=document.getElementById('corridor');trips.value=32;trips.dispatchEvent(new Event('input',{bubbles:true}));corridor.value=2;corridor.dispatchEvent(new Event('change',{bubbles:true}));corridor.value=7;corridor.dispatchEvent(new Event('change',{bubbles:true}));});
 await expect(page.locator('#trips')).toHaveValue('16');await expect(page.locator('#name')).toHaveText('Acela');await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#below')).toHaveText('8 → 8');
});

test('reset during WASM loading clears shared corridor and assumptions',async({page})=>{
 await page.route('**/pkg/gauge_web_bg.wasm',async route=>{await new Promise(resolve=>setTimeout(resolve,1000));await route.continue();});
 await page.goto('/GAUGE/?corridor=7&round_trips=16&bar=10');
 await page.getByRole('button',{name:'Reset',exact:true}).click();
 await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#name')).toHaveText('California Zephyr');await expect(page.locator('#trips')).toHaveValue('1');await expect(page.locator('#bar')).toHaveValue('7');
});
test('exact adequacy bar keeps displayed membership and classification consistent',async({page})=>{
 await page.goto('/GAUGE/?corridor=0&round_trips=5.8&bar=3.6');await expect(page.locator('#status')).toContainText('Ready');await expect(page.locator('#below')).toHaveText('6 → 5');await expect(page.locator('#classification')).toContainText('Concentrated frequency tail');
});
