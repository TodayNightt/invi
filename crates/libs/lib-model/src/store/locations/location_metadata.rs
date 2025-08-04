use crate::ModelManager;
use crate::error::{Error, Result};
use crate::store::locations::types::{LocationMetadata, RawLocationMetadata};
use std::any::Any;

//```sql
// CREATE TABLE IF NOT EXISTS location_metadata (
//      id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
//      name     TEXT                              NOT NULL,
//      metadata TEXT
//);
//```
pub(crate) struct LocationMetadataBmc;

impl LocationMetadataBmc {
    pub async fn create(mm: &ModelManager, name: &str, metadata: &str) -> Result<u32> {
        let db = mm.db();

        let result = sqlx::query("INSERT INTO location_metadata (name, metadata) VALUES ($1, $2)")
            .bind(name)
            .bind(metadata)
            .execute(db)
            .await?
            .last_insert_rowid();

        Ok(result.try_into()?)
    }

    pub async fn get(mm: &ModelManager, id: u32) -> Result<LocationMetadata> {
        let db = mm.db();

        let result = sqlx::query_as::<_, RawLocationMetadata>(
            "SELECT * FROM location_metadata WHERE id = $1",
        )
        .bind(id)
        .fetch_one(db)
        .await?;

        result.try_into()
    }

    pub async fn get_all(mm: &ModelManager) -> Result<Vec<LocationMetadata>> {
        let db = mm.db();

        let result = sqlx::query_as::<_, RawLocationMetadata>("SELECT * FROM location_metadata")
            .fetch_all(db)
            .await?;

        result.into_iter().map(|r| r.try_into()).collect()
    }

    pub async fn update_name(mm: &ModelManager, id: u32, name: &str) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE location_metadata SET name = $1 WHERE id = $2")
            .bind(name)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn update_metadata(mm: &ModelManager, id: u32, metadata: &str) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE location_metadata SET metadata = $1 WHERE id = $2")
            .bind(metadata)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, id: u32) -> Result<()> {
        let db = mm.db();

        sqlx::query("DELETE FROM location_metadata WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::get_dev_env;
    use crate::store::locations::location_metadata::LocationMetadataBmc;
    use lib_schema::{Value, ValueStore};
    use serde_json::json;
    use serial_test::serial;
    #[tokio::test]
    #[serial]
    async fn test_get() {
        let mm = get_dev_env().await.unwrap();

        let result = LocationMetadataBmc::get(&mm, 1).await.unwrap();

        assert_eq!(result.id(), 1);
        assert_eq!(result.name(), "Container 1");
        assert!(
            result
                .metadata()
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

        let metadata = ValueStore::new(None).array(
            "racks",
            vec![Value::from(json!({
                "name" : "Rack Uno"
            }))],
        );

        let _ = LocationMetadataBmc::create(&mm, "Can 1", &metadata.to_string())
            .await
            .unwrap();

        let _ = LocationMetadataBmc::create(&mm, "Can 2", &metadata.to_string())
            .await
            .unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 4);
    }

    #[tokio::test]
    #[serial]
    async fn test_create() {
        let mm = get_dev_env().await.unwrap();

        let metadata = ValueStore::new(None).array(
            "racks",
            vec![Value::from(json!({
                "name" : "Rack Uno"
            }))],
        );

        let id = LocationMetadataBmc::create(&mm, "Container Uno", &metadata.to_string())
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id(), 1);
        assert_eq!(result.name(), "Container Uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_update() {
        let mm = get_dev_env().await.unwrap();
        let metadata = ValueStore::new(None).array(
            "racks",
            vec![Value::from(json!({
                "name" : "Rack Uno"
            }))],
        );
        // Update name
        let id = LocationMetadataBmc::create(&mm, "Container Uno", &metadata.to_string())
            .await
            .unwrap();

        LocationMetadataBmc::update_name(&mm, id, "Hall Uno")
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id(), 1);
        assert_eq!(result.name(), "Hall Uno");

        let mm = get_dev_env().await.unwrap();

        let id = LocationMetadataBmc::create(&mm, "Container Uno", &metadata.to_string())
            .await
            .unwrap();

        let metadata = ValueStore::new(None).array(
            "racks",
            vec![Value::from(json!({
                "name" : "Cupboard Uno"
            }))],
        );
        LocationMetadataBmc::update_metadata(&mm, id, &metadata.to_string())
            .await
            .unwrap();

        let result = LocationMetadataBmc::get(&mm, id).await.unwrap();

        assert_ne!(result.id(), 1);

        // FIXME : Get better API for getting data
        let name = result
            .metadata()
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
            .as_str()
            .unwrap();

        assert_eq!(name, "Cupboard Uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete() {
        let mm = get_dev_env().await.unwrap();

        let metadata = ValueStore::new(None).array(
            "racks",
            vec![Value::from(json!({
                "name" : "Rack Uno"
            }))],
        );
        // Update name
        let id = LocationMetadataBmc::create(&mm, "Container Uno", &metadata.to_string())
            .await
            .unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 3);

        LocationMetadataBmc::delete(&mm, id).await.unwrap();

        let result = LocationMetadataBmc::get_all(&mm).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
