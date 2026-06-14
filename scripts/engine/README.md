# VAI Profile Engine

A small, configuration-driven engine that renders a GitHub profile README as a set
of **Tokyo-Night animated SVG cards** and keeps them fresh via GitHub Actions.
Fork it, edit [`content.py`](content.py), run one command — it's your profile.

The card renderer is **pure stdlib**. Only *game-art fetching* needs Pillow
(installed via the optional `art` extra).

---

## Commands (the easy way)

Run from the repo root. With [uv](https://docs.astral.sh/uv/) you don't need a
Python install — `uv run` provisions everything.

| Command | What it does |
|---------|--------------|
| `uv run build` | Render all cards + refresh the README image rows |
| `uv run build --only header,vibe` | Render just those cards (skips README) |
| `uv run preview` | Build, then open a stacked HTML preview in your browser |
| `uv run rebuild` | **One-shot: fetch best-game art *then* build everything** |
| `uv run --extra art rebuild --games` | …also re-fetch the game-shelf covers |
| `uv run --extra art art --only zzz,genshin` | (Re)download specific shelf covers |
| `uv run --extra art bestgame` | (Re)download the best-game cover + character art |

> **Added a 5th squad character and it shows only an avatar?** You skipped the
> fetch step. Just run **`uv run --extra art rebuild`** — it fetches the new
> character art and rebuilds in one go. (`build` only renders; it never downloads.)

Set `GH_TOKEN` (or `GITHUB_TOKEN`) for exact commit counts; without it the engine
falls back to the public contribution calendar. Art commands take a SteamGridDB key
via `--key` or `SGDB_KEY` (free: steamgriddb.com → Preferences → API).

---

## Layout

```
scripts/
├── build.py / fetch_game_art.py   # thin shims (used by CI)
└── engine/
    ├── cli.py        # friendly commands (build/rebuild/preview/art/bestgame)
    ├── run.py        # orchestrator: collect data → render cards → inject README
    ├── art.py        # SteamGridDB + URL/local art fetch (Pillow)
    ├── theme.py      # design system: palette, fonts, card() chrome, media helpers
    ├── content.py    # ALL editorial data — edit this to make it yours
    ├── data.py       # GitHub REST + GraphQL + contribution-calendar scrape
    ├── log.py        # pretty, dependency-free logger
    ├── readme.py     # README marker injection + per-asset cache-bust hashes
    └── cards/        # one module per card; each exposes build(ctx) -> svg
```

Each card renders in isolation — a broken card never takes down the rest.

---

## Configuration — [`content.py`](content.py)

| Name | Controls |
|------|----------|
| `ALIASES` | Cycling handles in the header `aka>` line |
| `GAMES` | "Now playing" shelf + SteamGridDB search terms |
| `GAME_NOTES`, `COMPOSERS`, `FOCUS`, `QUOTES` | Vibe-card flavour |
| `SOCIALS` | Verified social links |
| `RESEARCH` | Research-card title / subtitle / blurb |
| `BEST_GAME` | The best-game spotlight (below) |

Change the username via the `PROFILE_USER` env var (defaults to `Vadim-Khristenko`).

### Best-game card (`BEST_GAME`)

```python
BEST_GAME = {
    "title": "NIKKE", "subtitle": "GODDESS OF VICTORY",
    "query": "Goddess of Victory Nikke",     # SteamGridDB search if art_url is ""
    "art_url": "https://cdn2.steamgriddb.com/grid/….webp",  # cover (URL or local path)
    "cover_mode": "portrait",                 # "portrait" (2:3 side panel) | "landscape"
    "accent": "#e23b5a",
    "nick": "VAI", "level": "95", "server": "Global", "game_id": "12405515",
    "blurb": "…",
    "characters": [                            # up to 5, laid out automatically
        {"name": "Rapi: Red Hood", "short": "Red Hood", "accent": "#f7768e",
         "art_url": "https://…/sprite.png"},   # URL or local path; "" → initials avatar
    ],
}
```

- **Cover & character `art_url`** accept a remote URL **or a local path** under the
  repo (e.g. `assets/bestgame/cover.png`), including **animated WEBP/GIF** — kept
  animated (oversized animated covers are auto-recompressed, with a static fallback).
- After editing `BEST_GAME`, run `uv run --extra art rebuild` to pull the art.
- Art lands in `assets/bestgame/` (`cover.*`, `char_<short>.*`); the card embeds it
  as base64. Downloads retry with backoff and warn if they ultimately fail.

---

## README integration

`build` rewrites only the regions between markers — your prose is never touched:

```
<!-- ENGINE:HEADER --> RESEARCH · BESTGAME · GAMES · VIBE · STATS · FOOTER
```

Each `<img>` gets `?v=<content-hash>-<2-day-bucket>`: the cache busts both on any
byte change **and** at least every 2 days, so refreshed stats never serve stale.

---

## Automation (`.github/workflows/`)

- **`profile-engine.yml`** — every 2 days + manual + on push; runs `build` and
  commits. Optional secret `PROFILE_TOKEN` (PAT, `Contents:Read` + `Metadata:Read`)
  also counts **private** commits. `contents: write` is the only permission needed.
- **`refresh-game-art.yml`** — manual; needs secret `STEAMGRIDDB_API_KEY`. Optional
  repo **Variable** `GAME_ONLY` (e.g. `genshin,zzz`) pins a default subset.

---

## Cadence

| Content | Refresh |
|---------|---------|
| Stats / activity / languages | every run (every 2 days) |
| Vibe (game · track · focus) | every 2 days (`vibe_seed = ordinal/2`) |
| Game / best-game art | manual (`rebuild` / `refresh-game-art.yml`) |

Vibe **FOCUS** = the most recently-pushed repo, *excluding* the profile repo itself
(its engine auto-commits would otherwise always win).

---

## Theming & new cards

Colours live in [`theme.py`](theme.py). `card(w, h, title, inner, accent=…,
badge=…, texture=…)` wraps any card in the window chrome; `texture` ∈
`grid` (default) · `ternary` (1.58-bit motif) · `plain` (none).

To add a card: create `cards/mycard.py` with `ASSET` + `build(ctx)`, add it to
`CARDS` in `run.py`, add an `ENGINE:MYCARD` block in `readme.py` + the markers in
`README.md`. `ctx` has `data`, `seed`, `vibe_seed`, `week`, `year`, `stamp`, `raw_base`.
