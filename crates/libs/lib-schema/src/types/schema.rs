use std::collections::HashMap;

use serde::{Deserialize, Serialize, ser::SerializeMap};

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    name: String,
    description: Option<String>,
    #[serde(flatten)]
    root_schema: Type,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Field {
    #[serde(flatten)]
    field_type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    #[default]
    String,
    Boolean,
    Float,
    Integer,
    Array(Box<Type>),
    Object {
        required: Vec<String>,
        properties: HashMap<String, Field>,
    },
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => f.write_str("string"),
            Type::Boolean => f.write_str("boolean"),
            Type::Integer => f.write_str("integer"),
            Type::Float => f.write_str("number"),
            Type::Array(_) => f.write_str("array"),
            Type::Object {
                required: _,
                properties: _,
            } => f.write_str("object"),
        }
    }
}
pub trait InnerInfo {
    type RetVal;

    fn val(self) -> Self::RetVal;
}

#[derive(Debug, Clone, Serialize)]
pub struct ArrayType {
    #[serde(rename = "type")]
    type_name: String,
    items: Type,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectType {
    #[serde(rename = "type")]
    type_name: String,
    required: Vec<String>,
    properties: HashMap<String, Field>,
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Type::String | Type::Integer | Type::Boolean | Type::Float => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", &self.to_string());
                map.end()
            }
            Type::Array(inner) => {
                let type_info = ArrayType {
                    type_name: "array".to_string(),
                    items: (**inner).clone(),
                };
                type_info.serialize(serializer)
            }
            Type::Object {
                required,
                properties,
            } => {
                let type_info = ObjectType {
                    type_name: "object".to_string(),
                    required: required.clone(),
                    properties: properties.clone(),
                };

                type_info.serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn test_schema_serialization() {
        let schema = Schema {
            name: "Test".to_string(),
            description: Some("Test schema".to_string()),
            root_schema: Type::Object {
                required: vec!["name".to_string()],
                properties: HashMap::from([
                    (
                        "name".to_string(),
                        Field {
                            field_type: Type::String,
                            ..Default::default()
                        },
                    ),
                    (
                        "tags".to_string(),
                        Field {
                            field_type: Type::Array(Box::new(Type::Integer)),
                            ..Default::default()
                        },
                    ),
                ]),
            },
        };

        let serialized = serde_json::to_string_pretty(&schema).unwrap();
        let expected = json!({
            "name": "Test",
            "description": "Test schema",
            "type": "object",
            "required": ["name"],
            "properties": {
                "name" : {
                    "type" : "string"
                },
                "tags": {
                    "type" : "array",
                    "items" : {
                        "type" : "integer"
                    }
                }
            }
        });

        println!("{}", &serialized);

        assert_eq!(
            serde_json::from_str::<Value>(&serialized).unwrap(),
            expected
        );
    }

    #[test]
    fn test_schema_deserialization() {
        let json_str = r#"
        {
            "name": "Test",
            "description": "Test schema",
            "type": "object",
            "required": ["name"],
            "properties": {
                "name" : {
                    "type" : "string"
                },
                "tags": {
                    "type" : "array",
                    "items" : {
                        "type" : "integer"
                    }
                }
            }
        }"#;

        let deserialized: Schema = serde_json::from_str(json_str).unwrap();
        assert_eq!(deserialized.name, "Test");
        assert_eq!(deserialized.description, Some("Test schema".to_string()));
    }
}
