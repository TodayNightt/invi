use std::collections::HashMap;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

use super::{
    SchemaType,
    info::{
        TypeInfo, array::ArrayTypeInfo, float::FloatTypeInfo, integer::IntegerTypeInfo,
        object::ObjectTypeInfo,
    },
};

// Wrapper for Type to handle the nested serialization
pub struct TypeWrapper {
    inner: SchemaType,
}

impl TypeWrapper {
    pub fn new(inner: SchemaType) -> Self {
        TypeWrapper { inner }
    }
}

impl Serialize for TypeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.inner {
            SchemaType::String => {
                let type_info = TypeInfo::new("string");
                type_info.serialize(serializer)
            }

            SchemaType::Boolean => {
                let type_info = TypeInfo::new("boolean");
                type_info.serialize(serializer)
            }
            SchemaType::Integer { min, max } => {
                let type_info = IntegerTypeInfo::new("integer", *min, *max);

                type_info.serialize(serializer)
            }
            SchemaType::Float { min, max } => {
                let type_info = FloatTypeInfo::new("number", *min, *max);

                type_info.serialize(serializer)
            }
            SchemaType::Array(types) => {
                let inner_wrapper = TypeWrapper {
                    inner: (**types).clone(),
                };

                let type_info = ArrayTypeInfo::new("array", inner_wrapper);

                type_info.serialize(serializer)
            }

            SchemaType::Object {
                required,
                properties,
            } => {
                // Convert Property map to TypeWrapper map
                let mut wrapped_properties = HashMap::new();
                for (name, prop) in properties {
                    wrapped_properties.insert(
                        name.clone(),
                        TypeWrapper {
                            inner: prop.property_type.clone(),
                        },
                    );
                }

                let type_info = ObjectTypeInfo::new("object", required.clone(), wrapped_properties);
                type_info.serialize(serializer)
            }
        }
    }
}
