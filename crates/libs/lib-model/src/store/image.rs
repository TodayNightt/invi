use crate::ModelManager;
use crate::store::image::types::ImageKey;
use crate::{Error, Result};

pub(crate) mod types;

pub(crate) struct ImageBmc;

impl ImageBmc {
    pub async fn create(mm: &ModelManager, path: impl AsRef<str>) -> Result<u32> {
        let db = mm.db();

        let result = sqlx::query("INSERT INTO image (key) VALUES ($1)")
            .bind(path.as_ref())
            .execute(db)
            .await?
            .last_insert_rowid();

        Ok(result.try_into()?)
    }

    pub async fn get(mm: &ModelManager, id: u32) -> Option<ImageKey> {
        let db = mm.db();

        sqlx::query_as::<_, ImageKey>("SELECT key FROM image WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await
            .ok()?
    }

    pub async fn update(mm: &ModelManager, id: u32, key: impl AsRef<str>) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE image SET key = $1 WHERE id = $2")
            .bind(key.as_ref())
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, id: u32) -> Result<()> {
        let db = mm.db();

        sqlx::query("DELETE FROM image WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::get_dev_env;
    use crate::Error::ImageNotFound;
    use crate::store::image::ImageBmc;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get() {
        let mm = get_dev_env().await.unwrap();

        let result = ImageBmc::get(&mm, 1).await.unwrap();

        assert_eq!(result.key(), "abd0031")
    }

    #[tokio::test]
    #[serial]
    async fn test_create() {
        let mm = get_dev_env().await.unwrap();

        let key = "hellojason";

        let id = ImageBmc::create(&mm, key).await.unwrap();

        let result = ImageBmc::get(&mm, id).await.unwrap();

        assert_eq!(result.key(), "hellojason");
    }

    #[tokio::test]
    #[serial]
    async fn test_update() {
        let mm = get_dev_env().await.unwrap();

        let result = ImageBmc::get(&mm, 3).await.unwrap();

        ImageBmc::update(&mm, 3, "hello_wakana").await.unwrap();

        let updated = ImageBmc::get(&mm, 3).await.unwrap();

        assert_ne!(result.key(), updated.key());
        assert_eq!(updated.key(), "hello_wakana");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete() {
        let mm = get_dev_env().await.unwrap();

        ImageBmc::delete(&mm, 3).await.unwrap();

        let result = ImageBmc::get(&mm,3).await;
        
        assert!(result.is_none());
    }
}
