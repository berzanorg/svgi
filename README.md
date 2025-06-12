# svgi

Tool to generate X, Discord and Github icons from an SVG icon.

Under the hood, it uses [resvg](https://github.com/linebender/resvg) and [oxipng](https://github.com/oxipng/oxipng) to render and optimize SVG icons.

## Installation

```bash
cargo install svgi
```

## Usage

The command below will generate `path/to/x.png`, `path/to/dc.png`, `path/to/gh.png` files.

```
svgi path/to/icon.svg
```

## Resolutions

Below are the resolutions for the generated icons.

They are chosen based on my tests about related platforms.

- `x.png`: 400x400
- `dc.png`: 512x512
- `gh.png`: 1200x1200

> Developed by [Berzan](https://x.com/berzanorg) in Korea.
