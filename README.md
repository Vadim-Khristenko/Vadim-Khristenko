<!-- ════════════════════════════════════════════════════════════════════════        -->
<!--  This profile is ALIVE. The header, research card, flagship builds, stats,       -->
<!--  platform rollups, game shelf, weekly vibe and footer below are rendered as      -->
<!--  Tokyo-Night SVG "windows" by a custom Rust engine (engine/ → vai-profile).      -->
<!--  It aggregates GitHub + Codeberg + git.vai-rice.space, dedups mirrors, and       -->
<!--  injects everything between the markers. Edit the prose freely. NEVER touch      -->
<!--  the <!-- ENGINE:* --> <!-- markers — the engine owns whatever lives between     -->
<!--  each START/END pair.                                                            -->
<!-- ════════════════════════════════════════════════════════════════════════        -->

<!-- ENGINE:HEADER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/header.svg?v=bff8412a4b-69908" alt="Vadim Khristenko" width="100%"/>
</p>
<!-- ENGINE:HEADER:END -->

<p align="center">
  <a href="https://vai-rice.space"><img src="https://img.shields.io/badge/portfolio-vai--rice.space-dea584?style=for-the-badge&labelColor=1a1b26" alt="portfolio: vai-rice.space" /></a>
  <img src="https://komarev.com/ghpvc/?username=vadim-khristenko&label=PROFILE+VIEWS&color=dea584&style=for-the-badge" alt="profile views" />
  <img src="https://img.shields.io/badge/aliases-VAI__PROG%20·%20VAI%20·%20VOLT-bb9af7?style=for-the-badge&labelColor=1a1b26" alt="aliases" />
  <img src="https://img.shields.io/badge/primary-Rust-dea584?style=for-the-badge&logo=rust&logoColor=dea584&labelColor=1a1b26" alt="primary language: Rust" />
  <img src="https://img.shields.io/badge/open%20to-collab%20%26%20mischief-7dcfff?style=for-the-badge&labelColor=1a1b26" alt="open to collab" />
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider.svg?v=4" width="100%" alt="" />
</p>

## `whoami`

```rust
impl Developer for Vadim {
    const ALIASES: &[&str] = &["VAI_PROG", "VAI_Programmer", "VAI", "VOLT"]; // + a long tail

    fn role(&self)      -> &str { "backend & systems engineer · builder of fast, slightly unhinged things" }
    fn known_for(&self) -> Vec<&str> { vec!["The Wall Dev", "AmneziaWG Architect", "Aethelgard TQ-1.58"] }
    fn current(&self)   -> &str { "turning a billion on-the-fly ideas into running services" }
}
```

I write code that is **practical and a little playful**. Backend services, bots that genuinely help people, and performance-sensitive systems are my day-to-day. These days **Rust** is my primary — I pair its zero-cost control with high-level velocity (**Python / TypeScript**) and drop into **C++ / Cython** when the profiler demands it, gluing everything together through clean APIs and lightweight ML. When the clock frees up I write short books, novellas and fanfics — tiny stories squeezed into the gaps between commits.

**Why Rust first?** Because the borrow checker is the cheapest code reviewer I've ever hired: it charges nothing and never sleeps. Rust gives me C-grade performance with compiler-enforced memory safety, fearless concurrency, and binaries I can drop on any box in the fleet with zero runtime baggage — this very profile is rendered by a single Rust binary. **Where do C++ and Cython still win?** C++ when I need an existing ecosystem (game tooling, CUDA-adjacent code, decades of numeric libraries); Cython when a Python hot loop needs to become native *yesterday* without rewriting the world around it. Pick the tool by the constraint, not by the fashion.

> I've seen and done a surprising amount for how little life I've spent so far — from the **banal** to the **completely insane**. I run servers under just about any OS, and the ideas arrive faster than any single timeframe can hold them.

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_editor.svg?v=4" width="100%" alt="" />
</p>

## 🏗️ Flagship Builds

The engine looks these up **live** across every forge I publish on — stars and forks are summed over all mirrors, commits and language mix come from the canonical copy, and each row carries a 30-day health sparkline. Config, not screenshots: edit `config/flagship.toml` and the card follows.

<!-- ENGINE:FLAGSHIP:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/flagship.svg?v=a6be6501cb-69908" alt="Flagship projects with live stats" width="100%"/>
</p>
<!-- ENGINE:FLAGSHIP:END -->

