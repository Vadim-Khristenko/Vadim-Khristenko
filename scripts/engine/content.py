# -*- coding: utf-8 -*-
"""
Static content the engine rotates through or renders verbatim.
Keep editorial data here so the card modules stay about *layout*.
"""

ALIASES = ["VAI_PROG", "VAI_Programmer", "VAI", "VOLT"]

# Games. Each is a dict so the fetch pipeline (SteamGridDB) and the card share
# one source of truth: key (file/asset name), title (display), short (tile tag),
# query (SteamGridDB search term), ca/cb (fallback-tile accent + base colours).
GAMES = [
    {
        "key": "nikke",
        "title": "NIKKE",
        "short": "NIKKE",
        "query": "Goddess of Victory Nikke",
        "ca": "#e23b5a",
        "cb": "#2a1020",
    },
    {
        "key": "bluearchive",
        "title": "Blue Archive",
        "short": "BA",
        "query": "Blue Archive",
        "ca": "#4a9fe0",
        "cb": "#0e2030",
    },
    {
        "key": "nte",
        "title": "Neverness to Everness",
        "short": "NTE",
        "query": "Neverness to Everness",
        "ca": "#ff4d6d",
        "cb": "#1a0f22",
    },
    {
        "key": "endfield",
        "title": "Arknights: Endfield",
        "short": "ENDFIELD",
        "query": "Arknights Endfield",
        "ca": "#e0a64a",
        "cb": "#1f1a10",
    },
    {
        "key": "wuwa",
        "title": "Wuthering Waves",
        "short": "WUWA",
        "query": "Wuthering Waves",
        "ca": "#3fd0c9",
        "cb": "#0e2222",
    },
    {
        "key": "genshin",
        "title": "Genshin Impact",
        "short": "GENSHIN",
        "query": "Genshin Impact",
        "ca": "#7ec8e3",
        "cb": "#101a2a",
    },
    {
        "key": "honkai3",
        "title": "Honkai Impact 3rd",
        "short": "HI3",
        "query": "Honkai Impact 3rd",
        "ca": "#c98bdb",
        "cb": "#1a1024",
    },
    {
        "key": "starrail",
        "title": "Honkai: Star Rail",
        "short": "STAR RAIL",
        "query": "Honkai Star Rail",
        "ca": "#9b8cff",
        "cb": "#12102a",
    },
    {
        "key": "zzz",
        "title": "Zenless Zone Zero",
        "short": "ZZZ",
        "query": "Zenless Zone Zero",
        "ca": "#f3d23b",
        "cb": "#22200c",
    },
    {
        "key": "prsk",
        "title": "Project Sekai",
        "short": "PRSK",
        "query": "Project Sekai Colorful Stage",
        "ca": "#39d6b0",
        "cb": "#0e221c",
    },
    {
        "key": "minecraft",
        "title": "Minecraft",
        "short": "MC",
        "query": "Minecraft",
        "ca": "#6cc24a",
        "cb": "#10220e",
    },
    {
        "key": "fortnite",
        "title": "Fortnite",
        "short": "FORTNITE",
        "query": "Fortnite",
        "ca": "#8a5cf6",
        "cb": "#150f26",
    },
]

# vibe-card note keyed by game key.
GAME_NOTES = {
    "nikke": "holding the surface line",
    "bluearchive": "Sensei, the students need you",
    "nte": "unlicensed Anomaly hunting in Hethereau",
    "endfield": "running the Endfield night shift",
    "wuwa": "resonating across Solaris-3",
    "genshin": "still chasing region lore",
    "honkai3": "for everyone's smiles",
    "starrail": "may this journey lead us starward",
    "zzz": "running Hollows in New Eridu",
    "prsk": "chasing the perfect full-combo",
    "minecraft": "one more redstone contraption",
    "fortnite": "building 1x1s under pressure",
}

COMPOSERS = [
    ("Hatsune Miku", "the eternal 16-year-old diva"),
    ("Deco*27", "Vocaloid royalty"),
    ("Chiru-San", "on quiet loop"),
    ("DraGonis", "for the heavy sessions"),
    ("Kanro", "late-night coding fuel"),
    ("Exyl", "future-bass dopamine"),
    ("NoAki", "soft synth therapy"),
]

