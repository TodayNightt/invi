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

    #[derive(Debug,Clone)]
    pub struct GetAllRecordPayload {
        pub timeframe: Option<Timeframe>,
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

pub mod utils {
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