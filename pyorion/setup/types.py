# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""model-Typen und Enums für PyOrion."""

from __future__ import annotations

import base64
from enum import Enum
from pathlib import Path
from typing import Any, ClassVar

from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    field_validator,
    model_serializer,
    model_validator,
)
from pydantic.alias_generators import to_camel


class BaseSchema(BaseModel):
    """Basisklasse für alle Schemas mit einheitlicher Pydantic-Konfiguration."""

    model_config = ConfigDict(
        alias_generator=to_camel,
        populate_by_name=True,
        from_attributes=True,
    )


class WebSocketConfig(BaseSchema):
    """WebSocketConfig - Configuration model for PyOrionConnections.

    Fields:
    - url (str): WebSocket URL (must start with ws:// or wss://)
    - protocols (list[str], optional): Subprotocols for the handshake
    - auto_reconnect (bool, optional): Enable automatic reconnect
    - reconnect_interval (int, optional): Reconnect delay in milliseconds
    """

    url: str = Field(
        description="The WebSocket URL (e.g., ws://localhost:8765 or wss://example.com/socket).",
    )
    protocols: list[str] | None = Field(
        default=None, description="Optional list of WebSocket subprotocols."
    )
    auto_reconnect: bool | None = Field(
        default=True, description="Whether the client should automatically reconnect."
    )
    reconnect_interval: int | None = Field(
        default=3000, description="Reconnect interval in milliseconds."
    )

    class Config:
        """Validate example configuration"""

        extra = "forbid"
        json_schema_extra: ClassVar[dict] = {
            "example": {
                "url": "wss://example.com/socket",
                "protocols": ["json", "binary"],
                "auto_reconnect": True,
                "reconnect_interval": 5000,
            }
        }


""" # === Example usage ===
if __name__ == "__main__":
    # Validate example configuration
    cfg = WebSocketConfig(
        url="wss://example.com/socket",
        protocols=["json", "binary"],
        auto_reconnect=True,
        reconnect_interval=5000
    )

    # Export JSON for Rust/serde
    print(cfg.model_dump_json(indent=2))
 """


class RGBA(BaseSchema):
    """RGBA-Farbmodell, serialisierbar als Tupel (r, g, b, a)."""

    r: int
    g: int
    b: int
    a: int

    def as_tuple(self) -> tuple[int, int, int, int]:
        """Konvertiere RGBA in ein Tupel im Format (r, g, b, a)."""
        return (self.r, self.g, self.b, self.a)

    @model_serializer
    def serialize_as_tuple(self) -> tuple[int, int, int, int]:
        """Serialisiere RGBA als Tupel für Serde-Kompatibilität mit Rust."""
        return self.as_tuple()


class UnitType(str, Enum):
    """Darstellung von Längen-/Größeneinheiten (logisch oder physisch)."""

    logical = "logical"
    physical = "physical"


class Position(BaseSchema):
    """Fenster- oder WebView-Position mit optionalen Koordinaten und Einheit."""

    x: int | None = None
    y: int | None = None
    unit: UnitType


class Size(BaseSchema):
    """Fenster- oder WebView-Größe mit optionalen Dimensionen und Einheit."""

    width: int | None = None
    height: int | None = None
    unit: UnitType


class WebViewBounds(BaseSchema):
    """Kombination aus Position und Größe eines WebViews."""

    position: Position
    size: Size


class CursorIcon(str, Enum):
    """Unterstützte Mauszeiger-Icons für Fenster/WebViews."""

    default = "default"
    crosshair = "crosshair"
    hand = "hand"
    arrow = "arrow"
    move = "move"
    text = "text"
    wait = "wait"
    help = "help"
    progress = "progress"
    not_allowed = "not_allowed"
    context_menu = "context_menu"
    cell = "cell"
    vertical_text = "vertical_text"
    alias = "alias"
    copy = "copy"
    no_drop = "no_drop"
    grab = "grab"
    grabbing = "grabbing"
    all_scroll = "all_scroll"
    zoom_in = "zoom_in"
    zoom_out = "zoom_out"
    e_resize = "e_resize"
    n_resize = "n_resize"
    ne_resize = "ne_resize"
    nw_resize = "nw_resize"
    s_resize = "s_resize"
    se_resize = "se_resize"
    sw_resize = "sw_resize"
    w_resize = "w_resize"
    ew_resize = "ew_resize"
    ns_resize = "ns_resize"
    nesw_resize = "nesw_resize"
    nwse_resize = "nwse_resize"
    col_resize = "col_resize"
    row_resize = "row_resize"


