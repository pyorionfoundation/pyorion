// Copyright 2025-2030 Ari Bermeki @ YellowSiC within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use pyo3::{prelude::*, types::PyList, BoundObject};

pub fn json_to_py<'a>(py: Python<'a>, payload_value: &'a str) -> PyResult<Py<PyAny>> {
    let payload: serde_json::Value = serde_json::from_str(payload_value)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    match payload {
        serde_json::Value::Null => Ok(py.None()),

        serde_json::Value::Bool(b) => Ok(b.into_pyobject(py)?.clone().into_any().unbind()),

        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_u64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = num.as_i128() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = num.as_i64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = num.as_u128() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err("Invalid number"))
            }
        }

        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),

        serde_json::Value::Array(arr) => {
            let list: Vec<Py<PyAny>> = arr
                .iter()
                .map(|v| json_to_py(py, &v.to_string()))
                .collect::<PyResult<_>>()?;
            Ok(PyList::new(py, list).unwrap().into_any().into())
        }

        serde_json::Value::Object(map) => {
            let dict = pyo3::types::PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py(py, &v.to_string())?)?;
            }
            Ok(dict.into())
        }
    }
}
