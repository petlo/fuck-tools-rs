use serde_json::Value;
use std::collections::BTreeMap;

pub struct SortTools;

impl SortTools {
    pub fn sort_json_by_key(json_str: &str) -> Result<String, serde_json::Error> {
        let value: Value = serde_json::from_str(json_str)?;
        if let Value::Object(obj) = value {
            let mut sorted_map = BTreeMap::new();
            for (key, val) in obj {
                sorted_map.insert(key, val);
            }
            let sorted_value = Value::Object(sorted_map.into_iter().collect());
            Ok(serde_json::to_string(&sorted_value)?)
        } else {
            Ok(json_str.to_string())
        }
    }
}
