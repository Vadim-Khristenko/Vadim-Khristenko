# Fonts

The cards are drawn with these faces. The engine never ships them as fonts:
it shapes text with rustybuzz and writes the glyph outlines into each SVG as
paths, so the cards look the same on every machine and need no web fonts.

| File | Family | Used by |
|---|---|---|
| ZenKakuGothicNew-300/500/900 | Zen Kaku Gothic New | header, themes, research, music, stats, footer; fallback for Japanese and Cyrillic |
| JetBrainsMono-400/700 | JetBrains Mono | code, numbers, labels everywhere |
| Unbounded-400/800 | Unbounded | Any Tech Architect, NotAnOtherVTUBERwiki |
| BigShouldersStencil-800 | Big Shoulders Stencil | A CAT |
| PixelifySans-500/700 | Pixelify Sans | Snackers, FilianWIKI |
| PTSans-400/700, PTMono-400 | PT Sans, PT Mono | Snackers, FilianWIKI, The Wall |
| Caveat-500/700 | Caveat | Kumir 3 |
| Silkscreen-400 | Silkscreen | The Wall |
| SairaCondensed-500/700/800 | Saira Condensed | NIKKE |
| Michroma-400 | Michroma | music |

All of them are licensed under the SIL Open Font License 1.1, see `OFL.txt`.
Hinting was stripped and Zen Kaku Gothic New was trimmed to Latin, Cyrillic,
kana and kanji (the Light weight keeps no kanji and falls back to Medium)
to keep the repository small. Saira Condensed carries a
Reserved Font Name, so its files are kept exactly as published.
