### AI-Generated Readme -fix 

// https://www.nordnet.se/aktier/inspiration/listor/omx-stockholm-30
// https://crates.io/crates/yahoo_finance_api


# koll

A terminal application, written in Rust, for browsing Swedish news via SVT's Text-TV service.

## What it does

`koll` fetches and displays SVT Text-TV pages directly in your terminal. Text-TV is SVT's teletext-style news service — short, numbered pages of news, weather, sports, and more, originally designed for old TV remote controls.

On startup, `koll` shows the Text-TV front page (page 101) with the day's headlines, each followed by a page number you can jump to for the full text. The app automatically tracks every page number it encounters — including numbers found on pages you navigate to — so you can freely explore linked pages (weather, sports, alphabetical index, etc.), not just the initial headlines.

### Features

- Displays the Text-TV headline page on startup, and re-checks it for updates each time you return to the prompt.
- Enter any page number to view its full content.
- Automatically discovers and validates new page numbers as you browse, so previously-unseen pages become navigable.
- Handles messy real-world page references from the raw text (e.g. trailing letters like `421f`, continuation markers like `161-`, and hyphenated ranges like `405f-412f`, which are expanded into all pages in the range).
- Quit anytime with `q`, `quit` (case-insensitive), or `Ctrl+D`.

### Planned

- A `börs` command to show current OMX30 stock prices and market cap alongside the news.

## Data source

[api.texttv.nu](https://api.texttv.nu) — a free, unofficial but well-documented public JSON API that mirrors SVT's Text-TV pages. No API key required.

## Dependencies

| Crate | Purpose |
|---|---|
| [`tokio`](https://crates.io/crates/tokio) | Async runtime — required to run async functions and drive network requests. |
| [`reqwest`](https://crates.io/crates/reqwest) | HTTP client, used to fetch pages from the Text-TV API. |
| [`serde_json`](https://crates.io/crates/serde_json) | Parses the JSON responses into a queryable `Value` structure. |

## Project structure

```
src/
  main.rs    – Entry point; owns the main loop, orchestrates input and page display.
  input.rs   – Reads and validates user input from the terminal (page numbers, quit commands).
  pages.rs   – Fetches Text-TV pages, extracts/validates page numbers from page text, and
               tracks which page numbers have been discovered so far.
```

## Build and run

```bash
cargo run
```

## Install as a command

To run the app as a standalone command (`koll`) from anywhere in your terminal, without needing to `cd` into the project or type `cargo run`:

```bash
cargo install --path .
```

This builds an optimized binary and copies it to `~/.cargo/bin/koll` (make sure that directory is on your `PATH` — it is by default if you installed Rust via `rustup`).

```bash
koll
```

**Note:** if you change the source code, you need to re-run `cargo install --path .` to update the installed binary — it does not update automatically.

## Usage

```
Startsida: 101, Stänga: q eller quit
Ange sidnummer:
```

- Enter a page number (e.g. `106`, `400`, `700`) to view that page.
- Enter `q` or `quit` to exit.
- Press `Ctrl+D` to exit immediately.