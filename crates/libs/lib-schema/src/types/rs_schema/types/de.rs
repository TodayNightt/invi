use std::fmt;

use serde::{
    Deserialize,
    de::{self, Visitor},
};

use super::SchemaType;

impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SchemaTypeVisitor;

        impl<'de> Visitor<'de> for SchemaTypeVisitor {
            type Value = SchemaType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid SchemaType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<SchemaType, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut type_name = None;
                let mut required = None;
                let mut properties = None;
                let mut min_int = None;
                let mut max_int = None;
                let mut min_float = None;
                let mut max_float = None;
                let mut items = None;

                // Deserialize the object map for schema type fields
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => {
                            let value: String = map.next_value()?;
                            type_name = Some(value);
                        }
                        "required" => {
                            required = Some(map.next_value()?);
                        }
                        "properties" => {
                            properties = Some(map.next_value()?);
                        }
                        "min" => {
                            // Try to deserialize min as either integer or float
                            if let Ok(min) = map.next_value::<i64>() {
                                min_int = Some(min);
                            } else if let Ok(min) = map.next_value::<f64>() {
                                min_float = Some(min);
                            }
                        }
                        "max" => {
                            // Try to deserialize max as either integer or float
                            if let Ok(max) = map.next_value::<i64>() {
                                max_int = Some(max);
                            } else if let Ok(max) = map.next_value::<f64>() {
                                max_float = Some(max);
                            }
                        }
                        "items" => {
                            items = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                match type_name.as_deref() {
                    Some("string") => Ok(SchemaType::String),
                    Some("boolean") => Ok(SchemaType::Boolean),
                    Some("integer") => Ok(SchemaType::Integer {
                        min: min_int,
                        max: max_int,
                    }),
                    Some("number") => Ok(SchemaType::Float {
                        min: min_float,
                        max: max_float,
                    }),
                    Some("array") => Ok(SchemaType::Array(Box::new(
                        items.ok_or_else(|| de::Error::missing_field("items"))?,
                    ))),
                    Some("object") => Ok(SchemaType::Object {
                        required: required.ok_or_else(|| de::Error::missing_field("required"))?,
                        properties: properties
                            .ok_or_else(|| de::Error::missing_field("properties"))?,
                    }),
                    _ => Err(de::Error::unknown_variant(
                        type_name.as_deref().unwrap_or("unknown"),
                        &["string", "boolean", "integer", "number", "array", "object"],
                    )),
                }
            }
        }

        deserializer.deserialize_map(SchemaTypeVisitor)
    }
}
