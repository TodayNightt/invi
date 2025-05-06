use serde::{Deserialize, Serialize};

// Helper struct for Float type with min/max
#[derive(Serialize, Deserialize)]
pub(in crate::types) struct IntegerTypeInfo {
    #[serde(rename = "type")]
    type_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i64>,
}

impl IntegerTypeInfo {
    pub fn new(type_name: &'static str, min: Option<i64>, max: Option<i64>) -> Self {
        IntegerTypeInfo {
            type_name,
            min,
            max,
        }
    }
}
