use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type Db = Pool<Sqlite>;
use crate::{Error, Result};

pub(crate) async fn get_db_pool(url: &str) -> Result<Db> {
    let db_option = url.parse()?;

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_option)
        .await
}
