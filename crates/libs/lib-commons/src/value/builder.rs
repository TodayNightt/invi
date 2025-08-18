use crate::Value;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;

#[derive(Default)]
pub struct Obj;

#[derive(Default)]
pub struct Arr;

#[derive(Default)]
pub struct Any;

#[derive(Debug, Default)]
enum BuilderValue {
    #[default]
    Unset,
    String(String),
    Number(u64),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Null,
}

#[derive(Debug, Default)]
pub struct Builder<T> {
    types: PhantomData<T>,
    value: BuilderValue,
}

impl Value {
    pub fn builder() -> Builder<Any> {
        Builder::default()
    }
}

impl<T> Builder<T> {
    pub fn build(self) -> Option<Value> {
        match self.value {
            BuilderValue::Unset => None,
            BuilderValue::String(s) => Some(Value::String(s)),
            BuilderValue::Number(n) => Some(Value::Number(n)),
            BuilderValue::Array(a) => Some(Value::Array(a)),
            BuilderValue::Object(o) => Some(Value::Object(o)),
            BuilderValue::Null => Some(Value::Null),
        }
    }
}

impl Builder<Any> {
    pub fn array(self) -> Builder<Arr> {
        Builder {
            types: PhantomData,
            value: BuilderValue::Array(Vec::default()),
        }
    }

    pub fn object(self) -> Builder<Obj> {
        Builder {
            types: PhantomData,
            value: BuilderValue::Object(BTreeMap::default()),
        }
    }

    pub fn string(mut self, value: &str) -> Self {
        self.value = BuilderValue::String(value.to_string());
        self
    }

    pub fn number(mut self, value: u64) -> Self {
        self.value = BuilderValue::Number(value);
        self
    }

    pub fn null(mut self) -> Self {
        self.value = BuilderValue::Null;
        self
    }
}

impl Builder<Arr> {
    pub fn push_number(mut self, value: u64) -> Self {
        if let BuilderValue::Array(ref mut arr) = self.value {
            arr.push(Value::Number(value));
        }

        self
    }

    pub fn push_string(mut self, value: &str) -> Self {
        if let BuilderValue::Array(ref mut arr) = self.value {
            arr.push(Value::String(value.to_string()));
        }

        self
    }

    pub fn push_null(mut self) -> Self {
        if let BuilderValue::Array(ref mut arr) = self.value {
            arr.push(Value::Null);
        }

        self
    }

    pub fn push_array<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Builder<Arr>) -> Builder<Arr>,
    {
        if let BuilderValue::Array(ref mut arr) = self.value {
            let builder = Value::builder().array();
            let inner = f(builder);
            if let Some(inner) = inner.build() {
                arr.push(inner);
            }
        }

        self
    }

    pub fn push_object<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Builder<Obj>) -> Builder<Obj>,
    {
        if let BuilderValue::Array(ref mut arr) = self.value {
            let builder = Value::builder().object();
            let inner = f(builder);
            if let Some(inner) = inner.build() {
                arr.push(inner);
            }
        }

        self
    }
}

impl From<Builder<Arr>> for Vec<Value> {
    fn from(builder: Builder<Arr>) -> Self {
        match builder.value {
            BuilderValue::Array(arr) => arr,
            _ => vec![]
        }
    }
}

impl Builder<Obj> {
    pub fn push_number(mut self, key: &str, value: u64) -> Self {
        if let BuilderValue::Object(ref mut arr) = self.value {
            arr.entry(key.to_string()).or_insert(Value::Number(value));
        }

        self
    }

    pub fn push_string(mut self, key: &str, value: &str) -> Self {
        if let BuilderValue::Object(ref mut arr) = self.value {
            arr.entry(key.to_string())
                .or_insert(Value::String(value.to_string()));
        }

        self
    }

    pub fn push_null(mut self, key: &str) -> Self {
        if let BuilderValue::Object(ref mut arr) = self.value {
            arr.entry(key.to_string()).or_insert(Value::Null);
        }

        self
    }

    pub fn push_array<F>(mut self, key: &str, f: F) -> Self
    where
        F: FnOnce(Builder<Arr>) -> Builder<Arr>,
    {
        if let BuilderValue::Object(ref mut arr) = self.value {
            let builder = Value::builder().array();
            let inner = f(builder);
            if let Some(inner) = inner.build() {
                arr.entry(key.to_string()).or_insert(inner);
            }
        }

        self
    }

    pub fn push_object<F>(mut self, key: &str, f: F) -> Self
    where
        F: FnOnce(Builder<Obj>) -> Builder<Obj>,
    {
        if let BuilderValue::Object(ref mut arr) = self.value {
            let builder = Value::builder().object();
            let inner = f(builder);
            if let Some(inner) = inner.build() {
                arr.entry(key.to_string()).or_insert(inner);
            }
        }

        self
    }

    pub fn into_map(self) -> HashMap<String, Value> {
        HashMap::from(self)
    }
}


impl From<Builder<Obj>> for HashMap<String, Value> {
    fn from(builder: Builder<Obj>) -> Self {
        match builder.value {
            BuilderValue::Object(map) => map.into_iter().collect(),
            _ => HashMap::default()
        }
    }
}
