use serde::{Deserialize, Serialize};

pub mod array;
pub mod float;
pub mod integer;
pub mod object;

// Helper struct to represent the inner type format
#[derive(Serialize, Deserialize)]

pub(in crate::types) struct TypeInfo<'a> {
    #[serde(rename = "type")]
    type_name: &'a str,
}

impl<'a> TypeInfo<'a> {
    pub fn new(type_name: &'a str) -> Self {
        TypeInfo { type_name }
    }
}