class ProgressState(str, Enum):
    """Zustände einer Fortschrittsanzeige."""

    none = "none"
    normal = "normal"
    indeterminate = "indeterminate"
    paused = "paused"
    error = "error"


class ProgressBarState(BaseSchema):
    """Status einer Fortschrittsanzeige, inkl. Fortschritt und Zustand."""

    progress: int | None = None
    status: ProgressState | None = None
    desktop_filename: str | None = None


class Theme(str, Enum):
    """Farbthema für Fensterdarstellung."""

    light = "light"
    dark = "dark"


class UserAttentionType(str, Enum):
    """Art der Benutzeraufmerksamkeit (kritisch oder informativ)."""

    critical = "critical"
    informational = "informational"


class ByteIcon(BaseSchema):
    """Fenstersymbol als RGBA-Byte-Array (Base64-kodiert)."""

    rgba: str  # base64 encoded
    width: int
    height: int


class Icon(BaseSchema):
    """Fenstersymbol als Dateipfad."""

    path: str


class WindowSizeConstraints(BaseSchema):
    """Minimale und maximale Fenstergrößen mit Einheit."""

    min_width: float | None = None
    min_height: float | None = None
    max_width: float | None = None
    max_height: float | None = None
    unit: UnitType


class Dimensions(BaseSchema):
    """Breite und Höhe als Integer-Dimensionen."""

    width: int
    height: int


class MonitorPosition(BaseSchema):
    """Position eines Monitors auf dem Desktop (x,y)."""

    x: int
    y: int


class MonitorVideoMode(BaseSchema):
    """Video-Modus eines Monitors mit Auflösung, Farbtiefe und Refresh-Rate."""

    size: Dimensions
    bit_depth: int
    refresh_rate: int


class Monitor(BaseSchema):
    """Darstellung eines Monitors mit Name, Skalierung und Video-Modi."""

    name: str | None = None
    scale_factor: float
    size: Dimensions
    position: MonitorPosition
    video_modes: list[MonitorVideoMode]


class WebViewOptions(BaseSchema):
    """Optionen zur Konfiguration eines WebViews."""

    label: str | None = None
    render_protocol: Path | str | None = None
    transparent: bool | None = None
    visible: bool | None = None
    devtools: bool | None = None
    incognito: bool | None = None
    initialization_script: str | None = None
    accept_first_mouse: bool | None = None
    autoplay: bool | None = None
    focused: bool | None = None
    clipboard: bool | None = None
    hotkeys_zoom: bool | None = None
    background_color: RGBA | None = None
    bounds: WebViewBounds | None = None
    headers: dict[str, str] | None = None
    proxy_config: str | None = None
    zoom_hotkeys: bool | None = None
    background_throttling: bool | None = None
    back_forward_navigation_gestures: bool | None = None


class WindowOptions(BaseSchema):
    """Optionen zur Konfiguration eines Fensters."""

    always_on_bottom: bool | None = None
    always_on_top: bool | None = None
    background_color: RGBA | None = None
    closable: bool | None = None
    content_protection: bool | None = None
    decorations: bool | None = None
    focusable: bool | None = None
    focused: bool | None = None
    fullscreen: bool | None = None
    inner_size: Size | None = None
    max_inner_size: Size | None = None
    maximizable: bool | None = None
    maximized: bool | None = None
    min_inner_size: Size | None = None
    minimizable: bool | None = None
    position: Position | None = None
    resizable: bool | None = None
    theme: Theme | None = None
    title: str | None = None
    transparent: bool | None = None
    visible: bool | None = None
    visible_on_all_workspaces: bool | None = None
    window_icon: Icon | None = None
    webview: WebViewOptions | None = None


