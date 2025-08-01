use std::{
    collections::{BTreeMap, HashMap},
    path::Display,
    str::FromStr,
};

use serde::{Deserialize, Serialize, de};

use crate::ValueStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    field_type: FieldType,
    required: bool,
    default: Value,
}

impl Field {
    pub fn required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> &Value {
        &self.default
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

    pub fn create(name: &str, field_type: FieldType, required: bool, default: Value) -> Self {
        Field {
            name: name.to_string(),
            field_type,
            required,
            default,
        }
    }
}

/// This is a custom implementation of the 'Value' type apart from the 'serde_json' crate.
/// As I want to create custom API for this type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    // Note : Every measurement will be using millimeters(mm) as the base unit
    Number(u64),
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn as_str(&self) -> Option<String> {
        if let Value::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<u64> {
        if let Value::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Object(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn to_value_string(&self) -> String {
        match self {
            Value::String(v) => v.clone(),
            Value::Number(n) => n.to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(a) => serde_json::to_string_pretty(a).unwrap(),
            Value::Object(o) => serde_json::to_string_pretty(o).unwrap(),
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

impl FieldType {
    pub fn is_valid(&self, value: &Value) -> bool {
        matches!(
            (self, value),
            (FieldType::String, Value::String(_))
                | (FieldType::Number, Value::Number(_))
                | (FieldType::Boolean, Value::Boolean(_))
                | (FieldType::Array, Value::Array(_))
                | (FieldType::Object, Value::Object(_))
        )
    }
}
