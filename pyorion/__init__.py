# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Top-level package API.

This module exposes the primary entry points for working with the
OrionFrame runtime and its components. Users are expected to import
from this level rather than accessing internal submodules directly.

Exported symbols
----------------

* :func:`command` — Decorator to register Python functions as callable
  commands from the frontend.
* :class:`PyOrion` — Entry point to start the runtime environment.
* :class:`WebView` — Abstraction for managing embedded browser views.
* :class:`Window` — Abstraction for creating and managing application windows.
* :mod:`types` — Shared type definitions (Pydantic models and enums).
"""

from . import api, setup
from .pyinvoke import command
from .runtime import launch


__all__ = [
    "api",
    "command",
    "launch",
    "setup",
    "types",
]
