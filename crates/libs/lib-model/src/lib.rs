use object_store::local::LocalFileSystem;
// use cache::CacheDB;
use store::Db;

// mod cache;
mod error;
mod store;

#[cfg(debug_assertions)]
pub mod _dev_utils;

pub use error::{Error, Result};
pub struct ModelManager {
    db: Db,
    image_store : LocalFileSystem,
}

impl ModelManager {
    pub async fn new(db_url: &str, image_store_url : &str) -> Result<Self> {
        let db = store::get_db_pool(db_url).await?;

        let image_store = LocalFileSystem::new_with_prefix(image_store_url)?;
        
        Ok(Self { db, image_store })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
