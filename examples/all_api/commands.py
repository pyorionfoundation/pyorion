import base64
import io
from typing import Any

from PIL import Image

from pyorion import command
from pyorion.api.clipboard import ClipboardAPI
from pyorion.api.controlcenter import ControlCenterAPI
from pyorion.api.dialog import DialogAPI
from pyorion.api.dirs import DirsAPI
from pyorion.api.webview import WebView
from pyorion.api.window import Window
from pyorion.setup.types import ClipboardImage


# -------- Clipboard --------
@command
async def set_text(clipboard: ClipboardAPI, text: str) -> bool:
    """Set the given text to the system clipboard."""
    return await clipboard.set_text(text)


@command
async def get_text(clipboard: ClipboardAPI) -> str:
    """Retrieve the current text from the system clipboard."""
    return await clipboard.get_text()


@command
async def clear(clipboard: ClipboardAPI) -> bool:
    """Clear the system clipboard."""
    return await clipboard.clear()


@command
async def set_image(clipboard: ClipboardAPI) -> bool:
    """
    Create and set a sample RGBA image into the clipboard.
    Only sets, does not return image data for preview.
    """
    img = Image.new("RGBA", (100, 100), (200, 50, 50, 255))
    raw_bytes: bytes = img.tobytes()
    return await clipboard.set_image(100, 100, raw_bytes)


@command
async def get_image(clipboard: ClipboardAPI) -> ClipboardImage | None:
    """
    Retrieve image from the clipboard and return as Base64 PNG.
    This result is used by the frontend to display the image.
    """
    data: ClipboardImage | None = await clipboard.get_image()
    if data is None:
        return None

    # 1. Base64 zurück in raw bytes decodieren
    raw_bytes: bytes = base64.b64decode(data.bytes)

    # 2. Pillow-Image aus Bytes bauen
    img = Image.frombytes("RGBA", (data.width, data.height), raw_bytes)

    # 3. Als PNG exportieren
    buffer = io.BytesIO()
    img.save(buffer, format="PNG")
    png_bytes: bytes = buffer.getvalue()

    # 4. Wieder Base64 encodieren für Frontend
    b64: str = base64.b64encode(png_bytes).decode("utf-8")

    # 5. Mit MIME-Type zurückgeben
    return {
        "width": data.width,
        "height": data.height,
        "media_type": "image/png",  # immer gesetzt
        "bytes": b64,
    }


# -------- ControlCenter --------
@command
async def notification(control: ControlCenterAPI, summary: str, body: str) -> bool:
    """Display a notification via the system control center."""
    return await control.notification(summary=summary, body=body)


# -------- Dialog --------
@command
async def show_message(dialogs: DialogAPI, title: str, message: str) -> bool:
    """Show a simple message dialog."""
    return await dialogs.show_message(title, message)


@command
async def pick_file(dialogs: DialogAPI) -> dict[str, Any]:
    """Open a file picker dialog to select a file."""
    return await dialogs.pick_file()


# -------- Dirs --------
@command
async def home_dir(dirs: DirsAPI) -> str:
    """Get the path of the user's home directory."""
    return await dirs.home_dir()


# -------- WebView --------
@command
async def open_devtools(webview: WebView) -> None:
    """Open the developer tools for the given webview."""
    return await webview.open_devtools()


@command
async def close_devtools(webview: WebView) -> None:
    """Close the developer tools for the given webview."""
    return await webview.close_devtools()


# -------- Window --------
@command
async def set_title(window: Window, title: str) -> bool:
    """Set the title of the application window."""
    return await window.set_title(title)


@command
async def get_title(window: Window) -> str:
    """Get the current title of the application window."""
    return await window.get_title()


@command
async def set_fullscreen(window: Window, state: bool) -> bool:
    """Enable or disable fullscreen mode for the application window."""
    return await window.set_fullscreen(state)
