// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * PyOrion frontend connection bootstrap.
 *
 * Provides a Promise-based `invoke(cmd, args)` API that communicates
 * with the backend via PyOrionConnections (WebSocket).
 *
 * Features:
 *  - Auto reconnect with configurable interval.
 *  - Unique ID mapping for result/error callbacks.
 *  - Automatic cleanup of one-time callbacks.
 *  - Global `window.invoke` helper for command dispatch.
 */
(function () {

  function uid() {
    return window.crypto.getRandomValues(new Uint32Array(1))[0];
  }

  function jsonMakeObjectSafe(obj) {
    if (obj === null || obj === undefined) return null;
    if (typeof obj === "string" || typeof obj === "number" || typeof obj === "boolean") return obj;

    if (obj instanceof Date) return obj.toISOString();

    if (typeof obj === "object" && obj.constructor && obj.constructor.name === "Path") {
      return String(obj);
    }

    if (obj instanceof ArrayBuffer) {
      return btoa(String.fromCharCode(...new Uint8Array(obj)));
    }

    if (ArrayBuffer.isView(obj)) {
      return btoa(String.fromCharCode(...obj));
    }

    if (Array.isArray(obj) || obj instanceof Set) {
      return Array.from(obj, (v) => jsonMakeObjectSafe(v));
    }

    if (typeof obj === "object" && obj.constructor === Object) {
      const safe = {};
      for (const [k, v] of Object.entries(obj)) {
        safe[String(k)] = jsonMakeObjectSafe(v);
      }
      return safe;
    }

    return String(obj);
  }

  /**
   * Restore JSON-safe objects back into native JS objects (sync).
   * Converts {media_type, bytes} into a usable data: URL.
   */
  function jsonRestoreObjectSync(obj) {
    if (obj === null || obj === undefined) return null;

    if (typeof obj === "string") {
      if (/^\d{4}-\d{2}-\d{2}T/.test(obj)) {
        const date = new Date(obj);
        if (!isNaN(date.getTime())) return date;
      }
      return obj;
    }

    if (Array.isArray(obj)) {
      return obj.map((v) => jsonRestoreObjectSync(v));
    }

    if (typeof obj === "object" && obj.constructor === Object) {
      // 🔑 Media object detected
      if (obj.media_type && obj.bytes) {
        return `data:${obj.media_type};base64,${obj.bytes}`;
      }

      const restored = {};
      for (const [k, v] of Object.entries(obj)) {
        restored[k] = jsonRestoreObjectSync(v);
      }
      return restored;
    }

    return obj;
  }

  /**
   * Restore JSON-safe objects back into native JS objects (async).
   * Converts {media_type, bytes} into a usable data: URL.
   */
  async function jsonRestoreObjectAsync(obj) {
    if (obj === null || obj === undefined) return null;

    if (typeof obj === "string") {
      if (/^data:.*;base64,/.test(obj)) {
        return obj;
      }
      return obj;
    }

    if (Array.isArray(obj)) {
      return Promise.all(obj.map((v) => jsonRestoreObjectAsync(v)));
    }

    if (typeof obj === "object" && obj.constructor === Object) {
      // 🔑 Media object detected
      if (obj.media_type && obj.bytes) {
        return `data:${obj.media_type};base64,${obj.bytes}`;
      }

      const restored = {};
      for (const [k, v] of Object.entries(obj)) {
        restored[k] = await jsonRestoreObjectAsync(v);
      }
      return restored;
    }

    return obj;
  }

  function transformCallback(callback, once) {
    const identifier = uid();
    const prop = `_${identifier}`;
    const isAsync = callback && callback.constructor && callback.constructor.name === "AsyncFunction";

    Object.defineProperty(window, prop, {
      value: async (result) => {
        if (once) Reflect.deleteProperty(window, prop);
        if (!callback) return;

        if (isAsync) {
          const restored = await jsonRestoreObjectAsync(result);
          return callback(restored);
        } else {
          const restored = jsonRestoreObjectSync(result);
          return callback(restored);
        }
      },
      writable: false,
      configurable: true,
    });

    return identifier;
  }

  function invoke(cmd, args) {
    return new Promise((resolve, reject) => {
      if (!PyOrionConnections.is_connected()) {
        reject(new Error("Socket is not connected or unavailable!"));
        return;
      }

      const py_args = jsonMakeObjectSafe(args ?? {});
      const result_id = transformCallback((result) => resolve(result), true);
      const error_id = transformCallback((error) => reject(error), true);

      const message = {
        cmd,
        result_id,
        error_id,
        payload: py_args,
      };

      PyOrionConnections.send(message);
    });
  }

  PyOrionConnections.on("message", (raw) => {
    try {
      const data = jsonMakeObjectSafe(JSON.parse(raw));
      const { result_id, error_id, result, error } = data;

      if (result_id) {
        const prop = `_${result_id}`;
        if (window[prop]) window[prop](result);
      }

      if (error_id) {
        const prop = `_${error_id}`;
        if (window[prop]) window[prop](error);
      }
    } catch (err) {
      console.warn("Invalid message received:", raw);
    }
  });

  window.invoke = invoke;
})();
