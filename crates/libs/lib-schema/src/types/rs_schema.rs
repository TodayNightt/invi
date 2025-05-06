use std::collections::HashMap;

use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, Visitor},
    ser::{SerializeMap, SerializeStruct},
};
use serde_json::{Map, Value, json};
use types::{SchemaType, wrapper::TypeWrapper};

pub mod property;

pub mod types;

// This is the root of the schema as the type will only be an "object" and has the required fields
#[derive(Clone, Debug, Deserialize)]
pub struct RsSchema {
    pub title: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub schema_type: SchemaType,
}

// Implement Serialize for Schema
impl Serialize for RsSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut schema = serializer.serialize_map(Some(4))?;

        // // Add standard JSON Schema fields
        // schema.serialize_entry("$schema", "http://json-schema.org/draft-07/schema#")?;
        schema.serialize_entry("title", &self.title)?;

        if let Some(desc) = &self.description {
            schema.serialize_entry("description", desc)?;
        }

        // Serialize the root type directly into the schema
        match &self.schema_type {
            SchemaType::Object {
                required,
                properties,
            } => {
                schema.serialize_entry("type", "object")?;
                schema.serialize_entry("required", &required)?;

                // Convert Property map to TypeWrapper map for properties
                let mut wrapped_properties = HashMap::new();
                for (name, prop) in properties {
                    wrapped_properties
                        .insert(name.clone(), TypeWrapper::new(prop.property_type.clone()));
                }

                schema.serialize_entry("properties", &wrapped_properties)?;
            }
            _ => {
                // For non-object root types, just add the type info
                schema.serialize_entry("type", &self.schema_type)?;
            }
        }

        schema.end()
    }
}

// impl<'de> Deserialize<'de> for RsSchema {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct SchemaVisitor;

//         impl<'de> Visitor<'de> for SchemaVisitor {
//             type Value = RsSchema;
//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 todo!()
//             }

//             fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'de>,
//             {
//                 let mut title = None;
//                 let mut description = None;
//                 // Note : I just assume that the root schema are a object
//                 let mut type_name = None;
//                 let mut required = None;
//                 let mut properties = None;

//                 while let Some(key) = map.next_key::<String>()? {
//                     match key.as_str() {
//                         "title" => {
//                             let val: String = map.next_value()?;
//                             title = Some(val)
//                         }
//                         "description" => {
//                             let val: String = map.next_value()?;
//                             description = Some(val)
//                         }
//                         "type" => {
//                             let value: String = map.next_value()?;
//                             type_name = Some(value);
//                         }
//                         "required" => {
//                             required = Some(map.next_value()?);
//                         }
//                         "properties" => {
//                             properties = Some(map.next_value()?);
//                         }
//                         _ => {
//                             let _: serde::de::IgnoredAny = map.next_value()?;
//                         }
//                     }
//                 }
//                 let Some(type_name) = type_name else {
//                     return Err(de::Error::missing_field("type"));
//                 };

//                 println!("schema_visitor : {}", type_name);

//                 if type_name.ne("object") {
//                     return Err(de::Error::invalid_value(
//                         de::Unexpected::Str(&type_name),
//                         &"object",
//                     ));
//                 }
//                 let schema_type = SchemaType::Object {
//                     required: required.ok_or_else(|| de::Error::missing_field("required"))?,
//                     properties: properties.ok_or_else(|| de::Error::missing_field("properties"))?,
//                 };

//                 Ok(RsSchema {
//                     title: title.ok_or_else(|| de::Error::missing_field("title"))?,
//                     description,
//                     schema_type,
//                 })
//             }
//         }

//         deserializer.deserialize_map(SchemaVisitor)
//     }
// }
