use crate::exec::types::params::{
    GetAllRecordPayload, ItemDeletePayload, ItemEditPayload, ItemGetPayload, ItemImagePayload,
    ItemRecordDeletePayload, ItemRecordGetPayload, ItemRecordRegisterPayload,
    ItemRecordUpdatePayload, ItemRegisterPayload, LocationMetadataDeletePayload,
    LocationMetadataGetPayload, LocationMetadataUpdatePayload, LocationMetadateRegisterPayload,
    LocationRegisterPayload,
};
use crate::exec::types::{Item, Items, Location, Locations, Records, RecordsForItem};
use crate::store::image::ImageBmc;
use crate::store::items::ItemsBmc;
use crate::store::location_metadata::LocationMetadataBmc;
use crate::store::locations::LocationsBmc;
use crate::store::records::RecordsBmc;
use lib_model::{Error, ModelManager, Result};
use uuid::Uuid;

pub mod types {
    use crate::store::items::RawItem;
    use crate::store::location_metadata::RawLocationMetadata;
    use crate::store::records::{RawRecord, TransactionType};
    use chrono::{DateTime, Utc};
    use lib_commons::ValueStore;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    pub mod params {
        use crate::exec::types::utils::{Pagination, Timeframe};
        use crate::store::records::TransactionType;
        use chrono::Utc;
        use lib_commons::ValueStore;
        use std::sync::Arc;

        pub struct ItemRegisterPayload {
            name: String,
            metadata: ValueStore,
            image_data: ItemImagePayload,
            location: LocationRegisterPayload,
        }

        impl ItemRegisterPayload {
            pub fn new(
                name: impl Into<String>,
                metadata: impl Into<ValueStore>,
                image_data: ItemImagePayload,
                location: LocationRegisterPayload,
            ) -> Self {
                ItemRegisterPayload {
                    name: name.into(),
                    metadata: metadata.into(),
                    image_data,
                    location,
                }
            }
            pub fn image_data(&self) -> ItemImagePayload {
                self.image_data.clone()
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn location(&self) -> LocationRegisterPayload {
                self.location.clone()
            }

            pub fn metadata(&self) -> String {
                self.metadata.to_string()
            }
        }

        #[derive(Debug, Clone)]
        pub enum ItemImagePayload {
            Existing(i64),
            New(Arc<[u8]>),
        }

        #[derive(Debug, Clone)]
        pub enum LocationRegisterPayload {
            Existing(i64),
            New {
                location_metadata: i64,
                // TODO : This might change
                rack: Option<String>,
                bin: Option<String>,
            },
        }

        pub struct ItemGetPayload {
            with_image: bool,
            pagination: Option<Pagination>,
        }

        impl ItemGetPayload {
            pub fn pagination(&self) -> &Option<Pagination> {
                &self.pagination
            }

            pub fn with_image(&self) -> bool {
                self.with_image
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemRecordGetPayload {
            id: i64,
            timeframe: Option<Timeframe>,
        }

        impl ItemRecordGetPayload {
            pub fn id(&self) -> i64 {
                self.id
            }
            pub fn timeframe(&self) -> &Option<Timeframe> {
                &self.timeframe
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemRecordRegisterPayload {
            item_id: i64,
            date: chrono::DateTime<Utc>,
            transaction_type: TransactionType,
            quantity: u32,
            adjustment_remarks: Option<i64>,
        }

        impl ItemRecordRegisterPayload {
            pub fn new(
                item_id: i64,
                date: chrono::DateTime<Utc>,
                transaction_type: TransactionType,
                quantity: u32,
                adjustment_remarks: Option<i64>,
            ) -> Self {
                ItemRecordRegisterPayload {
                    item_id,
                    date,
                    transaction_type,
                    quantity,
                    adjustment_remarks,
                }
            }

            pub fn item_id(&self) -> i64 {
                self.item_id
            }

            pub fn date(&self) -> i64 {
                self.date.timestamp()
            }

            pub fn transaction_type(&self) -> TransactionType {
                self.transaction_type
            }

            pub fn quantity(&self) -> u32 {
                self.quantity
            }

            pub fn adjustment_remarks(&self) -> Option<i64> {
                self.adjustment_remarks
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemRecordUpdatePayload {
            item_id: i64,
            record_id: i64,
            quantity: u32,
        }

        impl ItemRecordUpdatePayload {
            pub fn item_id(&self) -> i64 {
                self.item_id
            }

            pub fn record_id(&self) -> i64 {
                self.record_id
            }
            pub fn quantity(&self) -> u32 {
                self.quantity
            }
        }

        pub struct ItemRecordDeletePayload {
            id: i64,
            item_id: i64,
        }

        impl ItemRecordDeletePayload {
            pub fn id(&self) -> i64 {
                self.id
            }

            pub fn item_id(&self) -> i64 {
                self.item_id
            }
        }

        pub struct GetAllRecordPayload {
            timeframe: Option<Timeframe>,
        }

        impl GetAllRecordPayload {
            pub fn timeframe(&self) -> &Option<Timeframe> {
                &self.timeframe
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemLocationEditPayload {
            rack: Option<Option<String>>,
            bin: Option<Option<String>>,
        }

        impl ItemLocationEditPayload {
            pub fn rack(&self) -> &Option<Option<String>> {
                &self.rack
            }

            pub fn bin(&self) -> &Option<Option<String>> {
                &self.bin
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemEditPayload {
            id: i64,
            name: Option<String>,
            metadata: Option<ValueStore>,
            image: Option<ItemImagePayload>,
            location: Option<ItemLocationEditPayload>,
        }

        impl ItemEditPayload {
            pub fn id(&self) -> i64 {
                self.id
            }

            pub fn metadata(&self) -> &Option<ValueStore> {
                &self.metadata
            }

            pub fn image(&self) -> &Option<ItemImagePayload> {
                &self.image
            }

            pub fn location(&self) -> &Option<ItemLocationEditPayload> {
                &self.location
            }

            pub fn name(&self) -> &Option<String> {
                &self.name
            }
        }

        #[derive(Debug, Clone)]
        pub struct ItemDeletePayload {
            id: i64,
        }

        impl ItemDeletePayload {
            pub fn id(&self) -> i64 {
                self.id
            }
        }

        pub struct LocationMetadateRegisterPayload {
            name: String,
            metadata: Option<ValueStore>,
        }

        impl LocationMetadateRegisterPayload {
            pub fn name(&self) -> &str {
                self.name.as_ref()
            }

            pub fn metadata_as_str(&self) -> Option<String> {
                self.metadata.as_ref().map(|m| m.to_string())
            }
        }

        pub struct LocationMetadataGetPayload {
            id: i64,
        }

        impl LocationMetadataGetPayload {
            pub fn id(&self) -> i64 {
                self.id
            }
        }

        pub struct LocationMetadataUpdatePayload {
            id: i64,
            name: Option<String>,
            metadata: Option<Option<ValueStore>>,
        }

        impl LocationMetadataUpdatePayload {
            pub fn id(&self) -> i64 {
                self.id
            }

            pub fn name(&self) -> Option<&str> {
                self.name.as_deref()
            }

            pub fn metadata(&self) -> Option<Option<String>> {
                self.metadata
                    .as_ref()
                    .map(|i| i.as_ref().map(|m| m.to_string()))
            }
        }

        pub struct LocationMetadataDeletePayload {
            id: i64,
        }

        impl LocationMetadataDeletePayload {
            pub fn id(&self) -> i64 {
                self.id
            }
        }
    }

    mod utils {
        use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
        use std::num::NonZeroU32;

        pub struct YearMonth(pub i32, pub u32);

        pub struct Pagination {
            get_until_id: i64,
            quantity: u32,
        }

        impl Pagination {
            pub fn get_until_id(&self) -> i64 {
                self.get_until_id
            }

            pub fn quantity(&self) -> u32 {
                self.quantity
            }
        }

        #[derive(Debug, Clone)]
        pub struct Timeframe {
            start: chrono::DateTime<Utc>,
            end: chrono::DateTime<Utc>,
        }

        impl Timeframe {
            pub fn start_timestamp(&self) -> i64 {
                self.start.timestamp()
            }

            pub fn end_timestamp(&self) -> i64 {
                self.end.timestamp()
            }

            pub fn for_month(year_month: YearMonth) -> Option<Self> {
                let year = year_month.0;
                let month = NonZeroU32::new(year_month.1)?.get();

                let first_day = NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(year, month, 1)?,
                    NaiveTime::from_hms_opt(0, 0, 0)?,
                )
                .and_local_timezone(Local)
                .single()?;

                let next_month = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, 1)?
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1)?
                };

                let last_day = next_month
                    .pred_opt()?
                    .and_time(NaiveTime::from_hms_opt(23, 59, 59)?)
                    .and_local_timezone(Local)
                    .single()?;

                let mapped_first_day = first_day.to_utc();

                let mapped_last_day = last_day.to_utc();

                Some(Timeframe {
                    start: mapped_first_day,
                    end: mapped_last_day,
                })
            }
        }
    }

    pub struct RecordsForItem {
        item_id: i64,
        records: Arc<[RecordForItem]>,
    }

    pub struct RecordForItem {
        id: i64,
        date: DateTime<Utc>,
        transaction_type: TransactionType,
        quantity: u32,
        adjustment_remarks: Option<i64>,
    }

    impl From<Vec<RawRecord>> for RecordsForItem {
        fn from(value: Vec<RawRecord>) -> Self {
            let item_id = value.first().unwrap().item_id;

            let records = value.into_iter().map(|r| r.into()).collect();

            RecordsForItem { item_id, records }
        }
    }

    impl From<RawRecord> for RecordForItem {
        fn from(value: RawRecord) -> Self {
            RecordForItem {
                id: value.id,
                date: value.date,
                transaction_type: value.transaction_type,
                quantity: value.quantity,
                adjustment_remarks: value.adjustment_remarks,
            }
        }
    }

    pub type Items = Arc<[Item]>;

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Item {
        id: i64,
        metadata: ValueStore,
        image_data: Option<Arc<[u8]>>,
        image_key: String,
        location_name: Option<String>,
        location_id: i64,
    }

    impl Item {
        pub fn id(&self) -> i64 {
            self.id
        }

        pub fn metadata(&self) -> &ValueStore {
            &self.metadata
        }

        pub fn image_data(&self) -> &Option<Arc<[u8]>> {
            &self.image_data
        }

        pub fn with_image_data(&mut self, data: Arc<[u8]>) {
            self.image_data = Some(data);
        }

        pub fn image_key(&self) -> &str {
            &self.image_key
        }

        pub fn location_name(&self) -> &Option<String> {
            &self.location_name
        }

        pub fn location_id(&self) -> i64 {
            self.location_id
        }
    }

    impl From<RawItem> for Item {
        fn from(value: RawItem) -> Self {
            Item {
                id: value.id,
                metadata: value.item_metadata.0,
                image_key: value.image,
                location_id: value.location,
                ..Default::default()
            }
        }
    }

    pub type Records = Arc<[Record]>;

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone)]
    pub struct Record {
        id: i64,
        item_id: i64,
        date: DateTime<Utc>,
        transaction_type: TransactionType,
        quantity: u32,
        total: u32,
        adjustment_remarks: Option<i64>,
    }

    impl Record {
        pub fn id(&self) -> i64 {
            self.id
        }

        pub fn item_id(&self) -> i64 {
            self.item_id
        }

        pub fn date(&self) -> DateTime<Utc> {
            self.date
        }

        pub fn transaction_type(&self) -> TransactionType {
            self.transaction_type
        }

        pub fn quantity(&self) -> u32 {
            self.quantity
        }

        pub fn total(&self) -> u32 {
            self.total
        }

        pub fn adjustment_remarks(&self) -> Option<i64> {
            self.adjustment_remarks
        }
    }

    impl From<RawRecord> for Record {
        fn from(value: RawRecord) -> Self {
            Record {
                id: value.id,
                item_id: value.item_id,
                date: value.date,
                transaction_type: value.transaction_type,
                quantity: value.quantity,
                total: value.total,
                adjustment_remarks: value.adjustment_remarks,
            }
        }
    }

    pub type Locations = Arc<[Location]>;

    pub struct Location {
        id: i64,
        name: String,
        metadata: Option<ValueStore>,
    }

    impl Location {
        pub fn id(&self) -> i64 {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn metadata(&self) -> &Option<ValueStore> {
            &self.metadata
        }
    }

    impl From<RawLocationMetadata> for Location {
        fn from(value: RawLocationMetadata) -> Self {
            Location {
                id: value.id,
                name: value.name,
                metadata: value.metadata.map(|m| m.0),
            }
        }
    }
}

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
    use crate::exec::types::params::{
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
