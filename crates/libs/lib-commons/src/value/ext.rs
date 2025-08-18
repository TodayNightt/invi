use crate::Value;
use std::collections::{BTreeMap, HashMap};

pub trait CommonsValue {
    fn as_commons_value(&self) -> Value;
    fn as_hash_map(&self) -> HashMap<String, Value>;
}

impl CommonsValue for serde_json::Value {
    fn as_commons_value(&self) -> Value {
        match self.clone() {
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Number(n) => Value::Number(n.as_u64().unwrap()),
            serde_json::Value::Object(o) => {
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k, Value::from(v));
                }
                Value::Object(map)
            }
            serde_json::Value::Array(a) => {
                let vec: Vec<Value> = a.into_iter().map(Value::from).collect();
                Value::Array(vec)
            }
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Null => Value::Null,
        }
    }

    fn as_hash_map(&self) -> HashMap<String, Value> {
        if let serde_json::Value::Object(map) = self.clone() {
            map.into_iter().map(|(k, v)| (k, v.as_commons_value())).collect()
        } else {
            HashMap::default()
        }
    }
}
