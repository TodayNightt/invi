mod exec;
mod items;
mod locations;
mod records;

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

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
