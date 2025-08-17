use std::sync::{Arc, Mutex};

use crate::{Registry, Schema};

pub(crate) use error::{Result, ValidatorError};

use lib_commons::{ValueStore, ValueStoreError};

pub(crate) struct Validator {
    registry: Arc<Mutex<Registry>>,
}

impl Validator {
    pub fn new(registry: Arc<Mutex<Registry>>) -> Self {
        Validator { registry }
    }

    fn validate_fields(&self, schema: &Schema, value: &ValueStore) -> Result<()> {
        let object_properties_schema = value.object_properties_schemas();
        for field in schema.fields() {
            let field_name = field.name();
            let field_value = value.get(field_name).ok_or_else(|| {
                ValidatorError::MissingField(format!(
                    "Field '{}' is missing in the value store",
                    field_name
                ))
            })?;

            // println!(
            //     "Validating for field : {} with the value of {}",
            //     field_name,
            //     field_value.to_value_string()
            // );

            if field.required() && field_value.is_null() {
                return Err(ValidatorError::RequiredFieldMissing(field_name.to_string()));
            }

            if !field_value.is_null() && !field.field_type().is_valid(field_value) {
                return Err(ValidatorError::InvalidType(format!(
                    "Field '{}' has invalid type: expected {:?}, got {:?}",
                    field_name,
                    field.field_type(),
                    field_value
                )));
            }
        }
        Ok(())
    }

    fn validate_inner_object(&self, value: &ValueStore) -> Result<()> {
        let object_properties_schema = value.object_properties_schemas();

        for (field_name, schema_name) in object_properties_schema.as_ref() {
            let Some(value) = value.get(field_name) else {
                continue;
            };

            let schema = {
                let l = self.registry.lock().unwrap();
                l.get_schema(schema_name)
            };

            let Some(schema) = schema else {
                return Err(ValidatorError::SchemaNotFound(schema_name.to_string()));
            };

            let temp = ValueStore::builder()
                .with_schema(schema_name)
                .with_object_properties_schemas(object_properties_schema.as_ref().clone())
                .with_value(value)?
                .build();

            // let temp = ValueStore::from_object_value(
            //     Some(schema_name.clone()),
            //     value,
            //     object_properties_schema.clone(),
            // ).map_err(ValidatorError::ValueStoreError)?;

            self._validate(&schema, &temp)?;
        }

        Ok(())
    }

    fn _validate(&self, schema: &Schema, value: &ValueStore) -> Result<()> {
        self.validate_fields(schema, value)?;
        self.validate_inner_object(value)?;

        Ok(())
    }

    pub fn validate(&self, value: &ValueStore) -> Result<()> {
        let schema_name = value.schema_name();

        let Some(schema_name) = schema_name else {
            return Err(ValidatorError::SchemaIndentifierMissing);
        };

        let schema = {
            let binding = self.registry.lock().unwrap();

            binding.get_schema(schema_name)
        };

        let Some(schema) = schema else {
            return Err(ValidatorError::SchemaNotFound(schema_name.to_string()));
        };

        self._validate(&schema, value)
    }
}

mod error {
    use std::fmt;
    pub type Result<T> = std::result::Result<T, ValidatorError>;

    #[derive(Debug)]
    pub enum ValidatorError {
        MissingField(String),
        InvalidType(String),
        RequiredFieldMissing(String),
        SchemaNotFound(String),
        SchemaIndentifierMissing,

        // -- ValueStoreError --
        ValueStoreError(lib_commons::ValueStoreError),
    }

    impl From<lib_commons::ValueStoreError> for ValidatorError {
        fn from(err: lib_commons::ValueStoreError) -> Self {
            ValidatorError::ValueStoreError(err)
        }
    }

    impl fmt::Display for ValidatorError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for ValidatorError {}
}
