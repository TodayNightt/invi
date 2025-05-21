mod registry;
mod schema;
mod types;
mod validator;
mod value_store;

pub use error::{Error, Result};
pub use registry::Registry;
pub use schema::Schema;
pub use types::{Field, FieldType, Value};
pub use value_store::ValueStore;

mod error {
    pub type Result<T> = core::result::Result<T, Error>;

    use lib_utils::types::ErrorDescriptor;

    use crate::validator;

    #[derive(Debug)]
    pub enum Error {
        ValidationError(validator::ValidatorError),
    }

    impl From<validator::ValidatorError> for Error {
        fn from(value: validator::ValidatorError) -> Self {
            Error::ValidationError(value)
        }
    }

    impl std::error::Error for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    use serde_json::json;

    use crate::{
        FieldType, Value,
        registry::{self, Registry},
        schema::Schema,
        types::Field,
        validator::Validator,
        value_store::ValueStore,
    };

    #[test]
    fn test_schema_creation() {
        let schema = Schema::create(
            "TestSchema",
            vec![
                Field::create("name", FieldType::String, true, Value::Null),
                Field::create("age", FieldType::Number, false, Value::Number(30)),
            ],
        );

        println!("{:?}", schema);

        assert_eq!(schema.name(), "TestSchema");
        assert_eq!(schema.fields().len(), 2);

        let age_field = schema.get_field("age");

        assert!(age_field.is_some());

        let age_field = age_field.unwrap();
        assert!(!age_field.required());
    }

    #[test]
    fn test_json_creation() {
        let schema = Schema::create(
            "TestSchema",
            vec![
                Field::create("name", FieldType::String, true, Value::Null),
                Field::create("age", FieldType::Number, false, Value::Number(30)),
            ],
        );

        let json = serde_json::to_string_pretty(&schema).unwrap();
        println!("{}", json);
    }

    #[test]
    fn test_value_validation() {
        let schema = Schema::create(
            "TestSchema",
            vec![
                Field::create("name", FieldType::String, true, Value::Null),
                Field::create("age", FieldType::Number, false, Value::Number(30)),
                Field::create("other", FieldType::Object, true, Value::Null),
            ],
        );

        let inner_schema = Schema::create(
            "InnerSchema",
            vec![Field::create("a", FieldType::Number, true, Value::Null)],
        );

        let mut registry = Arc::new(Mutex::new(Registry::new()));

        registry.lock().unwrap().register("TestSchema", schema);

        registry
            .lock()
            .unwrap()
            .register("InnerSchema", inner_schema);

        let validator = Validator::new(Arc::clone(&registry));

        let value = ValueStore::new(Some("TestSchema".to_string()))
            .with_object_properties_schemas(Arc::new(
                [("other".to_string(), "InnerSchema".to_string())].into(),
            ))
            .string("name", "Ookuma Wakana".to_string())
            .number("age", 23)
            .object(
                "other",
                json!({
                    "a" : 10,
                })
                .into(),
            );

        let result = validator.validate(&value);
        println!("Validation result: {:?}", result);
        assert!(result.is_ok());

        println!("{:?}", value);
    }

    #[test]
    fn test_value_invalidation() {
        let schema = Schema::create(
            "TestSchema",
            vec![
                Field::create("name", FieldType::String, true, Value::Null),
                Field::create("age", FieldType::Number, false, Value::Number(30)),
                Field::create("other", FieldType::Object, true, Value::Null),
            ],
        );

        let inner_schema = Schema::create(
            "InnerSchema",
            vec![Field::create("a", FieldType::Number, true, Value::Null)],
        );

        let mut registry = Arc::new(Mutex::new(Registry::new()));

        registry.lock().unwrap().register("TestSchema", schema);

        registry
            .lock()
            .unwrap()
            .register("InnerSchema", inner_schema);

        let validator = Validator::new(Arc::clone(&registry));

        let mut value = ValueStore::new(Some("TestSchema".to_string()))
            .with_object_properties_schemas(Arc::new(
                [("other".to_string(), "InnerSchema".to_string())].into(),
            ))
            .string("name", "Ookuma Wakana".to_string())
            .number("age", 23)
            .object(
                "other",
                json!({
                    "b" : 10,
                })
                .into(),
            );

        let result = validator.validate(&value);
        assert!(result.is_err());

        value.remove("other");

        let result = validator.validate(&value);
        assert!(result.is_err());

        println!("Validation failed: {:?}", result);

        println!("{:?}", value);
    }
}
