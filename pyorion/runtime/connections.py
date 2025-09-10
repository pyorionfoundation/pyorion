# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""WebSocket server connections for the PyOrion runtime.

This module provides the WebSocket server used for communication
between the PyOrion backend and frontend clients. It manages
client registration, message dispatching, and broadcasting responses.
"""

import asyncio
import json
import logging
from urllib.parse import urlparse

import websockets
from pydantic import BaseModel
from websockets import ServerConnection

from pyorion.pyinvoke import _event_callbacks, make_callback
from pyorion.runtime import core
from pyorion.utils import make_json_safe


def list_commands() -> dict[str, list[str]]:
    """Return a mapping of registered event names -> handler function names."""
    return {key: [f.__name__ for f in funcs] for key, funcs in _event_callbacks.items()}


async def handle_frontend_connections(websocket: ServerConnection) -> None:
    """Handle an individual frontend WebSocket connection."""
    core.connected_clients.add(websocket)
    try:
        async for message in websocket:
            try:
                payload = json.loads(message)

                if not isinstance(payload, dict):
                    logging.warning("Ignoring non-dict payload: %s", payload)
                    continue

                if all(
                    k in payload for k in ("cmd", "result_id", "error_id", "payload")
                ):
                    cmd = payload["cmd"]

                    # Prüfen, ob Command registriert ist
                    if cmd not in _event_callbacks:
                        logging.error(
                            "No handler registered for event '%s'. Available: %s",
                            cmd,
                            list_commands(),
                        )
                        continue

                    response = await make_callback(
                        cmd,
                        payload["result_id"],
                        payload["error_id"],
                        payload["payload"],
                    )

                    # Einheitliche Serialisierung
                    if isinstance(response, BaseModel):
                        response_msg = response.model_dump_json(by_alias=True)
                    else:
                        response_msg = json.dumps(response, default=make_json_safe)

                    # Broadcast response to all connected clients
                    await asyncio.gather(
                        *(
                            client.send(response_msg)
                            for client in core.connected_clients
                        )
                    )
                else:
                    logging.warning("Incomplete message keys: %s", payload)
            except json.JSONDecodeError as exc:
                logging.error(
                    "Malformed JSON from %s: %s", websocket.remote_address, exc
                )
            except Exception as exc:
                logging.exception("Unexpected error handling message: %s", exc)
    except websockets.ConnectionClosed:
        logging.info("Client disconnected: %s", websocket.remote_address)
    finally:
        core.connected_clients.discard(websocket)


async def create_websocket_server(url: str) -> None:
    """Create and run the frontend WebSocket server (with optional path)."""
    parsed = urlparse(url)
    host, port = parsed.hostname, parsed.port
    expected_path = parsed.path if parsed.path and parsed.path != "/" else None

    if not host or not port:
        raise ValueError(f"Invalid WebSocket URL: {url}")

    async def handler(websocket: ServerConnection):
        path_received = websocket.request.path

        if expected_path is not None and path_received != expected_path:
            await websocket.close(
                code=1008,
                reason=f"Invalid path {path_received}, expected {expected_path}",
            )
            return

        await handle_frontend_connections(websocket)

    async with websockets.serve(handler, host, port):
        await asyncio.Future()
