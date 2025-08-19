mod registry;
mod schema;
mod validator;

use crate::Error::ValidationError;
use crate::validator::{Validator, ValidatorError};
pub use error::{Error, Result};
use lib_commons::ValueStore;
pub use lib_commons::{Field, FieldType};
use lib_model::ModelManager;
pub use registry::Registry;
pub use schema::Schema;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct SchemaManager {
    registry: Arc<RwLock<Registry>>,
}

pub struct ValidationDescriptor<'a> {
    main: Arc<Schema>,
    properties: Arc<HashMap<&'a str, Arc<Schema>>>,
}

impl ValidationDescriptor<'_> {
    pub fn main(&self) -> Arc<Schema> {
        self.main.clone()
    }

    pub fn properties(&self) -> Arc<HashMap<&str, Arc<Schema>>> {
        self.properties.clone()
    }
}

impl SchemaManager {
    pub async fn new(mm: &ModelManager) -> Result<Self> {
        let registry = Registry::load_from_db(mm).await?;
        Ok(SchemaManager {
            registry: Arc::new(RwLock::new(registry)),
        })
    }

    pub fn register(&self, name: &str, schema: Schema) -> Result<()> {
        self.registry.write().map_err(|_| Error::LockError)?.register(name, schema);

        Ok(())
    }

    pub fn registry(&self) -> Arc<RwLock<Registry>> {
        self.registry.clone()
    }

    pub fn validate(&self, value: &ValueStore) -> Result<()> {
        let descriptor = value
            .schema_descriptor()
            .ok_or(ValidationError(ValidatorError::SchemaIdentifierMissing))?;

        let main_schema = {
            self.registry
                .read()
                .map_err(|_| Error::LockError)?
                .get_schema(descriptor.main())
                .ok_or(ValidationError(ValidatorError::SchemaNotFound(
                    descriptor.main().to_string(),
                )))?
        };

        let mut properties = HashMap::new();
        for (field, name) in descriptor.properties().as_ref() {
            let s = {
                self.registry
                    .read()
                    .map_err(|_| Error::LockError)?
                    .get_schema(name)
                    .ok_or(ValidationError(ValidatorError::SchemaNotFound(
                        name.to_string(),
                    )))?
            };
            properties.entry(*field).or_insert(s);
        }

        let validation_descriptor = ValidationDescriptor {
            main: main_schema,
            properties: Arc::new(properties),
        };

        let validator = Validator::new(validation_descriptor);

        validator.validate(value)?;

        Ok(())
    }
}

mod error {
    pub type Result<T> = core::result::Result<T, Error>;

    use crate::{registry, validator};

    #[derive(Debug)]
    pub enum Error {
        ValidationError(validator::ValidatorError),
        RegistryError(registry::Error),

        LockError,
    }

    impl From<validator::ValidatorError> for Error {
        fn from(value: validator::ValidatorError) -> Self {
            Error::ValidationError(value)
        }
    }

    impl From<registry::Error> for Error {
        fn from(value: registry::Error) -> Self {
            Error::RegistryError(value)
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
    use crate::{SchemaManager, registry::Registry, schema::Schema};
    use lib_commons::{Field, FieldType, Value, ValueStore};
    use std::sync::{Arc, RwLock};

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

        let schema_manager = SchemaManager {
            registry: Arc::new(RwLock::new(Registry::new())),
        };

        schema_manager.register("TestSchema", schema).unwrap();

        schema_manager
            .register("InnerSchema", inner_schema)
            .unwrap();

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

        let result = schema_manager.validate(&value);
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

        let schema_manager = SchemaManager {
            registry: Arc::new(RwLock::new(Registry::new())),
        };

        schema_manager.register("TestSchema", schema).unwrap();

        schema_manager
            .register("InnerSchema", inner_schema)
            .unwrap();

        let mut value = ValueStore::builder()
            .with_schema("TestSchema")
            .string("name", "Ookuma Wakana")
            .number("age", 23)
            .object(
                "other",
                Value::builder().object().push_number("b", 10).into_map(),
                Some("InnerSchema"),
            )
            .build();

        let result = schema_manager.validate(&value);
        assert!(result.is_err());

        value.remove("other");

        let result = schema_manager.validate(&value);
        assert!(result.is_err());

        println!("Validation failed: {:?}", result);

        println!("{:?}", value);
    }
}