<sub>🧱 <a href="https://the-wall.vai-rice.space">The Wall Dev</a> is built <b>for the <a href="https://twitch.tv/filian">Filian</a> community</b> — if the VTuber world can run <em>Hole in the Wall</em>, the dev side deserves its own Wall. 🛡️ <a href="https://architect.vai-rice.space">AmneziaWG Architect</a> generates DPI-evasion profiles for <a href="https://docs.amnezia.org">AmneziaWG</a> fully client-side: no server, no leaks. 🎀 And yes, the <a href="https://github.com/Vadim-Khristenko/HatsuneMikuEditorTheme-VSC">Hatsune Miku VS Code theme</a> exists because the editor should match the playlist.</sub>

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_circuit.svg?v=4" width="100%" alt="" />
</p>

## 🔬 Active Research

<!-- ENGINE:RESEARCH:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/research.svg?v=4c67ad0508-69908" alt="Active research: Aethelgard TQ-1.58" width="100%"/>
</p>
<!-- ENGINE:RESEARCH:END -->

I'm running a major ML effort — **Aethelgard TQ-1.58 HVRL**: an architecture for a **low-bit agentic reasoning model** with **verifiable training**, **hierarchical reinforcement learning**, and **domain specialization for software engineering**. Weights live in ternary space `{-1, 0, +1}` (≈1.58 bits per weight — hence the name) — the texture flickering across the cards above is that idea made visible.

**Why ternary?** A weight that can only be −1, 0 or +1 turns most multiplications into additions, and a zero into *nothing at all*. That means dramatically cheaper inference — the kind you can run on your own fleet instead of renting a datacenter — *if* you can train it without falling off the accuracy cliff. That "if" is the research. The learning card below tracks where the programme actually stands, phase by phase, straight from config — measured in shipped experiments, not vibes:

<!-- ENGINE:LEARNING:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/learning.svg?v=11490b3ca9-69908" alt="Now learning and building" width="100%"/>
</p>
<!-- ENGINE:LEARNING:END -->

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_pulse.svg?v=4" width="100%" alt="" />
</p>

## 🏆 Trophy Case

| | Achievement | Details |
|:--:|----|----|
| 🥈 | **PROD — International Industrial-Development Olympiad** | **2nd-degree prize-winner.** Ran the **Backend technologies track** inside the team and held a lot of the moving parts in my own hands. |
| 🎓 | **Algorithms & Data Structures @ T-Bank** | Graduate, **parallel B, 2025–2026.** The good kind of pain that rewires how you think about complexity. |
| 🏅 | **«Высшая проба» (HSE Olympiad)** | **Finalist across several profiles** — breadth on purpose, not by accident. |

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_editor.svg?v=4" width="100%" alt="" />
</p>

## 🛠️ Tech Arsenal

<details open>
<summary><b>Languages</b> — Rust first these days 🦀</summary>
<p>
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1a1b26?style=for-the-badge&logo=rust&logoColor=dea584" />
  <img alt="Python" src="https://img.shields.io/badge/Python-1a1b26?style=for-the-badge&logo=python&logoColor=7aa2f7" />
  <img alt="C++" src="https://img.shields.io/badge/C%2B%2B-1a1b26?style=for-the-badge&logo=c%2B%2B&logoColor=f7768e" />
  <img alt="Cython" src="https://img.shields.io/badge/Cython-1a1b26?style=for-the-badge&logo=cython&logoColor=fedf5b" />
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-1a1b26?style=for-the-badge&logo=typescript&logoColor=7dcfff" />
  <img alt="JavaScript" src="https://img.shields.io/badge/JavaScript-1a1b26?style=for-the-badge&logo=javascript&logoColor=e0af68" />
  <img alt="Kotlin" src="https://img.shields.io/badge/Kotlin-1a1b26?style=for-the-badge&logo=kotlin&logoColor=bb9af7" />
</p>
</details>

<details open>
<summary><b>Web & Frontend</b></summary>
<p>
  <img alt="Vue" src="https://img.shields.io/badge/Vue-1a1b26?style=for-the-badge&logo=vue.js&logoColor=9ece6a" />
  <img alt="Nuxt" src="https://img.shields.io/badge/Nuxt-1a1b26?style=for-the-badge&logo=nuxt&logoColor=9ece6a" />
  <img alt="Next.js" src="https://img.shields.io/badge/Next-1a1b26?style=for-the-badge&logo=next.js&logoColor=c0caf5" />
  <img alt="React" src="https://img.shields.io/badge/React-1a1b26?style=for-the-badge&logo=react&logoColor=7dcfff" />
</p>
</details>

