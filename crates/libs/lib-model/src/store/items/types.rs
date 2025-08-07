use lib_schema::ValueStore;
use serde_json::Value as SerdeValue;
use sqlx::{FromRow, types::Json};

#[derive(Debug, FromRow)]
pub struct RawItem {
    id: u32,
    name: String,
    #[sqlx(rename = "item_metadata")]
    metadata: Json<SerdeValue>,
    #[sqlx(rename = "key")]
    image: String,
    location: u32,
}

pub struct Item {
    id: u32,
    name: String,
    metadata: ValueStore,
    image: String,
    location: u32,
}

impl TryFrom<RawItem> for Item {
    type Error = crate::Error;

    fn try_from(raw: RawItem) -> Result<Self, Self::Error> {
        let metadata = raw.metadata.0.try_into()?;

        Ok(Item {
            id: raw.id,
            name: raw.name,
            metadata,
            image: raw.image,
            location: raw.location,
        })
    }
}

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

impl RawItem {
    pub fn metadata(&self) -> &SerdeValue {
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