# Rotating "current focus" for the weekly vibe card (replaces the AI block).
FOCUS = [
    ("Aethelgard TQ-1.58", "low-bit reasoning research"),
    ("AmneziaWG Architect", "shipping mimicry profiles"),
    ("The Wall Dev", "Filian community tooling"),
    ("Rust internals", "zero-cost everything"),
    ("inference pipelines", "making models cheap to run"),
    ("server fleet", "Linux & Windows, all green"),
]

QUOTES = [
    "I generate a billion ideas on the fly — time is the only bottleneck.",
    "Fast prototype, validate, harden — then build something weird for fun.",
    "From banal scripts to absolutely unhinged solutions. Both ship.",
    "Backend by trade, chaos by hobby.",
    "If it can be automated, it probably already is.",
    "Servers under any OS bend the same way — with patience and good logs.",
    "Tiny stories between commits keep the compiler honest.",
    "Three states are enough: minus one, zero, plus one.",
]

# Verified social presence (harvested from the user's own portfolio — no guesses).
SOCIALS = [
    ("GitHub", "https://github.com/Vadim-Khristenko"),
    ("Codeberg", "https://codeberg.org/VAI_PROG"),
    ("Twitch", "https://twitch.tv/VAI_PROG"),
    ("X", "https://x.com/VAI_PROG"),
    ("YouTube", "https://youtube.com/@VAI_PROG"),
    ("Telegram", "https://t.me/vscreator_life"),
    # ("Reddit", "https://reddit.com/u/VAI_PROG"), - Was banned due to a misunderstanding, will be back eventually.
    ("Patreon", "https://patreon.com/VAI_PROG"),
    ("Codeforces", "https://codeforces.com/profile/VAI_Programmer"),
]

RESEARCH = {
    "name": "Aethelgard TQ-1.58 HVRL",
    "subtitle": "low-bit agentic reasoning architecture",
    "blurb": (
        "Architecture for a low-bit agentic reasoning model with verifiable "
        "training, hierarchical RL and domain specialization for software engineering."
    ),
}

# ── Best game (fully configurable spotlight card) ───────────────────────────
# art_url: any image URL to use as the cover (e.g. a SteamGridDB hero/grid link).
#          Leave "" to auto-resolve a hero via the SteamGridDB API ("query").
# Each character's art_url accepts any image incl. animated WEBP/GIF; leave ""
# to show a stylised avatar. fetch_game_art.py --bestgame downloads everything
# into assets/bestgame/ (static images are cropped; animated are kept as-is).
# cover_mode: "portrait" (cover as a left 2:3 panel) or "landscape" (full-width
#   cover behind the text). art_url / character art_url accept a remote URL OR a
#   local path under the repo (e.g. "assets/bestgame/cover.png"); animated
#   WEBP/GIF are embedded as-is. Up to 5 characters are laid out automatically.
BEST_GAME = {
    "title": "NIKKE",
    "subtitle": "GODDESS OF VICTORY",
    "query": "Goddess of Victory Nikke",  # SteamGridDB search when art_url is ""
    "art_url": "https://cdn2.steamgriddb.com/grid/6125432a1d4dfa1f33dbffe6bb0b9a0e.jpg",
    "cover_mode": "portrait",  # "portrait" | "landscape"
    "accent": "#e23b5a",
    "nick": "VAI",
    "level": "95",
    "server": "Global",
    "game_id": "12405515",
    "blurb": "My main since launch — holding the surface against the Raptures.",
    "characters": [
        {
            "name": "Rapi: Red Hood",
            "short": "Red Hood",
            "accent": "#f7768e",
            "art_url": "https://nikke-db-legacy.pages.dev/images/sprite/si_c016_00_s.png",
        },
        {
            "name": "Anis: Star",
            "short": "Anis",
            "accent": "#FFC107",
            "art_url": "https://nikke-db-legacy.pages.dev/images/sprite/si_c017_00_s.png",
        },
        {
            "name": "Soda: Twinkling Bunny",
            "short": "Soda",
            "accent": "#98ffd9",
            "art_url": "https://nikke-db-legacy.pages.dev/images/sprite/si_c314_00_s.png",
        },
        {
            "name": "Frima: Sea Of Sloth",
            "short": "Frima",
            "accent": "#C08261",
            "art_url": "https://nikke-db-legacy.pages.dev/images/sprite/si_c142_01_00_s.png",
        },
        {
            "name": "Snow Crane",
            "short": "Snow Crane",
            "accent": "#EBF1F5",
            "art_url": "https://nikke-db-legacy.pages.dev/images/sprite/si_c620_00_s.png",
        },
    ],
}
