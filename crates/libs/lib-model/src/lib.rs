use cache::CacheDB;
use store::Db;

mod cache;
mod error;
mod store;

#[cfg(debug_assertions)]
pub mod _dev_utils;

pub use error::{Error, Result};
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new(db_url: &str) -> Result<Self> {
        let db = store::get_db_pool(db_url).await?;

        Ok(Self { db })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
