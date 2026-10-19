<!-- ════════════════════════════════════════════════════════════════════════ -->
<!--  Этот профиль ЖИВОЙ. Все карточки рисует собственный движок на Rust       -->
<!--  (engine/, бинарник vai-profile) в палитре Hatsune Miku Dark, настоящими  -->
<!--  шрифтами в виде контуров. Он читает GitHub, Codeberg и                   -->
<!--  git.vai-rice.space и заполняет маркеры ENGINE в README.md и              -->
<!--  README.ru.md. Текст правь как хочешь, маркеры не трогай.                 -->
<!-- ════════════════════════════════════════════════════════════════════════ -->

<!-- ENGINE:BADGES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/badges.svg?v=33a8be207f-69954" alt="Badges" width="100%"/>
</p>
<!-- ENGINE:BADGES:END -->

<!-- ENGINE:HEADER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/header.svg?v=0d2745f78a-69954" alt="Vadim Khristenko" width="100%"/>
</p>
<!-- ENGINE:HEADER:END -->

<p align="center">
  <a href="README.md">English</a> · <b>Русский</b>
</p>

<p align="center">
  <a href="#projects">Проекты</a> ·
  <a href="#research">Исследование</a> ·
  <a href="#snackers">Снакеры</a> ·
  <a href="#off-the-clock">После работы</a> ·
  <a href="#telemetry">Телеметрия</a> ·
  <a href="#infra">Инфраструктура</a> ·
  <a href="#reach-me">Связь</a>
</p>

<p align="center">
  <img src="https://komarev.com/ghpvc/?username=vadim-khristenko&label=ПРОСМОТРЫ&color=39c5bb&style=flat-square" alt="profile views" />
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

Пишу код **практичный и немного игривый**: бэкенд-сервисы, ботов, которые реально помогают людям, и системы, где важна скорость. **Rust** мой дом, **Python** и **TypeScript** успевают за мыслью, а **C++ / Cython** появляются, когда настаивает профайлер. Между коммитами пишу короткие книги, повести и фанфики.

**Почему Rust первым?** Borrow checker самый дешёвый ревьюер, которого я нанимал: денег не берёт и никогда не спит. Скорость уровня C, безопасность памяти от компилятора, спокойная многопоточность и один бинарник на любую машину во флоте. Этот профиль рисует как раз такой.

<!-- ENGINE:DIV_PROJECTS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_projects.svg?v=b3a19d026f-69954" alt="PROJECTS" width="100%"/>
</p>
<!-- ENGINE:DIV_PROJECTS:END -->

<a id="projects"></a>

## 🏗️ Проекты

### 💙 Главная репа: темы Hatsune Miku для редакторов

