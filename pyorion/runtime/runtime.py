# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Runtime management for PyOrion.

This module defines functions to launch, manage, and gracefully
terminate the PyOrion runtime, including background tasks,
subprocesses, and WebSocket communication.
"""

import asyncio
from collections.abc import Coroutine
from multiprocessing import get_context
from multiprocessing.context import SpawnProcess
from pathlib import Path

from pydantic import AnyHttpUrl

from pyorion._pyorion import create_webframe
from pyorion.setup.types import WebSocketConfig, WindowOptions
from . import core
from .connections import create_websocket_server
from .runtime_handle import eventloop_sender


shutdown_event = None  # Global shutdown event shared across the runtime


def locate_project_folder(folder_name: str) -> Path | None:
    """Locate a folder within the current working directory.

    :param folder_name: Name of the folder to search for.
    :type folder_name: str
    :return: Path to the folder if found, otherwise None.
    :rtype: Path | None
    """
    start_path = Path.cwd()
    base = Path(start_path).resolve()
    candidate = base / folder_name
    return candidate if candidate.exists() else None


def launch_background_task(coro: Coroutine) -> asyncio.Task:
    """Start an asyncio task and track it in the background task set.

    :param coro: Coroutine to be scheduled as a task.
    :type coro: Coroutine
    :return: The created asyncio task.
    :rtype: asyncio.Task
    """
    task = asyncio.create_task(coro)
    core.background_tasks.add(task)
    task.add_done_callback(core.background_tasks.discard)
    return task


async def cancel_all_tasks(loop: asyncio.AbstractEventLoop) -> None:
    """Cancel all running asyncio tasks and ensure a clean shutdown.

    :param loop: The asyncio event loop to operate on.
    :type loop: asyncio.AbstractEventLoop
    """
    for task in list(core.background_tasks):
        task.cancel()

    results = await asyncio.gather(*core.background_tasks, return_exceptions=True)
    for r in results:
        if isinstance(r, Exception) and not isinstance(r, asyncio.CancelledError):
            print(f"Task raised during shutdown: {r!r}")

    core.background_tasks.clear()


def terminate_process_safely(proc: SpawnProcess) -> None:
    """Terminate a spawned process gracefully, with fallback to kill.

    :param proc: The process to terminate.
    :type proc: multiprocessing.context.SpawnProcess
    """
    proc.join(3.0)
    if proc.is_alive():
        proc.terminate()
        proc.join(2.0)
    if proc.is_alive():
        proc.kill()
        proc.join()


async def run_native_runtime(
    app_cfg: WindowOptions,
    *,
    internal_proto: bool = True,
    websocket_url: AnyHttpUrl | None = None,
    protocols: list[str] | None = None,
    auto_reconnect: bool = True,
    reconnect_interval: int = 3000,
) -> None:
    """Start the native runtime environment.

    This function launches the native WebFrame subprocess and manages
    background servers (WebSocket and event loop sender). It ensures that
    processes and tasks are started, monitored, and terminated gracefully.

    A ``multiprocessing.Manager`` is used to create a shared shutdown event.
    This event allows communication between the WebFrame subprocess and the
    parent process to coordinate shutdown.

    :param app_cfg: Window configuration options serialized into JSON and passed
                    to the WebFrame process.
    :type app_cfg: WindowOptions
    :param internal_proto: Whether to enable the internal protocol handler.
    :type internal_proto: bool, optional
    :param websocket_url: The WebSocket URL used by the runtime (if provided).
    :type websocket_url: AnyHttpUrl | None, optional
    :param protocols: Optional list of supported WebSocket subprotocols.
    :type protocols: list[str] | None, optional
    :param auto_reconnect: Whether the WebSocket client should automatically reconnect.
    :type auto_reconnect: bool, optional
    :param reconnect_interval: Interval in milliseconds before attempting reconnect.
    :type reconnect_interval: int, optional
    :return: None
    :rtype: None
    """
    socket_cfg = None

    loop = asyncio.get_running_loop()
    if internal_proto and websocket_url is not None:
        socket_cfg = WebSocketConfig(
            url=websocket_url,
            protocols=protocols,
            auto_reconnect=auto_reconnect,
            reconnect_interval=reconnect_interval,
        )
        launch_background_task(create_websocket_server(str(websocket_url)))

    launch_background_task(eventloop_sender())
    socket_cfg_json = (
        socket_cfg.model_dump_json(by_alias=True) if socket_cfg is not None else None
    )
    ctx = get_context("spawn")
    with ctx.Manager() as manager:
        close_event = manager.Event()
        config = app_cfg.model_dump_json(by_alias=True)

        global shutdown_event
        core.close_signale = shutdown_event = close_event
        proc = ctx.Process(
            target=create_webframe,
            args=(config, socket_cfg_json, "pyframe_pipe", shutdown_event),
            daemon=False,
        )
        proc.start()

        try:
            # Wait until the subprocess signals shutdown via the shared Event
            await loop.run_in_executor(None, close_event.wait)
        finally:
            await loop.run_in_executor(None, lambda: terminate_process_safely(proc))

    await cancel_all_tasks(loop)
