use serde::{Deserialize, Serialize};

use crate::types::rs_schema::types::wrapper::TypeWrapper;

// Helper struct for Array type
#[derive(Serialize)]
pub(in crate::types) struct ArrayTypeInfo {
    #[serde(rename = "type")]
    type_name: &'static str,
    items: TypeWrapper,
}

impl ArrayTypeInfo {
    pub fn new(type_name: &'static str, items: TypeWrapper) -> Self {
        ArrayTypeInfo { type_name, items }
    }
}
