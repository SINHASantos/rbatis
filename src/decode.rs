//! Types and traits for decoding values from the database.

use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,i64,bool,String
/// or object used rbs::Value macro object
/// values = [{k:v},...]
/// T = Vec<YourStruct>
pub fn decode_ref<T>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    match values {
        Value::Array(_) => {
            // First try rbs direct decode (handles [{k:v},...] format to Vec<T>)
            rbs::from_value_ref::<T>(values).or_else(|_| try_decode_elements(values))
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

/// Iterate over all elements, trying each as T.
/// Unwraps single-element containers ({k:v} → v, [v] → v) so that
/// the raw rbs type error propagates instead of a misleading wrapper.
pub fn try_decode_elements<T: DeserializeOwned>(datas: &Value) -> Result<T, Error> {
    let items = match datas {
        Value::Array(items) => items,
        _ => return rbs::from_value_ref::<T>(datas).map_err(Into::into),
    };

    let mut last_err = None;
    for item in items {
        if let Ok(r) = rbs::from_value_ref::<T>(item) {
            return Ok(r);
        }
        let is_map = matches!(item, Value::Map(_));
        last_err = rbs::from_value_ref::<T>(item).err().map(Error::from);

        // Unwrap single-element containers so rbs produces the raw type error
        let inner = match item {
            Value::Map(m) if m.len() == 1 => m.into_iter().next().map(|(_, v)| v),
            Value::Array(a) if a.len() == 1 => Some(&a[0]),
            _ => None,
        };
        if let Some(v) = inner {
            if let Ok(r) = rbs::from_value_ref::<T>(v) {
                return Ok(r);
            }
            if !is_map {
                last_err = rbs::from_value_ref::<T>(v).err().map(Error::from);
            }
        }
    }

    Err(last_err.unwrap_or_else(|| Error::from("decode fail")))
}

pub fn is_debug_mode() -> bool {
    cfg!(all(debug_assertions, feature = "debug_mode"))
}
