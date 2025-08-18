mod registry;
mod schema;
mod validator;

pub use error::{Error, Result};
pub use lib_commons::{Field, FieldType};
pub use registry::Registry;
pub use schema::Schema;

mod error {
    pub type Result<T> = core::result::Result<T, Error>;

    use crate::validator;

    #[derive(Debug)]
    pub enum Error {
        ValidationError(validator::ValidatorError),
        ValueStoreError(lib_commons::ValueStoreError),
    }

    impl From<validator::ValidatorError> for Error {
        fn from(value: validator::ValidatorError) -> Self {
            Error::ValidationError(value)
        }
    }

    impl From<lib_commons::ValueStoreError> for Error {
        fn from(value: lib_commons::ValueStoreError) -> Self {
            Error::ValueStoreError(value)
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
    use std::sync::{Arc, Mutex};

    use crate::{registry::Registry, schema::Schema, validator::Validator};
    use lib_commons::{Field, FieldType, Value, ValueStore};
    use serde_json::json;

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

        let value = ValueStore::builder()
            .with_schema("TestSchema")
            .string("name", "Ookuma Wakana")
            .number("age", 23)
            .object(
                "other",
                Value::builder().object().push_number("a", 10).into_map(),
                Some("InnerSchema"),
            )
            .build();

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

        let mut value = ValueStore::builder()
            .with_schema("TestSchema")
            .string("name", "Ookuma Wakana")
            .number("age", 23)
            .object(
                "other",
                Value::builder().object().push_number("b",10).into_map(),
                Some("InnerSchema"),
            )
            .build();

        let result = validator.validate(&value);
        assert!(result.is_err());

        value.remove("other");

        let result = validator.validate(&value);
        assert!(result.is_err());

        println!("Validation failed: {:?}", result);

        println!("{:?}", value);
    }
}
