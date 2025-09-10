// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

(function () {
function jsonMakeObjectSafe(obj) {
  if (obj === null || obj === undefined) {
    return null;
  }

  // primitive Werte
  if (typeof obj === "string" || typeof obj === "number" || typeof obj === "boolean") {
    return obj;
  }

  // Date / Path -> String
  if (obj instanceof Date) {
    return obj.toISOString();
  }
  if (typeof obj === "object" && obj.constructor && obj.constructor.name === "Path") {
    return String(obj);
  }

  // Buffer / ArrayBuffer / TypedArray -> Base64
  if (obj instanceof ArrayBuffer) {
    return btoa(String.fromCharCode(...new Uint8Array(obj)));
  }
  if (ArrayBuffer.isView(obj)) { // Uint8Array, Float32Array, etc.
    return btoa(String.fromCharCode(...obj));
  }

  // plain Object -> rekursiv
  if (typeof obj === "object" && !Array.isArray(obj)) {
    const safe = {};
    for (const [k, v] of Object.entries(obj)) {
      safe[String(k)] = jsonMakeObjectSafe(v);
    }
    return safe;
  }

  // Array / Set -> rekursiv
  if (Array.isArray(obj) || obj instanceof Set) {
    return Array.from(obj, v => jsonMakeObjectSafe(v));
  }

  // Fallback -> String
  return String(obj);
}
  var PyOrionConnections = {};
  var ws = null;
  var config = {
    protocols: __TEMPLATE_protocols__,
    reconnectInterval: __TEMPLATE_reconnect_interval__ || 3000,
    autoReconnect: __TEMPLATE_auto_reconnect__ ?? true
  };

  var eventListeners = {};
  var reconnectTimer = null;
  var shouldReconnect = true;

  function addEventListener(event, listener) {
    if (!eventListeners[event]) {
      eventListeners[event] = [];
    }
    eventListeners[event].push(listener);
  }

  function removeEventListener(event, listener) {
    if (!eventListeners[event]) return;
    eventListeners[event] = eventListeners[event].filter(l => l !== listener);
  }

  function removeAllEventListeners(event) {
    if (eventListeners[event]) {
      eventListeners[event] = [];
    }
  }

  function dispatchEvent(event, data) {
    if (eventListeners[event]) {
      eventListeners[event].forEach(listener => listener(data));
    }
  }

  PyOrionConnections.connect = function () {
    if (!__TEMPLATE_url__) {
      console.error("PyOrionConnections: No URL configured.");
      return;
    }

    shouldReconnect = config.autoReconnect;

    if (config.protocols) {
      ws = new WebSocket(__TEMPLATE_url__, config.protocols);
    } else {
      ws = new WebSocket(__TEMPLATE_url__);
    }

    ws.onopen = function (e) { dispatchEvent('open', e); };
    ws.onmessage = function (e) { dispatchEvent('message', e.data); };
    ws.onerror = function (e) { dispatchEvent('error', e); };
    ws.onclose = function (e) {
      dispatchEvent('close', e);
      if (shouldReconnect) {
        reconnectTimer = setTimeout(PyOrionConnections.connect, config.reconnectInterval);
      }
    };
  };

  PyOrionConnections.send = function (data) {
    if (ws && ws.readyState === WebSocket.OPEN) {

      ws.send(JSON.stringify(jsonMakeObjectSafe(data)));
    } else {
      console.warn("PyOrionConnections: Connection not open.");
    }
  };

  PyOrionConnections.close = function (code = 1000, reason = "Normal Closure") {
    shouldReconnect = false;
    clearTimeout(reconnectTimer);
    if (ws) { ws.close(code, reason); }
  };

  PyOrionConnections.is_connected = function () {
    return ws && ws.readyState === WebSocket.OPEN;
  };

  PyOrionConnections.is_disconnected = function () {
    return !ws || ws.readyState === WebSocket.CLOSED;
  };

  PyOrionConnections.on = addEventListener;
  PyOrionConnections.off = removeEventListener;
  PyOrionConnections.offAll = removeAllEventListeners;

  if (!window) { window = {}; }
  window.PyOrionConnections = PyOrionConnections;

  // === Automatisch Verbindung starten ===
  PyOrionConnections.connect();
})();
