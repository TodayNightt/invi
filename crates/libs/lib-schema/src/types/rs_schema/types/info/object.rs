use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::rs_schema::types::wrapper::TypeWrapper;

// Helper struct for Object type
#[derive(Serialize)]
pub(in crate::types) struct ObjectTypeInfo {
    #[serde(rename = "type")]
    type_name: &'static str,
    required: Vec<String>,
    properties: HashMap<String, TypeWrapper>,
}

impl ObjectTypeInfo {
    pub fn new(
        type_name: &'static str,
        required: Vec<String>,
        properties: HashMap<String, TypeWrapper>,
    ) -> Self {
        ObjectTypeInfo {
            type_name,
            required,
            properties,
        }
    }
}
