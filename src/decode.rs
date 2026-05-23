//! Types and traits for decoding values from the database.

use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,i64,bool,String
/// values = [{k:v},...]
/// T = Vec<YourStruct>
pub fn decode_ref<T>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    match values {
        Value::Array(_) => {
            // First try rbs direct decode. This handles [{k:v},...] -> Vec<T>.
            // The fallback only unwraps single-column rows for scalar values.
            match rbs::from_value_ref::<T>(values) {
                Ok(v) => Ok(v),
                Err(e) => match try_decode_single_column(values) {
                    Ok(v) => Ok(v),
                    Err(Some(fallback_err)) => Err(fallback_err),
                    Err(None) => Err(e.into()),
                },
            }
        }
        _ => Err(Error::from("decode error: expected array value")),
    }
}

pub fn decode<T>(bs: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    decode_ref(&bs)
}

/// Decode single-column query results.
/// Unwraps single-element containers ({k:v} -> v, [v] -> v) so scalar and
/// Option<scalar> targets can be decoded without allowing single-row struct
/// decoding.
pub fn try_decode_elements<T: DeserializeOwned>(datas: &Value) -> Result<T, Error> {
    try_decode_single_column(datas).map_err(|e| {
        e.unwrap_or_else(|| Error::from("decode error: unsupported single row struct decode"))
    })
}

fn try_decode_single_column<T: DeserializeOwned>(datas: &Value) -> Result<T, Option<Error>> {
    let items = match datas {
        Value::Array(items) => items,
        _ => return rbs::from_value_ref::<T>(datas).map_err(|e| Some(e.into())),
    };

    let mut last_err = None;
    for item in items {
        let inner = match item {
            Value::Map(m) if m.len() == 1 => m.into_iter().next().map(|(_, v)| v),
            Value::Array(a) if a.len() == 1 => Some(&a[0]),
            _ => None,
        };
        if let Some(v) = inner {
            if let Ok(r) = rbs::from_value_ref::<T>(v) {
                return Ok(r);
            }
            last_err = rbs::from_value_ref::<T>(v).err().map(Error::from);
        }
    }

    Err(last_err)
}

pub fn is_debug_mode() -> bool {
    cfg!(all(debug_assertions, feature = "debug_mode"))
}
