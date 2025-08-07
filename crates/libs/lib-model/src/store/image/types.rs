use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize, FromRow)]
pub struct ImageKey(String);

impl ImageKey {
    pub fn key(&self) -> &str {
        &self.0
    }
}