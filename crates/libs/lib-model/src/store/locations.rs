use crate::store::locations::types::{Location, LocationMetadata, RawLocationMetadata};
use crate::ModelManager;
use crate::Result;

pub mod types;

mod location_metadata;

// ```sql
// CREATE TABLE IF NOT EXISTS location_data (
//      id       INTEGER PRIMARY KEY AUTOINCREMENT,
//      location INTEGER REFERENCES location_metadata (id) ON DELETE CASCADE ON UPDATE CASCADE NOT NULL,
//      rack     TEXT,
//      bin      TEXT
//);
// ```
pub struct LocationsBmc;

impl LocationsBmc {
    pub async fn create(
        mm: &ModelManager,
        location: u32,
        rack: Option<&str>,
        bin: Option<&str>,
    ) -> Result<u32> {
        let db = mm.db();

        let result =
            sqlx::query("INSERT INTO location_data (location, rack, bin) VALUES ($1, $2, $3)")
                .bind(location)
                .bind(rack)
                .bind(bin)
                .execute(db)
                .await?
                .last_insert_rowid();

        Ok(result.try_into()?)
    }

    pub async fn get_location_metadata_for_id(
        mm: &ModelManager,
        id: u32,
    ) -> Option<LocationMetadata> {
        let db = mm.db();

        let result =
            sqlx::query_as::<_,RawLocationMetadata>("SELECT * FROM location_metadata WHERE id = (SELECT location FROM location_data WHERE id = $1)")
                .bind(id)
                .fetch_one(db)
                .await.ok()?;

        Some(result.into())
    }

    pub async fn get(mm: &ModelManager, id: u32) -> Option<Location> {
        let db = mm.db();

        sqlx::query_as::<_, Location>(
            "SELECT ld.id, ld.rack, ld.bin, lm.name as location FROM location_data ld JOIN location_metadata lm ON ld.location = lm.id WHERE ld.id = $1"
        )
            .bind(id)
            .fetch_optional(db)
            .await
            .ok()?
    }

    pub async fn update_location(mm: &ModelManager, id: u32, location: u32) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE location_data SET location = $1 WHERE id = $2")
            .bind(location)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn update_rack(mm: &ModelManager, id: u32, rack: Option<&str>) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE location_data SET rack = $1 WHERE id = $2")
            .bind(rack)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn update_bin(mm: &ModelManager, id: u32, bin: Option<&str>) -> Result<()> {
        let db = mm.db();

        sqlx::query("UPDATE location_data SET bin = $1 WHERE id = $2")
            .bind(bin)
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, id: u32) -> Result<()> {
        let db = mm.db();

        sqlx::query("DELETE FROM location_data WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::get_dev_env;
    use crate::store::locations::LocationsBmc;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get() {
        let mm = get_dev_env().await.unwrap();

        let result = LocationsBmc::get(&mm, 1).await.unwrap();

        assert_eq!(result.location(), "Container 1");
        assert!(matches!(result.rack(), Some(rack) if rack.eq("Rack 1")));

        let result = LocationsBmc::get(&mm, 2).await.unwrap();

        assert_eq!(result.location(), "Hall 1");
        assert!(result.rack().is_none())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_location_metadata_for_id() {
        let mm = get_dev_env().await.unwrap();

        let result = LocationsBmc::get_location_metadata_for_id(&mm, 1)
            .await
            .unwrap();

        assert_eq!(result.name(), "Container 1");
    }

    #[tokio::test]
    #[serial]
    async fn test_create() {
        let mm = get_dev_env().await.unwrap();

        let id = LocationsBmc::create(&mm, 2, None, Some("Bin 3"))
            .await
            .unwrap();

        let result = LocationsBmc::get(&mm, id).await.unwrap();

        assert!(matches!(result.bin(), Some(a) if a.eq("Bin 3")));
        assert!(result.rack().is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_update_rack() {
        let mm = get_dev_env().await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert!(result.rack().is_none());
        LocationsBmc::update_rack(&mm, 2, Some("Rack uno")).await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert!(result.rack().is_some());
        assert_eq!(result.rack().clone().unwrap(), "Rack uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_bin() {
        let mm = get_dev_env().await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert!(result.bin().is_none());
        LocationsBmc::update_bin(&mm, 2, Some("Bin uno")).await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert!(result.bin().is_some());
        assert_eq!(result.bin().clone().unwrap(), "Bin uno");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_location() {
        let mm = get_dev_env().await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert_eq!(result.location(),"Hall 1");
        LocationsBmc::update_location(&mm, 2,1).await.unwrap();
        let result = LocationsBmc::get(&mm,2).await.unwrap();
        assert_eq!(result.location(),"Container 1");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete() {
        let mm = get_dev_env().await.unwrap();

        LocationsBmc::delete(&mm,2).await.unwrap();

        let result = LocationsBmc::get(&mm,2).await;

        assert!(result.is_none());
    }
}
