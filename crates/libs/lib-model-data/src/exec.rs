use crate::types::params::{
    GetAllRecordPayload, ItemDeletePayload, ItemEditPayload, ItemGetPayload, ItemImagePayload,
    ItemRecordDeletePayload, ItemRecordGetPayload, ItemRecordRegisterPayload,
    ItemRecordUpdatePayload, ItemRegisterPayload, LocationMetadataDeletePayload,
    LocationMetadataGetPayload, LocationMetadataUpdatePayload, LocationMetadateRegisterPayload,
    LocationRegisterPayload,
};
use crate::types::{Item, Items, Location, Locations, Records, RecordsForItem};
use crate::store::image::ImageBmc;
use crate::store::items::ItemsBmc;
use crate::store::location_metadata::LocationMetadataBmc;
use crate::store::locations::LocationsBmc;
use crate::store::records::RecordsBmc;
use lib_model::{Error, ModelManager, Result};
use uuid::Uuid;

pub(crate) mod types;

fn store_image(mm: &ModelManager, data: Vec<u8>) -> Result<String> {
    let key = Uuid::now_v7().to_string();
    mm.image_store().store(&key, data)?;

    Ok(key)
}

async fn register_new_image_or_get_existing(
    mm: &ModelManager,
    image_payload: ItemImagePayload,
) -> Result<i64> {
    Ok(match image_payload {
        ItemImagePayload::New(data) => {
            let key = store_image(mm, data.to_vec())?;

            // Insert the image key into the database, and get the id
            ImageBmc::create(mm, &key).await?
        }
        ItemImagePayload::Existing(id) => id,
    })
}

// region : Item
pub async fn register_item(mm: &ModelManager, params: ItemRegisterPayload) -> Result<i64> {
    // Create the image first
    let image_id = register_new_image_or_get_existing(mm, params.image_data()).await?;

    // Create new location if needed
    let location_id = match params.location() {
        LocationRegisterPayload::New {
            location_metadata,
            rack,
            bin,
        } => LocationsBmc::create(mm, location_metadata, rack.as_deref(), bin.as_deref()).await?,
        LocationRegisterPayload::Existing(id) => id,
    };

    // Create the item entity.
    // Get the newly create item's id
    let metadata = params.metadata();
    let item_id = ItemsBmc::create(mm, params.name(), &metadata, image_id, location_id).await?;
    // return the id
    Ok(item_id)
}

// TODO : consider sanitize the return type for unwanted data
pub async fn get_items(mm: &ModelManager, params: ItemGetPayload) -> Result<Items> {
    let mut result: Vec<Item> = match params.pagination() {
        Some(pagination) => {
            ItemsBmc::get_from_range(mm, pagination.get_until_id(), pagination.quantity()).await?
        }
        None => ItemsBmc::get_all(mm).await?,
    }
    .into_iter()
    .map(|item| item.into())
    .collect();

    if params.with_image() {
        result.iter_mut().for_each(|item| {
            let key = item.image_key();
            let data = mm.image_store().get(key);

            if let Ok(data) = data {
                item.with_image_data(data.into());
            }
        })
    }

    Ok(result.into())
}

pub async fn edit_item(mm: &ModelManager, params: ItemEditPayload) -> Result<()> {
    if let Some(metadata) = params.metadata() {
        ItemsBmc::update_metadata(mm, params.id(), &metadata.to_string())
            .await
            .ok_or(Error::ItemNotFound(params.id()))?;
    }

    if let Some(name) = params.name() {
        ItemsBmc::update_name(mm, params.id(), name)
            .await
            .ok_or(Error::ItemNotFound(params.id()))?;
    }

    if let Some(location) = params.location() {
        // Get the location id
        let item = ItemsBmc::get(mm, params.id())
            .await
            .ok_or(Error::ItemNotFound(params.id()))?;

        let location_id = item.location;

        // Update the location
        LocationsBmc::update(
            mm,
            location_id,
            location.rack().as_ref().map(|i| i.as_deref()),
            location.bin().as_ref().map(|i| i.as_deref()),
        )
        .await?;
    }

    if let Some(image) = params.image() {
        let id = register_new_image_or_get_existing(mm, image.clone()).await?;

        ItemsBmc::update_image(mm, params.id(), id)
            .await
            .ok_or(Error::ItemNotFound(id))?;
    }

    Ok(())
}

pub async fn remove_item(mm: &ModelManager, params: ItemDeletePayload) -> Result<()> {
    ItemsBmc::delete(mm, params.id())
        .await
        .ok_or(Error::ItemNotFound(params.id()))?;

    Ok(())
}

// endregion

// region : Record
pub async fn get_records_for_item(
    mm: &ModelManager,
    params: ItemRecordGetPayload,
) -> Result<RecordsForItem> {
    let result = match params.timeframe() {
        Some(timeframe) => {
            RecordsBmc::get_in_timeframe_for_item(
                mm,
                params.id(),
                timeframe.start_timestamp(),
                timeframe.end_timestamp(),
            )
            .await?
        }
        None => RecordsBmc::get_all_for_item(mm, params.id()).await?,
    };

    Ok(result.into())
}

