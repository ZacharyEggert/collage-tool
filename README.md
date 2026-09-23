# Collage Tool

Desktop app for building photo collages. Drop photos into a rows × cols grid, preview live, save a collage with a 3840px long side.

## Install

1. Install [ImageMagick](https://imagemagick.org) 7 (`magick`):
   - macOS: `brew install imagemagick`
   - Windows: installer from imagemagick.org (tick "Add to PATH")
   - Linux: distro package, or AppImage on PATH
2. Download the app from [Releases](https://github.com/ZacharyEggert/collage-tool/releases).

## Use

- Set **Columns**, **Rows**, **Aspect (W:H)** of each tile, **Gap** (px) and **Background** color.
- Click a slot or drag a photo onto it. Drag between slots to swap; × clears.
- First photo sets the aspect automatically.
- **Save collage…** renders via ImageMagick. Photos are fit inside tiles (not cropped), padded with the background color; gaps are even between tiles and around the edge.

## Develop

Needs Rust, pnpm, and `magick` on PATH.

```sh
pnpm install
pnpm dev     # run
pnpm build   # bundle
```

No bundler: UI is `src/index.html`; rendering is the `render` command in `src-tauri/src/main.rs`, which shells out to `magick`.

## Release

Push a `vX.Y.Z` tag. `.github/workflows/release.yml` stamps the version from the tag and builds macOS (universal), Windows, and Linux into a GitHub release.

```sh
git tag v0.1.2 && git push origin v0.1.2
```
