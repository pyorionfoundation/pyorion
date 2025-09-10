"""Entry point for the basic PyOrion example.

This script launches the PyOrion runtime with predefined
window options and registers basic commands.
"""

import asyncio
import sys

from pyorion import launch

from . import commands
from .config import window_options_config


if __name__ == "__main__":
    try:
        asyncio.run(
            launch(
                app_cfg=window_options_config, websocket_url="ws://127.0.0.1:8765/ws"
            )
        )
    except KeyboardInterrupt:
        print("Exiting...")
        sys.exit(0)
