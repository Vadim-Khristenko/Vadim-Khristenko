<!-- ════════════════════════════════════════════════════════════════════════ -->
<!--  This profile is ALIVE. Every card below is rendered by a custom Rust     -->
<!--  engine (engine/, the vai-profile binary) in the Hatsune Miku Dark       -->
<!--  palette, with real fonts drawn as paths. It reads GitHub, Codeberg and   -->
<!--  git.vai-rice.space and fills the ENGINE markers in README.md and         -->
<!--  README.ru.md. Edit the prose freely; never touch the markers.            -->
<!-- ════════════════════════════════════════════════════════════════════════ -->

<!-- ENGINE:BADGES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/badges.svg?v=33a8be207f-69957" alt="Badges" width="100%"/>
</p>
<!-- ENGINE:BADGES:END -->

<!-- ENGINE:HEADER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/header.svg?v=0d2745f78a-69957" alt="Vadim Khristenko" width="100%"/>
</p>
<!-- ENGINE:HEADER:END -->

<p align="center">
  <b>English</b> · <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <a href="#projects">Projects</a> ·
  <a href="#research">Research</a> ·
  <a href="#snackers">Snackers</a> ·
  <a href="#off-the-clock">Off the clock</a> ·
  <a href="#telemetry">Telemetry</a> ·
  <a href="#infra">Infra</a> ·
  <a href="#reach-me">Reach me</a>
</p>

<p align="center">
  <img src="https://komarev.com/ghpvc/?username=vadim-khristenko&label=PROFILE+VIEWS&color=39c5bb&style=flat-square" alt="profile views" />
</p>

## `whoami`

```rust
impl Developer for Vadim {
    const ALIASES: &[&str] = &["VAI_PROG", "VAI_Programmer", "VAI", "VOLT"];

    fn role(&self)      -> &str { "backend & systems engineer, builder of fast, slightly unhinged things" }
    fn known_for(&self) -> Vec<&str> { vec!["Hatsune Miku editor themes", "Any Tech Architect", "A CAT", "FilianWIKI"] }
    fn current(&self)   -> &str { "turning a billion on-the-fly ideas into running services" }
}
```

I write code that is **practical and a little playful**: backend services, bots that genuinely help people, and systems where performance matters. **Rust** is home; **Python** and **TypeScript** keep up with my thinking, and **C++ / Cython** come out when the profiler insists. Between commits I write short books, novellas and fanfics.

**Why Rust first?** The borrow checker is the cheapest code reviewer I have ever hired: it costs nothing and never sleeps. C-grade speed, memory safety the compiler enforces, fearless concurrency and one binary for any box in the fleet. This profile is drawn by one.

<!-- ENGINE:DIV_PROJECTS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_projects.svg?v=b3a19d026f-69957" alt="PROJECTS" width="100%"/>
</p>
<!-- ENGINE:DIV_PROJECTS:END -->

<a id="projects"></a>

## 🏗️ Projects

### 💙 Top repo: Hatsune Miku editor themes

