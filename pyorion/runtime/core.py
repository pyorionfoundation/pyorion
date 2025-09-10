# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Core runtime state for the WebSocket/asyncio system.

This module provides global registries that are shared across different
parts of the runtime:

* ``task_queue`` — Queue of tasks to be processed by the event loop.
* ``background_tasks`` — Set of tracked asyncio tasks to prevent
  premature garbage collection.
* ``connected_clients`` — Active WebSocket client connections.

These globals act as singletons for coordination between the Python
frontend and the Rust-backed runtime.
"""

import asyncio
from typing import Any

from websockets import ServerConnection

import multiprocessing





close_signale = multiprocessing.Event()
#: Central task queue for inter-coroutine communication.
#: Each task is represented as a ``dict[str, Any]``.
task_queue: asyncio.Queue[dict[str, Any]] = asyncio.Queue()

#: Registry of active asyncio tasks.
#: Prevents tasks from being garbage collected while still running.
background_tasks: set[asyncio.Task] = set()

#: Set of currently connected frontend WebSocket clients.
#: Populated by :func:`handle_frontend_connections`.
connected_clients: set[ServerConnection] = set()
