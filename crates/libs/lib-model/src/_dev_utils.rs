use std::fs;
use std::path::{Path, PathBuf};
use sqlx::sqlite::SqliteConnectOptions;
use crate::ModelManager;

use crate::Result;
use crate::store::{get_db_pool, Db};

pub async fn get_dev_env() -> Result<ModelManager> {
    let url = env!("LIBMODEL_DEV_DB_URL");

    let db_url = format!("sqlite://file:{}", url);

    let migration_dir = env!("LIBMODEL_MIGRATIONS_DIR");
    let mut paths: Vec<PathBuf> = fs::read_dir(migration_dir).unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter_map(|path| {
            if let Some(extension) = path.extension() {
                if extension.eq("sql") {
                    return Some(path);
                }
            }
            None
        })
        .collect();

    paths.sort();

    let db = get_db_pool(&db_url).await?;

    for path in paths.into_iter() {
        if let Some(path) = path.to_str() {
            if !path.ends_with(".sql") {
                continue;
            }
            pexec(&db, path).await?;
        }
    }

    drop(db);


    ModelManager::new(&db_url).await
}

async fn pexec(db: &Db, file: &str) -> Result<()> {

    // println!("Executing SQL file: {}", file);

    // -- Read the file.
    let content = fs::read_to_string(file).unwrap();

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        if sql.is_empty() || sql.trim().is_empty() {
            continue;
        }
        sqlx::query(sql.trim()).execute(db).await?;
    }

    Ok(())
}


