use crate::store::schema::SchemaBmc;
use crate::types::params::{SchemaDeletePayload, SchemaGetPayload, SchemaRegisterPayload, SchemaUpdatePayload};
use crate::types::{Schema, Schemas};
use lib_model::ModelManager;
use lib_model::{Error, Result};

pub(crate) mod types {
    use crate::store::schema::RawSchema;
    use lib_commons::Field;
    use std::sync::Arc;

    pub mod params {
        use lib_commons::Field;
        use serde_json::json;
        use std::sync::Arc;

        #[derive(Debug, Clone)]
        pub struct SchemaRegisterPayload {
            name: String,
            fields: Arc<[Field]>,
        }

        impl SchemaRegisterPayload {
            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn fields(&self) -> String {
                json!(self.fields).to_string()
            }
        }

        #[derive(Debug, Clone)]
        pub struct SchemaUpdatePayload {
            id: i64,
            name: Option<String>,
            fields: Option<Arc<[Field]>>,
        }

        impl SchemaUpdatePayload {
            pub fn id(&self) -> i64 {
                self.id
            }
            pub fn name(&self) -> Option<&str> {
                self.name.as_deref()
            }

            pub fn fields(&self) -> Option<String> {
                self.fields.as_ref().map(|f| json!(f).to_string())
            }
        }

        #[derive(Debug, Clone)]
        pub struct SchemaGetPayload {
            id: Option<i64>,
        }

        impl SchemaGetPayload {
            pub fn id(&self) -> Option<i64> {
                self.id
            }
        }

        #[derive(Debug, Clone)]
        pub struct SchemaDeletePayload {
            id: i64,
        }

        impl SchemaDeletePayload {
            pub fn id(&self) -> i64 {
                self.id
            }
        }
    }

    pub enum Schemas {
        Bulk(BulkSchema),
        Single(Schema),
    }

    pub type BulkSchema = Arc<[Schema]>;

    pub struct Schema {
        id: i64,
        name: String,
        fields: Arc<[Field]>,
    }

    impl From<RawSchema> for Schema {
        fn from(value: RawSchema) -> Self {
            Schema {
                id: value.id,
                name: value.name,
                fields: value.fields.0.into(),
            }
        }
    }
}

pub async fn register_schema(mm: &ModelManager, params: SchemaRegisterPayload) -> Result<i64> {
    let id = SchemaBmc::create(mm, params.name(), params.fields().as_ref()).await?;

    Ok(id)
}

pub async fn get_schema(mm: &ModelManager, params: SchemaGetPayload) -> Result<Schemas> {
    if let Some(id) = params.id() {
        let single = SchemaBmc::get(mm, id)
            .await
            .ok_or(Error::SchemaNotFound(id))?;
        return Ok(Schemas::Single(single.into()));
    }

    let bulk = SchemaBmc::get_all(mm).await?;

    Ok(Schemas::Bulk(bulk.into_iter().map(|s| s.into()).collect()))
}

pub async fn update_schema(mm: &ModelManager, params: SchemaUpdatePayload) -> Result<()> {
    if let Some(name) = params.name() {
        SchemaBmc::update_name(mm, params.id(), name)
            .await
            .ok_or(Error::SchemaNotFound(params.id()))?;
    }

    if let Some(fields) = params.fields() {
        SchemaBmc::update_fields(mm, params.id(), fields.as_ref())
            .await
            .ok_or(Error::SchemaNotFound(params.id()))?;
    }

    Ok(())
}

pub async fn delete_schema(mm: &ModelManager, params: SchemaDeletePayload) -> Result<()> {
    SchemaBmc::delete(mm, params.id()).await.ok_or(Error::SchemaNotFound(params.id()))?;

    Ok(())
}
