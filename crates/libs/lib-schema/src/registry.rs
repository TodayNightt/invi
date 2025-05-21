use std::{collections::HashMap, sync::Arc};

use crate::{
    schema::{self, Schema},
    validator::Validator,
    value_store::ValueStore,
};

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

    pub fn load_from_file(schemas: BulkSchema) -> Self {
        let mut registry = Self::new();
        for schema in schemas.schemas {
            let name = schema.name();
            registry.register(&name, schema);
        }
        registry
    }
}

pub struct BulkSchema {
    schemas: Vec<Schema>,
}