class Color(BaseModel):
    """RGBA-Farbmodell (0-255 je Kanal)."""

    r: int = Field(..., ge=0, le=255, description="Rotanteil (0-255)")
    g: int = Field(..., ge=0, le=255, description="Grünanteil (0-255)")
    b: int = Field(..., ge=0, le=255, description="Blauanteil (0-255)")
    a: int = Field(255, ge=0, le=255, description="Alpha-Kanal (0-255)")

    model_config = {"populate_by_name": True}

    def to_hex(self) -> str:
        """Hex-String wie `#RRGGBB` oder `#RRGGBBAA`."""
        if self.a == 255:
            return f"#{self.r:02X}{self.g:02X}{self.b:02X}"
        return f"#{self.r:02X}{self.g:02X}{self.b:02X}{self.a:02X}"

    @classmethod
    def from_hex(cls, hex_str: str) -> Color:
        """Erzeuge eine Color-Instanz aus Hex-String."""
        hex_str = hex_str.lstrip("#")
        if len(hex_str) == 6:
            r = int(hex_str[0:2], 16)
            g = int(hex_str[2:4], 16)
            b = int(hex_str[4:6], 16)
            return cls(r=r, g=g, b=b, a=255)
        elif len(hex_str) == 8:
            r = int(hex_str[0:2], 16)
            g = int(hex_str[2:4], 16)
            b = int(hex_str[4:6], 16)
            a = int(hex_str[6:8], 16)
            return cls(r=r, g=g, b=b, a=a)
        else:
            raise ValueError("Ungültiger Hex-String, muss 6 oder 8 Zeichen haben.")

    def to_rgba(self) -> list[int]:
        """Umwandlung nach RGBA-Array (z. B. für Rendering)."""
        return [self.r, self.g, self.b, self.a]

    def to_tuple(self) -> tuple[int, int, int, int]:
        """Umwandlung in ein (r, g, b, a)-Tuple."""
        return self.r, self.g, self.b, self.a


class ClipboardImage(BaseSchema):
    """Clipboard image or media representation."""

    width: int = Field(..., description="Image width in pixels.")
    height: int = Field(..., description="Image height in pixels.")
    media_type: str = Field(
        ..., description="MIME type of the media (e.g., image/png)."
    )
    bytes: str = Field(..., description="Base64-encoded media bytes.")

    @field_validator("bytes")
    @classmethod
    def validate_base64(cls, value: str) -> str:
        """Validate that the given string is valid Base64."""
        base64.b64decode(value, validate=True)
        return value

    @field_validator("media_type")
    @classmethod
    def validate_mime(cls, value: str) -> str:
        """Validate that the media type string is a valid MIME type."""
        if "/" not in value:
            raise ValueError(f"Invalid MIME type: {value}")
        return value

    @model_validator(mode="before")
    @classmethod
    def accept_list(cls, v: list[Any] | dict[str, Any]) -> dict[str, Any] | list[Any]:
        """Allow initialization from a list of [width, height, media_type, bytes]."""
        if isinstance(v, list) and len(v) == 4:
            return {"width": v[0], "height": v[1], "media_type": v[2], "bytes": v[3]}
        return v


class WindowEffect(str, Enum):
    """Window visual effect types.

    Supported effects vary by platform (e.g. macOS blur, Windows acrylic).
    """

    Titlebar = "titlebar"
    Selection = "selection"
    Menu = "menu"
    Popover = "popover"
    Sidebar = "sidebar"
    HeaderView = "headerView"
    Sheet = "sheet"
    WindowBackground = "windowBackground"
    HudWindow = "hudWindow"
    FullScreenUI = "fullScreenUI"
    Tooltip = "tooltip"
    ContentBackground = "contentBackground"
    UnderWindowBackground = "underWindowBackground"
    UnderPageBackground = "underPageBackground"
    Mica = "mica"
    MicaDark = "micaDark"
    MicaLight = "micaLight"
    Tabbed = "tabbed"
    TabbedDark = "tabbedDark"
    TabbedLight = "tabbedLight"
    Blur = "blur"
    Acrylic = "acrylic"


class WindowEffectState(str, Enum):
    """Window effect state.

    Controls whether an effect follows the window active state or is fixed.
    """

    FollowsWindowActiveState = "followsWindowActiveState"
    Active = "active"
    Inactive = "inactive"


class WindowEffectsConfig(BaseModel):
    """Konfiguration für Window-Effekte."""

    effects: list[WindowEffect] = Field(
        default_factory=list,
        description="Liste von Window-Effekten. Konflikte: nur der erste wird angewendet.",
    )
    state: WindowEffectState | None = Field(
        None, description="Window effect state (nur macOS)."
    )
    radius: float | None = Field(
        None, description="Fenster-Effekt Corner-Radius (nur macOS)."
    )
    color: Color | None = Field(
        None, description="Farbe (betrifft nur Blur/Acrylic unter Windows 10 v1903+)."
    )
    model_config = {"populate_by_name": True, "extra": "forbid"}
