use crate::ModelManager;
use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::store::{Db, get_db_pool};

pub async fn get_dev_env() -> Result<ModelManager> {
    let url = env!("DEV_DB_URL");

    let db_url = format!("sqlite://file:{}", url);

    let migration_dir = env!("MIGRATIONS_DIR");
    // For data db
    let data_db = PathBuf::from(migration_dir).join("initial/data");

    init_db(&db_url,&data_db).await?;

    // For schema db
    let schema_url = env!("DEV_SCHEMA_DB_URL");
    let schema_db_url = format!("sqlite://file:{}", schema_url);

    let schema_db_migration_dir = PathBuf::from(migration_dir).join("initial/schema");
    init_db(&schema_db_url, &schema_db_migration_dir).await?;


    let image_store_url = env!("IMAGE_STORE_URL");


    ModelManager::new(&db_url, image_store_url, &schema_db_url).await
}

async fn init_db(db_url : &str, migration_dir :&Path) -> Result<()> {
    let mut paths: Vec<PathBuf> = fs::read_dir(migration_dir)
        .unwrap()
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

    let db = get_db_pool(db_url).await?;

    for path in paths.into_iter() {
        if let Some(path) = path.to_str() {
            if !path.ends_with(".sql") {
                continue;
            }
            pexec(&db, path).await?;
        }
    }

    drop(db);
    Ok(())
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