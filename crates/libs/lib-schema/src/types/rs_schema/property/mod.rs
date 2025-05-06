use serde::Deserialize;

use crate::types::SchemaType;

mod se;

#[derive(Clone, Debug, Deserialize)]
pub struct Property {
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: SchemaType,
    pub default_value: Option<String>,
}