<details open>
<summary><b>Data, Infra & DevOps</b> — Linux & Windows fleets, plus a self-hosted Forgejo</summary>
<p>
  <img alt="Postgres" src="https://img.shields.io/badge/Postgres-1a1b26?style=for-the-badge&logo=postgresql&logoColor=7aa2f7" />
  <img alt="Redis" src="https://img.shields.io/badge/Redis-1a1b26?style=for-the-badge&logo=redis&logoColor=f7768e" />
  <img alt="MongoDB" src="https://img.shields.io/badge/MongoDB-1a1b26?style=for-the-badge&logo=mongodb&logoColor=9ece6a" />
  <img alt="Cassandra" src="https://img.shields.io/badge/Cassandra-1a1b26?style=for-the-badge&logo=apache-cassandra&logoColor=7dcfff" />
  <img alt="RabbitMQ" src="https://img.shields.io/badge/RabbitMQ-1a1b26?style=for-the-badge&logo=rabbitmq&logoColor=ff9e64" />
  <img alt="Docker" src="https://img.shields.io/badge/Docker-1a1b26?style=for-the-badge&logo=docker&logoColor=7dcfff" />
  <img alt="Forgejo" src="https://img.shields.io/badge/Forgejo-1a1b26?style=for-the-badge&logo=forgejo&logoColor=ff9e64" />
  <img alt="GitHub Actions" src="https://img.shields.io/badge/GitHub%20Actions-1a1b26?style=for-the-badge&logo=githubactions&logoColor=7aa2f7" />
  <img alt="nginx" src="https://img.shields.io/badge/nginx-1a1b26?style=for-the-badge&logo=nginx&logoColor=9ece6a" />
  <img alt="Linux" src="https://img.shields.io/badge/Linux-1a1b26?style=for-the-badge&logo=linux&logoColor=e0af68" />
</p>
</details>

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_circuit.svg?v=4" width="100%" alt="" />
</p>

## 🧠 The AI Lab

I don't just *use* models — I've **shipped with the ecosystems behind them**: fine-tuning, prompt engineering, and efficient inference pipelines wired into real services. The current bench:

<p>
  <img alt="Fable 5" src="https://img.shields.io/badge/Fable%205-1a1b26?style=for-the-badge&logo=anthropic&logoColor=dea584" />
  <img alt="GPT 5.6 Sol" src="https://img.shields.io/badge/GPT%205.6%20Sol-1a1b26?style=for-the-badge&logo=openai&logoColor=c0caf5" />
  <img alt="Grok 4.5" src="https://img.shields.io/badge/Grok%204.5-1a1b26?style=for-the-badge&logo=x&logoColor=c0caf5" />
  <img alt="Kimi K3" src="https://img.shields.io/badge/Kimi%20K3-1a1b26?style=for-the-badge&logoColor=7aa2f7" />
  <img alt="Qwen 3.8 Max" src="https://img.shields.io/badge/Qwen%203.8%20Max-1a1b26?style=for-the-badge&logo=alibabacloud&logoColor=ff9e64" />
  <img alt="MiMo V2.5 Pro" src="https://img.shields.io/badge/MiMo%20V2.5%20Pro-1a1b26?style=for-the-badge&logo=xiaomi&logoColor=ff9e64" />
</p>

**Favourite companions right now** — `Fable 5` · `Qwen 3.8 Max` · `MiMo V2.5 Pro`.
> ⚠️ Note the word *companions*: I keep these as **light copilots / sparring partners**, not as an active vibe-coding autopilot. The thinking stays mine; they just keep good company.

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_wave.svg?v=4" width="100%" alt="" />
</p>

## 🎮 Off the Clock

<!-- ENGINE:BESTGAME:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/bestgame.svg?v=904c3cd552-69908" alt="Best game" width="100%"/>
</p>
<!-- ENGINE:BESTGAME:END -->

<!-- ENGINE:GAMES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/games.svg?v=e90bedcb95-69908" alt="Now playing" width="100%"/>
</p>
<!-- ENGINE:GAMES:END -->

**🎧 On heavy loop:**
`Hatsune Miku` · `Deco*27` · `Chiru-San` · `DraGonis` · `Kanro` · `Exyl` · `NoAki`

<!-- ENGINE:VIBE:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/vibe.svg?v=6dc70ffda5-69908" alt="Current vibe" width="100%"/>
</p>
<!-- ENGINE:VIBE:END -->

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_pulse.svg?v=4" width="100%" alt="" />
</p>

## 📮 Reach Me — the Inbox Router

