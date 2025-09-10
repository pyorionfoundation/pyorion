"""Window Options Configuration Example.
====================================

This example demonstrates how to configure a window with an embedded WebView
using the ``PyOrion`` types module.

``render_protocol`` usage
--------------------------
- ``"web"``
  If only a folder is given, ``index.html`` will be automatically loaded by Rust.

- ``"folder/<file.html>"``
  Load a specific HTML start page from a folder (e.g., ``"dashboard/start.html"``).

- **Raw HTML string**
  Must start with ``<!DOCTYPE html>`` or ``<html>``.
  Example::

      render_protocol=\"\"\"<!DOCTYPE html>
      <html>
        <head><title>Inline</title></head>
        <body><h1>Hello World</h1></body>
      </html>\"\"\"

- ``http`` / ``https`` URL
  An external website can be used as the start page (e.g., ``"https://example.com"``).
"""

from pyorion.setup.types import Size, UnitType, WebViewOptions, WindowOptions


window_options_config = WindowOptions(
    title="Scientific Dashboard",
    inner_size=Size(width=900, height=700, unit=UnitType.logical),
    resizable=True,
    webview=WebViewOptions(
        label="root",
        render_protocol="examples/basic/web",  # could also be folder/index.html, HTML string, or URL
        visible=True,
        devtools=True,
    ),
)
