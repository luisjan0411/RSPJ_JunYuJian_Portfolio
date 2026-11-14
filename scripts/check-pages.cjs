// Build first: trunk build --release --locked --public-url /RSPJ_JunYuJian_Portfolio/ --dist tmp/pages-dist
// Requires Playwright and Microsoft Edge. An optional URL checks the live site.
const { chromium } = require('playwright');
const fs = require('node:fs/promises');
const path = require('node:path');
const http = require('node:http');
const assert = require('node:assert/strict');

(async () => {
  const root = path.resolve('tmp/pages-dist');
  const prefix = '/RSPJ_JunYuJian_Portfolio/';
  const types = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css', '.pdf': 'application/pdf', '.mp4': 'video/mp4', '.png': 'image/png' };
  const server = http.createServer(async (req, res) => {
    const pathname = decodeURIComponent(new URL(req.url, 'http://localhost').pathname);
    if (!pathname.startsWith(prefix)) { res.writeHead(404).end(); return; }
    const file = path.resolve(root, pathname.slice(prefix.length) || 'index.html');
    if (!file.startsWith(root + path.sep)) { res.writeHead(403).end(); return; }
    try {
      const data = await fs.readFile(file);
      res.writeHead(200, { 'Content-Type': types[path.extname(file)] || 'application/octet-stream' }).end(data);
    } catch { res.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  let browser;
  try {
    const base = process.argv[2] || `http://127.0.0.1:${server.address().port}${prefix}`;
    browser = await chromium.launch({ channel: 'msedge', headless: true });
    const page = await browser.newPage({ reducedMotion: 'reduce' });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(`${base}#projects`);
    for (const [label, heading] of [['Home', 'home-heading'], ['About', 'about-heading'], ['Résumé', 'resume-heading'], ['Projects', 'projects-heading']]) {
      await page.getByRole('navigation', { name: 'Main navigation' }).getByRole('link', { name: label, exact: true }).click();
      await page.locator(`#${heading}`).waitFor({ state: 'visible' });
      assert.equal(await page.locator('.page-panel:visible').count(), 1);
      assert.equal(await page.getByRole('navigation').getByRole('link', { name: label, exact: true }).getAttribute('aria-current'), 'page');
    }
    await page.locator('.project-card').first().waitFor();
    const routes = await page.locator('.project-card').evaluateAll(cards => cards.map(card => card.getAttribute('href')));
    assert.equal(routes.length, 7);
    const assets = new Set();
    for (const route of routes) {
      await page.goto(`${base}${route}`);
      await page.locator('.project-detail-media').first().waitFor();
      for (const src of await page.locator('video, img').evaluateAll(nodes => nodes.map(node => node.src))) assets.add(src);
    }
    await page.goto(`${base}#resume`);
    await page.locator('.resume-sheet').waitFor();
    const pdf = page.getByRole('link', { name: 'Download résumé PDF' });
    assets.add(await pdf.evaluate(link => link.href));
    for (const asset of assets) {
      assert.ok(new URL(asset).pathname.startsWith(prefix), `Incorrect asset path: ${asset}`);
      const response = await page.request.get(asset);
      assert.ok(response.ok(), `${response.status()} ${asset}`);
      if (asset.endsWith('.pdf')) assert.equal((await response.body()).subarray(0, 5).toString(), '%PDF-');
      await response.dispose();
    }
    await page.reload();
    await page.locator('.resume-sheet').waitFor();
    assert.deepEqual(errors, []);
    console.log(`Passed: 7 project routes, ${assets.size} media/PDF assets, and direct résumé reload at ${base}`);
  } finally {
    if (browser) await browser.close();
    server.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