Primary address: **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)** — I read mail *constantly*. To land in the right lane, use the matching alias (`+tag` sub-addressing works too):

| When you're writing about… | Send to |
|---|---|
| 🤝 Partnerships, collaboration, any business | **[business@vai-rice.space](mailto:business@vai-rice.space)** |
| 🛡️ AmneziaWG Architect | **[architect@vai-rice.space](mailto:architect@vai-rice.space)** |
| 🧱 The Wall Dev / anything from the Filian community | **[the-wall-dev@vai-rice.space](mailto:the-wall-dev@vai-rice.space)** |
| 📱 A specific social network | **vadim+{social_network}@vai-rice.space** *(e.g. `vadim+twitch@…`)* |
| ✉️ Everything else | **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)** |

<p>
  <a href="https://vai-rice.space"><img src="https://img.shields.io/badge/Portfolio-1a1b26?style=for-the-badge&logo=firefoxbrowser&logoColor=dea584" alt="Portfolio" /></a>
  <a href="https://github.com/Vadim-Khristenko"><img src="https://img.shields.io/badge/GitHub-1a1b26?style=for-the-badge&logo=github&logoColor=c0caf5" alt="GitHub" /></a>
  <a href="https://git.vai-rice.space/VAI_PROG"><img src="https://img.shields.io/badge/VAI%20Git-1a1b26?style=for-the-badge&logo=forgejo&logoColor=dea584" alt="VAI Git (Forgejo)" /></a>
  <a href="https://codeberg.org/VAI_PROG"><img src="https://img.shields.io/badge/Codeberg-1a1b26?style=for-the-badge&logo=codeberg&logoColor=2185d0" alt="Codeberg" /></a>
  <a href="https://twitch.tv/VAI_PROG"><img src="https://img.shields.io/badge/Twitch-1a1b26?style=for-the-badge&logo=twitch&logoColor=bb9af7" alt="Twitch" /></a>
  <a href="https://x.com/VAI_PROG"><img src="https://img.shields.io/badge/X-1a1b26?style=for-the-badge&logo=x&logoColor=c0caf5" alt="X" /></a>
  <a href="https://youtube.com/@VAI_PROG"><img src="https://img.shields.io/badge/YouTube-1a1b26?style=for-the-badge&logo=youtube&logoColor=f7768e" alt="YouTube" /></a>
  <a href="https://t.me/vscreator_life"><img src="https://img.shields.io/badge/Telegram-1a1b26?style=for-the-badge&logo=telegram&logoColor=7dcfff" alt="Telegram" /></a>
  <a href="https://patreon.com/VAI_PROG"><img src="https://img.shields.io/badge/Patreon-1a1b26?style=for-the-badge&logo=patreon&logoColor=ff9e64" alt="Patreon" /></a>
  <a href="https://codeforces.com/profile/VAI_Programmer"><img src="https://img.shields.io/badge/Codeforces-1a1b26?style=for-the-badge&logo=codeforces&logoColor=7aa2f7" alt="Codeforces" /></a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider.svg?v=4" width="100%" alt="" />
</p>

## 📊 Live Telemetry

One number can lie; a fleet of them lies less. Everything below is pulled fresh by the engine from **three forges at once** — GitHub, Codeberg and my own `git.vai-rice.space` — with mirrors deduplicated so the code is counted once while every platform's stars still count. That's also why the numbers here can be *bigger and more honest* than any single-site widget.

<!-- ENGINE:STATS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/dashboard.svg?v=d9639e11e1-69908" alt="Live telemetry: stats, activity, languages" width="100%"/>
</p>
<!-- ENGINE:STATS:END -->

<!-- ENGINE:PLATFORMS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platforms_all.svg?v=4a53468d1c-69908" alt="All platforms combined" width="100%"/>
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platform_vai-git.svg?v=361d9a0249-69908" alt="Platform stats: vai-git" width="100%"/>
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platform_github.svg?v=365754c1a5-69908" alt="Platform stats: github" width="100%"/>
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platform_codeberg.svg?v=4f9d77e1b9-69908" alt="Platform stats: codeberg" width="100%"/>
</p>
<!-- ENGINE:PLATFORMS:END -->

<!-- ENGINE:FOOTER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/footer.svg?v=96ab180272-69908" alt="Generated by the VAI Profile Engine" width="100%"/>
</p>
<!-- ENGINE:FOOTER:END -->

<p align="center">
  <img src="https://hit.yhype.me/github/profile?account_id=124452155" alt="Hit Analytics" />
</p>
