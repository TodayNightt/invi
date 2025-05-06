use std::collections::HashMap;

use crate::types::Property;

mod de;
pub mod info;
mod ser;
pub mod wrapper;
// This struct skip the "enum" type as it is not possible to represent it in rust
// spec :  https://json-schema.org/learn/miscellaneous-examples#enumerated-values
#[derive(Clone, Debug)]
pub enum SchemaType {
    String,
    Integer {
        min: Option<i64>,
        max: Option<i64>,
    },
    Float {
        min: Option<f64>,
        max: Option<f64>,
    },
    Boolean,
    Array(Box<SchemaType>),
    Object {
        required: Vec<String>,
        properties: HashMap<String, Property>,
    },
}
