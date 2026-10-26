import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';

async function ready(page){await page.goto('/GAUGE/');await expect(page.locator('#status')).toContainText('Ready');}

test('actual WASM matches four native canonical results with explicit source and service holds',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>{const module=await import('./pkg/gauge_web.js');await module.default();return refs.map(ref=>module.evaluate_json(ref.input));},refs);
 expect(actual).toEqual(refs.map(ref=>ref.canonical));
 for(const text of actual){const out=JSON.parse(text);expect(out.evidence.primary_source_verified).toBe(false);expect(out.evidence.unassessed_dimensions).toHaveLength(12);expect(Object.values(out.evidence.observations).every(v=>v===null)).toBe(true);expect(Object.values(out.evidence.authority).every(v=>v===false)).toBe(true);for(const row of [...out.baseline.corridors,...out.scenario.corridors]){expect(row.historical_evidence_label).toBe('source-needed');expect(row.historical_primary_source_verified).toBe(false);}}
 await expect(page.locator('#source')).toContainText('primary source not verified');await expect(page.locator('#evidence-holds')).toContainText('service span hours');
});

test('source audit and exports preserve exact derivative custody without promoting cleared model scores',async({page})=>{
 await ready(page);await page.getByText('Inspect all twelve retained model records',{exact:true}).click();await expect(page.locator('#audit-rows tr')).toHaveCount(12);
 const audit=await(await page.request.get('/GAUGE/source-audit.json')).json();expect(audit.quantity_count).toBe(12);expect(audit.source_needed_quantities).toBe(12);expect(audit.verified_primary_quantities).toBe(0);
 for(const row of audit.entries){const response=await page.request.get('/GAUGE/'+row.published_corpus);expect(response.status()).toBe(200);expect(createHash('sha256').update(await response.body()).digest('hex')).toBe(row.corpus_sha256);expect(row.evidence_label).toBe('source-needed');expect(row.primary_source_verified).toBe(false);expect(row.primary_source_locator).toBeNull();expect(row.primary_source_sha256).toBeNull();expect(row.operating_days_per_week).toBeNull();expect(row.measured_span_hours).toBeNull();}
 const original=await(await page.request.get('/GAUGE/'+audit.original_finding.published)).body();expect(createHash('sha256').update(original).digest('hex')).toBe('33270912db6e861169d34380d18453f41f70b4d5974b74de16cf0910a26ea7d7');expect(original.toString()).toContain('the whole class needs');
 const corrected=await(await page.request.get('/GAUGE/evidence/finding.md')).text();expect(corrected).toContain('are withdrawn');expect(corrected).not.toContain('the whole class needs');
 const custody=await(await page.request.get('/GAUGE/source-custody.json')).json();expect(custody).toHaveLength(16);
 await page.locator('#minimum').click();await expect(page.locator('#below')).toHaveText('8 → 0');const [file]=await Promise.all([page.waitForEvent('download'),page.locator('#download').click()]);const result=JSON.parse(readFileSync(await file.path(),'utf8'));expect(result.scenario.below).toBe(0);expect(result.evidence.observations.service_span_hours).toBeNull();expect(result.evidence.observations.national_representativeness).toBeNull();expect(result.evidence.authority.timetable).toBe(false);expect(result.evidence.authority.fiscal_admission).toBe(false);expect(result.scenario.corridors.every(c=>c.historical_evidence_label==='source-needed')).toBe(true);
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('link',{name:'Download source audit JSON'}).click()]);expect(JSON.parse(readFileSync(await download.path(),'utf8'))).toEqual(audit);
});

test('keyboard and mobile source audit stays usable when WASM is unavailable',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));await page.route('**/pkg/gauge_web_bg.wasm',route=>route.abort());await page.setViewportSize({width:390,height:844});await page.goto('/GAUGE/#source-audit');await expect(page.locator('#status')).toContainText('could not load');await expect(page.locator('#download')).toBeDisabled();
 const summary=page.getByText('Inspect all twelve retained model records',{exact:true});await summary.focus();await page.keyboard.press('Enter');await expect(page.locator('#audit-rows tr')).toHaveCount(12);const region=page.getByRole('region',{name:'Historical source custody'});await region.focus();await page.keyboard.press('ArrowRight');await expect.poll(()=>region.evaluate(el=>el.scrollLeft)).toBeGreaterThan(0);expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);await expect(page.getByRole('link',{name:'Source audit and next evidence gate'})).toBeVisible();expect(errors).toEqual([]);
});
