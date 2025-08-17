use lib_commons::ValueStore;
use lib_model::{Error, ModelManager, Result};
use sqlx::types::Json;

#[derive(Debug, sqlx::FromRow)]
pub struct RawItem {
    pub id: i64,
    pub location: i64,
    pub name: String,
    pub item_metadata: Json<ValueStore>,
    pub image: String,
}

/// ```sql
//    /table items{
//     /id: integer (Primary Key),
//    / name : text ,
//   /  item_metadata : text (JSON_OBJECT),
//  /   location : integer (Foreign Key from locations),
// /    image_data : integer (Foreign Key from images),
/// }
/// ```
pub(crate) struct ItemsBmc;

impl ItemsBmc {
    // Implement CRUD operations
    pub async fn create(
        mm: &ModelManager,
        name: &str,
        metadata: &str,
        image_data: i64,
        location: i64,
    ) -> Result<i64> {
        // Create an item
        let db = mm.db();

        let id = sqlx::query!(
            "INSERT INTO items (name, item_metadata, location, image) VALUES ($1, $2, $3, $4)",
            name,
            metadata,
            location,
            image_data
        )
        .execute(db)
        .await
        .map_err(|err| match err {
            sqlx::error::Error::RowNotFound => {
                Error::QueryError(format!("Failed to create item: {}", name))
            }
            _ => err.into(),
        })?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_from_range(
        mm: &ModelManager,
        until_id: i64,
        limit: u32,
    ) -> Result<Vec<RawItem>> {
        let db = mm.db();

        let result = sqlx::query_as!(
            RawItem,
            r#"SELECT i.id, i.name, i.item_metadata as "item_metadata: Json<ValueStore>", im.key as image, i.location
                FROM items i JOIN image im ON i.image = im.id
                WHERE i.id > $1 ORDER BY i.id LIMIT $2"#,
            until_id, limit
        )
            .fetch_all(db)
            .await?;

        Ok(result)
    }

    pub async fn get(mm: &ModelManager, item_id: i64) -> Option<RawItem> {
        let db = mm.db();

        // Read an item by ID
        sqlx::query_as!(
            RawItem,
            r#"SELECT i.id, i.name, i.item_metadata as "item_metadata: Json<ValueStore>", im.key as image, i.location
                FROM items i JOIN image im ON i.image = im.id
                WHERE i.id = $1"#,
            item_id
        )
            .fetch_optional(db)
            .await
            .ok()?
    }

    pub async fn get_all(mm: &ModelManager) -> Result<Vec<RawItem>> {
        let db = mm.db();

        let result = sqlx::query_as!(
            RawItem,
            r#"SELECT i.id, i.name, i.item_metadata as "item_metadata: Json<ValueStore>", im.key as image, i.location
                FROM items i JOIN image im ON i.image = im.id"#
        )
            .fetch_all(db).await?;

        Ok(result)
    }

    pub async fn update_name(mm: &ModelManager, item_id: i64, updated_name: &str) -> Option<()> {
        let db = mm.db();

        let result = sqlx::query!(
            "UPDATE items SET name = $1 WHERE id = $2",
            updated_name,
            item_id
        )
        .execute(db)
        .await
        .ok()?
        .rows_affected();

        if result.lt(&1) {
            return None;
        }

        Some(())
    }

    pub async fn update_metadata(mm: &ModelManager, item_id: i64, metadata: &str) -> Option<()> {
        let db = mm.db();

        let result = sqlx::query!(
            "UPDATE items SET item_metadata = $1 WHERE id = $2",
            metadata,
            item_id
        )
        .execute(db)
        .await
        .ok()?
        .rows_affected();

        if result.lt(&1) {
            return None;
        }

        Some(())
    }

    pub async fn update_image(mm: &ModelManager, item_id: i64, updated_image: i64) -> Option<()> {
        let db = mm.db();

        let result = sqlx::query!(
            "UPDATE items SET image = $1 WHERE id = $2",
            updated_image,
            item_id
        )
        .execute(db)
        .await
        .ok()?
        .rows_affected();

        if result.lt(&1) {
            return None;
        }

        Some(())
    }

    pub async fn delete(mm: &ModelManager, item_id: i64) -> Option<i64> {
        let db = mm.db();

        // Delete an item by ID
        let rows_affected = sqlx::query!("DELETE FROM items WHERE id = $1", item_id)
            .execute(db)
            .await
            .ok()?
            .rows_affected();

        if rows_affected.lt(&1) {
            return None;
        }

        Some(item_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::store::items::ItemsBmc;
    use lib_commons::ValueStore;
    use lib_model::_dev_utils::get_dev_env;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_item_get() {
        let mm = get_dev_env().await.unwrap();

        let item = ItemsBmc::get(&mm, 1).await.unwrap();

        assert_eq!(item.id, 1);

        assert_eq!(item.name, "Item 1");
    }

    #[tokio::test]
    #[serial]
    async fn test_item_create_and_delete() {
        let mm = get_dev_env().await.unwrap();

        let name = "TestItem2";

        let metadata = ValueStore::builder()
            .string("test_key", "test_value")
            .build();

        let location = 1;

        let image_data = 1;

        let id = ItemsBmc::create(&mm, name, &metadata.to_string(), image_data, location)
            .await
            .unwrap();

        let item = ItemsBmc::get(&mm, id).await.unwrap();

        assert_eq!(item.id, id);

        assert_eq!(item.name, "TestItem2");

        let metadata = item.item_metadata;

        let metadata = metadata.get("test_key").unwrap();

        assert_eq!(metadata.as_str().unwrap(), "test_value");

        let deleted_id = ItemsBmc::delete(&mm, id).await.unwrap();

        assert_eq!(deleted_id, id);
    }

    #[tokio::test]
    #[serial]
    async fn test_item_update_name() {
        let mm = get_dev_env().await.unwrap();

        let name = "TestItem2";
        let metadata = ValueStore::new(None);

        let id = ItemsBmc::create(&mm, name, &metadata.to_string(), 1, 1)
            .await
            .unwrap();

        let updated_name = "UpdatedItemName";

        ItemsBmc::update_name(&mm, id, updated_name).await.unwrap();

        let result_item = ItemsBmc::get(&mm, id).await.unwrap();

        assert_eq!(result_item.name, updated_name);

        // Clean up
        ItemsBmc::delete(&mm, id).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_item_update_metadata() {
        let mm = get_dev_env().await.unwrap();

        let name = "TestItem2";

        let metadata = ValueStore::builder()
            .string("test_key", "test_value")
            .build();

        let id = ItemsBmc::create(&mm, name, &metadata.to_string(), 1, 1)
            .await
            .unwrap();

        let updated_metadata = ValueStore::builder()
            .with_schema("TestSchema")
            .string("a", "this is a string")
            .number("b", 10)
            .build();

        ItemsBmc::update_metadata(&mm, id, &updated_metadata.to_string())
            .await
            .unwrap();

        let result_item = ItemsBmc::get(&mm, id).await.unwrap();

        let metadata = result_item.item_metadata;

        let metadata_value = metadata.get("a").unwrap();

        assert_eq!(metadata_value.as_str().unwrap(), "this is a string");

        ItemsBmc::delete(&mm, id).await.unwrap();
    }
}
