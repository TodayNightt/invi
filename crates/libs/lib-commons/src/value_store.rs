use crate::value::Value;
pub use error::{Result, ValueStoreError};
use serde::{Deserialize, Serialize};
use serde_json::Value as SerdeValue;
use std::fmt::{Display, Formatter};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

pub mod builder;

mod macros;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValueStore {
    schema_name: Option<String>,
    object_properties_schemas: Arc<HashMap<String, String>>,
    values: BTreeMap<String, Value>,
}

pub struct SchemaDescriptor<'a> {
    main: Arc<str>,
    properties: Arc<HashMap<&'a str, &'a str>>,
}

impl SchemaDescriptor<'_> {
    pub fn main(&self) -> &str {
        self.main.as_ref()
    }

    pub fn properties(&self) -> Arc<HashMap<&str, &str>> {
        self.properties.clone()
    }
}

impl ValueStore {
    pub fn new(schema_name: Option<String>) -> Self {
        ValueStore {
            schema_name,
            object_properties_schemas: Arc::new(HashMap::new()),
            values: BTreeMap::new(),
        }
    }

    pub fn schema_descriptor(&self) -> Option<SchemaDescriptor> {
        let main = self.schema_name().as_ref().map(|s| Arc::from(s.as_str()))?;
        let properties: Arc<HashMap<&str, &str>> = Arc::new(
            self.object_properties_schemas
                .as_ref()
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        );
        Some(SchemaDescriptor { main, properties })
    }

    pub fn schema_name(&self) -> &Option<String> {
        &self.schema_name
    }
    pub fn object_properties_schemas(&self) -> Arc<HashMap<String, String>> {
        self.object_properties_schemas.clone()
    }
    pub fn as_value(&self) -> Value {
        Value::Object(self.values.clone())
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
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
}

impl TryFrom<Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: Value) -> core::result::Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let schema_name = map.get("schema_name").and_then(|v| v.as_string());

                let object_properties_schemas: Option<HashMap<String, String>> = map
                    .get("object_properties_schemas")
                    .and_then(|v| v.as_object())
                    .map(|v| {
                        v.iter()
                            .map(|(k, v)| {
                                (k.to_owned(), v.as_string().unwrap_or_default().to_owned())
                            })
                            .collect()
                    });

                let mut builder = if let Some(schema_name) = schema_name {
                    ValueStore::builder().with_schema(schema_name)
                } else {
                    ValueStore::builder()
                };

                if let Some(object_properties_schemas) = object_properties_schemas {
                    builder = builder.with_object_properties_schemas(object_properties_schemas);
                }

                let Some(values) = map.get("values").and_then(|v| v.as_object()) else {
                    return Err(ValueStoreError::CannotConvertFromValue);
                };

                builder = builder.with_values(values.to_owned());

                Ok(builder.build())
            }
            _ => Err(ValueStoreError::NotAnObject),
        }
    }
}

impl Display for ValueStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}

impl TryFrom<&Value> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: &Value) -> core::result::Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl TryFrom<SerdeValue> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: SerdeValue) -> core::result::Result<Self, Self::Error> {
        let value: Value = value.into();
        value.try_into()
    }
}

impl TryFrom<&SerdeValue> for ValueStore {
    type Error = ValueStoreError;
    fn try_from(value: &SerdeValue) -> core::result::Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

#[macro_export]
macro_rules! value {
    ($schema_name:ident => $value:expr) => {{
        let values: Map<String, SerdeValue> = $value.into();
        let values = values
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect();
        ValueStore::new($schema_name).with_values(values)
    }};
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