**[HatsuneMikuEditorTheme](https://github.com/Vadim-Khristenko/HatsuneMikuEditorTheme)**: одна палитра, четыре варианта, шесть редакторов (VS Code, Zed, JetBrains, Neovim, Sublime Text, Notepad++). **Скоро будут темы и для других программ**, редакторы только начало. Весь этот профиль покрашен в её Miku Dark.

<!-- ENGINE:THEMES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_miku_themes.svg?v=40135a7704-69954" alt="Hatsune Miku editor themes" width="100%"/>
</p>
<!-- ENGINE:THEMES:END -->

### 🛡️ Any Tech Architect

**[architect.vai-rice.space](https://architect.vai-rice.space)**: конфиги обфускации, где объяснено каждое число. Два движка, **AmneziaWG 3.1** и **XRay REALITY**, и всё считается в браузере, ключи и конфиги никуда не уходят.

<!-- ENGINE:ARCHITECT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_architect.svg?v=38257d165d-69954" alt="Any Tech Architect" width="100%"/>
</p>
<!-- ENGINE:ARCHITECT:END -->

### 🐈‍⬛ A CAT

**[awg-containers-and-tools](https://github.com/Vadim-Khristenko/awg-containers-and-tools)**, напарник Architect: свой AmneziaWG любого поколения от 1.0 до 3.1 в маленьких образах, резолвер только для туннеля и `awg-tool`, CLI на Rust, который ставит серверы по SSH, выдаёт пиров и меняет обфускацию.

<!-- ENGINE:ACAT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_acat.svg?v=d70722236f-69954" alt="A CAT: awg-containers-and-tools" width="100%"/>
</p>
<!-- ENGINE:ACAT:END -->

### 📖 NotAnOtherVTUBERwiki

**[NotAnOtherVTUBERwiki](https://github.com/Vadim-Khristenko/NotAnOtherVTUBERwiki)**: мой движок вики для фан-сообществ. Rust и Axum, Markdown в основе, быстрый даже на Raspberry Pi, работает без JavaScript. На нём живёт **[FilianWIKI](https://filian.wiki)**.

<!-- ENGINE:WIKI:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_wiki.svg?v=140e8e59e0-69954" alt="NotAnOtherVTUBERwiki and FilianWIKI" width="100%"/>
</p>
<!-- ENGINE:WIKI:END -->

### 🧱 The Wall Dev

**[the-wall.vai-rice.space](https://the-wall.vai-rice.space)**: расширение в стиле Win95 для The Wall, общего пиксельного холста снакеров размером 6000 × 4200. Оверлей шаблонов, кланы, общие шаблоны и уведомления от бота.

<!-- ENGINE:THEWALL:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_thewall.svg?v=9a032c1ab4-69954" alt="The Wall Dev" width="100%"/>
</p>
<!-- ENGINE:THEWALL:END -->

### 📜 Кумир 3

**[kumir3-core](https://github.com/Vadim-Khristenko/kumir3-core)**: школьный язык Кумир, пересобранный на Rust. Интерпретатор, TUI и компилятор в Rust и WASM, и да, Робот всё так же красит коридоры.

<!-- ENGINE:KUMIR:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/repo_kumir.svg?v=31716f9fd2-69954" alt="Kumir 3" width="100%"/>
</p>
<!-- ENGINE:KUMIR:END -->

<!-- ENGINE:DIV_RESEARCH:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_research.svg?v=097b911d31-69954" alt="RESEARCH" width="100%"/>
</p>
<!-- ENGINE:DIV_RESEARCH:END -->

<a id="research"></a>

## 🔬 Исследование

<!-- ENGINE:RESEARCH:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/research.svg?v=5a453e6f3d-69954" alt="Active research: Aethelgard TQ-1.58" width="100%"/>
</p>
<!-- ENGINE:RESEARCH:END -->

**Aethelgard TQ-1.58 HVRL** это архитектура **низкобитной агентной модели для рассуждений** с **проверяемым обучением**, **иерархическим обучением с подкреплением** и **специализацией на разработке ПО**. Веса живут в троичном пространстве `{-1, 0, +1}`, примерно 1.58 бита на вес, отсюда и название.

**Почему троичные?** Вес, который бывает только −1, 0 или +1, превращает большинство умножений в сложения, а ноль вообще в ничто. Инференс становится настолько дешёвым, что модель можно гонять на своём флоте, а не арендовать датацентр. *Если* её получится обучить и не свалиться с обрыва по точности. Это «если» и есть исследование.

<!-- ENGINE:DIV_SNACKERS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_snackers.svg?v=64285e0fae-69954" alt="SNACKERS" width="100%"/>
</p>
<!-- ENGINE:DIV_SNACKERS:END -->

<a id="snackers"></a>

## 🍿 Снакеры

<!-- ENGINE:SNACKERS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/fandom_snackers.svg?v=6c579f7595-69954" alt="Snackers" width="100%"/>
</p>
<!-- ENGINE:SNACKERS:END -->

Я из **Snackers**, сообщества вокруг втубера **[Filian](https://twitch.tv/filian)**, и делаю для него вещи:

- 📖 **[FilianWIKI](https://filian.wiki)**: вики для **Snackers HQ+**, **Snackers RU** и любого снакера в мире, на моём движке. Открытая бета: входишь через Discord или Telegram и правишь.
- 💗 **[Filian.ru](https://filian.ru)**: дом **Snackers RU**, русскоязычного сообщества.
- 🧱 **[The Wall Dev](https://the-wall.vai-rice.space)**: инструменты для The Wall, нашего общего пиксельного холста.

🎁 **Snackers RU подарили мне фигурку Хацунэ Мику.** Спасибо, Snackers RU! 💙

<sub>Всё это фанатские проекты: неофициальные и не часть ARG Филиан.</sub>

<!-- ENGINE:DIV_TAILS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_tails.svg?v=87d308d876-69954" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_TAILS:END -->

## 🏆 Награды

| | Достижение | Подробности |
|:--:|----|----|
| 🥈 | **PROD, Международная олимпиада по промышленной разработке** | **Призёр 2 степени.** Вёл в команде **бэкенд-трек** и держал на себе много движущихся частей. |
| 🎓 | **Алгоритмы и структуры данных в Т-Банке** | Выпускник, **параллель B, 2025-2026.** Хорошая боль, после которой иначе думаешь о сложности. |
| 🏅 | **«Высшая проба» (олимпиада ВШЭ)** | **Финалист по нескольким профилям.** Ширина намеренно, а не случайно. |

## 🛠️ Арсенал

<details>
<summary><b>Языки</b>: Rust первым 🦀</summary>
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
<summary><b>Веб и фронтенд</b></summary>
<p>
  <img alt="Vue" src="https://img.shields.io/badge/Vue-0d0e1a?style=for-the-badge&logo=vue.js&logoColor=9ece6a" />
  <img alt="Nuxt" src="https://img.shields.io/badge/Nuxt-0d0e1a?style=for-the-badge&logo=nuxt&logoColor=9ece6a" />
  <img alt="Next.js" src="https://img.shields.io/badge/Next-0d0e1a?style=for-the-badge&logo=next.js&logoColor=c0caf5" />
  <img alt="React" src="https://img.shields.io/badge/React-0d0e1a?style=for-the-badge&logo=react&logoColor=7dcfff" />
</p>
</details>

<details>
<summary><b>Данные, инфраструктура и DevOps</b>: флоты на Linux и Windows плюс свой Forgejo</summary>
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
<summary><b>AI-лаборатория</b>: напарники, а не автопилот</summary>
<p>
  <img alt="Fable 5" src="https://img.shields.io/badge/Fable%205-0d0e1a?style=for-the-badge&logo=anthropic&logoColor=dea584" />
  <img alt="GPT 5.6 Sol" src="https://img.shields.io/badge/GPT%205.6%20Sol-0d0e1a?style=for-the-badge&logo=openai&logoColor=c0caf5" />
  <img alt="Grok 4.5" src="https://img.shields.io/badge/Grok%204.5-0d0e1a?style=for-the-badge&logo=x&logoColor=c0caf5" />
  <img alt="Kimi K3" src="https://img.shields.io/badge/Kimi%20K3-0d0e1a?style=for-the-badge&logoColor=7aa2f7" />
  <img alt="Qwen 3.8 Max" src="https://img.shields.io/badge/Qwen%203.8%20Max-0d0e1a?style=for-the-badge&logo=alibabacloud&logoColor=ff9e64" />
  <img alt="MiMo V2.5 Pro" src="https://img.shields.io/badge/MiMo%20V2.5%20Pro-0d0e1a?style=for-the-badge&logo=xiaomi&logoColor=ff9e64" />
</p>

С экосистемами за этими моделями я работал в реальных сервисах: дообучение, промпт-инжиниринг и быстрый инференс. Любимые напарники сейчас: `Fable 5` · `Qwen 3.8 Max` · `MiMo V2.5 Pro`. Это **лёгкие копилоты и спарринг-партнёры**, а не автопилот для вайб-кодинга. Думаю я сам.
</details>

<!-- ENGINE:DIV_OFFCLOCK:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_offclock.svg?v=f692caac08-69954" alt="OFF THE CLOCK" width="100%"/>
</p>
<!-- ENGINE:DIV_OFFCLOCK:END -->

<a id="off-the-clock"></a>

## 🎮 После работы

<!-- ENGINE:BESTGAME:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/bestgame.svg?v=037f9a3677-69954" alt="Best game: NIKKE" width="100%"/>
</p>
<!-- ENGINE:BESTGAME:END -->

<p align="center">
  <a href="https://www.blablalink.com/user?openid=MjkwODAtNjA0NTA5MTUyNjQxNzk5OTA3MA=="><img src="https://img.shields.io/badge/NIKKE-командир%20VAI%20в%20Blablalink-1e9bff?style=for-the-badge&labelColor=060912" alt="Профиль NIKKE в Blablalink" /></a>
</p>

<!-- ENGINE:GAMES:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/games.svg?v=937f8021d3-69954" alt="Now playing" width="100%"/>
</p>
<!-- ENGINE:GAMES:END -->

<!-- ENGINE:DIV_WAVE:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_wave.svg?v=6370f9cf59-69954" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_WAVE:END -->

**🎧 На повторе:** `Hatsune Miku` · `Deco*27` · `Chiru-San` · `DraGonis` · `Kanro` · `Exyl` · `NoAki`

<!-- ENGINE:MUSIC:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/music.svg?v=58d944212e-69954" alt="Music via Last.fm" width="100%"/>
</p>
<!-- ENGINE:MUSIC:END -->

<!-- ENGINE:DIV_TELEMETRY:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_telemetry.svg?v=0208dca9aa-69954" alt="TELEMETRY" width="100%"/>
</p>
<!-- ENGINE:DIV_TELEMETRY:END -->

<a id="telemetry"></a>

## 📊 Телеметрия

Всё здесь приходит свежим сразу с **трёх площадок**: GitHub, Codeberg и моего `git.vai-rice.space`. Зеркала склеиваются, поэтому код считается один раз, а звёзды с каждой площадки всё равно учитываются.

<!-- ENGINE:STATS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/dashboard.svg?v=e5791acd27-69954" alt="Live telemetry" width="100%"/>
</p>
<!-- ENGINE:STATS:END -->

<!-- ENGINE:PLATFORMS:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/platforms.svg?v=c446099494-69954" alt="Platforms" width="100%"/>
</p>
<!-- ENGINE:PLATFORMS:END -->

<!-- ENGINE:DIV_INFRA:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_infra.svg?v=1c4e94f0bd-69954" alt="INFRA" width="100%"/>
</p>
<!-- ENGINE:DIV_INFRA:END -->

<a id="infra"></a>

## 🏠 Инфраструктура

Почти всё, что у меня крутится, живёт дома, в **хомлабе из ноутбуков**: **два работают серверами** на **Arch** и **NixOS**, а **основной ноутбук** стоит в дуалбуте **Windows 11** и **Arch Linux**. Ноутбуки неожиданно хороши в роли хомлабы: едят мало, свой ИБП уже встроен (батарея), а экран с клавиатурой под рукой на тот день, когда SSH скажет «нет».

А снаружи, в мире, край держат **VPS на Arch, NixOS, Ubuntu и Debian**.

<!-- ENGINE:HOMELAB:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/homelab.svg?v=71eafad201-69954" alt="Homelab: laptops at home, VPS out in the world" width="100%"/>
</p>
<!-- ENGINE:HOMELAB:END -->

<!-- ENGINE:DIV_CONTACT:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_contact.svg?v=334d809edc-69954" alt="REACH ME" width="100%"/>
</p>
<!-- ENGINE:DIV_CONTACT:END -->

<a id="reach-me"></a>

## 📮 Связь

Основной адрес: **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)**, почту читаю постоянно. Чтобы письмо попало куда надо, пиши на нужный алиас (`+тег` тоже работает):

| Если пишешь про… | Адрес |
|---|---|
| 🤝 Партнёрство, коллаборации, любой бизнес | **[business@vai-rice.space](mailto:business@vai-rice.space)** |
| 🛡️ Any Tech Architect или A CAT | **[architect@vai-rice.space](mailto:architect@vai-rice.space)** |
| 🧱 The Wall Dev | **[the-wall-dev@vai-rice.space](mailto:the-wall-dev@vai-rice.space)** |
| 📖 FilianWIKI, snackers.wiki или движок вики: статьи, правки, баги | **[vadim@filian.wiki](mailto:vadim@filian.wiki)** · **[vadim@snackers.wiki](mailto:vadim@snackers.wiki)** |
| 🇷🇺 filian.ru и Snackers RU | **[vadim@filian.ru](mailto:vadim@filian.ru)** |
| 📱 Конкретную соцсеть | **vadim+{соцсеть}@vai-rice.space** *(например, `vadim+twitch@…`)* |
| ✉️ Всё остальное | **[vadim@vai-rice.space](mailto:vadim@vai-rice.space)** |

Фанатские адреса те же, что указаны на [filian.wiki](https://filian.wiki), [snackers.wiki](https://snackers.wiki) и [filian.ru](https://filian.ru). Каждый из них попадает ко мне, живому человеку.

<p>
  <a href="https://vai-rice.space"><img src="https://img.shields.io/badge/Портфолио-0d0e1a?style=for-the-badge&logo=firefoxbrowser&logoColor=39c5bb" alt="Портфолио" /></a>
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
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/divider_staff.svg?v=39a2fcfb5d-69954" alt="" width="100%"/>
</p>
<!-- ENGINE:DIV_STAFF:END -->

<!-- ENGINE:FOOTER:START -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Vadim-Khristenko/Vadim-Khristenko/main/assets/footer.svg?v=c650efcabb-69954" alt="Fine. Rendered by the VAI Profile Engine" width="100%"/>
</p>
<!-- ENGINE:FOOTER:END -->

<p align="center">
  <img src="https://hit.yhype.me/github/profile?account_id=124452155" alt="Hit Analytics" />
</p>
