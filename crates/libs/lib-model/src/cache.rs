pub type CacheDB = sled::Db;
use crate::{Error, Result};

pub(crate) fn get_cache_db(url: &str) -> Result<CacheDB> {
    let db = sled::open(url)?;
    Ok(db)
}
