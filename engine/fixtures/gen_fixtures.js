#!/usr/bin/env node
// Regenerates the recorded provider fixtures (deterministic pseudo-random).
// Usage: node gen_fixtures.js
const fs = require("fs");
const path = require("path");

let seed = 158;
const rnd = () => (seed = (seed * 48271) % 0x7fffffff) / 0x7fffffff;

function heatmap(days, density, peak, end = "2026-07-19") {
  const out = [];
  const endD = new Date(end + "T00:00:00Z");
  for (let i = days - 1; i >= 0; i--) {
    const d = new Date(endD.getTime() - i * 86400000);
    const iso = d.toISOString().slice(0, 10);
    const active = rnd() < density;
    out.push([iso, active ? 1 + Math.floor(rnd() * peak) : 0]);
  }
  // Keep the last 5 days active so streak math has something to chew on.
  for (let i = out.length - 5; i < out.length; i++) out[i][1] = Math.max(1, out[i][1]);
  return out;
}

function daily30(density, peak) {
  const out = [];
  for (let i = 0; i < 30; i++) out.push(rnd() < density ? 1 + Math.floor(rnd() * peak) : 0);
  return out;
}

const repo = (name, o = {}) => ({
  name,
  fork: false,
  stars: 0,
  forks: 0,
  watchers: 0,
  open_issues: 0,
  open_prs: null,
  language: null,
  pushed_at: "2026-07-01T10:00:00Z",
  html_url: "",
  description: null,
  ...o,
});

const github = {
  profile: { login: "Vadim-Khristenko", name: "Vadim Khristenko", followers: 61, following: 18 },
  repos: [
    repo("Vadim-Khristenko", { stars: 9, forks: 2, open_issues: 1, language: "Rust", pushed_at: "2026-07-19T06:04:00Z", html_url: "https://github.com/Vadim-Khristenko/Vadim-Khristenko", description: "Self-regenerating profile" }),
    repo("AmneziaWG-Architect", { stars: 218, forks: 31, open_issues: 14, language: "TypeScript", pushed_at: "2026-07-18T21:12:00Z", html_url: "https://github.com/Vadim-Khristenko/AmneziaWG-Architect", description: "Client-side DPI-evasion profile generator" }),
    repo("the-wall", { stars: 37, forks: 6, open_issues: 5, language: "TypeScript", pushed_at: "2026-07-17T19:40:00Z", html_url: "https://github.com/Vadim-Khristenko/the-wall", description: "Community tooling for the Filian world" }),
    repo("HatsuneMikuEditorTheme-VSC", { stars: 24, forks: 4, open_issues: 2, language: "JSON", pushed_at: "2026-06-28T15:00:00Z", html_url: "https://github.com/Vadim-Khristenko/HatsuneMikuEditorTheme-VSC", description: "Teal all the way down" }),
    repo("aethelgard", { stars: 12, forks: 1, open_issues: 3, language: "Python", pushed_at: "2026-07-15T09:30:00Z", html_url: "https://github.com/Vadim-Khristenko/aethelgard", description: "TQ-1.58 HVRL mirror" }),
    repo("fleet-scripts", { stars: 5, forks: 0, open_issues: 0, language: "Shell", pushed_at: "2026-05-02T08:00:00Z" }),
    repo("upstream-fork", { fork: true, stars: 2, forks: 0, open_issues: 0, language: "C++", pushed_at: "2026-04-01T08:00:00Z" }),
  ],
  languages: {
    "Vadim-Khristenko": { Rust: 92000, Python: 41000 },
    "AmneziaWG-Architect": { TypeScript: 310000, Vue: 88000, CSS: 21000 },
    "the-wall": { TypeScript: 150000, HTML: 12000 },
    "HatsuneMikuEditorTheme-VSC": { JavaScript: 8000 },
    "aethelgard": { Python: 60000 },
    "fleet-scripts": { Shell: 15000 },
  },
  heatmap: heatmap(140, 0.72, 14),
  commit_windows: { d7: 26, d30: 104, y1: 1483 },
  pulses: {
    "AmneziaWG-Architect": { total_commits: 347, daily_30: daily30(0.55, 5) },
    "the-wall": { total_commits: 152, daily_30: daily30(0.4, 4) },
    "HatsuneMikuEditorTheme-VSC": { total_commits: 41, daily_30: daily30(0.12, 2) },
  },
};

