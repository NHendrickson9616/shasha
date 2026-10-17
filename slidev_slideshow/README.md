# Shasha Slidev slideshow

An editable presentation based on the Shasha demo brief and Manim explainer.
The deck presents the same argument as eight native slides, with speaker notes
for a short live pitch.

## Run locally

```sh
cd slidev_slideshow
npm install
npm run dev
```

Use the arrow keys to navigate. Press `o` for the overview and `p` for
presenter mode.

## Build

```sh
npm run build
```

The static slideshow is written to `dist/`.

To export a PDF, install Playwright's Chromium browser once and run:

```sh
npx playwright install chromium
npm run export
```
