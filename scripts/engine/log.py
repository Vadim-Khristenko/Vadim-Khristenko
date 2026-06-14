# -*- coding: utf-8 -*-
"""
Tiny, pretty, dependency-free logger for the engine.

Goals: l/aconic and scannable. Section headers, aligned step lines, status
glyphs and elapsed timing. ANSI colour when the output is a TTY and NO_COLOR
is unset; plain text otherwise (so CI logs stay clean).
"""

from __future__ import annotations

import os
import sys
import time

_USE_COLOR = sys.stdout.isatty() and not os.environ.get("NO_COLOR")


def _c(code: str, text: str) -> str:
    return f"\033[{code}m{text}\033[0m" if _USE_COLOR else text


def _dim(t): return _c("2;37", t)
def _rust(t): return _c("38;5;215", t)
def _green(t): return _c("38;5;150", t)
def _red(t): return _c("38;5;210", t)
def _yellow(t): return _c("38;5;222", t)
def _cyan(t): return _c("38;5;117", t)
def _bold(t): return _c("1", t)


_T0 = time.perf_counter()


def _elapsed() -> str:
    return f"{time.perf_counter() - _T0:6.1f}s"


def banner(title: str, subtitle: str = "") -> None:
    bar = "═" * 58
    print(_rust(f"╔{bar}╗"))
    line = f"  {title}"
    print(_rust("║") + _bold(line.ljust(58)) + _rust("║"))
    if subtitle:
        print(_rust("║") + _dim(f"  {subtitle}".ljust(58)) + _rust("║"))
    print(_rust(f"╚{bar}╝"))


def section(name: str) -> None:
    print(f"\n{_cyan('▸')} {_bold(name)}")


def step(label: str, value="", note: str = "") -> None:
    line = f"  {_dim('·')} {label.ljust(16)} {_cyan(str(value))}"
    if note:
        line += f"  {_dim(note)}"
    print(line)


def ok(label: str, note: str = "") -> None:
    print(f"  {_green('✓')} {label.ljust(16)} {_dim(note)}")


def warn(msg: str) -> None:
    print(f"  {_yellow('⚠')} {msg}")


def fail(label: str, msg: str) -> None:
    print(f"  {_red('✗')} {label.ljust(16)} {_red(msg)}", file=sys.stderr)


def done(extra: str = "") -> None:
    tail = f"  {_dim(extra)}" if extra else ""
    print(f"\n{_green('◆')} {_bold('done')} {_dim('in ' + _elapsed())}{tail}\n")
