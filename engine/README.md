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
| `vai-profile build --pin-game miside` | Force a game into the rotation slot (see below) |
| `vai-profile preview [--no-open]` | Build, then open `tmp_prev/preview.html` with all cards stacked |
| `vai-profile rebuild [--games]` | Fetch best-game art (and optionally shelf covers), then build |
| `vai-profile art [--only key,key]` | (Re)download the game-shelf covers from SteamGridDB |
| `vai-profile bestgame` | (Re)download the best-game cover + character art |

Art commands take a SteamGridDB key via `--key` or the environment —
`SGDB_KEY`, `STEAMGRIDDB_KEY` or `STEAMGRIDDB_API_KEY` (the historical CI
secret name), first non-empty wins; the engine logs which source was used as
set/not-set, never the value. `VAI_FIXTURES=1` is equivalent to `--fixtures`.

### Pinning the game rotation

The vibe card's PLAYING slot and the games shelf's IN ROTATION hero normally
rotate together on a 2-day seed. `--pin-game <key>` (on `build`, `rebuild`
and `preview`) — or the `PIN_GAME` environment variable, flag wins — forces a
configured game key (e.g. `miside`, `nikke`) into that slot; both cards stay
in agreement. An unknown key logs a warning and falls back to the automatic
rotation; unset = unchanged behaviour. Combine with `--only vibe,games` to
regenerate just the two affected cards.

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
— **daily at 06:00 UTC**, on manual dispatch, and on pushes touching
`engine/**` or `config/**`. It runs **only on the self-hosted runner**
(label `docker` by default; override with the `RUNNER_LABEL` repo variable),
so automatic regeneration happens exclusively on the user's own
infrastructure. The compiled release binary is **cached under a hash of the
engine sources** (`engine/**/*.rs` + `Cargo.toml` + `Cargo.lock`), so daily
stat runs skip compilation entirely unless the code changed. The engine runs
with read-only tokens, commits as `VIA GIT <actions@git.vai-rice.space>`
with `[skip ci]`, then mirror-pushes to GitHub and Codeberg. **Each push
token is only used against its own host.**

Manual dispatch inputs:

* `only` — comma card list (→ `vai-profile build --only …`) for single-block
  regeneration; leaves the README rewrite out, per the CLI contract;
* `pin_game` — game key (→ `PIN_GAME`) to force the rotation slot;
* `refetch_art` — re-fetch game art from SteamGridDB before rendering
  (needs the `STEAMGRIDDB_API_KEY` secret).

Every run starts with a **preflight inventory** that lists each secret /
variable by name with a set/not-set marker (values are never printed) and
ends with a **run summary table** (cache hit, providers fetched, cards
rendered, commit, mirror pushes, notifications) in the step summary.

After a successful regenerate+push the workflow can send an **optional
notification** — Telegram and/or a generic JSON webhook. Both are entirely
optional and skipped cleanly when their secrets are absent; messages carry
only a short summary (providers, card count, commit sha), never secrets.

**Fallback:** `.github/workflows/profile-engine.yml` is
`workflow_dispatch`-only **by design — no schedule**; it regenerates from
GitHub if the Forgejo runner is down and pushes only to the GitHub repo. It
supports the same dispatch inputs, preflight inventory, binary cache,
summary and optional notifications.

### Secrets & repo variables — the complete list

Set **Forgejo** entries on git.vai-rice.space (repo → Settings → Actions),
**GitHub** entries on the github.com profile repo. Everything marked
*optional* degrades gracefully when absent.

| Name | Where | Scope | Required? | Purpose |
|---|---|---|---|---|
| `VAI_GIT_TOKEN` | Forgejo secret (+ GitHub for fallback runs) | read-only API | recommended | vai-git provider reads (private counts) |
| `GH_TOKEN` | Forgejo secret | read-only: `read:user` + public repo | recommended | GitHub provider reads (GraphQL commit windows, private contribution counts) |
| `CODEBERG_TOKEN` | Forgejo secret (+ GitHub for fallback runs) | read-only API | recommended | Codeberg provider reads |
| `LASTFM_API_KEY` | Forgejo + GitHub secret | Last.fm API (read) | optional | live now-playing / top artists in the vibe card |
| `STEAMGRIDDB_API_KEY` | Forgejo + GitHub secret | SteamGridDB API (read) | optional | game-art refetch (`refetch_art` input, `art`/`bestgame`/`rebuild`); also read as `SGDB_KEY` / `STEAMGRIDDB_KEY` |
| `GITHUB_MIRROR_TOKEN` | Forgejo secret | push to the GitHub profile repo ONLY | required for mirroring | mirror push → github.com (never used against another host) |
| `CODEBERG_MIRROR_TOKEN` | Forgejo secret | push to the Codeberg mirror ONLY | required for mirroring | mirror push → codeberg.org (never used against another host) |
| `PROFILE_TOKEN` | GitHub secret | read-only: `read:user` | optional | fallback-run private contribution counts (else `GITHUB_TOKEN`) |
| `TELEGRAM_BOT_TOKEN` | Forgejo + GitHub secret | Telegram bot sendMessage | optional | regeneration notification via Telegram |
| `TELEGRAM_CHAT_ID` | Forgejo + GitHub secret | chat/channel id | optional | where the Telegram notification goes |
| `NOTIFY_WEBHOOK_URL` | Forgejo + GitHub secret | HTTPS endpoint (POST JSON) | optional | generic regeneration webhook |
| `RUNNER_LABEL` | Forgejo repo **variable** | — | optional (default `docker`) | which self-hosted runner label the job targets |
| `GITHUB_MIRROR_REPO` | Forgejo repo **variable** | — | optional (default `Vadim-Khristenko/Vadim-Khristenko`) | GitHub mirror path |
| `CODEBERG_MIRROR_REPO` | Forgejo repo **variable** | — | optional (default `VAI_PROG/Vadim-Khristenko`) | Codeberg mirror path |

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
