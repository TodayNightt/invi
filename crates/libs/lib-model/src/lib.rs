// use cache::CacheDB;
use store::Db;

// mod cache;
mod error;
mod store;

mod image_store;

#[cfg(debug_assertions)]
pub mod _dev_utils;

use crate::image_store::ImageStore;
pub use error::{Error, Result};

pub struct ModelManager {
    db: Db,
    image_store: ImageStore,
}

impl ModelManager {
    pub async fn new(db_url: &str, image_store_url: &str) -> Result<Self> {
        let db = store::get_db_pool(db_url).await?;

        let image_store = ImageStore::new(image_store_url)?;

        Ok(Self { db, image_store })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn image_store(&self) -> &ImageStore {
        &self.image_store
    }
}
