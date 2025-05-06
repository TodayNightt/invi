use jsonschema::Validator;
use serde_json::Value;
use std::collections::HashMap;

use crate::{Error, Result, types::JsonSchema};

pub struct SchemaRegistry {
    schemas: HashMap<String, JsonSchema>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn register_schema(&mut self, name: &str, value: &Value) -> Result<()> {
        let validator =
            Validator::new(value).map_err(|e| Error::FailToCreateValidator(e.into()))?;

        let schema = JsonSchema::new(value.clone(), validator);

        self.schemas.insert(name.to_owned(), schema);
        Ok(())
    }

    pub fn get_schema(&self, name: &str) -> Option<&Value> {
        self.schemas.get(name).map(|schema| schema.schema())
    }

    pub fn validate(&self, name: &str, data: &Value) -> Result<()> {
        if let Some(schema) = self.schemas.get(name) {
            let validator = schema.validator();
            if !validator.is_valid(data) {
                let errors = validator.iter_errors(data);
                Err(Error::ValidateError(errors.map(|e| e.into()).collect()))
            } else {
                Ok(())
            }
        } else {
            Err(Error::UnknownType(name.to_string()))
        }
    }
}
