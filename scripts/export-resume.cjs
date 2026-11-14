// Run after `trunk build`, with Playwright and Microsoft Edge installed.
// Usage: node scripts/export-resume.cjs [site URL]
// The PDF uses the same content and print styles as the website.
const { chromium } = require('playwright');
const fs = require('node:fs/promises');
const path = require('node:path');
const http = require('node:http');

(async () => {
  const root = path.resolve('dist');
  const types = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css', '.pdf': 'application/pdf' };
  const server = http.createServer(async (request, response) => {
    const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
    const file = path.resolve(root, `.${pathname === '/' ? '/index.html' : pathname}`);
    if (!file.startsWith(root + path.sep)) { response.writeHead(403).end(); return; }
    try {
      const data = await fs.readFile(file);
      response.writeHead(200, { 'Content-Type': types[path.extname(file)] || 'application/octet-stream' });
      response.end(data);
    } catch { response.writeHead(404).end(); }
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const base = process.argv[2] || `http://127.0.0.1:${server.address().port}`;
  let browser;
  try {
    browser = await chromium.launch({ channel: 'msedge', headless: true });
    const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(`${base}/#resume`);
    await page.locator('.resume-sheet').waitFor({ state: 'visible' });
    await page.evaluate(() => document.fonts.ready);
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await fs.mkdir('tmp/resume', { recursive: true });
    await page.locator('.resume-sheet').screenshot({ path: 'tmp/resume/desktop.png' });
    // Local hash routes cannot be opened by recipients of the downloaded PDF.
    await page.locator('.resume-entry a').evaluateAll(links => links.forEach(link => {
      link.dataset.href = link.getAttribute('href');
      link.removeAttribute('href');
    }));
    await page.pdf({ path: 'public/resume.pdf', printBackground: true, preferCSSPageSize: true });
    await page.locator('.resume-entry a').evaluateAll(links => links.forEach(link => {
      link.setAttribute('href', link.dataset.href);
      delete link.dataset.href;
    }));
    await fs.copyFile('public/resume.pdf', 'dist/public/resume.pdf');
    await page.setViewportSize({ width: 375, height: 812 });
    await page.screenshot({ path: 'tmp/resume/mobile.png', fullPage: true });
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth);
    if (overflow) throw new Error('Résumé overflows the mobile viewport.');
    const download = await Promise.all([
      page.waitForEvent('download'),
      page.getByRole('link', { name: 'Download résumé PDF' }).click(),
    ]);
    if (await download[0].failure()) throw new Error('PDF download failed.');
    if (errors.length) throw new Error(errors.join('\n'));
    console.log(`Exported ${path.resolve('public/resume.pdf')}; desktop, mobile, and download checks passed.`);
  } finally {
    if (browser) await browser.close();
    server.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
