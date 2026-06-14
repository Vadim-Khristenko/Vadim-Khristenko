#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Thin shim → engine.art. Kept so CI can call `uv run scripts/fetch_game_art.py`.
For the friendly commands use: uv run --extra art art | bestgame."""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine.art import main  # noqa: E402

if __name__ == "__main__":
    main()
