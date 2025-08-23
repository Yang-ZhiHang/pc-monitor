use std::collections::HashMap;

pub fn jsonify<K, V>(mp: &HashMap<K, V>) -> String
where
    K: serde::Serialize,
    V: serde::Serialize,
{
    serde_json::to_string_pretty(&mp).unwrap_or_default()
}
