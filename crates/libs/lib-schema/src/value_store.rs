pub(crate) use error::ValueStoreError;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use crate::{Value, schema};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueStore {
    schema_name: Option<String>,
    object_properties_schemas: Arc<HashMap<String, String>>,
    values: BTreeMap<String, Value>,
}

impl ValueStore {
    pub fn new(schema_name: Option<String>) -> Self {
        ValueStore {
            schema_name: schema_name.map(|s| s.to_string()),
            object_properties_schemas: Arc::new(HashMap::new()),
            values: BTreeMap::new(),
        }
    }
    pub fn with_object_properties_schemas(
        mut self,
        object_properties_schemas: Arc<HashMap<String, String>>,
    ) -> Self {
        self.object_properties_schemas = object_properties_schemas;

        self
    }

    pub fn schema_name(&self) -> &Option<String> {
        &self.schema_name
    }
    pub fn insert(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn string(mut self, key: &str, value: String) -> Self {
        self.insert(key.to_string(), Value::String(value));
        self
    }

    pub fn number(mut self, key: &str, value: u64) -> Self {
        self.insert(key.to_string(), Value::Number(value));
        self
    }
    // pub fn object(mut self, key: &str, value: BTreeMap<String, Value>) -> Self {
    //     self.insert(key.to_string(), Value::Object(value));
    //     self
    // }

    pub fn object(mut self, key: &str, value: Value) -> Self {
        self.insert(key.to_string(), value);
        self
    }
    pub fn array(mut self, key: &str, value: Vec<Value>) -> Self {
        self.insert(key.to_string(), Value::Array(value));
        self
    }

    pub fn object_properties_schemas(&self) -> Arc<HashMap<String, String>> {
        self.object_properties_schemas.clone()
    }

    pub fn get_all(&self) -> &BTreeMap<String, Value> {
        &self.values
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn from_object_value(
        schema_name: Option<String>,
        value: &Value,
        object_properties_schemas: Arc<HashMap<String, String>>,
    ) -> Result<ValueStore, ValueStoreError> {
        let mut vs: ValueStore = ValueStore::from_value_shallow(value)?;
        vs.schema_name = schema_name;

        Ok(vs.with_object_properties_schemas(object_properties_schemas))
    }

    pub fn from_value_shallow(value: &Value) -> error::Result<Self> {
        if let Some(map) = value.as_object() {
            let mut value_store = ValueStore::new(None);

            for (key, val) in map {
                value_store.insert(key.clone(), val.clone());
            }
            Ok(value_store)
        } else {
            Err(ValueStoreError::NotAnObject)
        }
    }
}

impl TryFrom<Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let schema_name = map.get("schema_name").and_then(|v| v.as_str());

                let object_properties_schamas: Option<HashMap<String, String>> = map
                    .get("object_properties_schamas")
                    .and_then(|v| v.as_object())
                    .map(|v| {
                        v.iter()
                            .map(|(k, v)| (k.to_owned(), v.as_str().unwrap_or_default().to_owned()))
                            .collect()
                    });

                let mut value_store = {
                    if let Some(object_properties_schemas) = object_properties_schamas {
                        let object_properties_schema = Arc::new(object_properties_schemas);
                        ValueStore::new(schema_name)
                            .with_object_properties_schemas(object_properties_schema)
                    } else {
                        ValueStore::new(schema_name)
                    }
                };

                let Some(values) = map.get("values").and_then(|v| v.as_object()) else {
                    return Err(ValueStoreError::CannotConvertFromValue);
                };

                for (key, value) in values {
                    value_store.insert(key.clone(), value.clone());
                }

                Ok(value_store)
            }
            _ => Err(ValueStoreError::NotAnObject),
        }
    }
}

impl Display for ValueStore{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}

impl TryFrom<&Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl TryFrom<serde_json::Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let value: Value = value.into();
        value.try_into()
    }
}

impl TryFrom<&serde_json::Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

mod error {
    use std::fmt;
    pub type Result<T> = core::result::Result<T, ValueStoreError>;

    #[derive(Debug)]
    pub enum ValueStoreError {
        NotAnObject,
        NotAString,
        NotANumber,
        NotABool,
        NotAnArray,
        NotANull,
        CannotConvertFromValue,
    }

    impl fmt::Display for ValueStoreError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