pub async fn register_record(mm: &ModelManager, params: ItemRecordRegisterPayload) -> Result<i64> {
    let id = RecordsBmc::create(
        mm,
        params.item_id(),
        params.date(),
        params.transaction_type(),
        params.quantity(),
        None,
    )
    .await?;
    Ok(id)
}

pub async fn update_record(mm: &ModelManager, params: ItemRecordUpdatePayload) -> Result<()> {
    RecordsBmc::update(mm, params.item_id(), params.record_id(), params.quantity()).await?;
    Ok(())
}

pub async fn delete_record(mm: &ModelManager, params: ItemRecordDeletePayload) -> Result<()> {
    RecordsBmc::delete(mm, params.id(), params.item_id()).await?;
    Ok(())
}

pub async fn get_records_for_all(
    mm: &ModelManager,
    params: GetAllRecordPayload,
) -> Result<Records> {
    let result = match params.timeframe() {
        Some(timeframe) => {
            RecordsBmc::get_in_timeframe(mm, timeframe.start_timestamp(), timeframe.end_timestamp())
                .await?
        }
        None => RecordsBmc::get_all(mm).await?,
    }
    .into_iter()
    .map(|item| item.into())
    .collect();

    Ok(result)
}
// endregion

// region : Location Metadata
pub async fn register_new_location(
    mm: &ModelManager,
    params: LocationMetadateRegisterPayload,
) -> Result<i64> {
    let id =
        LocationMetadataBmc::create(mm, params.name(), params.metadata_as_str().as_deref()).await?;

    Ok(id)
}

pub async fn get_location(
    mm: &ModelManager,
    params: LocationMetadataGetPayload,
) -> Result<Location> {
    let result = LocationMetadataBmc::get(mm, params.id())
        .await
        .ok_or(Error::LocationMetadataNotFound(params.id()))?;

    Ok(result.into())
}

pub async fn list_location(mm: &ModelManager) -> Result<Locations> {
    let result: Vec<Location> = LocationMetadataBmc::get_all(mm)
        .await?
        .into_iter()
        .map(|i| i.into())
        .collect();

    Ok(result.into())
}

pub async fn edit_location(mm: &ModelManager, params: LocationMetadataUpdatePayload) -> Result<()> {
    if let Some(name) = params.name() {
        LocationMetadataBmc::update_name(mm, params.id(), name).await?;
    }

    if let Some(metadata) = params.metadata() {
        LocationMetadataBmc::update_metadata(mm, params.id(), metadata.as_deref()).await?;
    }

    Ok(())
}

#[allow(unused)]
pub async fn remove_location(
    mm: &ModelManager,
    params: LocationMetadataDeletePayload,
) -> Result<()> {
    LocationMetadataBmc::delete(mm, params.id()).await?;
    Ok(())
}

// endregion

#[cfg(test)]
mod tests {
    use crate::exec::register_item;
    use crate::exec::store_image;
    use crate::types::params::{
        ItemImagePayload, ItemRegisterPayload, LocationRegisterPayload,
    };
    use crate::store::items::ItemsBmc;
    use lib_commons::ValueStore;
    use lib_model::_dev_utils::get_dev_env;
    use serde_json::json;
    use serial_test::serial;
    use std::fs;
    use uuid::{Uuid, Version};

    #[tokio::test]
    #[serial]
    async fn test_store_image() {
        let mm = get_dev_env().await.unwrap();

        let image_url = env!("TESTING_IMAGE");
        let image = fs::read(image_url).unwrap();

        let key = store_image(&mm, image).unwrap();

        let id = Uuid::parse_str(&key).unwrap();

        let ver = id.get_version().unwrap();
        assert_eq!(ver, Version::SortRand);

        // Cleanup
        mm.image_store().remove_last().unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_register_item() {
        let mm = get_dev_env().await.unwrap();

        // TODO : More dx friendly interface
        let metadata = ValueStore::new(None)
            .with_values(json!({"h" :"Hello", "i" : 124}))
            .unwrap();

        // TODO : Create more robust and dx friendly interface for struct creation
        let item_register_payload = ItemRegisterPayload::new(
            "Hello",
            metadata,
            ItemImagePayload::New(b"bjkd".to_vec().into()),
            LocationRegisterPayload::Existing(1),
        );

        let id = register_item(&mm, item_register_payload).await.unwrap();

        let item = ItemsBmc::get(&mm, id).await.unwrap();

        assert_eq!(item.name, "Hello");

        let image = mm.image_store().get(&item.image).unwrap();

        assert_eq!(image, b"bjkd");

        //Cleanup
        mm.image_store().remove_last().unwrap();
    }
}
