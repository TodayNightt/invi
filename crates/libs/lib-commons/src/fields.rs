use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Value;

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

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_field_type(&mut self, field_type: FieldType) {
        self.field_type = field_type;
    }

    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }

    pub fn set_default(&mut self, default: Value) {
        self.default = default;
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
