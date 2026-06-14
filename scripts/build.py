#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Thin shim → engine.run. Kept so CI can call `uv run scripts/build.py`.
For the friendly commands use: uv run build | rebuild | preview."""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine.run import main  # noqa: E402

if __name__ == "__main__":
    main()
