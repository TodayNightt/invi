use jsonschema::Validator;
use serde_json::Value;

pub struct JsonSchema {
    value: Value,
    validator: Validator,
}

impl JsonSchema {
    pub fn new(value: Value, validator: Validator) -> Self {
        Self { value, validator }
    }
    pub fn schema(&self) -> &Value {
        &self.value
    }
    pub fn validator(&self) -> &Validator {
        &self.validator
    }
}
