# examples/basic/watchers.py
import sys
from pyorion.runtime import core


def check_signal_process():
    """
    Multiprocessing handler.
    Runs in a subprocess, waits for the global core.close_signale.
    Safe to use with Windows spawn.
    """
    core.close_signale.wait()
    print("Close (multiprocessing)", flush=True)
    sys.exit(0)
