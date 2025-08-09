use crate::Error;
use lib_schema::ValueStore;
use serde::{Deserialize, Serialize};
use serde_json::Value as SerdeValue;
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Location {
    id: u32,
    location: String,
    rack: Option<String>,
    bin: Option<String>,
}

impl Location {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn rack(&self) -> &Option<String> {
        &self.rack
    }

    pub fn bin(&self) -> &Option<String> {
        &self.bin
    }
}

// Location Metadata
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct RawLocationMetadata {
    id: u32,
    name: String,
    metadata: Option<Json<ValueStore>>,
}

impl From<RawLocationMetadata> for LocationMetadata {
    fn from(raw: RawLocationMetadata) -> Self {
        let metadata = raw.metadata.map(|m| m.0);
        LocationMetadata {
            id: raw.id,
            name: raw.name,
            metadata,
        }
    }
}

pub struct LocationMetadata {
    id: u32,
    name: String,
    metadata: Option<ValueStore>,
}

impl LocationMetadata {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn metadata(&self) -> &Option<ValueStore> {
        &self.metadata
    }
}
