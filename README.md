# Jun Yu Jian — Portfolio

Rust / Leptos portfolio built with Trunk.

## Local development

Install Rust and Trunk 0.22.1, then run:

```sh
rustup target add wasm32-unknown-unknown
trunk serve
```

## GitHub Pages

Repository Settings → Pages → Source must be **GitHub Actions**.
Pushing to `master` runs `.github/workflows/deploy.yml`: build the release
site, upload `dist`, and deploy it to Pages. It can also be started manually
from the Actions tab using **Run workflow**.

The workflow reads the Pages base path, and Trunk inserts it into the HTML
`base` element. Keep media and PDF paths relative (`public/...`) so they work
both locally and under the repository's URL prefix.

Generated `dist` files are not committed. Edit `src/main.rs`, `style.css`, or
files under `public`, then commit and push. The downloadable résumé is
`public/resume.pdf`; regenerate it after résumé changes using
`node scripts/export-resume.cjs` after a local `trunk build` (requires
Playwright and Microsoft Edge).

To check the repository URL layout locally:

```sh
trunk build --release --locked --public-url /RSPJ_JunYuJian_Portfolio/ --dist tmp/pages-dist
node scripts/check-pages.cjs
```

The browser check requires Playwright and Microsoft Edge. Pass a deployed
site URL ending with `/` to check the live site instead.
