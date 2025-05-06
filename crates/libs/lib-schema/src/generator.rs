use jsonschema::Validator;
// It generates the json schema using rust types
use crate::{Error, Result};
use serde::{Deserialize, Serialize, de::value};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::types::{JsonSchema, RsSchema, SchemaType};

pub struct SchemaGenerator {}

impl SchemaGenerator {
    pub fn gen_rs_schema(json_schema: Value) -> Result<RsSchema> {
        let rs_schema = serde_json::from_value(json_schema).unwrap();

        Ok(rs_schema)
    }

    pub fn gen_json_schema(rs_schema: RsSchema) -> JsonSchema {
        // let mut properties: HashMap<String, Value> = HashMap::new();
        let val = serde_json::to_value(&rs_schema).unwrap();
        println!("{}", serde_json::to_string_pretty(&val).unwrap());

        let validator = jsonschema::Validator::new(&val).unwrap();

        JsonSchema::new(val, validator)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Property;

    use super::*;
    use serde_json::json;

    // #[test]
    // fn test_gen_json_schema() {
    //     let mut properties = HashMap::new();

    //     properties.insert(
    //         "name".to_string(),
    //         Property {
    //             property_type: SchemaType::String,
    //             name: "name".to_string(),
    //             default_value: None,
    //         },
    //     );

    //     properties.insert(
    //         "age".to_string(),
    //         Property {
    //             name: "age".to_string(),
    //             property_type: SchemaType::Integer {
    //                 min: None,
    //                 max: None,
    //             },
    //             default_value: None,
    //         },
    //     );

    //     properties.insert(
    //         "ids".to_string(),
    //         Property {
    //             name: "ids".to_string(),
    //             property_type: SchemaType::Array(Box::new(SchemaType::String)),
    //             default_value: None,
    //         },
    //     );

    //     properties.insert(
    //         "tags".to_string(),
    //         Property {
    //             name: "tags".to_string(),
    //             property_type: SchemaType::Array(Box::new(SchemaType::Array(Box::new(
    //                 SchemaType::String,
    //             )))),
    //             default_value: None,
    //         },
    //     );

    //     properties.insert(
    //         "metadata".to_string(),
    //         Property {
    //             name: "metadata".to_string(),
    //             property_type: SchemaType::Object {
    //                 required: vec!["name".to_string()],
    //                 properties: HashMap::from([(
    //                     "name".to_string(),
    //                     Property {
    //                         name: "name".to_string(),
    //                         property_type: SchemaType::String,
    //                         default_value: None,
    //                     },
    //                 )]),
    //             },
    //             default_value: None,
    //         },
    //     );

    //     let rs_schema = RsSchema {
    //         title: "Test".to_string(),
    //         description: None,
    //         schema_type: SchemaType::Object {
    //             required: vec!["name".to_string()],
    //             properties,
    //         },
    //     };

    //     // let json = serde_json::to_value(&rs_schema).unwrap();
    //     let json_schema = SchemaGenerator::gen_json_schema(rs_schema.clone());

    //     let control = json!({
    //         "required": ["name"],
    //         "properties": {
    //             "name": {
    //                 "type": "string"
    //             },
    //             "age": {
    //                 "type": "integer"
    //             },
    //             "ids": {
    //                 "type": "array",
    //                 "items": {
    //                     "type": "integer"
    //                 }
    //             },
    //             "tags":{
    //                 "type" : "array",
    //                 "items":{
    //                     "type" : "array",
    //                     "items" : {
    //                         "type" : "string"
    //                     }
    //                 }
    //             }
    //         }
    //     });

    //     println!("{}", serde_json::to_string_pretty(&control).unwrap());

    //     // println!(
    //     //     "{}",
    //     //     serde_json::to_string_pretty(json_schema.schema()).unwrap()
    //     // );

    //     // assert_eq!(json_schema.schema(), &control);
    // }

    #[test]
    fn test_gen_rs_schema() {
        let control = json!({
            "title": "Complex Schema",
            "description": "A schema with nested arrays and objects",
                "type": "object",
                "required": ["user", "tags"],
                "properties": {
                    "user": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer",
                                "min": 1,
                                "max": 100
                            },
                            "name": {
                                "type": "string"
                            }
                        }
                    },
                    "tags": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "label": {
                                    "type": "string"
                                },
                                "value": {
                                    "type": "string"
                                }
                            }
                        }
                    }
            }
        });

        let rs_schema = SchemaGenerator::gen_rs_schema(control).unwrap();

        // assert_eq!(rs_schema.title, "value");
        // assert_eq!(rs_schema.required, vec!["name".to_string()]);

        println!("gen_rs : {:?}", rs_schema);
    }
}
