import asyncio
import sys
import threading
import multiprocessing

from pyorion import launch
from pyorion.runtime import core
from .config import window_options_config
from . import commands
from .watchers import check_signal_process  # 👈 import aus watchers.py


# --- SYNC HANDLER (runs in a background thread) ---
def check_signal_sync():
    """
    Blocking synchronous handler.
    Runs in a thread and waits for the multiprocessing.Event.
    """
    core.close_signale.wait()  # blocks until set
    print("Close (sync)")
    sys.exit(0)


# --- ASYNC HANDLER (runs inside asyncio event loop) ---
async def check_signal_async():
    """
    Asynchronous handler.
    Uses run_in_executor to integrate blocking wait() into asyncio.
    """
    loop = asyncio.get_running_loop()
    await loop.run_in_executor(None, core.close_signale.wait)
    print("Close (async)")
    sys.exit(0)


async def main():
    """
    Main entry point for asyncio.
    Launches async watcher and the PyOrion runtime.
    """
    watcher = asyncio.create_task(check_signal_async())

    try:
        await launch(
            app_cfg=window_options_config,
            websocket_url="ws://127.0.0.1:8765/ws",
        )
    finally:
        watcher.cancel()


if __name__ == "__main__":
    multiprocessing.freeze_support()  # important for Windows

    # --- Start SYNC watcher in a thread ---
    sync_watcher = threading.Thread(target=check_signal_sync, daemon=True)
    sync_watcher.start()

    # --- Start MULTIPROCESSING watcher in its own process ---
    mp_watcher = multiprocessing.Process(target=check_signal_process, daemon=True)
    mp_watcher.start()

    # --- Run ASYNC watcher + launch() ---
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("Exiting...")
        sys.exit(0)
