# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Runtime handle for communication with the Rust event loop.

This module defines request/response models, error types, and
the asynchronous registry/dispatch logic used to send and
receive messages between Python and the Rust backend.
"""

import asyncio
import json
import traceback
from typing import Any, Callable, Optional, TypeVar, cast

from pydantic import BaseModel

from pyorion._pyorion import send_event_over_platform
from pyorion.utils import make_json_safe, normalize_args


R = TypeVar("R", bound=BaseModel)
T = TypeVar("T")


class ApiRequestModel(BaseModel):
    """Representation of a request sent to the Rust event loop."""

    id: int
    method: str
    args: list[Any]

    def to_json_array(self) -> list[Any]:
        """Convert the request into a JSON-safe array format.

        The `args` list is normalized via :func:`make_json_safe`.

        :return: List representation ``[id, method, args]`` suitable for JSON.
        :rtype: list[Any]
        """
        return [self.id, self.method, [make_json_safe(a) for a in self.args]]


class ApiResponseModel(BaseModel):
    """Representation of a response returned from the Rust event loop."""

    id: int
    code: int
    msg: str
    result: Any

    @classmethod
    def from_array(cls, arr: list[Any]) -> "ApiResponseModel":
        """Construct a response model from an array received from Rust.

        :param arr: List of four elements: ``[id, code, msg, result]``.
        :type arr: list[Any]
        :return: Parsed response model.
        :rtype: ApiResponseModel
        :raises ValueError: If the array format is invalid.
        """
        if not isinstance(arr, list) or len(arr) != 4:
            raise ValueError(f"Invalid ApiResponse array: {arr}")
        return cls(id=arr[0], code=arr[1], msg=arr[2], result=arr[3])


class ApiError(Exception):
    """Error raised when the Rust event loop returns a non-zero status code."""

    def __init__(self, code: int, msg: str) -> None:
        """Initialize the API error.

        :param code: Numeric error code returned by Rust.
        :type code: int
        :param msg: Human-readable error message.
        :type msg: str
        """
        super().__init__(f"[API-{code}] {msg}")
        self.code = code
        self.msg = msg


class PendingRegistry:
    """Registry for managing in-flight requests and their associated futures."""

    def __init__(self, max_id: int = 255) -> None:
        """Initialize the registry with a maximum ID limit."""
        self._pending: dict[int, asyncio.Future[Any]] = {}
        self._counter: int = 0
        self._max_id = max_id

    def next_id(self) -> int:
        """Return the next free request ID, cycling with wrap-around logic."""
        for _ in range(self._max_id):
            self._counter = (self._counter + 1) % self._max_id
            if self._counter not in self._pending:
                return self._counter
        raise RuntimeError("No free request IDs available")

    def register(self, req_id: int, future: asyncio.Future[Any]) -> None:
        """Register a future for a given request ID."""
        self._pending[req_id] = future

    def pop(
        self, req_id: int, default: Any | None = None
    ) -> asyncio.Future[Any] | None:
        """Remove and return the future associated with a request ID."""
        return self._pending.pop(req_id, default)

    def resolve(
        self,
        req_id: int,
        result: Any = None,
        error: Exception | None = None,
    ) -> None:
        """Resolve the future for a request ID with result or error."""
        future = self._pending.pop(req_id, None)
        if future and not future.done():
            if error:
                future.set_exception(error)
            else:
                future.set_result(result)

    def cancel_all(self, exc: Exception | None = None) -> None:
        """Cancel all registered futures, optionally with an exception."""
        for fut in self._pending.values():
            if not fut.done():
                if exc:
                    fut.set_exception(exc)
                else:
                    fut.cancel()
        self._pending.clear()


_pending = PendingRegistry()
task_queue: asyncio.Queue[dict[str, Any]] = asyncio.Queue()


async def send_loop_event(data: list[Any]) -> list[Any] | None:
    """Send an event to the Rust event loop over the platform bridge."""
    try:
        response_str: Optional[Any] = await send_event_over_platform(
            name="pyframe_pipe",
            message=json.dumps(data),
        )

        if response_str is None:
            return None
        return response_str

    except Exception as exc:
        print(f"Connection error: {exc}")
        traceback.print_exc()
        return None


async def handle_event_loop_response(
    arr: list[Any], future: asyncio.Future[Any] | None = None
) -> None:
    """Process a single response from the Rust event loop."""
    resp = ApiResponseModel.from_array(arr)
    if future:
        if resp.code != 0:
            future.set_exception(ApiError(resp.code, resp.msg))
        else:
            future.set_result(resp.result)
    else:
        _pending.resolve(
            resp.id,
            error=ApiError(resp.code, resp.msg) if resp.code != 0 else None,
            result=None if resp.code != 0 else resp.result,
        )


async def eventloop_sender() -> None:
    """Continuous dispatcher loop for sending tasks to the Rust event loop."""
    try:
        while True:
            task = await task_queue.get()
            future: asyncio.Future[Any] | None = task.pop("future", None)
            data: list[Any] | None = task.get("data")
            if data is None:
                return
            try:
                arr = await send_loop_event(data)
                if arr:
                    await handle_event_loop_response(arr, future=future)
            except Exception as exc:
                if future and not future.done():
                    future.set_exception(exc)
            finally:
                await asyncio.sleep(0.01)

    except asyncio.CancelledError as exc:
        print(f"ui_endless_event_loop_tasks() cancelled: {exc}")
    finally:
        print("gui_endless_event_loop_tasks() terminated.")
        _pending.cancel_all(RuntimeError("Event loop terminated"))


# Helper to avoid B008 (cast() in default arg)
def _identity(x: Any) -> Any:
    """Default identity function for result_type in event_register."""
    return x


async def event_register(
    method: str,
    args: Any | None = None,
    result_type: type[R] | Callable[[Any], T] = _identity,
) -> R | T:
    """Send a typed request to the Rust event loop and wait for a typed response.

    This is the main high-level API for Python callers. It automatically:

    * Allocates a unique request ID.
    * Serializes the request into JSON-safe format.
    * Registers a pending future in :class:`PendingRegistry`.
    * Waits for the result with timeout handling.
    * Converts the raw result into the desired type.

    :param method: Name of the Rust API method to call.
    :type method: str
    :param args: Optional arguments for the method, normalized automatically.
    :type args: Any | None
    :param result_type: Expected result type. Can be:
                        * A subclass of :class:`pydantic.BaseModel` (validated).
                        * A callable for custom transformations.
                        * Any type for raw passthrough.
    :type result_type: type[BaseModel] | Callable | Any
    :return: Parsed and typed response.
    :rtype: R | T
    :raises ApiError: If the Rust backend returned an error code.
    :raises asyncio.TimeoutError: If no response is received in time.
    :raises Exception: For connection or serialization errors.
    """
    req_id = _pending.next_id()
    request = ApiRequestModel(id=req_id, method=method, args=normalize_args(args))
    future: asyncio.Future[Any] = asyncio.get_event_loop().create_future()
    _pending.register(req_id, future)

    await task_queue.put({"data": request.to_json_array(), "future": future})

    try:
        raw_result = await asyncio.wait_for(future, timeout=10.0)

        if isinstance(result_type, type) and issubclass(result_type, BaseModel):
            if hasattr(result_type, "model_validate"):  # Pydantic v2
                return cast(R, result_type.model_validate(raw_result))
            return cast(R, result_type.parse_obj(raw_result))  # Pydantic v1

        elif callable(result_type) and not isinstance(result_type, type):
            return cast(T, result_type(raw_result))

        return cast(R | T, raw_result)

    except Exception:
        _pending.pop(req_id)
        raise
    finally:
        _pending.pop(req_id, None)
