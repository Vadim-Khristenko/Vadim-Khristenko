# VAI Profile Engine v3

A single Rust binary (`vai-profile`) that keeps the profile README alive:
it pulls live stats from **multiple forges at once** (GitHub · Codeberg ·
self-hosted Forgejo at `git.vai-rice.space`), aggregates them with
mirror-aware dedup, renders every Tokyo-Night SVG card, and injects
cache-busted image rows between the `<!-- ENGINE:* -->` markers in
`README.md`. Prose outside the markers is never touched.

```
cargo build --release          # engine/target/release/vai-profile
```

## Commands

| Command | What it does |
|---|---|
| `vai-profile build` | Collect → render all cards → refresh README markers |
| `vai-profile build --only header,vibe` | Render a subset (skips the README rewrite) |
| `vai-profile build --no-readme` | Render everything, leave README alone |
| `vai-profile build --fixtures` | **Offline dry-run** from `engine/fixtures/*.json` — no network, no tokens |
| `vai-profile preview [--no-open]` | Build, then open `tmp_prev/preview.html` with all cards stacked |
| `vai-profile rebuild [--games]` | Fetch best-game art (and optionally shelf covers), then build |
| `vai-profile art [--only key,key]` | (Re)download the game-shelf covers from SteamGridDB |
| `vai-profile bestgame` | (Re)download the best-game cover + character art |

Art commands take a SteamGridDB key via `--key` or `SGDB_KEY` /
`STEAMGRIDDB_KEY`. `VAI_FIXTURES=1` is equivalent to `--fixtures`.

## Configuration (`config/`, all TOML)

* **`profile.toml`** — editorial content: name, aliases, quotes, research
  programme (with `phase` + `progress` for the learning card), games,
  composers, socials, the `[ai]` companion bench, `[learning]` topics and the
  full `[best_game]` spotlight. Power-ups:
  * `[[best_game.extra]]` — free-form labelled stats on the spotlight card
    (`label` + `value`, or numeric `current`/`max` for a micro progress bar).
  * `[cards.<name>]` — per-card tweaks: `enabled = false` skips a card,
    `accent = "#…"` overrides its chrome colour. Existing configs need no
    changes; every key is optional.
  * `[lastfm]` — `username` for live music in the vibe card. The API key is
    ONLY read from the environment (`LASTFM_API_KEY`, name configurable via
    `api_key_env`); without it the static composers list is used.
* **`providers.toml`** — the platform list. `kind = "github"` talks to
  api.github.com (REST + GraphQL + tokenless calendar scrape);
  `kind = "forgejo"` talks to any Gitea-compatible `/api/v1` host, so
  Codeberg and `git.vai-rice.space` are the same code with different
  `base_url`s. **Adding a platform = one more `[[provider]]` entry.**
  At most one entry may set `primary = true` (the source of truth).
* **`flagship.toml`** — the coolest-projects card. Each `[[project]]` is
  looked up live across every provider; `prefer` picks the headline copy.
* **`stats.toml`** *(optional)* — rollup hygiene. `exclude_repos` drops a
  repository from every rollup metric (languages, LOC, stars, forks, open
  items, repo count, most-active), matched with mirror normalization so one
  entry covers all platforms — seeded with the profile repo itself.
  `exclude_commit_authors` filters commit series by author email, name or
  login (case-insensitive) so CI auto-commits (`VIA GIT`,
  `github-actions[bot]`) never count as activity. Delete the file to disable
  all exclusions.

### Aggregation rules

Repos sharing a normalized name (lowercase, `.git` stripped) across
platforms are **mirrors**:

* **code metrics counted once** — language bytes → LOC and the repo count use
  the copy with the largest byte total;
* **social metrics summed everywhere** — stars, forks, watchers, open issues
  and open PRs from every platform copy all count.

Per-platform rollups are kept alongside the combined aggregate; the platform
cards show both.

### Graceful degradation

A missing token → tokenless public reads. An unreachable host → logged and
skipped, never fatal. No tokens at all still produces a full README from
public data. One broken card never stops the others (each card renders in
isolation and must parse as valid XML before it is written).

## CI / automation

**Primary:** `.forgejo/workflows/profile-engine.yml` on `git.vai-rice.space`
— every 2 days at 06:00 UTC, on manual dispatch, and on pushes touching
`engine/**` or `config/**`. It builds with a cached `cargo build --release`,
runs the engine with read-only tokens, commits as
`VIA GIT <actions@git.vai-rice.space>` with `[skip ci]`, then mirror-pushes
to GitHub and Codeberg. **Each push token is only used against its own
host.**

**Fallback:** `.github/workflows/profile-engine.yml` is
`workflow_dispatch`-only; it regenerates from GitHub if the Forgejo runner is
down and pushes only to the GitHub repo.

### Secrets

| Secret | Where | Scope | Purpose |
|---|---|---|---|
| `VAI_GIT_TOKEN` | Forgejo | read-only API | vai-git provider reads |
| `GH_TOKEN` | Forgejo | `read:user`, public repo read | GitHub provider reads (GraphQL commit windows, private contribution counts) |
| `CODEBERG_TOKEN` | Forgejo | read-only API | Codeberg provider reads |
| `GITHUB_MIRROR_TOKEN` | Forgejo | push to the GitHub profile repo only | mirror push → github.com |
| `CODEBERG_MIRROR_TOKEN` | Forgejo | push to the Codeberg mirror only | mirror push → codeberg.org |
| `PROFILE_TOKEN` | GitHub (optional) | `read:user` | receiver-run private contribution counts |
| `STEAMGRIDDB_API_KEY` | GitHub (optional) | SteamGridDB API | game-art refresh workflow |
| `LASTFM_API_KEY` | Forgejo + GitHub (optional) | Last.fm API (read) | live now-playing / top artists in the vibe card |

Repo variables `GITHUB_MIRROR_REPO` / `CODEBERG_MIRROR_REPO` override the
default mirror paths.

## Development

```
cargo test                       # offline: fixtures under engine/fixtures/
cargo run -- build --fixtures    # full render + README injection, no network
node fixtures/gen_fixtures.js    # regenerate the recorded fixture data
```

Layout: `src/providers/` (trait + GitHub/Forgejo/fixture impls +
aggregator), `src/cards/` (one module per card), `src/theme.rs` (palette,
chrome, contrast tiers), `src/svg.rs` (escaping, XML validation and the
`fit_text`/`text_width_px` metrics every card uses to keep text inside its
column), `src/readme.rs` (marker injection + `?v=` tokens: sha1 content hash
+ 2-day salt), `src/lastfm.rs` (Last.fm REST), `src/art.rs` (SteamGridDB +
crop/recompress via the image crate).

Text overflow is a tested invariant: `cards/overflow_tests.rs` renders every
card from fixtures (plus a hostile-config variant with absurdly long
strings), parses each `<text>` element and asserts its estimated extents stay
inside the card. The divider family renders five distinct 1000×64 scenes
(`divider`, `divider_wave`, `divider_circuit`, `divider_pulse`,
`divider_editor`) that the README rotates between sections.
