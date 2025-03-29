use cache::CacheDB;
use store::Db;

mod cache;
mod error;
mod store;

pub use error::{Error, Result};
pub struct ModelManager {
    db: Db,
    cache: CacheDB,
}

impl ModelManager {
    pub async fn new(db_url: &str, cache_url: &str) -> Result<Self> {
        let db = store::get_db_pool(db_url).await?;
        let cache = cache::get_cache_db(cache_url)?;

        Ok(Self { db, cache })
    }
}
