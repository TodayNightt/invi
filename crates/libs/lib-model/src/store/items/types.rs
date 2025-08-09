use lib_schema::ValueStore;
use serde_json::Value as SerdeValue;
use sqlx::{FromRow, types::Json};

#[derive(Debug, FromRow)]
pub struct Item {
    id: u32,
    name: String,
    #[sqlx(rename = "item_metadata")]
    metadata: Json<ValueStore>,
    #[sqlx(rename = "key")]
    image: String,
    location: u32,
}

pub type Items = Vec<Item>;



impl Item {
    pub fn metadata(&self) -> &ValueStore {
        &self.metadata
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn location(&self) -> u32 {
        self.location
    }
}