const codeberg = {
  profile: { login: "VAI_PROG", name: "Vadim Khristenko", followers: 7, following: 3 },
  repos: [
    repo("AmneziaWG-Architect", { stars: 11, forks: 2, open_issues: 1, open_prs: 1, language: "TypeScript", pushed_at: "2026-07-18T22:00:00Z", html_url: "https://codeberg.org/VAI_PROG/AmneziaWG-Architect", description: "Mirror" }),
    repo("the-wall", { stars: 3, forks: 0, open_issues: 0, open_prs: 0, language: "TypeScript", pushed_at: "2026-07-17T20:00:00Z", html_url: "https://codeberg.org/VAI_PROG/the-wall" }),
    repo("vai-scripts", { stars: 2, forks: 0, open_issues: 1, open_prs: 0, language: "Shell", pushed_at: "2026-06-10T12:00:00Z", html_url: "https://codeberg.org/VAI_PROG/vai-scripts" }),
  ],
  languages: {
    "AmneziaWG-Architect": { TypeScript: 305000, Vue: 87000, CSS: 20000 },
    "the-wall": { TypeScript: 149000, HTML: 12000 },
    "vai-scripts": { Shell: 22000 },
  },
  heatmap: heatmap(140, 0.18, 4),
  commit_windows: null,
  pulses: {
    "AmneziaWG-Architect": { total_commits: 340, daily_30: daily30(0.3, 3) },
  },
};

const vaiGit = {
  profile: { login: "VAI_PROG", name: "Vadim Khristenko", followers: 4, following: 2 },
  repos: [
    repo("aethelgard", { stars: 6, forks: 0, open_issues: 4, open_prs: 2, language: "Python", pushed_at: "2026-07-19T05:45:00Z", html_url: "https://git.vai-rice.space/VAI_PROG/aethelgard", description: "TQ-1.58 HVRL — source of truth" }),
    repo("AmneziaWG-Architect", { stars: 4, forks: 1, open_issues: 2, open_prs: 1, language: "TypeScript", pushed_at: "2026-07-18T23:10:00Z", html_url: "https://git.vai-rice.space/VAI_PROG/AmneziaWG-Architect" }),
    repo("the-wall", { stars: 2, forks: 0, open_issues: 1, open_prs: 0, language: "TypeScript", pushed_at: "2026-07-17T21:00:00Z", html_url: "https://git.vai-rice.space/VAI_PROG/the-wall" }),
    repo("vai-rice-space", { stars: 3, forks: 0, open_issues: 2, open_prs: 0, language: "Vue", pushed_at: "2026-07-16T18:00:00Z", html_url: "https://git.vai-rice.space/VAI_PROG/vai-rice-space", description: "The portfolio" }),
    repo("fleet-tools", { stars: 1, forks: 0, open_issues: 0, open_prs: 0, language: "Rust", pushed_at: "2026-07-12T14:00:00Z", html_url: "https://git.vai-rice.space/VAI_PROG/fleet-tools" }),
  ],
  languages: {
    "aethelgard": { Python: 512000, Rust: 148000, Cython: 36000 },
    "AmneziaWG-Architect": { TypeScript: 308000, Vue: 88000, CSS: 20500 },
    "the-wall": { TypeScript: 150500, HTML: 12000 },
    "vai-rice-space": { Vue: 96000, TypeScript: 44000, CSS: 18000 },
    "fleet-tools": { Rust: 83000 },
  },
  heatmap: heatmap(140, 0.6, 10),
  commit_windows: null,
  pulses: {
    "aethelgard": { total_commits: 611, daily_30: daily30(0.7, 6) },
    "AmneziaWG-Architect": { total_commits: 344, daily_30: daily30(0.35, 3) },
    "the-wall": { total_commits: 150, daily_30: daily30(0.3, 3) },
  },
};

for (const [name, data] of [["github", github], ["codeberg", codeberg], ["vai-git", vaiGit]]) {
  fs.writeFileSync(path.join(__dirname, `${name}.json`), JSON.stringify(data, null, 2) + "\n");
  console.log(`wrote ${name}.json`);
}
