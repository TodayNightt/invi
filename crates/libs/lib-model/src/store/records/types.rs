use chrono::Utc;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::{Database, Decode, FromRow, Sqlite, Type};
use std::sync::Arc;

pub type Records = Arc<[Record]>;
#[derive(Debug,Copy, Clone, FromRow)]
pub struct Record {
    id : u32,
    item_id : u32,
    date: chrono::DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    correction: bool,
}


pub struct ItemRecords {
    item_id: u32,
    records: Arc<[ItemRecord]>,
}

impl ItemRecords {
    pub fn new(item_id: u32, records: Vec<ItemRecord>) -> Self {
        ItemRecords {
            item_id,
            records: records.into(),
        }
    }

    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    pub fn records(&self) -> Arc<[ItemRecord]> {
        self.records.clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct _Record {
    id: u32,
    date: chrono::DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    correction: bool,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TransactionType {
    In,
    Out,
}

impl<'r, DB: Database> Decode<'r, DB> for TransactionType
where
    bool: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let value = <bool as Decode<DB>>::decode(value)?;

        match value {
            false => Ok(TransactionType::In),
            true => Ok(TransactionType::Out),
        }
    }
}

impl<DB> Type<DB> for TransactionType
where
    DB: Database,
    bool: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <bool as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <bool as Type<DB>>::compatible(ty)
    }
}

impl TransactionType {
    pub fn opposite(&self) -> Self {
        match self {
            TransactionType::In => TransactionType::Out,
            TransactionType::Out => TransactionType::In,
        }
    }
}

impl From<TransactionType> for bool {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::In => false,
            TransactionType::Out => true,
        }
    }
}

impl From<bool> for TransactionType {
    fn from(value: bool) -> Self {
        match value {
            false => TransactionType::In,
            true => TransactionType::Out,
        }
    }
}

#[derive(Debug, Copy, Clone, sqlx::FromRow)]
pub struct ItemRecord {
    id: u32,
    date: chrono::DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    correction: bool,
}
impl ItemRecord {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn date(&self) -> chrono::DateTime<Utc> {
        self.date
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn correction(&self) -> bool {
        self.correction
    }
}
