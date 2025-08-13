pub(crate) mod items;
pub(crate) mod locations;
pub(crate) mod records;
pub(crate) mod image;
pub(crate) mod location_metadata;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub type Db = Pool<Sqlite>;
use crate::Result;

pub(crate) async fn get_db_pool(url: &str) -> Result<Db> {
    let db_option = url.parse()?;

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_option)
        .await?;

    Ok(db)
}
