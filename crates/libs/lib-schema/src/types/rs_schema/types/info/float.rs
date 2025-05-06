use serde::{Deserialize, Serialize};

// Helper struct for Float type with min/max
#[derive(Serialize, Deserialize)]
pub(in crate::types) struct FloatTypeInfo {
    #[serde(rename = "type")]
    type_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
}

impl FloatTypeInfo {
    pub fn new(type_name: &'static str, min: Option<f64>, max: Option<f64>) -> Self {
        FloatTypeInfo {
            type_name,
            min,
            max,
        }
    }
}
