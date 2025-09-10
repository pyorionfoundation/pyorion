# Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

"""Command registration and callback dispatch system.

This module provides a lightweight framework for:

* Registering Python functions as command handlers using :func:`command`.
* Validating incoming event payloads against type hints with Pydantic.
* Supporting dependency injection for constructor-less types.
* Executing the correct callback for a given event with error handling.

It is the central mechanism for bridging frontend events into Python
logic, with optional async execution and input validation.
"""

import asyncio
import inspect
from collections.abc import Awaitable
from typing import Annotated, Any, Callable, cast, get_args, get_origin, get_type_hints

from pydantic import ValidationError, create_model


_event_callbacks: dict[str, list[Callable[..., None | Awaitable[None]]]] = {}
_dependency_cache: dict[type, Any] = {}


def command(func_or_event: str | Callable | None = None) -> Callable:
    """Register a function as an event command handler.

    Supports three usage patterns:

    1. **Explicit event name**::

        @command("greet")
        def my_handler(name: str) -> str:
            return f"Hello {name}!"

    2. **Implicit event name (function name is used)**::

        @command
        def ping() -> str:
            return "pong"

    3. **Inline registration (direct call)**::

        command("use_service")(lambda svc: f"Hello {svc.msg}")

    :param func_or_event: Event name as string, or function to register.
    :type func_or_event: str | Callable | None
    :return: The decorated function if used as a decorator, otherwise
             the decorator closure.
    :rtype: Callable
    """
    if callable(func_or_event):  # @command
        func = func_or_event
        key = func.__name__
        _event_callbacks.setdefault(key, []).append(func)
        return func

    def decorator(func: Callable):  # @command("event") or command("event")(func)
        key = func_or_event or func.__name__
        _event_callbacks.setdefault(key, []).append(func)
        return func

    return decorator


def _resolve_final_type(tp: Any) -> Any:
    """Resolve the base type from complex type hints."""
    origin = get_origin(tp)

    if origin is None:
        return tp
    if origin is Annotated:
        return get_args(tp)[0]
    return origin if origin else tp


async def resolve_dependency(dep_type: type) -> Any:
    """Resolve or instantiate a dependency type."""
    if dep_type in _dependency_cache:
        return _dependency_cache[dep_type]
    try:
        instance = dep_type()
        if asyncio.iscoroutine(instance):
            instance = await instance
        _dependency_cache[dep_type] = instance
        return instance
    except Exception as exc:
        msg = f"Cannot instantiate type '{dep_type}': {exc}"
        raise RuntimeError(msg) from exc


async def _collect_parameters(
    func: Callable, data: dict[str, Any]
) -> tuple[dict[str, Any], dict[str, tuple[Any, Any]], list[str]]:
    """Inspect function signature and prepare parameters for validation and execution."""
    sig = inspect.signature(func)
    type_hints = get_type_hints(func)
    values: dict[str, Any] = {}
    data_fields: dict[str, tuple[Any, Any]] = {}
    errors: list[str] = []

    for name, param in sig.parameters.items():
        hint = type_hints.get(name, Any)
        final_type = _resolve_final_type(hint)
        is_dependency = inspect.isclass(final_type) and not isinstance(
            final_type, type(Any)
        )

        if name in data:
            data_fields[name] = (final_type, ...)
            values[name] = data[name]
        elif param.default is not inspect.Parameter.empty:
            data_fields[name] = (final_type, param.default)
        elif is_dependency:
            try:
                values[name] = await resolve_dependency(final_type)
            except Exception as exc:
                errors.append(f"{name}: {exc}")
        else:
            errors.append(f"{name} is missing")

    return values, data_fields, errors


def _validate_with_pydantic(
    func: Callable, values: dict[str, Any], data_fields: dict[str, tuple[Any, Any]]
) -> dict[str, Any] | None:
    """Validate collected parameters using a dynamic Pydantic model."""
    if not data_fields:
        return None

    field_defs = dict(data_fields)
    model_cls = create_model(  # type: ignore[call-overload]
        f"{func.__name__}_Validator",
        **cast(dict[str, tuple[Any, Any]], field_defs),
    )

    validated = model_cls(**{k: values[k] for k in data_fields})
    for field in data_fields:
        values[field] = getattr(validated, field)
    return values


async def make_callback(
    event: str,
    result_id: int,
    error_id: int,
    data: dict[str, Any],
) -> dict[str, Any]:
    """Execute a registered callback for a given event."""
    for func in _event_callbacks.get(event, []):
        values, data_fields, errors = await _collect_parameters(func, data)
        if errors:
            return {
                "error_id": error_id,
                "error": f"Invalid parameters: {', '.join(errors)}",
            }

        try:
            values = _validate_with_pydantic(func, values, data_fields) or values
        except ValidationError as exc:
            return {"error_id": error_id, "error": f"Pydantic validation failed: {exc}"}

        try:
            result = (
                await func(**values)
                if inspect.iscoroutinefunction(func)
                else func(**values)
            )
            return {"result_id": result_id, "result": result}
        except Exception as exc:
            return {"error_id": error_id, "error": str(exc)}

    return {"error_id": error_id, "error": f"No handler registered for event '{event}'"}
