use crate::schema::Schema;
use lib_model::ModelManager;
use lib_model_schema::exec::{get_schema, register_schema, update_schema};
use lib_model_schema::types::Schemas;
use lib_model_schema::types::params::{
    SchemaGetPayload, SchemaRegisterPayload, SchemaUpdatePayload,
};
use std::{collections::HashMap, sync::Arc};

use crate::registry::error::Result;

pub use crate::registry::error::Error;

#[derive(Debug, Default)]
pub struct Registry {
    schemas: HashMap<String, Arc<Schema>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            schemas: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, schema: Schema) {
        self.schemas.insert(name.to_string(), Arc::new(schema));
    }

    pub fn get_schema(&self, name: &str) -> Option<Arc<Schema>> {
        self.schemas.get(name).cloned()
    }

    pub fn edit_schema(&mut self, name: &str) -> Option<&mut Arc<Schema>> {
        self.schemas.get_mut(name)
    }

    pub fn load_from_file(schemas: BulkSchema) -> Self {
        let mut registry = Self::new();
        for schema in schemas.schemas {
            let name = schema.name();
            registry.register(&name, schema);
        }
        registry
    }

    pub async fn finalize_schema(&self, mm: &ModelManager, name: &str) -> Result<Option<i64>> {
        let schema = self
            .schemas
            .get(name)
            .cloned()
            .ok_or(Error::SchemaNotFound(name.to_string()))?;

        // Check if id exists,
        // If exists update the data
        if let Some(id) = schema.id() {
            let params = SchemaUpdatePayload {
                id,
                name: Some(name.to_string()),
                fields: Some(Arc::from(schema.fields().clone())),
            };
            update_schema(mm, params).await?;

            return Ok(None);
        }

        // Or else create a new one
        let params = SchemaRegisterPayload {
            name: name.to_string(),
            fields: Arc::from(schema.fields().clone()),
        };
        let id = register_schema(mm, params).await?;
        Ok(Some(id))
    }

    pub async fn load_from_db(mm: &ModelManager) -> Result<Self> {
        let mut registry = Registry::new();
        let schemas = get_schema(mm, SchemaGetPayload::default()).await?;

        match schemas {
            Schemas::Bulk(schemas) => {
                schemas
                    .iter()
                    .map(|schema| schema.into())
                    .for_each(|s: Schema| {
                        let name = s.name();
                        registry.register(&name, s);
                    });
            }
            Schemas::Single(schema) => {
                let name = schema.name();
                registry.register(name, schema.clone().into());
            }
        }
        Ok(registry)
    }
}

mod error {
    use std::fmt::Formatter;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        SchemaNotFound(String),

        LibModelError(lib_model::Error),
    }

    impl From<lib_model::Error> for Error {
        fn from(value: lib_model::Error) -> Self {
            Error::LibModelError(value)
        }
    }

    impl core::error::Error for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

pub struct BulkSchema {
    schemas: Vec<Schema>,
}
