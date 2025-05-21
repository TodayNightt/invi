use serde::{Deserialize, Serialize};

use crate::Field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    name: String,
    fields: Vec<Field>,
}

impl Schema {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn create(name: &str, fields: Vec<Field>) -> Self {
        Schema {
            name: name.to_string(),
            fields,
        }
    }
}