**[HatsuneMikuEditorTheme](https://github.com/Vadim-Khristenko/HatsuneMikuEditorTheme)**: one palette contract, four looks, six editors (VS Code, Zed, JetBrains, Neovim, Sublime Text, Notepad++). **And now the browser:** a Miku new tab for Chromium, Firefox, Zen and Firefox for Android, with four layouts, widgets and Helium style !bangs, plus window colors for Chromium and a Zen mod. This whole profile is painted in its Miku Dark.

<!-- ENGINE:THEMES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_miku_themes.svg?v=7032f183be-69957" alt="Hatsune Miku editor themes" width="100%"/>
</p>
<!-- ENGINE:THEMES:END -->

### 🛡️ Any Tech Architect

**[architect.vai-rice.space](https://architect.vai-rice.space)**: obfuscation configs with every number explained. Two engines, **AmneziaWG 3.1** and **XRay REALITY**, and everything is computed in your browser, so keys and configs never leave it.

<!-- ENGINE:ARCHITECT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_architect.svg?v=b162f6ac8a-69957" alt="Any Tech Architect" width="100%"/>
</p>
<!-- ENGINE:ARCHITECT:END -->

### 🐈‍⬛ A CAT

**[awg-containers-and-tools](https://github.com/Vadim-Khristenko/awg-containers-and-tools)**, Architect's companion: self-hosted AmneziaWG for every generation from 1.0 to 3.1 as small images, a tunnel-only resolver, and `awg-tool`, a Rust CLI that installs servers over SSH, issues peers and rotates obfuscation.

<!-- ENGINE:ACAT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_acat.svg?v=d70722236f-69957" alt="A CAT: awg-containers-and-tools" width="100%"/>
</p>
<!-- ENGINE:ACAT:END -->

### 📖 NotAnOtherVTUBERwiki

**[NotAnOtherVTUBERwiki](https://github.com/Vadim-Khristenko/NotAnOtherVTUBERwiki)**: my wiki engine for fan communities. Rust and Axum, Markdown first, fast on a Raspberry Pi, works with JavaScript off. It runs **[FilianWIKI](https://filian.wiki)**.

<!-- ENGINE:WIKI:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_wiki.svg?v=140e8e59e0-69957" alt="NotAnOtherVTUBERwiki and FilianWIKI" width="100%"/>
</p>
<!-- ENGINE:WIKI:END -->

### 🧱 The Wall Dev

**[the-wall.vai-rice.space](https://the-wall.vai-rice.space)**: a Win95-styled extension for The Wall, the Snackers' 6000 × 4200 pixel canvas. Template overlay, clans, shared templates and bot alerts.

<!-- ENGINE:THEWALL:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_thewall.svg?v=9a032c1ab4-69957" alt="The Wall Dev" width="100%"/>
</p>
<!-- ENGINE:THEWALL:END -->

### 📜 Kumir 3

**[kumir3-core](https://github.com/Vadim-Khristenko/kumir3-core)**: the classroom language Kumir, rebuilt in Rust. Interpreter, TUI and a compiler to Rust and WASM, and yes, the Robot still paints corridors.

<!-- ENGINE:KUMIR:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_kumir.svg?v=31716f9fd2-69957" alt="Kumir 3" width="100%"/>
</p>
<!-- ENGINE:KUMIR:END -->

<!-- ENGINE:DIV_RESEARCH:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_research.svg?v=097b911d31-69957" alt="RESEARCH" width="100%"/>
</p>
<!-- ENGINE:DIV_RESEARCH:END -->

<a id="research"></a>

## 🔬 Research

<!-- ENGINE:RESEARCH:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/research.svg?v=5a453e6f3d-69957" alt="Active research: Aethelgard TQ-1.58" width="100%"/>
</p>
<!-- ENGINE:RESEARCH:END -->

**Aethelgard TQ-1.58 HVRL** is an architecture for a **low-bit agentic reasoning model** with **verifiable training**, **hierarchical reinforcement learning** and a **specialization in software engineering**. Its weights live in ternary space `{-1, 0, +1}`, about 1.58 bits each, hence the name.

**Why ternary?** A weight that can only be −1, 0 or +1 turns most multiplications into additions and a zero into nothing at all. Inference gets cheap enough for my own fleet instead of a rented datacenter, *if* the model trains without falling off the accuracy cliff. That "if" is the research.

<!-- ENGINE:DIV_SNACKERS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_snackers.svg?v=64285e0fae-69957" alt="SNACKERS" width="100%"/>
</p>
<!-- ENGINE:DIV_SNACKERS:END -->

<a id="snackers"></a>

## 🍿 Snackers

<!-- ENGINE:SNACKERS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/fandom_snackers.svg?v=6c579f7595-69957" alt="Snackers" width="100%"/>
</p>
<!-- ENGINE:SNACKERS:END -->

I am part of the **Snackers**, the community around the VTuber **[Filian](https://twitch.tv/filian)**, and I build things for it:

- 📖 **[FilianWIKI](https://filian.wiki)**: the wiki for **Snackers HQ+**, **Snackers RU** and every snacker in the world, on my own engine. Open beta: sign in with Discord or Telegram and start editing.
- 💗 **[Filian.ru](https://filian.ru)**: the home of **Snackers RU**, the Russian-speaking community.
- 🧱 **[The Wall Dev](https://the-wall.vai-rice.space)**: tools for The Wall, our shared pixel canvas.

🎁 **Snackers RU gifted me a Hatsune Miku figure.** Thank you, Snackers RU! 💙

<sub>All of the above are fan projects: unofficial, and not part of Filian's ARG.</sub>

<!-- ENGINE:DIV_TAILS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_tails.svg?v=87d308d876-69957" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_TAILS:END -->

## 🏆 Trophy case

| | Achievement | Details |
|:--:|----|----|
| 🥈 | **PROD, International Industrial-Development Olympiad** | **2nd-degree prize-winner.** Ran the **backend track** inside the team and held a lot of the moving parts myself. |
| 🎓 | **Algorithms & Data Structures @ T-Bank** | Graduate, **parallel B, 2025-2026.** The good kind of pain that rewires how you think about complexity. |
| 🏅 | **«Высшая проба» (HSE Olympiad)** | **Finalist in several profiles.** Breadth on purpose, not by accident. |

## 🛠️ Tech arsenal

<details>
<summary><b>Languages</b>: Rust first 🦀</summary>
<p>
  <img alt="Rust" src="https://img.shields.io/badge/Rust-0d0e1a?style=for-the-badge&logo=rust&logoColor=dea584" />
  <img alt="Python" src="https://img.shields.io/badge/Python-0d0e1a?style=for-the-badge&logo=python&logoColor=7aa2f7" />
  <img alt="C++" src="https://img.shields.io/badge/C%2B%2B-0d0e1a?style=for-the-badge&logo=c%2B%2B&logoColor=f7768e" />
  <img alt="Cython" src="https://img.shields.io/badge/Cython-0d0e1a?style=for-the-badge&logo=cython&logoColor=fedf5b" />
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-0d0e1a?style=for-the-badge&logo=typescript&logoColor=7dcfff" />
  <img alt="JavaScript" src="https://img.shields.io/badge/JavaScript-0d0e1a?style=for-the-badge&logo=javascript&logoColor=e0af68" />
  <img alt="Kotlin" src="https://img.shields.io/badge/Kotlin-0d0e1a?style=for-the-badge&logo=kotlin&logoColor=bb9af7" />
</p>
</details>

<details>
<summary><b>Web & frontend</b></summary>
<p>
  <img alt="Vue" src="https://img.shields.io/badge/Vue-0d0e1a?style=for-the-badge&logo=vue.js&logoColor=9ece6a" />
  <img alt="Nuxt" src="https://img.shields.io/badge/Nuxt-0d0e1a?style=for-the-badge&logo=nuxt&logoColor=9ece6a" />
  <img alt="Next.js" src="https://img.shields.io/badge/Next-0d0e1a?style=for-the-badge&logo=next.js&logoColor=c0caf5" />
  <img alt="React" src="https://img.shields.io/badge/React-0d0e1a?style=for-the-badge&logo=react&logoColor=7dcfff" />
</p>
</details>

<details>
<summary><b>Data, infra & DevOps</b>: Linux and Windows fleets, plus a self-hosted Forgejo</summary>
<p>
  <img alt="Postgres" src="https://img.shields.io/badge/Postgres-0d0e1a?style=for-the-badge&logo=postgresql&logoColor=7aa2f7" />
  <img alt="Redis" src="https://img.shields.io/badge/Redis-0d0e1a?style=for-the-badge&logo=redis&logoColor=f7768e" />
  <img alt="MongoDB" src="https://img.shields.io/badge/MongoDB-0d0e1a?style=for-the-badge&logo=mongodb&logoColor=9ece6a" />
  <img alt="Cassandra" src="https://img.shields.io/badge/Cassandra-0d0e1a?style=for-the-badge&logo=apache-cassandra&logoColor=7dcfff" />
  <img alt="RabbitMQ" src="https://img.shields.io/badge/RabbitMQ-0d0e1a?style=for-the-badge&logo=rabbitmq&logoColor=ff9e64" />
  <img alt="Docker" src="https://img.shields.io/badge/Docker-0d0e1a?style=for-the-badge&logo=docker&logoColor=7dcfff" />
  <img alt="Forgejo" src="https://img.shields.io/badge/Forgejo-0d0e1a?style=for-the-badge&logo=forgejo&logoColor=ff9e64" />
  <img alt="GitHub Actions" src="https://img.shields.io/badge/GitHub%20Actions-0d0e1a?style=for-the-badge&logo=githubactions&logoColor=7aa2f7" />
  <img alt="nginx" src="https://img.shields.io/badge/nginx-0d0e1a?style=for-the-badge&logo=nginx&logoColor=9ece6a" />
  <img alt="Linux" src="https://img.shields.io/badge/Linux-0d0e1a?style=for-the-badge&logo=linux&logoColor=e0af68" />
</p>
</details>

<details>
<summary><b>The AI lab</b>: companions, not an autopilot</summary>
<p>
  <img alt="Fable 5" src="https://img.shields.io/badge/Fable%205-0d0e1a?style=for-the-badge&logo=anthropic&logoColor=dea584" />
  <img alt="GPT 5.6 Sol" src="https://img.shields.io/badge/GPT%205.6%20Sol-0d0e1a?style=for-the-badge&logo=openai&logoColor=c0caf5" />
  <img alt="Grok 4.5" src="https://img.shields.io/badge/Grok%204.5-0d0e1a?style=for-the-badge&logo=x&logoColor=c0caf5" />
  <img alt="Kimi K3" src="https://img.shields.io/badge/Kimi%20K3-0d0e1a?style=for-the-badge&logoColor=7aa2f7" />
  <img alt="Qwen 3.8 Max" src="https://img.shields.io/badge/Qwen%203.8%20Max-0d0e1a?style=for-the-badge&logo=alibabacloud&logoColor=ff9e64" />
  <img alt="MiMo V2.5 Pro" src="https://img.shields.io/badge/MiMo%20V2.5%20Pro-0d0e1a?style=for-the-badge&logo=xiaomi&logoColor=ff9e64" />
</p>

I have shipped with the ecosystems behind these: fine-tuning, prompt engineering and efficient inference wired into real services. Favourite companions right now: `Fable 5` · `Qwen 3.8 Max` · `MiMo V2.5 Pro`. I keep them as **light copilots and sparring partners**, not as a vibe-coding autopilot. The thinking stays mine.
</details>

<!-- ENGINE:DIV_OFFCLOCK:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_offclock.svg?v=f692caac08-69957" alt="OFF THE CLOCK" width="100%"/>
</p>
<!-- ENGINE:DIV_OFFCLOCK:END -->

<a id="off-the-clock"></a>

## 🎮 Off the clock

<!-- ENGINE:BESTGAME:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/bestgame.svg?v=037f9a3677-69957" alt="Best game: NIKKE" width="100%"/>
</p>
<!-- ENGINE:BESTGAME:END -->

<p align="center">
  <a href="https://www.blablalink.com/user?openid=MjkwODAtNjA0NTA5MTUyNjQxNzk5OTA3MA=="><img src="https://img.shields.io/badge/NIKKE-commander%20VAI%20on%20Blablalink-1e9bff?style=for-the-badge&labelColor=060912" alt="NIKKE profile on Blablalink" /></a>
</p>

<!-- ENGINE:GAMES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/games.svg?v=abad112a81-69957" alt="Now playing" width="100%"/>
</p>
<!-- ENGINE:GAMES:END -->

<!-- ENGINE:DIV_WAVE:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_wave.svg?v=6370f9cf59-69957" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_WAVE:END -->

**🎧 On heavy loop:** `Hatsune Miku` · `Deco*27` · `Chiru-San` · `DraGonis` · `Kanro` · `Exyl` · `NoAki`

<!-- ENGINE:MUSIC:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/music.svg?v=58d944212e-69957" alt="Music via Last.fm" width="100%"/>
</p>
<!-- ENGINE:MUSIC:END -->

<!-- ENGINE:DIV_TELEMETRY:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_telemetry.svg?v=0208dca9aa-69957" alt="TELEMETRY" width="100%"/>
</p>
<!-- ENGINE:DIV_TELEMETRY:END -->

<a id="telemetry"></a>

## 📊 Telemetry

Everything here comes fresh from **three forges at once**: GitHub, Codeberg and my own `git.vai-rice.space`. Mirrors are deduplicated, so code is counted once while every platform's stars still count.

<!-- ENGINE:STATS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/dashboard.svg?v=ba20f7eda2-69957" alt="Live telemetry" width="100%"/>
</p>
<!-- ENGINE:STATS:END -->

<!-- ENGINE:PLATFORMS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platforms.svg?v=cec07379a2-69957" alt="Platforms" width="100%"/>
</p>
<!-- ENGINE:PLATFORMS:END -->

<!-- ENGINE:DIV_INFRA:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_infra.svg?v=1c4e94f0bd-69957" alt="INFRA" width="100%"/>
</p>
<!-- ENGINE:DIV_INFRA:END -->

<a id="infra"></a>

## 🏠 Infrastructure

Most of what I run lives at home, in a **homelab of laptops**: **two of them work as servers** on **Arch** and **NixOS**, and my **main laptop** dual boots **Windows 11** and **Arch Linux**. Laptops make a surprisingly good homelab: they sip power, carry their own UPS (the battery) and come with a screen and a keyboard for the day SSH says no.

Out in the world, **VPS on Arch, NixOS, Ubuntu and Debian** hold the edge.

<!-- ENGINE:HOMELAB:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/homelab.svg?v=71eafad201-69957" alt="Homelab: laptops at home, VPS out in the world" width="100%"/>
</p>
<!-- ENGINE:HOMELAB:END -->

<!-- ENGINE:DIV_CONTACT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_contact.svg?v=334d809edc-69957" alt="REACH ME" width="100%"/>
</p>
<!-- ENGINE:DIV_CONTACT:END -->

<a id="reach-me"></a>

## 📮 Reach me

Primary address: **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)**. I read mail constantly. To land in the right lane, use the matching alias (`+tag` sub-addressing works too):

| When you are writing about… | Send to |
|---|---|
| 🤝 Partnerships, collaboration, any business | **[business@vai-rice.space](mailto:business@vai-rice.space)** |
| 🛡️ Any Tech Architect or A CAT | **[architect@vai-rice.space](mailto:architect@vai-rice.space)** |
| 🧱 The Wall Dev | **[the-wall-dev@vai-rice.space](mailto:the-wall-dev@vai-rice.space)** |
| 📖 FilianWIKI, snackers.wiki or the wiki engine: articles, edits, bugs | **[vadim@filian.wiki](mailto:vadim@filian.wiki)** · **[vadim@snackers.wiki](mailto:vadim@snackers.wiki)** |
| 🇷🇺 filian.ru and Snackers RU | **[vadim@filian.ru](mailto:vadim@filian.ru)** |
| 📱 A specific social network | **vadim+{social_network}@vai-rice.space** *(e.g. `vadim+twitch@…`)* |
| ✉️ Everything else | **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)** |

The fan addresses are the same ones printed on [filian.wiki](https://filian.wiki), [snackers.wiki](https://snackers.wiki) and [filian.ru](https://filian.ru). Every one of them lands with me, a human.

<p>
  <a href="https://vai-rice.space"><img src="https://img.shields.io/badge/Portfolio-0d0e1a?style=for-the-badge&logo=firefoxbrowser&logoColor=39c5bb" alt="Portfolio" /></a>
  <a href="https://github.com/Vadim-Khristenko"><img src="https://img.shields.io/badge/GitHub-0d0e1a?style=for-the-badge&logo=github&logoColor=c0caf5" alt="GitHub" /></a>
  <a href="https://git.vai-rice.space/VAI_PROG"><img src="https://img.shields.io/badge/VAI%20Git-0d0e1a?style=for-the-badge&logo=forgejo&logoColor=dea584" alt="VAI Git (Forgejo)" /></a>
  <a href="https://codeberg.org/VAI_PROG"><img src="https://img.shields.io/badge/Codeberg-0d0e1a?style=for-the-badge&logo=codeberg&logoColor=2185d0" alt="Codeberg" /></a>
  <a href="https://twitch.tv/VAI_PROG"><img src="https://img.shields.io/badge/Twitch-0d0e1a?style=for-the-badge&logo=twitch&logoColor=bb9af7" alt="Twitch" /></a>
  <a href="https://x.com/VAI_PROG"><img src="https://img.shields.io/badge/X-0d0e1a?style=for-the-badge&logo=x&logoColor=c0caf5" alt="X" /></a>
  <a href="https://youtube.com/@VAI_PROG"><img src="https://img.shields.io/badge/YouTube-0d0e1a?style=for-the-badge&logo=youtube&logoColor=f7768e" alt="YouTube" /></a>
  <a href="https://t.me/vscreator_life"><img src="https://img.shields.io/badge/Telegram-0d0e1a?style=for-the-badge&logo=telegram&logoColor=7dcfff" alt="Telegram" /></a>
  <a href="https://patreon.com/VAI_PROG"><img src="https://img.shields.io/badge/Patreon-0d0e1a?style=for-the-badge&logo=patreon&logoColor=ff9e64" alt="Patreon" /></a>
  <a href="https://codeforces.com/profile/VAI_Programmer"><img src="https://img.shields.io/badge/Codeforces-0d0e1a?style=for-the-badge&logo=codeforces&logoColor=7aa2f7" alt="Codeforces" /></a>
</p>

<!-- ENGINE:DIV_STAFF:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_staff.svg?v=39a2fcfb5d-69957" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_STAFF:END -->

<!-- ENGINE:FOOTER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/footer.svg?v=8f47c65e73-69957" alt="Fine. Rendered by the VAI Profile Engine" width="100%"/>
</p>
<!-- ENGINE:FOOTER:END -->

<p align="center">
  <img src="https://hit.yhype.me/github/profile?account_id=124452155" alt="Hit Analytics" />
</p>
