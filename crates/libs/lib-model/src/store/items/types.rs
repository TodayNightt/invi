use serde_json::Value as SerdeValue;
use sqlx::{types::Json, FromRow};

#[derive(Debug, FromRow)]
pub struct Item {
    id: u32,
    name: String,
    #[sqlx(rename = "item_metadata")]
    metadata: Json<SerdeValue>,
    image: u32,
    location: u32,
}

impl Item {
    pub fn metadata(&self) -> &SerdeValue {
        &self.metadata
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image(&self) -> u32 {
        self.image
    }

    pub fn location(&self) -> u32 {
        self.location
    }
}


