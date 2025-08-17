use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use crate::{Value, ValueStore, ValueStoreError};
use serde_json::Value as SerdeJson;
use crate::value_store::{Result};

#[derive(Debug, Clone, Default)]
pub struct Builder {
    schema_name : Option<String>,
    object_properties_schemas : HashMap<String, String>,
    values : BTreeMap<String,Value>
}

impl ValueStore {
    pub fn builder()-> Builder{
        Builder::default()
    }
}

impl Builder {
    pub fn build(self) -> ValueStore {
        ValueStore{
            schema_name : self.schema_name,
            object_properties_schemas : Arc::new(self.object_properties_schemas),
            values: self.values.into_iter().collect()
        }
    }
}

impl Builder {
    pub fn insert(mut self, key : impl Into<String>, value : impl Into<Value>) -> Self {
        self.values.entry(key.into()).or_insert(value.into());
        self
    }

    pub fn with_schema(mut self,schema : impl Into<String>)-> Self {
        self.schema_name = Some(schema.into());

        self
    }
    pub fn string(mut self, key : impl AsRef<str>, value : impl AsRef<str>) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(Value::String(value.as_ref().to_string()));
        self
    }

    pub fn bool(mut self, key : impl AsRef<str>, value : bool) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(Value::Boolean(value));
        self
    }

    pub fn null(mut self, key : impl AsRef<str>) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(Value::Null);
        self
    }

    pub fn number(mut self, key : impl AsRef<str>, value : u64) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(Value::Number(value));
        self
    }

    pub fn array(mut self, key : impl AsRef<str>, value: Vec<Value>, schema : Option<&str>) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(Value::Array(value));

        if let Some(schema) = schema {
            self.object_properties_schemas.entry(key.as_ref().to_string()).or_insert(schema.into());
        }

        self
    }

    pub fn object(mut self, key : impl AsRef<str>, value : impl Into<Value>, schema: Option<&str>) -> Self {
        self.values.entry(key.as_ref().to_string()).or_insert(value.into());

        if let Some(schema) = schema {
            self.object_properties_schemas.entry(key.as_ref().to_string()).or_insert(schema.into());
        }

        self
    }

    pub fn with_values(mut self, values : BTreeMap<String,Value>) -> Self {
        self.values = values;
        self
    }

    pub fn with_value(mut self, value : &Value) -> Result<Self> {
        self.values = value.as_object().ok_or(ValueStoreError::NotAnObject)?.to_owned();
        Ok(self)
    }

    pub fn with_serde_values(mut self, values: SerdeJson) -> Result<Self> {
        self.values = values.as_object().ok_or(ValueStoreError::NotAnObject)?
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone().into()))
            .collect();
        Ok(self)
    }

    pub fn with_object_properties_schemas(
        mut self,
        object_properties_schemas: HashMap<String, String>
    ) -> Self {
        self.object_properties_schemas = object_properties_schemas;

        self
    }
}