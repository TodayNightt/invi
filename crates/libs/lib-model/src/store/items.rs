use crate::store::items::types::{Item, RawItem};
use crate::{Error, ModelManager, Result};
use sqlx::encode::IsNull::No;

pub mod types;

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
        image_data: i32,
        location: u32,
    ) -> Result<u32> {
        // Create an item
        let db = mm.db();

        let id: i64 = sqlx::query(
            "INSERT INTO items (name, item_metadata, location, image) VALUES ($1, $2, $3, $4)",
        )
        .bind(name)
        .bind(metadata)
        .bind(location)
        .bind(image_data)
        .execute(db)
        .await
        .map_err(|err| match err {
            sqlx::error::Error::RowNotFound => {
                Error::QueryError(format!("Failed to create item: {}", name))
            }
            _ => err.into(),
        })?
        .last_insert_rowid();

        Ok(id.try_into()?)
    }

    pub async fn get_raw(mm: &ModelManager, item_id: u32) -> Option<RawItem> {
        let db = mm.db();

        // Read an item by ID
        sqlx::query_as::<_, RawItem>(
            "SELECT i.id, i.name, i.item_metadata, im.key, i.location FROM items i JOIN image im ON i.image = im.id WHERE i.id = $1")
            .bind(item_id)
            .fetch_optional(db)
            .await
            .ok()?
    }

    pub async fn get(mm: &ModelManager, item_id: u32) -> Option<Item> {
        let result = ItemsBmc::get_raw(mm, item_id).await?;

        result.try_into().ok()
    }

    pub async fn update_name(mm: &ModelManager, item_id: u32, updated_name: &str) -> Option<()> {
        let db = mm.db();

        let result = sqlx::query("UPDATE items SET name = $1 WHERE id = $2")
            .bind(updated_name)
            .bind(item_id)
            .execute(db)
            .await
            .ok()?
            .rows_affected();

        if result.lt(&1) {
            return None;
        }

        Some(())
    }

    pub async fn update_metadata(mm: &ModelManager, item_id: u32, metadata: &str) -> Option<()> {
        let db = mm.db();

        let result = sqlx::query("UPDATE items SET item_metadata = $1 WHERE id = $2")
            .bind(metadata)
            .bind(item_id)
            .execute(db)
            .await
            .ok()?
            .rows_affected();

        if result.lt(&1) {
            return None;
        }

        Some(())
    }

    pub async fn delete(mm: &ModelManager, item_id: u32) -> Option<u32> {
        let db = mm.db();

        // Delete an item by ID
        let rows_affected = sqlx::query("DELETE FROM items WHERE id = $1")
            .bind(item_id)
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
    use crate::_dev_utils::get_dev_env;
    use crate::store::items::ItemsBmc;
    use lib_schema::ValueStore;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_item_get() {
        let mm = get_dev_env().await.unwrap();

        let item = ItemsBmc::get(&mm, 1).await.unwrap();

        assert_eq!(item.id(), 1);

        assert_eq!(item.name(), "Item 1");
    }

    #[tokio::test]
    #[serial]
    async fn test_item_create_and_delete() {
        let mm = get_dev_env().await.unwrap();

        let name = "TestItem2";

        let metadata = ValueStore::new(None).string("test_key", "test_value".to_string());

        let location = 1;

        let image_data = 1;

        let id = ItemsBmc::create(&mm, name, &metadata.to_string(), image_data, location)
            .await
            .unwrap();

        let item = ItemsBmc::get_raw(&mm, id).await.unwrap();

        assert_eq!(item.id(), id);

        assert_eq!(item.name(), "TestItem2");

        let metadata: ValueStore = item.metadata().try_into().unwrap();

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

        assert_eq!(result_item.name(), updated_name);

        // Clean up
        ItemsBmc::delete(&mm, id).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_item_update_metadata() {
        let mm = get_dev_env().await.unwrap();

        let name = "TestItem2";

        let metadata = ValueStore::new(None).string("test_key", "test_value".to_string());

        let id = ItemsBmc::create(&mm, name, &metadata.to_string(), 1, 1)
            .await
            .unwrap();

        let updated_metadata = ValueStore::new(Some("TestSchema".to_string()))
            .string("a", "this is a string".to_string())
            .number("b", 10);

        ItemsBmc::update_metadata(&mm, id, &updated_metadata.to_string())
            .await
            .unwrap();

        let result_item = ItemsBmc::get_raw(&mm, id).await.unwrap();

        let metadata: ValueStore = result_item.metadata().try_into().unwrap();

        let metadata_value = metadata.get("a").unwrap();

        assert_eq!(metadata_value.as_str().unwrap(), "this is a string");

        ItemsBmc::delete(&mm, id).await.unwrap();
    }
}
