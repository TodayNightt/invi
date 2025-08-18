use lib_commons::ValueStore;
use lib_model::ModelManager;
use lib_model::{Error, Result};
use sqlx::types::Json;

// region : Types
#[derive(sqlx::FromRow)]
pub struct RawLocationMetadata {
    pub id: i64,
    pub name: String,
    pub metadata: Option<Json<ValueStore>>,
}
// endregion

//```sql
// CREATE TABLE IF NOT EXISTS location_metadata (
//      id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
//      name     TEXT                              NOT NULL,
//      metadata TEXT
//);
//```
pub(crate) struct LocationMetadataBmc;

impl LocationMetadataBmc {
    pub async fn create(mm: &ModelManager, name: &str, metadata: Option<&str>) -> Result<i64> {
        let db = mm.db();

        let result = sqlx::query!(
            "INSERT INTO location_metadata (name, metadata) VALUES ($1, $2)",
            name,
            metadata
        )
        .execute(db)
        .await?
        .last_insert_rowid();

        Ok(result)
    }

    pub async fn get(mm: &ModelManager, id: i64) -> Option<RawLocationMetadata> {
        let db = mm.db();

        let result = sqlx::query_as!(
            RawLocationMetadata,
            r#"SELECT id, name, metadata as "metadata?: Json<ValueStore>"
                FROM location_metadata
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(db)
        .await
        .ok()?;

        Some(result)
    }

    pub async fn get_all(mm: &ModelManager) -> Result<Vec<RawLocationMetadata>> {
        let db = mm.db();

        let result = sqlx::query_as!(
            RawLocationMetadata,
            r#"SELECT id, name, metadata as "metadata?: Json<ValueStore>"
                FROM location_metadata"#
        )
        .fetch_all(db)
        .await?;

        Ok(result)
    }

    pub async fn update_name(mm: &ModelManager, id: i64, name: &str) -> Result<()> {
        let db = mm.db();

        sqlx::query!(
            "UPDATE location_metadata SET name = $1 WHERE id = $2",
            name,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn update_metadata(mm: &ModelManager, id: i64, metadata: Option<&str>) -> Result<()> {
        let db = mm.db();

        sqlx::query!(
            "UPDATE location_metadata SET metadata = $1 WHERE id = $2",
            metadata,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        sqlx::query!("DELETE FROM location_metadata WHERE id = $1", id)
            .execute(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::location_metadata::LocationMetadataBmc;
    use lib_commons::{Value, ValueStore};
    use lib_model::_dev_utils::get_dev_env;
    use serde_json::json;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get() {
        let mm = get_dev_env().await.unwrap();

        let result = LocationMetadataBmc::get(&mm, 1).await.unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Container 1");
        assert!(
            result
                .metadata
                .clone()
                .unwrap()
                .get("racks")
                .unwrap()
                .as_array()
                .unwrap()
                .len()
                .eq(&3)
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_all() {
        let mm = get_dev_env().await.unwrap();

        let metadata = ValueStore::builder()
            .array(
                "racks",
                vec![Value::from(json!({
                    "name" : "Rack Uno"
                }))],
                None,
            )
            .build();

        let _ = LocationMetadataBmc::create(&mm, "Can 1", Some(&metadata.to_string()))
            .await
            .unwrap();

        let _ = LocationMetadataBmc::create(&mm, "Can 2", Some(&metadata.to_string()))
            .await
            .unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 4);
    }

    #[tokio::test]
    #[serial]
    async fn test_create() {
        let mm = get_dev_env().await.unwrap();

        let metadata = ValueStore::builder()
            .array(
                "racks",
                vec![Value::from(json!({
                    "name" : "Rack Uno"
                }))],
                None,
            )
            .build();

        let id = LocationMetadataBmc::create(&mm, "Container Uno", Some(&metadata.to_string()))
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id, 1);
        assert_eq!(result.name, "Container Uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_update() {
        let mm = get_dev_env().await.unwrap();
        let metadata = ValueStore::builder()
            .array(
                "racks",
                vec![Value::from(json!({
                    "name" : "Rack Uno"
                }))],
                None,
            )
            .build();
        // Update name
        let id = LocationMetadataBmc::create(&mm, "Container Uno", Some(&metadata.to_string()))
            .await
            .unwrap();

        LocationMetadataBmc::update_name(&mm, id, "Hall Uno")
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id, 1);
        assert_eq!(result.name, "Hall Uno");

        drop(mm);

        let mm = get_dev_env().await.unwrap();

        let id = LocationMetadataBmc::create(&mm, "Container Uno", Some(&metadata.to_string()))
            .await
            .unwrap();

        let metadata = ValueStore::builder()
            .array(
                "racks",
                vec![Value::from(json!({
                    "name" : "Cupboard Uno"
                }))],
                None,
            )
            .build();
        LocationMetadataBmc::update_metadata(&mm, id, Some(&metadata.to_string()))
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id, 1);

        // FIXME : Get better API for getting data
        let name = result
            .metadata
            .clone()
            .unwrap()
            .get("racks")
            .unwrap()
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .as_object()
            .unwrap()
            .get("name")
            .unwrap()
            .as_string()
            .unwrap();

        assert_eq!(name, "Cupboard Uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete() {
        let mm = get_dev_env().await.unwrap();

        let metadata = ValueStore::builder()
            .array(
                "racks",
                vec![Value::from(json!({
                    "name" : "Rack Uno"
                }))],
                None,
            )
            .build();
        // Update name
        let id = LocationMetadataBmc::create(&mm, "Container Uno", Some(&metadata.to_string()))
            .await
            .unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 3);

        LocationMetadataBmc::delete(&mm, id).await.unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
